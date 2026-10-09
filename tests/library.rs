use boxlinkbox::{
    domain::{Group, Library, Link, normalize_url},
    repository::Repository,
};

#[test]
fn url_validation() {
    assert_eq!(
        normalize_url(" example.com ").unwrap(),
        "https://example.com/"
    );
    for bad in [
        "",
        "javascript:alert(1)",
        "file:///C:/Windows",
        "https://user:secret@example.com",
        "https://",
        "data:text/html,x",
    ] {
        assert!(normalize_url(bad).is_err(), "{bad}");
    }
    assert!(normalize_url(&"x".repeat(8193)).is_err());
}
#[test]
fn duplicate_and_boundary_validation() {
    let mut data = Library::default();
    data.groups.push(Group {
        id: 1,
        name: "Test".into(),
        links: vec![],
    });
    data.groups[0].links.push(Link {
        id: 3,
        title: "Example".into(),
        url: "https://example.com".into(),
        note: "".into(),
    });
    assert!(data.validate().is_ok());
    let mut duplicate = data.groups[0].links[0].clone();
    duplicate.id = 4;
    data.groups[0].links.push(duplicate);
    assert!(data.validate().is_err());
    data.groups[0].links.pop();
    data.groups[0].name = "".into();
    assert!(data.validate().is_err());
}
#[test]
fn persistence_recovery_and_atomic_rejection() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("test.db");
    let mut repo = Repository::open(&path).unwrap();
    let first = Library::default();
    repo.save(&first).unwrap();
    let mut second = first.clone();
    second.groups.push(Group {
        id: 1,
        name: "研究".into(),
        links: vec![],
    });
    repo.save(&second).unwrap();
    assert_eq!(repo.recover().unwrap(), first);
    let mut bad = second.clone();
    bad.version = 2;
    assert!(repo.save(&bad).is_err());
    drop(repo);
    assert_eq!(Repository::open(&path).unwrap().load().unwrap(), second);
}

#[test]
fn invalid_primary_record_can_recover_last_good_data() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("recovery.db");
    let mut repo = Repository::open(&path).unwrap();
    let first = Library::default();
    repo.save(&first).unwrap();
    let mut second = first.clone();
    second.theme = "dark".into();
    repo.save(&second).unwrap();
    drop(repo);
    let connection = rusqlite::Connection::open(&path).unwrap();
    connection
        .execute("UPDATE library SET data='broken'", [])
        .unwrap();
    drop(connection);
    let repo = Repository::open(&path).unwrap();
    assert!(repo.load().is_err());
    assert_eq!(repo.recover().unwrap(), first);
}

#[test]
fn backup_roundtrip_and_unknown_version() {
    let data = Library::default();
    let json = serde_json::to_vec_pretty(&data).unwrap();
    let imported: Library = serde_json::from_slice(&json).unwrap();
    assert_eq!(imported, data);
    assert!(imported.validate().is_ok());
    assert!(serde_json::from_slice::<Library>(b"not json").is_err());
    let mut unknown = data;
    unknown.version = 99;
    assert!(unknown.validate().is_err());
}
