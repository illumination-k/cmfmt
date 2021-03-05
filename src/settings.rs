use anyhow::Result;
use std::{collections::HashMap, path::Path};

use crate::utils::{read_string, write_string};

pub const DEFAULT_SETTINGS: &'static str = r#"
[fmt.python]
command = "black"
name = ["py", "python", "python3"]
extention = "py"

[fmt.rust]
command = "rustfmt"
name = ["rs", "rust"]
extention = "rs"

[fmt.js]
command = "prettier"
args = ["--write"]
name = ["js", "ts", "javascript", "typescript"]
extention = "js"
"#;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub fmt: HashMap<String, Lang>,
}

impl Default for Settings {
    fn default() -> Self {
        toml::from_str(DEFAULT_SETTINGS).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lang {
    pub command: String,
    pub name: Vec<String>,
    pub args: Option<Vec<String>>,
    pub extention: Option<String>,
}

impl Lang {
    #[cfg(test)]
    fn new<S: ToString>(command: S, name: Vec<S>, args: Option<Vec<S>>, ext: Option<S>) -> Self {
        Self {
            command: command.to_string(),
            name: name.iter().map(|x| x.to_string()).collect(),
            args: match args {
                Some(v) => Some(v.iter().map(|x| x.to_string()).collect()),
                None => None,
            },
            extention: match ext {
                Some(s) => Some(s.to_string()),
                None => None,
            },
        }
    }

    pub fn contain_language_name(&self, language_name: &String) -> bool {
        self.name.contains(language_name)
    }

    pub fn command(&self) -> String {
        self.command.clone()
    }

    pub fn args(&self) -> Option<Vec<String>> {
        self.args.clone()
    }

    pub fn ext(&self) -> Option<String> {
        match self.extention.clone() {
            Some(ext) => {
                if ext.starts_with(".") {
                    Some(ext)
                } else {
                    Some(format!(".{}", ext))
                }
            }
            None => None,
        }
    }
}

pub fn read_settings<P: AsRef<Path>>(config: &P) -> Result<Settings> {
    let buf = read_string(&config)?;

    let settings = toml::from_str(&buf)?;
    Ok(settings)
}

pub fn write_default_settings<P: AsRef<Path>>(config: &P) -> Result<()> {
    let default_settings = Settings::default();
    let toml = toml::to_string(&default_settings)?;
    write_string(&config, &toml)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use toml;

    use super::*;

    #[test]
    fn test_serialize() {
        let settings: Settings = toml::from_str(DEFAULT_SETTINGS).unwrap();

        let expected: HashMap<String, Lang> = convert_args!(
            keys = String::from,
            hashmap!(
                "python" => Lang::new("black", vec!["py", "python", "python3"], None, Some("py")),
                "rust" => Lang::new("rustfmt", vec!["rs", "rust"], None, Some("rs")),
                "js" => Lang::new("prettier", vec!["js", "ts", "javascript", "typescript"], Some(vec!["--write"]), Some("js")),
            )
        );

        let expected_settings = Settings { fmt: expected };
        assert_eq!(
            settings.fmt.get("python"),
            expected_settings.fmt.get("python")
        );
        assert_eq!(settings.fmt.get("rust"), expected_settings.fmt.get("rust"));
        assert_eq!(settings.fmt.get("js"), expected_settings.fmt.get("js"));
    }
}
