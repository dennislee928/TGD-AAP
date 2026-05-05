#[path = "../src/data_engine/mod.rs"]
mod data_engine;

use std::fs;

use data_engine::cleaner::clean;
use data_engine::fetcher::RawRecord;
use data_engine::toon::{deserialize, serialize};

fn load_fixture(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be readable")
}

#[test]
fn cleaner_filters_and_normalizes_mixed_fixture() {
    let raw_json = load_fixture("tests/fixtures/raw_records_mixed.json");
    let expected_json = load_fixture("tests/fixtures/clean_records_expected.json");

    let raw: Vec<RawRecord> = serde_json::from_str(&raw_json).expect("valid raw fixture JSON");
    let cleaned = clean(raw).expect("clean should succeed");
    let expected: serde_json::Value =
        serde_json::from_str(&expected_json).expect("valid expected fixture JSON");
    let cleaned_value = serde_json::to_value(&cleaned).expect("serialize clean records to JSON");

    assert_eq!(cleaned_value, expected);
}

#[test]
fn toon_round_trip_with_cleaned_fixture_data() {
    let raw_json = load_fixture("tests/fixtures/raw_records_mixed.json");
    let raw: Vec<RawRecord> = serde_json::from_str(&raw_json).expect("valid raw fixture JSON");
    let cleaned = clean(raw).expect("clean should succeed");

    let bytes = serialize(&cleaned).expect("serialize should succeed");
    let decoded = deserialize(&bytes).expect("deserialize should succeed");

    assert_eq!(decoded.len(), cleaned.len());
    for (lhs, rhs) in decoded.iter().zip(cleaned.iter()) {
        assert_eq!(lhs.id, rhs.id);
        assert_eq!(lhs.name, rhs.name);
        assert!((lhs.value - rhs.value).abs() < f64::EPSILON);
    }
}

#[test]
fn toon_deserialize_rejects_truncated_header() {
    let err = deserialize(b"TOON").expect_err("truncated payload should fail");
    assert!(err.to_string().contains("too short"));
}
