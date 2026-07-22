use std::{fs, path::PathBuf};

use dispatch_desk_starter::DispatchDesk;

fn unique_path() -> PathBuf {
    std::env::temp_dir().join(format!(
        "dispatch-capstone-starter-{}.txt",
        std::process::id()
    ))
}

#[test]
#[ignore = "complete the capstone workflow"]
fn complete_workflow_survives_persistence() {
    let path = unique_path();
    let mut desk = DispatchDesk::new();
    let id = desk
        .add(
            "Amina".to_owned(),
            "404 Test Route".to_owned(),
            12.0,
            30.0,
            10,
            true,
        )
        .unwrap();
    desk.dispatch(id, "Noor".to_owned()).unwrap();
    desk.save(&path).unwrap();
    let loaded = DispatchDesk::load(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert_eq!(loaded.orders()[0].id(), id);
}
