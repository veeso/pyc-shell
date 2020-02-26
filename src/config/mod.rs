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
    pub output_config: OutputConfig
}

pub struct OutputConfig {
    pub translate_output: bool
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
            ConfigErrorCode::YamlSyntaxError => "YamlSyntaxError",
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
    /// ### default
    ///
    /// Instantiates a default configuration struct
    pub fn default() -> Config {
        let alias_config: HashMap<String, String> = HashMap::new();
        Config {
            alias: alias_config,
            output_config: OutputConfig::default()
        }
    }

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
        let yaml_docs: Vec<Yaml> = match YamlLoader::load_from_str(config_str.as_str()) {
            Ok(doc) => doc,
            Err(_) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from(["Could not parse file", config_file.as_str()].join(" ")),
                });
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
        let alias_config: HashMap<String, String> = if alias_config_yaml.is_badvalue() {
            HashMap::new()
        } else {
            //Otherwise parse alias object
            match Config::parse_alias(&alias_config_yaml) {
                Ok(config) => config,
                Err(err) => return Err(err),
            }
        };
        //Check if output exists
        let mut output_config_yaml = &yaml_doc["output"];
        if output_config_yaml.is_badvalue() {
            //If alias doesn't exist, then use 'аляс'
            output_config_yaml = &yaml_doc["оутпут"];
        }
        let output_config: OutputConfig = if output_config_yaml.is_badvalue() {
            OutputConfig::default()
        } else {
            //Otherwise parse alias object
            match OutputConfig::parse_config(&output_config_yaml) {
                Ok(config) => config,
                Err(err) => return Err(err),
            }
        };
        Ok(Config {
            alias: alias_config,
            output_config: output_config
        })
    }

    /// ### get_alias
    ///
    ///  Get alias from configuration
    pub fn get_alias(&self, alias: &String) -> Option<String> {
        match self.alias.get(alias) {
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

impl OutputConfig {
    pub fn default() -> OutputConfig {
        OutputConfig {
            translate_output: true
        }
    }

    pub fn parse_config(output_yaml: &Yaml) -> Result<OutputConfig, ConfigError> {
        let mut translate_output_yaml = &output_yaml["translate"];
        if translate_output_yaml.is_badvalue() {
            //If translate doesn't exist, then use 'транслатэ'
            translate_output_yaml = &output_yaml["транслатэ"];
        }
        if translate_output_yaml.is_badvalue() {
            return Err(ConfigError {
                code: ConfigErrorCode::YamlSyntaxError,
                message: String::from("Error in 'output' config: Key translate/транслатэ is missing"),
            })
        }
        let translate_output: bool = match translate_output_yaml.as_bool() {
            Some(flag) => flag,
            None => return Err(ConfigError {
                code: ConfigErrorCode::YamlSyntaxError,
                message: String::from("Error in 'output' config: Key translate/транслатэ is not boolean"),
            })
        };
        Ok(OutputConfig {
            translate_output: translate_output
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_config_default() {
        let config: Config = Config::default();
        assert!(config.get_alias(&String::from("чд")).is_none());
        assert_eq!(config.output_config.translate_output, true);
    }

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
                    config.get_alias(&String::from("чд")).unwrap(),
                    String::from("cd")
                );
                assert_eq!(
                    config.get_alias(&String::from("пвд")).unwrap(),
                    String::from("pwd")
                );
                assert_eq!(
                    config.get_alias(&String::from("уич")).unwrap(),
                    String::from("which")
                );
                assert!(config
                    .get_alias(&String::from("thiskeydoesnotexist"))
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
                    config.get_alias(&String::from("чд")).unwrap(),
                    String::from("cd")
                );
                assert_eq!(
                    config.get_alias(&String::from("пвд")).unwrap(),
                    String::from("pwd")
                );
                assert_eq!(
                    config.get_alias(&String::from("уич")).unwrap(),
                    String::from("which")
                );
                assert!(config
                    .get_alias(&String::from("thiskeydoesnotexist"))
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
                assert!(config.get_alias(&String::from("чд")).is_none());
            }
            Err(error) => panic!(
                "Parse_config should have returned OK, but returned {} ({:?})",
                error.message, error.code
            ),
        };
    }

    #[test]
    fn test_config_output_config() {
        //Try to parse a configuration file
        let config_file: tempfile::NamedTempFile = write_config_output_config();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        match Config::parse_config(config_file_path) {
            Ok(config) => {
                //Verify alias parameters
                assert!(config.output_config.translate_output)
            }
            Err(error) => panic!(
                "Parse_config should have returned OK, but returned {} ({:?})",
                error.message, error.code
            ),
        };
        //Try to parse a configuration file
        let config_file: tempfile::NamedTempFile = write_config_output_config_ru_false();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        match Config::parse_config(config_file_path) {
            Ok(config) => {
                //Verify alias parameters
                assert_eq!(config.output_config.translate_output, false)
            }
            Err(error) => panic!(
                "Parse_config should have returned OK, but returned {} ({:?})",
                error.message, error.code
            ),
        };
    }

    #[test]
    fn test_config_bad_output_config() {
        let config_file: tempfile::NamedTempFile = write_config_bad_output_config();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        if let Err(err) = Config::parse_config(config_file_path) {
            match err.code {
                ConfigErrorCode::YamlSyntaxError => {
                    println!("Okay, YamlSynaxError has been returned")
                }
                _ => panic!("Expected YamlSynaxError, got {}", err.code),
            }
        } else {
            panic!("parse_config of bad syntax returned OK");
        }
    }

    #[test]
    fn test_bad_syntax() {
        let config_file: tempfile::NamedTempFile = write_config_bad_syntax();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        if let Err(err) = Config::parse_config(config_file_path) {
            match err.code {
                ConfigErrorCode::YamlSyntaxError => {
                    println!("Okay, YamlSynaxError has been returned")
                }
                _ => panic!("Expected YamlSynaxError, got {}", err.code),
            }
        } else {
            panic!("parse_config of bad syntax returned OK");
        }
    }

    #[test]
    fn test_alias_not_array() {
        let config_file: tempfile::NamedTempFile = write_config_alias_as_int();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        if let Err(err) = Config::parse_config(config_file_path) {
            match err.code {
                ConfigErrorCode::YamlSyntaxError => {
                    println!("Okay, YamlSynaxError has been returned")
                }
                _ => panic!("Expected YamlSynaxError, got {}", err.code),
            }
        } else {
            panic!("parse_config of bad syntax returned OK");
        }
    }

    #[test]
    fn test_no_file() {
        if let Err(err) = Config::parse_config(String::from("config.does.not.exist.yml")) {
            match err.code {
                ConfigErrorCode::NoSuchFileOrDirectory => {
                    println!("Okay, nosuchfileordirectory is correct!")
                }
                _ => panic!("Expected NoSuchFileOrDirectory, but returned {}", err.code),
            }
        } else {
            panic!("parse_config of not existing file returned OK");
        }
    }

    #[test]
    fn test_not_accessible() {
        if let Err(err) = Config::parse_config(String::from("/dev/ttyS0")) {
            match err.code {
                ConfigErrorCode::CouldNotReadFile => println!("Okay, CouldNotReadFile is correct!"),
                _ => panic!("Expected CouldNotReadFile, but returned {}", err.code),
            }
        } else {
            panic!("parse_config of not not accessible file returned OK");
        }
    }

    #[test]
    fn test_empty_yaml() {
        let config_file: tempfile::NamedTempFile = write_config_empty();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        if let Err(err) = Config::parse_config(config_file_path) {
            match err.code {
                ConfigErrorCode::YamlSyntaxError => {
                    println!("Okay, YamlSynaxError has been returned")
                }
                _ => panic!("Expected YamlSynaxError, got {}", err.code),
            }
        } else {
            panic!("parse_config of bad syntax returned OK");
        }
    }

    #[test]
    fn test_error_display() {
        println!(
            "{};{};{}",
            ConfigErrorCode::CouldNotReadFile,
            ConfigErrorCode::NoSuchFileOrDirectory,
            ConfigErrorCode::YamlSyntaxError
        );
        println!(
            "{}",
            ConfigError {
                code: ConfigErrorCode::NoSuchFileOrDirectory,
                message: String::from("No such file or directory ~/.config/pyc/pyc.yml")
            }
        );
    }

    /// ### write_config_file_ru
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

    /// ### write_config_file_en
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

    /// ### write_config_no_alias
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_no_alias() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "foobar: 5\n").unwrap();
        tmpfile
    }

    fn write_config_output_config() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "output:\n  translate: true\n").unwrap();
        tmpfile
    }

    fn write_config_bad_output_config() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "output:\n  foobar: 5\n").unwrap();
        tmpfile
    }

    fn write_config_output_config_ru_false() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "оутпут:\n  транслатэ: false\n").unwrap();
        tmpfile
    }

    /// ### write_config_bad_syntax
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_bad_syntax() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "foobar: 5:\n").unwrap();
        tmpfile
    }

    /// ### write_config_alias_as_int
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_alias_as_int() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "alias: 5\n").unwrap();
        tmpfile
    }

    /// ### Write empty yaml file
    /// Write configuration file to a temporary directory and return the file path
    fn write_config_empty() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(tmpfile, "\n").unwrap();
        tmpfile
    }
}
