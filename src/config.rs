use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::ConfigArgs;

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub commit: Commit,
    pub branch: Branch,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Commit {
    pub conventional_commits: bool,
    pub ticket_suffix: bool,
    pub types: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(default)]
pub struct Branch {
    pub conventional_branches: bool,
    pub types: Vec<String>,
}

impl Default for Commit {
    fn default() -> Self {
        Self {
            conventional_commits: false,
            ticket_suffix: false,
            types: vec![
                "build".into(),
                "ci".into(),
                "docs".into(),
                "feat".into(),
                "fix".into(),
                "perf".into(),
                "refactor".into(),
                "style".into(),
                "test".into(),
                "revert".into(),
            ],
        }
    }
}

impl Default for Branch {
    fn default() -> Self {
        Self {
            conventional_branches: false,
            types: vec![
                "feature".into(),
                "bugfix".into(),
                "hotfix".into(),
                "release".into(),
                "chore".into(),
            ],
        }
    }
}

pub fn load_config() -> Config {
    let config_path = get_config_path();

    if let Ok(config_content) = fs::read_to_string(&config_path) {
        toml::from_str(&config_content).expect("Failed to parse config")
    } else {
        let config = Config::default();
        let _ = save_config(&config, &config_path);
        config
    }
}

pub fn run_config(args: &ConfigArgs) -> Result<(), String> {
    let mut config = load_config();
    let config_path = get_config_path();
    let setting: Vec<&str> = args.key.split('.').collect();
    if setting.len() != 2 {
        return Err("Key format must be <section>.<field>".into());
    }
    let (section, field) = (setting[0], setting[1]);

    match section {
        "commit" => match field {
            "conventional_commits" => set_bool(&mut config.commit.conventional_commits, &args.value)?,
            "ticket_suffix" => set_bool(&mut config.commit.ticket_suffix, &args.value)?,
            "types" => set_vec(&mut config.commit.types, &args.value)?,
            _ => return Err(format!("Unknown commit setting '{}'", field)),
        },

        "branch" => match field {
            "conventional_branches" => set_bool(&mut config.branch.conventional_branches, &args.value)?,
            "types" => set_vec(&mut config.branch.types, &args.value)?,
            _ => return Err(format!("Unknown branch setting '{}'", field)),
        }

        _ => return Err(format!("Unknown section '{}'", section)),
    }
    save_config(&config, &config_path).map_err(|e| format!("Failed to save config: {}", e))?;
    println!("✅ Config created successfuly!");
    Ok(())
}

fn set_bool(target: &mut bool, value: &str) -> Result<(), String> {
    match value.to_lowercase().as_str() {
        "1" | "true" => *target = true,
        "0" | "false" => *target = false,
        _ => return Err("Value must be boolean".into()),
    }
    Ok(())
}

fn set_vec(target: &mut Vec<String>, value: &str) -> Result<(), String> {
    *target = value.split(',').map(|s| s.trim().to_string()).collect();
    Ok(())
}

fn get_config_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("", "", "gitquick").expect("Failed to get project directory");

    let directory = proj_dirs.config_dir();
    directory.join("config.toml")
}

fn save_config(config: &Config, config_path: &PathBuf) -> io::Result<()> {
    if let Some(directory) = config_path.parent() {
        fs::create_dir_all(directory)?
    };
    let content = toml::to_string(config).unwrap();
    let mut file = fs::File::create(config_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
