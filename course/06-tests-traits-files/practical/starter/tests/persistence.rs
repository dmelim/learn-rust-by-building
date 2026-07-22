use std::{fs, path::PathBuf};

use persistent_dispatch_starter::{Order, StoreError, load_records, save_records};

fn unique_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "learn-rust-starter-{label}-{}.txt",
        std::process::id()
    ))
}

#[test]
#[ignore = "implement record persistence"]
fn saves_and_loads_public_order_records() {
    let path = unique_path("round-trip");
    let orders = vec![Order {
        id: 42,
        customer: "Amina Nuru".to_owned(),
        priority: true,
    }];
    save_records(&path, &orders).unwrap();
    let loaded = load_records::<Order>(&path).unwrap();
    fs::remove_file(&path).unwrap();
    assert_eq!(loaded, orders);
}

#[test]
#[ignore = "report malformed record line numbers"]
fn malformed_record_reports_line_number() {
    let path = unique_path("bad-line");
    fs::write(&path, "1|Amina|false\nbad record\n").unwrap();
    let error = load_records::<Order>(&path).unwrap_err();
    fs::remove_file(&path).unwrap();
    assert!(matches!(error, StoreError::InvalidRecord { line: 2, .. }));
}
