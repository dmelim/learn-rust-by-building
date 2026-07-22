use std::{fs, path::PathBuf};

use dispatch_desk_solution::{DispatchDesk, Status};

fn unique_path() -> PathBuf {
    std::env::temp_dir().join(format!("dispatch-capstone-{}.txt", std::process::id()))
}

#[test]
fn complete_workflow_survives_persistence() {
    let path = unique_path();
    let mut desk = DispatchDesk::new();
    let id = desk
        .add(
            "Amina Nuru".to_owned(),
            "404 Test Route".to_owned(),
            12.0,
            30.0,
            10,
            true,
        )
        .unwrap();
    desk.dispatch(id, "Noor Rahman".to_owned()).unwrap();
    desk.save(&path).unwrap();

    let mut loaded = DispatchDesk::load(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert_eq!(loaded.orders()[0].id(), id);
    assert!(
        matches!(loaded.orders()[0].status(), Status::OutForDelivery { courier } if courier == "Noor Rahman")
    );
    loaded.deliver(id).unwrap();
    assert_eq!(loaded.orders()[0].status(), &Status::Delivered);
}
