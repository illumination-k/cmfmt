use anyhow::Result;
use std::{
    collections::HashMap,
    fs,
    io::{BufReader, Read, Write},
    path::Path,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub fmt: HashMap<String, Lang>,
}

impl Default for Settings {
    fn default() -> Self {
        toml::from_str(
            r#"
        # comments!
        [fmt.rust]
        command = "rustfmt"
        name = ["rs", "rust"]
        
        [fmt.python]
        command = "black"
        name = ["py", "python", "python3"]
        
        [fmt.js]
        command = "eslint"
        name = ["js", "ts", "javascript", "typescript"]
        "#,
        )
        .unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lang {
    pub command: String,
    pub name: Vec<String>,
    pub args: Option<Vec<String>>,
}

impl Lang {
    #[cfg(test)]
    fn new<S: ToString>(command: S, name: Vec<S>, args: Option<Vec<S>>) -> Self {
        Self {
            command: command.to_string(),
            name: name.iter().map(|x| x.to_string()).collect(),
            args: match args {
                Some(v) => Some(v.iter().map(|x| x.to_string()).collect()),
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
}

pub fn read_settings<P: AsRef<Path>>(config: &P) -> Result<Settings> {
    let mut buf = String::new();
    let mut file = fs::File::open(config).map(|f| BufReader::new(f))?;

    file.read_to_string(&mut buf)?;

    let settings = toml::from_str(&buf)?;
    Ok(settings)
}

pub fn write_default_settings<P: AsRef<Path>>(config: &P) -> Result<()> {
    let mut file = fs::File::create(config)?;
    let default_settings = Settings::default();
    let toml = toml::to_string(&default_settings)?;
    write!(file, "{}", toml)?;
    file.flush()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use toml;

    use super::*;

    #[test]
    fn test_serialize() {
        let settings: Settings = toml::from_str(
            r#"
        # comments!
        [fmt.rust]
        command = "rustfmt"
        name = ["rs", "rust"]
        
        [fmt.python]
        command = "black"
        name = ["py", "python", "python3"]
        
        [fmt.js]
        command = "eslint"
        name = ["js", "ts", "javascript", "typescript"]
        "#,
        )
        .unwrap();

        let expected: HashMap<String, Lang> = convert_args!(
            keys = String::from,
            hashmap!(
                "js" => Lang::new("eslint", vec!["js", "ts", "javascript", "typescript"], None),
                "python" => Lang::new("black", vec!["py", "python", "python3"], None),
                "rust" => Lang::new("rustfmt", vec!["rs", "rust"], None)
            )
        );

        assert_eq!(settings, Settings { fmt: expected })
    }
}
