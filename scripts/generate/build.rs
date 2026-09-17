//! Finds the `comfy-table` source that Cargo fetched, and the part of
//! `crossterm` it styles with, for `main.rs` to fingerprint: `src/` ports them
//! by hand.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut sources = String::new();
    let dir = upstream_dir("comfy-table");
    println!("cargo:rustc-env=UPSTREAM_DIR={}", dir.display());
    let crossterm = upstream_dir("crossterm");
    let files = [
        "src/cell.rs",
        "src/column.rs",
        "src/row.rs",
        "src/table.rs",
        "src/style/attribute.rs",
        "src/style/cell.rs",
        "src/style/color.rs",
        "src/style/column.rs",
        "src/style/presets.rs",
        "src/style/table.rs",
        "src/style/table_style.rs",
        "src/utils/mod.rs",
        "src/utils/arrangement/mod.rs",
        "src/utils/arrangement/constraint.rs",
        "src/utils/arrangement/disabled.rs",
        "src/utils/arrangement/dynamic.rs",
        "src/utils/arrangement/helper.rs",
        "src/utils/formatting/borders.rs",
        "src/utils/formatting/content_format.rs",
        "src/utils/formatting/content_split/mod.rs",
        "src/utils/formatting/content_split/normal.rs",
    ]
    .map(|f| dir.join(f))
    .into_iter()
    .chain(
        [
            "src/style.rs",
            "src/style/attributes.rs",
            "src/style/types/attribute.rs",
            "src/style/types/colored.rs",
        ]
        .map(|f| crossterm.join(f)),
    );
    for path in files {
        sources.push_str(
            &std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("could not read {}: {e}", path.display())),
        );
        println!("cargo:rerun-if-changed={}", path.display());
    }
    std::fs::write(out.join("sources.rs.txt"), sources).unwrap();
    println!("cargo:rerun-if-changed=Cargo.toml");
}

/// Where Cargo put the package `name` this build depends on.
fn upstream_dir(name: &str) -> PathBuf {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let manifest = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");
    let out = Command::new(cargo)
        .args(["metadata", "--format-version", "1", "--manifest-path"])
        .arg(&manifest)
        .output()
        .expect("could not run `cargo metadata`");
    assert!(out.status.success(), "`cargo metadata` failed");
    let meta: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let pkg = meta["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == name)
        .unwrap_or_else(|| panic!("{name} is not among the dependencies"));
    Path::new(pkg["manifest_path"].as_str().unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
}
