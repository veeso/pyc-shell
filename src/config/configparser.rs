//! ### configparser
//!
//! `configparser` is the module which provides some functions to facilitate the parsing of a YAML

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

use super::{ConfigError, ConfigErrorCode};
use yaml_rust::Yaml;

/// ### Config Parser
pub struct ConfigParser {}

impl ConfigParser {
    /// ### get_child
    ///
    /// Get child from YAML
    pub fn get_child(yaml_doc: &Yaml, child: String) -> Result<&Yaml, ConfigError> {
        match yaml_doc[child.as_str()].is_badvalue() {
            true => Err(ConfigError {
                code: ConfigErrorCode::YamlSyntaxError,
                message: String::from(format!("Missing key '{}'", child)),
            }),
            false => Ok(&yaml_doc[child.as_str()]),
        }
    }

    /// ### get_bool
    ///
    /// get YAML value as bool
    pub fn get_bool(yaml_doc: &Yaml, key: String) -> Result<bool, ConfigError> {
        match ConfigParser::get_child(&yaml_doc, key.clone()) {
            Ok(child) => match child.as_bool() {
                Some(v) => Ok(v),
                None => Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from(format!("'{}' is not a bool", key)),
                }),
            },
            Err(err) => Err(err),
        }
    }

    /// ### get_usize
    ///
    /// get YAML value as usize
    pub fn get_usize(yaml_doc: &Yaml, key: String) -> Result<usize, ConfigError> {
        match ConfigParser::get_child(&yaml_doc, key.clone()) {
            Ok(child) => match child.as_i64() {
                Some(v) => Ok(v as usize),
                None => Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from(format!("'{}' is not a number", key)),
                }),
            },
            Err(err) => Err(err),
        }
    }

    /// ### get_string
    ///
    /// get YAML value as string
    pub fn get_string(yaml_doc: &Yaml, key: String) -> Result<String, ConfigError> {
        match ConfigParser::get_child(&yaml_doc, key.clone()) {
            Ok(child) => match child.as_str() {
                Some(s) => Ok(String::from(s)),
                None => Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from(format!("'{}' is not a string", key)),
                }),
            },
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_rust::{Yaml, YamlLoader};
    
    #[test]
    fn test_configparser_yaml_parser() {
        let yaml_doc: Yaml = gen_sample_yaml();
        let sample_doc: &Yaml = ConfigParser::get_child(&yaml_doc, String::from("sample")).ok().unwrap();
        //Test usize
        assert_eq!(ConfigParser::get_usize(&sample_doc, String::from("usize")).ok().unwrap(), 2048);
        //Test bool
        assert_eq!(ConfigParser::get_bool(&sample_doc, String::from("bool")).ok().unwrap(), true);
        //Test string
        assert_eq!(ConfigParser::get_string(&sample_doc, String::from("str")).ok().unwrap(), String::from("foobar"));
        //Test child
        assert!(ConfigParser::get_child(&sample_doc, String::from("array")).is_ok());
        assert!(ConfigParser::get_child(&sample_doc, String::from("map")).is_ok());
    }

    #[test]
    fn test_configparser_yaml_bad_values() {
        let yaml_doc: Yaml = gen_sample_yaml();
        let sample_doc: &Yaml = ConfigParser::get_child(&yaml_doc, String::from("sample")).ok().unwrap();
        assert!(ConfigParser::get_bool(&sample_doc, String::from("str")).is_err());
        assert!(ConfigParser::get_usize(&sample_doc, String::from("str")).is_err());
        assert!(ConfigParser::get_string(&sample_doc, String::from("array")).is_err());
        assert!(ConfigParser::get_child(&sample_doc, String::from("foobar")).is_err());
    }

    fn gen_sample_yaml() -> Yaml {
        let sample: String = String::from("sample:\n  usize: 2048\n  bool: true\n  str: \"foobar\"\n  array:\n    - 1\n    - 2\n    - 3\n  map:\n    foo: true\n    bar: \"pluto\"\n");
        println!("{}", sample.clone());
        match YamlLoader::load_from_str(sample.as_str()) {
            Ok(mut doc) => doc.pop().unwrap(),
            Err(_) => {
                panic!("Could not parse YAML");
            }
        }
    }
}
