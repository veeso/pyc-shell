//! ### config
//!
//! `config` is the module which handles pyc configuration parsing

/*
*
*   Copyright (C) 2020 Christian Visintin - christian.visintin1997@gmail.com
*
* 	This file is part of "Pyc"
*
*   Pyc is free software: you can redistribute it and/or modify
*   it under the terms of the GNU General Public License as published by
*   the Free Software Foundation, either version 3 of the License, or
*   (at your option) any later version.
*
*   Pyc is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU General Public License for more details.
*
*   You should have received a copy of the GNU General Public License
*   along with Pyc.  If not, see <http://www.gnu.org/licenses/>.
*
*/

extern crate yaml_rust;

use std::collections::HashMap;
use std::fmt;
use yaml_rust::{Yaml, YamlLoader};

//Types
pub struct Config {
    alias: HashMap<String, String>,
}

#[derive(Copy, Clone, PartialEq, fmt::Debug)]
pub enum ConfigErrorCode {
    NoSuchFileOrDirectory,
    CouldNotReadFile,
    YamlSyntaxError,
}

pub struct ConfigError {
    pub code: ConfigErrorCode,
    pub message: String,
}

impl fmt::Display for ConfigErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code_str: &str = match self {
            ConfigErrorCode::NoSuchFileOrDirectory => "NoSuchFileOrDirectory",
            ConfigErrorCode::CouldNotReadFile => "CouldNotReadFile",
            ConfigErrorCode::YamlSyntaxError => "YamlSyntaxError"
        };
        write!(f, "{}", code_str)
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.code)
    }
}

impl Config {
    /// ### parse_config
    ///
    /// `parse_config` parse a YAML configuration file and return a Config struct
    pub fn parse_config(config_file: String) -> Result<Config, ConfigError> {
        let config_str: String;
        //Read configuration file
        match std::fs::read_to_string(config_file.clone()) {
            Ok(config) => config_str = config,
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    return Err(ConfigError {
                        code: ConfigErrorCode::NoSuchFileOrDirectory,
                        message: String::from(
                            ["No such file or directory: ", config_file.as_str()].join(" "),
                        ),
                    })
                }
                _ => {
                    return Err(ConfigError {
                        code: ConfigErrorCode::CouldNotReadFile,
                        message: String::from(
                            ["Could not read file ", config_file.as_str()].join(" "),
                        ),
                    })
                }
            },
        };
        //Parse YAML file
        let yaml_docs: Vec<Yaml>;
        match YamlLoader::load_from_str(config_str.as_str()) {
            Ok(doc) => yaml_docs = doc,
            Err(_) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from(["Could not parse file", config_file.as_str()].join(" ")),
                })
            }
        };
        //Check there is at least one document
        if yaml_docs.len() == 0 {
            return Err(ConfigError {
                code: ConfigErrorCode::YamlSyntaxError,
                message: String::from("File does not contain any YAML document"),
            });
        };
        let yaml_doc: &Yaml = &yaml_docs[0];
        //Look for keys and get configuration parts
        //Check if alias exists
        let mut alias_config_yaml = &yaml_doc["alias"];
        if alias_config_yaml.is_badvalue() {
            //If alias doesn't exist, then use 'аляс'
            alias_config_yaml = &yaml_doc["аляс"];
        }
        let alias_config: HashMap<String, String>;
        //If 'аляс' doesn't exist, alias is empty
        if alias_config_yaml.is_badvalue() {
            alias_config = HashMap::new();
        } else {
            //Otherwise parse alias object
            alias_config = match Config::parse_alias(&alias_config_yaml) {
                Ok(config) => config,
                Err(err) => return Err(err),
            };
        }
        Ok(Config {
            alias: alias_config,
        })
    }

    /// ### get_alias
    ///
    ///  Get alias from configuratio
    pub fn get_alias(&self, alias: String) -> Option<String> {
        match self.alias.get(&alias) {
            Some(cmd) => Some(cmd.clone()),
            None => None,
        }
    }

    /// ### parse_alias
    ///
    /// Parse alias in Pyc configuration file
    fn parse_alias(alias_yaml: &Yaml) -> Result<HashMap<String, String>, ConfigError> {
        if !alias_yaml.is_array() {
            return Err(ConfigError {
                code: ConfigErrorCode::YamlSyntaxError,
                message: String::from("'alias' key is not an array"),
            });
        }
        let mut alias_table: HashMap<String, String> = HashMap::new();
        //Iterate over alias
        for pair in alias_yaml.as_vec().unwrap() {
            for p in pair.as_hash().unwrap().iter() {
                let key: String = String::from(p.0.as_str().unwrap());
                let value: String = String::from(p.1.as_str().unwrap());
                alias_table.insert(key, value);
            }
        }
        Ok(alias_table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_config_ru_alias() {
        //Try to parse a configuration file
        let config_file: tempfile::NamedTempFile = write_config_file_ru();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        match Config::parse_config(config_file_path) {
            Ok(config) => {
                //Verify alias parameters
                assert_eq!(
                    config.get_alias(String::from("чд")).unwrap(),
                    String::from("cd")
                );
                assert_eq!(
                    config.get_alias(String::from("пвд")).unwrap(),
                    String::from("pwd")
                );
                assert_eq!(
                    config.get_alias(String::from("уич")).unwrap(),
                    String::from("which")
                );
                assert!(config
                    .get_alias(String::from("thiskeydoesnotexist"))
                    .is_none());
            }
            Err(error) => panic!(
                "Parse_config should have returned OK, but returned {} ({:?})",
                error.message, error.code
            ),
        };
    }

    #[test]
    fn test_config_en_alias() {
        //Try to parse a configuration file
        let config_file: tempfile::NamedTempFile = write_config_file_en();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        match Config::parse_config(config_file_path) {
            Ok(config) => {
                //Verify alias parameters
                assert_eq!(
                    config.get_alias(String::from("чд")).unwrap(),
                    String::from("cd")
                );
                assert_eq!(
                    config.get_alias(String::from("пвд")).unwrap(),
                    String::from("pwd")
                );
                assert_eq!(
                    config.get_alias(String::from("уич")).unwrap(),
                    String::from("which")
                );
                assert!(config
                    .get_alias(String::from("thiskeydoesnotexist"))
                    .is_none());
            }
            Err(error) => panic!(
                "Parse_config should have returned OK, but returned {} ({:?})",
                error.message, error.code
            ),
        };
    }

    #[test]
    fn test_config_no_alias() {
        //Try to parse a configuration file
        let config_file: tempfile::NamedTempFile = write_config_no_alias();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        match Config::parse_config(config_file_path) {
            Ok(config) => {
                //Verify alias parameters
                assert!(config.get_alias(String::from("чд")).is_none());
            }
            Err(error) => panic!(
                "Parse_config should have returned OK, but returned {} ({:?})",
                error.message, error.code
            ),
        };
    }

    /// ### write_config_file
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_file_ru() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "аляс:\n  - чд: \"cd\"\n  - пвд: \"pwd\"\n  - уич: \"which\""
        )
        .unwrap();
        tmpfile
    }

    /// ### write_config_file
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_file_en() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "alias:\n  - чд: \"cd\"\n  - пвд: \"pwd\"\n  - уич: \"which\""
        )
        .unwrap();
        tmpfile
    }

    /// ### write_config_file
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_no_alias() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "foobar: 5\n").unwrap();
        tmpfile
    }
}
