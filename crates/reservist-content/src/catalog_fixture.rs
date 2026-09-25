//! Temporary copies of the real catalog for mutation tests.

use crate::Issue;
use crate::catalog::validate_catalog;
use csv::ReaderBuilder;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_CASE: AtomicUsize = AtomicUsize::new(0);

pub(crate) fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

pub(crate) fn catalog_copy() -> (PathBuf, PathBuf) {
    let source_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let case = std::env::temp_dir().join(format!(
        "reservist-content-catalog-{}-{}",
        std::process::id(),
        NEXT_CASE.fetch_add(1, Ordering::Relaxed),
    ));
    let catalog = case.join("catalog");
    fs::create_dir_all(&catalog).unwrap();
    fs::copy(
        source_root.join("catalog/schema.json"),
        catalog.join("schema.json"),
    )
    .unwrap();
    copy_tree(
        &source_root.join("catalog/inventory"),
        &catalog.join("inventory"),
    );
    copy_tree(&source_root.join("assets"), &case.join("assets"));
    for entry in fs::read_dir(source_root.join("catalog")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|extension| extension == "csv") {
            fs::copy(&path, catalog.join(path.file_name().unwrap())).unwrap();
        }
    }
    (case, catalog)
}

pub(crate) fn has_category(errors: &[Issue], category: &str) -> bool {
    errors.iter().any(|issue| issue.category == category)
}

pub(crate) fn mutate_inventory(
    catalog: &Path,
    table: &str,
    key_field: &str,
    key: &str,
    field: &str,
    value: &str,
) {
    for path in inventory_files(catalog, table) {
        let mut reader = ReaderBuilder::new().from_path(&path).unwrap();
        let headers = reader.headers().unwrap().clone();
        let key_index = headers
            .iter()
            .position(|header| header == key_field)
            .unwrap();
        let field_index = headers.iter().position(|header| header == field).unwrap();
        let mut records = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
        if let Some(record) = records
            .iter_mut()
            .find(|record| record.get(key_index) == Some(key))
        {
            *record = record
                .iter()
                .enumerate()
                .map(|(index, original)| {
                    if index == field_index {
                        value
                    } else {
                        original
                    }
                })
                .collect();
            let mut writer = csv::WriterBuilder::new().from_path(&path).unwrap();
            writer.write_record(&headers).unwrap();
            for record in records {
                writer.write_record(&record).unwrap();
            }
            writer.flush().unwrap();
            return;
        }
    }
    panic!("missing {table} fixture row {key_field}={key}");
}

/// Every authored copy of `table` across the domain, profile, and scenario
/// partitions, in path order.
pub(crate) fn inventory_files(catalog: &Path, table: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(catalog.join("inventory")).unwrap() {
        let folder = entry.unwrap().path();
        if !folder.is_dir() {
            continue;
        }
        let nested = matches!(
            folder.file_name().and_then(|name| name.to_str()),
            Some("profiles" | "scenarios")
        );
        let partitions = if nested {
            fs::read_dir(&folder)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_dir())
                .collect()
        } else {
            vec![folder]
        };
        files.extend(
            partitions
                .into_iter()
                .map(|partition| partition.join(table))
                .filter(|path| path.is_file()),
        );
    }
    files.sort();
    files
}

pub(crate) fn assert_category(catalog: &Path, category: &str) {
    match validate_catalog(catalog) {
        Err(errors) if has_category(&errors, category) => {}
        Err(errors) => panic!("expected {category}; got {errors:?}"),
        Ok(_) => panic!("expected {category} failure"),
    }
}

pub(crate) fn mutation_case(category: &str, mutation: impl FnOnce(&Path)) {
    let (case, catalog) = catalog_copy();
    mutation(&catalog);
    assert_category(&catalog, category);
    fs::remove_dir_all(case).unwrap();
}
