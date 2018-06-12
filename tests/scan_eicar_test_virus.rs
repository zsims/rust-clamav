extern crate clamav;
extern crate tempfile;

use std::io::Write;

use clamav::{default_database_directory, initialize, Engine, ScanResult, ScanSettingsBuilder};
use tempfile::NamedTempFile;

mod common;

#[test]
fn scan_using_system_databases() {
    common::setup();

    let mut test_file: NamedTempFile = NamedTempFile::new().unwrap();
    // Per http://www.eicar.org/86-0-Intended-use.html
    write!(test_file, r"X5O!P%@AP[4\PZX54(P^)7CC)7}}$EICAR").unwrap();
    write!(test_file, r"-STANDARD-ANTIVIRUS-TEST-FILE!$H+H*").unwrap();
    test_file.flush().unwrap();

    let scan_settings = ScanSettingsBuilder::new().build();

    initialize().expect("initialize failed");
    let engine = Engine::new();
    engine
        .load_databases(&default_database_directory())
        .expect("load failed");
    engine.compile().expect("compile failed");

    let result = engine
        .scan_file(test_file.path().to_str().unwrap(), &scan_settings)
        .unwrap();
    match result {
        ScanResult::Virus(name) => assert_eq!(name, "Eicar-Test-Signature"),
        _ => panic!("Expected test virust to be picked up as a virus"),
    }
}
