use std::{
    collections::{BTreeMap, HashMap},
    fs::File,
    path::{Path, PathBuf},
};

use log::LevelFilter;
use serde::{Deserialize, Serialize, Serializer};
use strum::EnumIter;

/// Support tiers: https://doc.rust-lang.org/nightly/rustc/platform-support.html.
#[derive(
    Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, EnumIter,
)]
pub enum Tier {
    /// Tier 1 platforms.
    #[serde(rename = "Tier 1")]
    Tier1,
    /// Tier 2 platforms.
    #[serde(rename = "Tier 2")]
    Tier2,
    /// Tier 2.5 platforms.
    #[serde(rename = "Tier 2.5")]
    Tier25,
    /// Tier 3 platforms.
    #[serde(rename = "Tier 3")]
    Tier3,
    #[doc(hidden)]
    UnknownTier,
}

fn default_verbosity() -> LevelFilter {
    LevelFilter::Warn
}

fn default_channel() -> String {
    String::from("nightly")
}

fn default_additional_days() -> usize {
    0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(flatten)]
    pub html: Html,
    /// For how many days in the past would you like to peek.
    pub days_in_past: usize,
    /// For how many additional days should we look into to calculate "the last
    /// available" date.
    #[serde(default = "default_additional_days")]
    pub additional_lookup_days: usize,
    /// A release channel to check.
    ///
    /// If omitted, the default channel is nightly.
    #[serde(default = "default_channel")]
    pub channel: String,
    /// Verbosity level, might be one of the following: OFF, ERROR, WARN, INFO,
    /// DEBUG or TRACE.
    ///
    /// If omitted, the default level is WARN.
    #[serde(default = "default_verbosity")]
    pub verbosity: LevelFilter,
    /// A path where to store the downloaded manifests.
    ///
    /// If omitted, no cache will be used, i.e. all the manifests will be
    /// re-downloaded every time you run the tool.
    #[serde(default)]
    pub cache_path: Option<PathBuf>,
    ///A path where a file tree of available packages will be created. The tool
    /// will generate a set of files under a given *output* directory with the
    /// following pattern: file_tree_output/$target/$package, where $target
    /// stands for a target host architecture, like x86_64-unknown-linux-gnu,
    /// and $package stands for a package name, like miri or rust-src. Each of
    /// those files will contain a date in a "%Y-%m-%d" format (e.g. 2019-12-24)
    /// which represents the latest date when the package was (is) available for
    /// that specific target.
    pub file_tree_output: PathBuf,
}

/// Html-related configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Html {
    /// Path to a handlebars template file.
    pub template_path: PathBuf,
    /// A pattern that will be used to render output files. Any instance of a
    /// `{{target}}` will be replaced with a target name.
    pub output_pattern: String,
    /// Platform tiers lists.
    #[serde(default)]
    #[serde(serialize_with = "ordered_map")]
    pub tiers: HashMap<Tier, Vec<String>>,
}

fn ordered_map<S: Serializer, K: Ord + Serialize, V: Serialize>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    value
        .iter()
        .collect::<BTreeMap<_, _>>()
        .serialize(serializer)
}

impl Config {
    pub fn load(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let f = File::open(path)?;
        Ok(serde_yaml::from_reader(f)?)
    }
}
