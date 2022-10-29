use crate::linter::Rule;
use crate::rules::*;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub option: ConfigOption,
    #[serde(default)]
    pub rules: ConfigRules,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigOption {
    #[serde(with = "serde_regex", default)]
    pub exclude_paths: Vec<Regex>,

    #[serde(default = "default_indent")]
    pub indent: usize,
    #[serde(default = "default_prefix_inout")]
    pub prefix_inout: String,
    #[serde(default = "default_prefix_input")]
    pub prefix_input: String,
    #[serde(default = "default_prefix_output")]
    pub prefix_output: String,
    #[serde(default = "default_prefix_label")]
    pub prefix_label: String,
    #[serde(default = "default_prefix_instance")]
    pub prefix_instance: String,
    #[serde(default = "default_prefix_module")]
    pub prefix_module: String,
    #[serde(default = "default_prefix_package")]
    pub prefix_package: String,
    #[serde(default = "default_prefix_interface")]
    pub prefix_interface: String,
}

include!(concat!(env!("OUT_DIR"), "/config_rules.rs"));

impl Default for ConfigOption {
    fn default() -> Self {
        toml::from_str("").unwrap()
    }
}

impl Default for ConfigRules {
    fn default() -> Self {
        toml::from_str("").unwrap()
    }
}

#[allow(dead_code)]
fn default_as_true() -> bool {
    true
}

#[allow(dead_code)]
fn default_as_false() -> bool {
    false
}

fn default_indent() -> usize {
    2
}

fn default_prefix_inout() -> String {
    String::from("b_")
}

fn default_prefix_input() -> String {
    String::from("i_")
}

fn default_prefix_output() -> String {
    String::from("o_")
}

fn default_prefix_label() -> String {
    String::from("l_")
}

fn default_prefix_instance() -> String {
    String::from("u_")
}

fn default_prefix_module() -> String {
    String::from("mod_")
}

fn default_prefix_package() -> String {
    String::from("pkg_")
}

fn default_prefix_interface() -> String {
    String::from("ifc_")
}

include!(concat!(env!("OUT_DIR"), "/impl_config.rs"));
