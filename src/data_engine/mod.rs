//! data_engine/mod.rs — Data processing and Toon format serialization module.
//!
//! Responsible for:
//! - Fetching raw data from Taiwan Government Open Data APIs
//! - Cleaning and validating records
//! - Serializing structured data into the Toon format
//! - Uploading datasets to Hugging Face Hub

pub mod fetcher;
pub mod cleaner;
pub mod toon;
