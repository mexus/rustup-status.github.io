//! A rustup manifest types.
//!
//! Currently only fields required to get an availability status are implemented, but if you need
//! more please feel free to send a PR.

use crate::Error;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

/// A rustup manifest.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    /// A date for which the manifest is generated.
    pub date: NaiveDate,
    /// A map of available packages and their targets.
    #[serde(rename = "pkg")]
    pub packages: HashMap<String, PackageTargets>,
    /// A map of package "renames".
    #[serde(default)]
    pub renames: HashMap<String, Rename>,
}

/// Package renaming
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rename {
    /// New name of the package.
    pub to: String,
}

/// Package info.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PackageTargets {
    /// Maps targets onto package availability info.
    #[serde(rename = "target")]
    pub targets: HashMap<String, PackageInfo>,
}

/// A per-target package information.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PackageInfo {
    /// If a package is available for a specific target.
    pub available: bool,
}

impl Manifest {
    /// Tries to load a `Manifest` from the file system.
    pub fn load_from_fs(path: impl AsRef<Path>) -> Result<Self, Error> {
        let path = path.as_ref();
        let mut f = File::open(path).map_err(|e| Error::Io(e, format!("opening {:?}", path)))?;
        let mut data = String::new();
        f.read_to_string(&mut data)
            .map_err(|e| Error::Io(e, format!("reading {:?}", path)))?;
        toml::from_str(&data).map_err(|e| Error::TomlDe(e, format!("{:?}", path)))
    }

