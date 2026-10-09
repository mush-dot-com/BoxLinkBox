use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Link {
    pub id: u64,
    pub title: String,
    pub url: String,
    #[serde(default)]
    pub note: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Group {
    pub id: u64,
    pub name: String,
    pub links: Vec<Link>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Library {
    pub version: u32,
    pub theme: String,
    pub groups: Vec<Group>,
}
impl Default for Library {
    fn default() -> Self {
        Self {
            version: 1,
            theme: "system".into(),
            groups: vec![],
        }
    }
}
pub fn normalize_url(value: &str) -> Result<String, String> {
    let value = value.trim();
    if value.len() > 8192 || value.is_empty() {
        return Err("URLは1〜8192文字で入力してください。".into());
    }
    let input = if value.contains("://") || value.contains(':') {
        value.to_owned()
    } else {
        format!("https://{value}")
    };
    let parsed = url::Url::parse(&input).map_err(|_| "URLの形式を確認してください。")?;
    if !matches!(parsed.scheme(), "http" | "https") || parsed.host_str().is_none() {
        return Err("http または https のURLを入力してください。".into());
    }
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err(
            "ID・パスワードを含むURLは保存できません。サイト上でログインしてください。".into(),
        );
    }
    Ok(parsed.to_string())
}
impl Library {
    pub fn next_id(&self) -> u64 {
        self.groups
            .iter()
            .flat_map(|g| std::iter::once(g.id).chain(g.links.iter().map(|l| l.id)))
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.version != 1 {
            return Err("対応していないバックアップ形式です。".into());
        }
        if !["light", "dark", "system"].contains(&self.theme.as_str()) {
            return Err("テーマの設定が不正です。".into());
        }
        if self.groups.len() > 200 {
            return Err("グループは200個までです。".into());
        }
        let mut ids = HashSet::new();
        for g in &self.groups {
            if g.id == 0
                || !ids.insert(g.id)
                || g.name.trim().is_empty()
                || g.name.len() > 240
                || g.links.len() > 500
            {
                return Err("グループ名・件数・IDを確認してください。".into());
            }
            let mut urls = HashSet::new();
            for l in &g.links {
                if l.id == 0
                    || !ids.insert(l.id)
                    || l.title.trim().is_empty()
                    || l.title.len() > 600
                    || l.note.len() > 6000
                {
                    return Err("URLの名前・メモ・IDを確認してください。".into());
                }
                let normalized = normalize_url(&l.url)?;
                if !urls.insert(normalized) {
                    return Err("同じグループにURLが重複しています。".into());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_https_and_explicit_local_port() {
        assert_eq!(
            normalize_url("https://example.com/a?q=1#section").unwrap(),
            "https://example.com/a?q=1#section"
        );
        assert_eq!(
            normalize_url("http://localhost:8123/").unwrap(),
            "http://localhost:8123/"
        );
    }

    #[test]
    fn enforces_group_count_and_global_ids() {
        let mut data = Library {
            groups: vec![
                Group {
                    id: 1,
                    name: "A".into(),
                    links: vec![],
                },
                Group {
                    id: 2,
                    name: "B".into(),
                    links: vec![],
                },
            ],
            ..Library::default()
        };
        data.groups[1].id = data.groups[0].id;
        assert!(data.validate().is_err());
        data.groups = (1..=200)
            .map(|id| Group {
                id,
                name: format!("Group {id}"),
                links: vec![],
            })
            .collect();
        assert!(data.validate().is_ok());
        data.groups.push(Group {
            id: 201,
            name: "overflow".into(),
            links: vec![],
        });
        assert!(data.validate().is_err());
    }

    #[test]
    fn new_library_has_no_default_groups() {
        let data = Library::default();
        assert!(data.groups.is_empty());
        assert_eq!(data.next_id(), 1);
        assert!(data.validate().is_ok());
    }
}
