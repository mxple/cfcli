use serde::{Deserialize, Serialize};
use std::io::Read;
use std::str::FromStr;
use std::{fs, path::PathBuf};

use crate::{Contest, Problem};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub cf_dir: String,
    pub workspace_dir: String,
    pub solution_filename: String,
    pub default_lang: usize,
    pub workspace_creation_cmd: String,
    pub open_cmd: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            cf_dir: String::from(""),
            workspace_dir: String::from("{%contest_id%}/{%problem_id%}"),
            solution_filename: String::from("{%problem_id%}"),
            default_lang: 54,
            workspace_creation_cmd: String::from(""),
            open_cmd: String::from("vim main.cpp"),
        }
    }
}

pub fn map_id_to_lang(id: usize) -> Result<String, Box<dyn std::error::Error>> {
    let lang: Result<&str, Box<dyn std::error::Error>> = match id {
        43 => Ok("GNU GCC C11 5.1.0"),
        50 => Ok("GNU G++14 6.4.0"),
        54 => Ok("GNU G++17 7.3.0"),
        65 => Ok("C# 8, .NET Core 3.1"),
        79 => Ok("C# 10, .NET SDK 6.0"),
        9 => Ok("C# Mono 6.8"),
        28 => Ok("D DMD32 v2.105.0"),
        32 => Ok("Go 1.19.5"),
        12 => Ok("Haskell GHC 8.10.1"),
        87 => Ok("Java 21 64bit"),
        36 => Ok("Java 8 32bit"),
        83 => Ok("Kotlin 1.7.20"),
        88 => Ok("Kotlin 1.9.21"),
        19 => Ok("OCaml 4.02.1"),
        3 => Ok("Delphi 7"),
        4 => Ok("Free Pascal 3.2.2"),
        51 => Ok("PascalABC.NET 3.8.3"),
        13 => Ok("Perl 5.20.1"),
        6 => Ok("PHP 8.1.7"),
        7 => Ok("Python 2.7.18"),
        31 => Ok("Python 3.8.10"),
        40 => Ok("PyPy 2.7.13 (7.3.0)"),
        41 => Ok("PyPy 3.6.9 (7.3.0)"),
        70 => Ok("PyPy 3.9.10 (7.3.9, 64bit)"),
        67 => Ok("Ruby 3.2.2"),
        75 => Ok("Rust 1.75.0 (2021)"),
        20 => Ok("Scala 2.12.8"),
        34 => Ok("JavaScript V8 4.8.0"),
        55 => Ok("Node.js 15.8.0 (64bit)"),
        _ => Err(Box::from("Invalid lang code found")),
    };
    lang.map(|s| s.to_string())
}

#[derive(Debug)]
enum ReadError {
    IoError(std::io::Error),
    InvalidConfig(toml::de::Error),
}

impl From<std::io::Error> for ReadError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

impl From<toml::de::Error> for ReadError {
    fn from(value: toml::de::Error) -> Self {
        Self::InvalidConfig(value)
    }
}

pub fn try_read_config() -> Result<Config, ReadError> {
    let config_path =
        std::env::var("HOME").expect("No home environment found") + "/.config/cfcli.conf";

    let mut config_file =
        fs::File::open(config_path).expect("No config file found at ~/.config/cfcli.conf");
    let mut buffer = String::new();
    config_file
        .read_to_string(&mut buffer)
        .expect("Error reading config file");
    let config: Config = toml::from_str(&buffer)?;

    Ok(config)
}

pub fn try_write_config(config: &Config) -> Result<(), std::io::Error> {
    let config_path =
        std::env::var("HOME").expect("No home environment found") + "/.config/cfcli.conf";
    let toml_string = toml::to_string(&config).unwrap();
    fs::write(config_path, toml_string).expect("Unable to write to file");
    Ok(())
}

// In addition to a config, we need an app state that keeps track of current
// contest, current problem, and similar
#[derive(Debug, Serialize, Deserialize)]
pub struct AppState {
    pub current_contest: Option<Contest>,
    pub current_problem: Option<Problem>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            current_contest: None,
            current_problem: None,
        }
    }
}

pub fn try_read_app_state(config: &Config) -> Result<AppState, ReadError> {
    let mut path = PathBuf::from_str(&config.cf_dir).expect("Invalid cf directory");
    path.push(".cfcli");
    path.push("state.toml");

    let mut file = fs::File::open(path).expect("No file found at `cf_dir`/.cfcli/state.toml");
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)
        .expect("Error reading state file");
    let state: AppState = toml::from_str(&buffer)?;

    Ok(state)
}

pub fn try_write_app_state(state: &AppState, config: &Config) -> Result<(), std::io::Error> {
    let mut path = PathBuf::from_str(&config.cf_dir).expect("Invalid cf directory");
    path.push(".cfcli");
    path.push("state.toml");
    let toml_string = toml::to_string(&state).unwrap();
    fs::write(path, toml_string).expect("Unable to write to file");
    Ok(())
}
