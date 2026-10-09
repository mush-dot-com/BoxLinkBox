use boxlinkbox::{
    backup,
    domain::{Group, Library, Link},
    history::History,
    presentation::State,
};

#[test]
fn backup_replaces_existing_file_and_preserves_it_on_invalid_export() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("backup.json");
    std::fs::write(&path, b"old backup").unwrap();
    let data = Library::default();
    backup::export(&path, &data).unwrap();
    assert_eq!(backup::import(&path).unwrap(), data);
    let saved = std::fs::read(&path).unwrap();
    let mut invalid = data;
    invalid.version = 99;
    assert!(backup::export(&path, &invalid).is_err());
    assert_eq!(std::fs::read(&path).unwrap(), saved);
    assert_eq!(std::fs::read_dir(folder.path()).unwrap().count(), 1);
}

#[test]
fn failed_backup_commit_cleans_temporary_file_and_import_is_bounded() {
    let folder = tempfile::tempdir().unwrap();
    let target = folder.path().join("directory");
    std::fs::create_dir(&target).unwrap();
    assert!(backup::export(&target, &Library::default()).is_err());
    assert_eq!(std::fs::read_dir(folder.path()).unwrap().count(), 1);
    let oversized = folder.path().join("oversized.json");
    std::fs::File::create(&oversized)
        .unwrap()
        .set_len(backup::MAX_BACKUP_BYTES + 1)
        .unwrap();
    assert!(backup::import(&oversized).unwrap_err().contains("16MB"));
}

#[test]
fn history_respects_byte_and_count_limits_without_skipping_an_unretained_state() {
    let data = Library::default();
    let bytes = serde_json::to_vec(&data).unwrap().len();
    let mut history = History::with_limit(bytes * 2);
    for _ in 0..4 {
        history.push(data.clone());
    }
    history.pop();
    assert!(!history.is_empty());
    history.pop();
    assert!(history.is_empty());
    history.push(data.clone());
    let mut large = data.clone();
    large.groups.push(Group {
        id: 1,
        name: "a".repeat(200),
        links: vec![],
    });
    history.push(large);
    assert!(history.is_empty());
    let mut history = History::default();
    for _ in 0..31 {
        history.push(data.clone());
    }
    for _ in 0..30 {
        assert!(history.last().is_some());
        history.pop();
    }
    assert!(history.is_empty());
}

#[test]
fn browser_updates_do_not_reserialize_a_large_library() {
    let mut data = Library::default();
    data.groups.push(Group {
        id: 1,
        name: "Large".into(),
        links: (2..502)
            .map(|id| Link {
                id,
                title: "Example".into(),
                url: format!("https://example.com/{id}"),
                note: "x".repeat(1000),
            })
            .collect(),
    });
    let full = serde_json::to_vec(&State {
        kind: "state",
        library: Some(&data),
        tabs: Vec::<u64>::new(),
        active: None,
        browsing: false,
        undo: false,
        data_directory: "test",
    })
    .unwrap();
    let update = serde_json::to_vec(&State {
        kind: "state",
        library: None,
        tabs: vec![1_u64],
        active: Some(1),
        browsing: true,
        undo: false,
        data_directory: "test",
    })
    .unwrap();
    assert!(full.len() > 500_000);
    assert!(update.len() < 256);
    assert!(!String::from_utf8(update).unwrap().contains("\"library\""));
}