    /// Serializes the `Manifest` to a given path.
    pub fn save_to_file(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let path = path.as_ref();
        let mut f = File::create(path).map_err(|e| Error::Io(e, format!("creating {:?}", path)))?;
        let data = toml::to_vec(self)
            .map_err(|e| Error::TomlSer(e, format!("serializing {}", self.date)))?;
        f.write_all(&data)
            .map_err(|e| Error::Io(e, format!("writing to {:?}", path)))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check() {
        let data = r#"date = "2018-09-03"
[pkg.cargo.target.aarch64-unknown-linux-gnu]
available = true

[pkg.cargo.target.arm-unknown-linux-gnueabi]
available = true

[pkg.cargo.target.arm-unknown-linux-gnueabihf]
available = false

[pkg.rustfmt-preview.target.aarch64-unknown-linux-gnu]
available = true

[pkg.rustfmt-preview.target.x86_64-unknown-freebsd]
available = false

[pkg.rustfmt-preview.target.x86_64-unknown-linux-gnu]
available = true

[renames.rustfmt]
to = "rustfmt-preview"
"#;
        let parsed_manifest: Manifest = toml::from_str(data).unwrap();
        let reference_manifest = Manifest {
            date: NaiveDate::from_ymd(2018, 9, 3),
            packages: vec![
                (
                    "cargo".to_string(),
                    PackageTargets {
                        targets: vec![
                            (
                                "aarch64-unknown-linux-gnu".to_string(),
                                PackageInfo { available: true },
                            ),
                            (
                                "arm-unknown-linux-gnueabi".to_string(),
                                PackageInfo { available: true },
                            ),
                            (
                                "arm-unknown-linux-gnueabihf".to_string(),
                                PackageInfo { available: false },
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    },
                ),
                (
                    "rustfmt-preview".to_string(),
                    PackageTargets {
                        targets: vec![
                            (
                                "aarch64-unknown-linux-gnu".to_string(),
                                PackageInfo { available: true },
                            ),
                            (
                                "x86_64-unknown-freebsd".to_string(),
                                PackageInfo { available: false },
                            ),
                            (
                                "x86_64-unknown-linux-gnu".to_string(),
                                PackageInfo { available: true },
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    },
                ),
            ]
            .into_iter()
            .collect(),
            renames: vec![
                (
                    "rustfmt".to_string(),
                    Rename {
                        to: "rustfmt-preview".to_string(),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        };
        assert_eq!(reference_manifest, parsed_manifest);
    }

    #[test]
    fn check_wildcard() {
        let data = r#"date = "2018-09-03"
[pkg.rust-src.target."*"]
available = true
url = "https://static.rust-lang.org/dist/2018-09-03/rust-src-nightly.tar.gz"
hash = "79f524d39ccd7fc28be473d1ec35e77ff18322488d455e046c7fe850f2a56b67"
xz_url = "https://static.rust-lang.org/dist/2018-09-03/rust-src-nightly.tar.xz"
xz_hash = "dbb913da0a207ae80c53bc6a42074b637920c2a80121420416579fed3e7f2499"
"#;
        let parsed_manifest: Manifest = toml::from_str(data).unwrap();
        let reference_manifest = Manifest {
            date: NaiveDate::from_ymd(2018, 9, 3),
            packages: vec![(
                "rust-src".to_string(),
                PackageTargets {
                    targets: vec![("*".to_string(), PackageInfo { available: true })]
                        .into_iter()
                        .collect(),
                },
            )]
            .into_iter()
            .collect(),
            renames: vec![].into_iter().collect(),
        };
        assert_eq!(reference_manifest, parsed_manifest);
    }

    #[test]
    fn check_validity() {
        let data = r#"manifest-version = "2"
date = "2018-09-03"
[pkg.cargo]
version = "0.30.0-nightly (0ec7281b9 2018-08-20)"
[pkg.cargo.target.aarch64-unknown-linux-gnu]
available = true
url = "https://static.rust-lang.org/dist/2018-09-04/cargo-nightly-aarch64-unknown-linux-gnu.tar.gz"
hash = "15b6e8011af001afb8bc4ec0b951b2b7fcd139f5d83ce82fea7c711c259d217a"
xz_url = "https://static.rust-lang.org/dist/2018-09-04/cargo-nightly-aarch64-unknown-linux-gnu.tar.xz"
xz_hash = "23844e04a62c5fc74a2aebb8e084d6d066deae486f080e0f746eb34148e787f9"

[pkg.cargo.target.arm-unknown-linux-gnueabi]
available = true
url = "https://static.rust-lang.org/dist/2018-09-04/cargo-nightly-arm-unknown-linux-gnueabi.tar.gz"
hash = "bae7e0f9450b64a42e75db53c3f733ccacc1108393473c64cc13ef79900dbe71"
xz_url = "https://static.rust-lang.org/dist/2018-09-04/cargo-nightly-arm-unknown-linux-gnueabi.tar.xz"
xz_hash = "d9ccfec25501c9a9a247867b711d59886c2913367d16f6ba5887612abb8325d1"

[pkg.cargo.target.arm-unknown-linux-gnueabihf]
available = true
url = "https://static.rust-lang.org/dist/2018-09-04/cargo-nightly-arm-unknown-linux-gnueabihf.tar.gz"
hash = "62f6cbfa88e7301414a40d2bfcdc77809a3eace1dbbc8c47b839bdaa5756d326"
xz_url = "https://static.rust-lang.org/dist/2018-09-04/cargo-nightly-arm-unknown-linux-gnueabihf.tar.xz"
xz_hash = "ca0e68e7e9827ba6221a40bb17170e8dc6271a9f6991ee5030dcf6acb1a1d8c8"

[pkg.rustfmt-preview]
version = "0.99.2-nightly (5c9a2b6c1 2018-08-07)"
[pkg.rustfmt-preview.target.aarch64-unknown-linux-gnu]
available = true
url = "https://static.rust-lang.org/dist/2018-09-03/rustfmt-nightly-aarch64-unknown-linux-gnu.tar.gz"
hash = "cbb44cfb0148ec10335f312a74459c2874b2c2e65c11940cb762f55b75a846e1"
xz_url = "https://static.rust-lang.org/dist/2018-09-03/rustfmt-nightly-aarch64-unknown-linux-gnu.tar.xz"
xz_hash = "04afbca685b921b5bbbce9bce728430f410d4ebc293407893795acc9054dec2a"

[pkg.rustfmt-preview.target.x86_64-unknown-freebsd]
available = true
url = "https://static.rust-lang.org/dist/2018-09-03/rustfmt-nightly-x86_64-unknown-freebsd.tar.gz"
hash = "76566ab4d9373b3fe54466984aadebc89f1ce91c79c3e8fd60d559744af5b40c"
xz_url = "https://static.rust-lang.org/dist/2018-09-03/rustfmt-nightly-x86_64-unknown-freebsd.tar.xz"
xz_hash = "6aa97c093f923399b5b00dbc489120b6f1a225eb1e54e2cb3135ec7674aa1d48"

[pkg.rustfmt-preview.target.x86_64-unknown-linux-gnu]
available = true
url = "https://static.rust-lang.org/dist/2018-09-03/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz"
hash = "74b5d5ff8434c15359eccd15dfc097a1d9c7a3ac44a21718b2998e8cccc347a9"
xz_url = "https://static.rust-lang.org/dist/2018-09-03/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz"
xz_hash = "85c786cfd3f7531a26e004819651da00540e24f83f5d8de0e3ab991730b4cc0d"
"#;
        let _manifest: Manifest = toml::from_str(data).unwrap();
    }
}
