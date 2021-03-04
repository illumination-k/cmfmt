use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    fmt: HashMap<String, Lang>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Lang {
    command: String,
    name: Vec<String>,
    args: Option<Vec<String>>,
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
            }
        }
    }
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

        let expected: HashMap<String, Lang> = convert_args!(keys=String::from, hashmap!(
            "js" => Lang::new("eslint", vec!["js", "ts", "javascript", "typescript"], None),
            "python" => Lang::new("black", vec!["py", "python", "python3"], None), 
            "rust" => Lang::new("rustfmt", vec!["rs", "rust"], None)
        ));

        assert_eq!(settings, Settings { fmt: expected })
    }
}
