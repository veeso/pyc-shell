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

mod configparser;

use configparser::ConfigParser;
use std::collections::HashMap;
use std::fmt;
use yaml_rust::{Yaml, YamlLoader};

//Types
pub struct Config {
    pub language: String,
    alias: HashMap<String, String>,
    pub output_config: OutputConfig,
    pub prompt_config: PromptConfig,
}

pub struct OutputConfig {
    pub translate_output: bool,
}

pub struct PromptConfig {
    pub prompt_line: String,
    pub history_size: usize,
    pub translate: bool,
    pub break_enabled: bool,
    pub break_str: String,
    pub min_duration: usize,
    pub rc_ok: String,
    pub rc_err: String,
    pub git_branch: String,
    pub git_commit_ref: usize,
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
            language: String::from("ru"),
            alias: alias_config,
            output_config: OutputConfig::default(),
            prompt_config: PromptConfig::default(),
        }
    }

    /// ### parse_config
    ///
    /// `parse_config` parse a YAML configuration file and return a Config struct
    pub fn parse_config(config_file: String) -> Result<Config, ConfigError> {
        //Read configuration file
        let config_str: String = match std::fs::read_to_string(config_file.clone()) {
            Ok(config) => config,
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
        Config::parse_config_str(config_str)
    }

    /// ### parse_config_str
    ///
    /// Parse configuration as string
    fn parse_config_str(config: String) -> Result<Config, ConfigError> {
        //Parse YAML file
        let yaml_docs: Vec<Yaml> = match YamlLoader::load_from_str(config.as_str()) {
            Ok(doc) => doc,
            Err(_) => {
                return Err(ConfigError {
                    code: ConfigErrorCode::YamlSyntaxError,
                    message: String::from("Configuration is not a valid YAML"),
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
        //Get language
        let language: String = match ConfigParser::get_child(&yaml_doc, String::from("language")) {
            Ok(node) => match Config::parse_language(&node) {
                Ok(l) => l,
                Err(err) => return Err(err),
            },
            Err(_) => String::from("ru"),
        };
        //Get alias
        let alias_config: HashMap<String, String> =
            match ConfigParser::get_child(&yaml_doc, String::from("alias")) {
                Ok(node) => match Config::parse_alias(&node) {
                    Ok(cfg) => cfg,
                    Err(err) => return Err(err),
                },
                Err(_) => HashMap::new(),
            };
        //Get output config
        let output_config: OutputConfig =
            match ConfigParser::get_child(&yaml_doc, String::from("output")) {
                Ok(node) => match OutputConfig::parse_config(&node) {
                    Ok(config) => config,
                    Err(err) => return Err(err),
                },
                Err(_) => OutputConfig::default(),
            };
        //Get prompt config
        let prompt_config: PromptConfig =
            match ConfigParser::get_child(&yaml_doc, String::from("prompt")) {
                Ok(node) => match PromptConfig::parse_config(&node) {
                    Ok(config) => config,
                    Err(err) => return Err(err),
                },
                Err(_) => PromptConfig::default(),
            };
        Ok(Config {
            language: language,
            alias: alias_config,
            output_config: output_config,
            prompt_config: prompt_config,
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

    /// ### parse_language
    ///
    /// Parse language YAML object
    fn parse_language(language_yaml: &Yaml) -> Result<String, ConfigError> {
        match language_yaml.as_str() {
            Some(s) => Ok(String::from(s)),
            None => Err(ConfigError {
                code: ConfigErrorCode::YamlSyntaxError,
                message: String::from("'language' is not a string"),
            }),
        }
    }
}

impl OutputConfig {
    pub fn default() -> OutputConfig {
        OutputConfig {
            translate_output: true,
        }
    }

    pub fn parse_config(output_yaml: &Yaml) -> Result<OutputConfig, ConfigError> {
        let translate_output: bool =
            match ConfigParser::get_bool(&output_yaml, String::from("translate")) {
                Ok(t) => t,
                Err(err) => return Err(err),
            };
        Ok(OutputConfig {
            translate_output: translate_output,
        })
    }
}

impl PromptConfig {
    /// ### default
    ///
    /// Instantiate a default PromptConfig struct
    pub fn default() -> PromptConfig {
        PromptConfig {
            prompt_line: String::from("${USER}@${HOSTNAME}:${WRKDIR}$"),
            history_size: 256,
            translate: false,
            break_enabled: false,
            break_str: String::from("❯"),
            min_duration: 2000,
            rc_ok: String::from("✔"),
            rc_err: String::from("✖"),
            git_branch: String::from("on "),
            git_commit_ref: 8,
        }
    }

    /// ### parse_config
    ///
    /// Parse a PromptConfig from YAML configuration file
    pub fn parse_config(prompt_config_yaml: &Yaml) -> Result<PromptConfig, ConfigError> {
        //Prompt line
        let prompt_line: String =
            match ConfigParser::get_string(&prompt_config_yaml, String::from("prompt_line")) {
                Ok(ret) => ret,
                Err(err) => return Err(err),
            };
        //History size
        let history_size: usize =
            match ConfigParser::get_usize(&prompt_config_yaml, String::from("history_size")) {
                Ok(ret) => ret,
                Err(err) => return Err(err),
            };
        //History size
        let translate: bool =
            match ConfigParser::get_bool(&prompt_config_yaml, String::from("translate")) {
                Ok(ret) => ret,
                Err(err) => return Err(err),
            };
        //Break
        let brk: &Yaml = match ConfigParser::get_child(&prompt_config_yaml, String::from("break")) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };
        //Break enabled
        let break_enabled: bool = match ConfigParser::get_bool(&brk, String::from("enabled")) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };
        //Break with
        let break_str: String = match ConfigParser::get_string(&brk, String::from("with")) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };
        //Duration
        let duration: &Yaml =
            match ConfigParser::get_child(&prompt_config_yaml, String::from("duration")) {
                Ok(ret) => ret,
                Err(err) => return Err(err),
            };
        //Minimum duration
        let min_duration: usize =
            match ConfigParser::get_usize(&duration, String::from("min_elapsed_time")) {
                Ok(ret) => ret,
                Err(err) => return Err(err),
            };
        //Rc
        let rc: &Yaml = match ConfigParser::get_child(&prompt_config_yaml, String::from("rc")) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };
        //Rc_ok
        let rc_ok: String = match ConfigParser::get_string(&rc, String::from("ok")) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };
        //Rc err
        let rc_err: String = match ConfigParser::get_string(&rc, String::from("error")) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };
        //Git
        let git: &Yaml = match ConfigParser::get_child(&prompt_config_yaml, String::from("git")) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };
        //Git branch
        let git_branch: String = match ConfigParser::get_string(&git, String::from("branch")) {
            Ok(ret) => ret,
            Err(err) => return Err(err),
        };
        //Git commit ref
        let git_commit_ref: usize =
            match ConfigParser::get_usize(&git, String::from("commit_ref_len")) {
                Ok(ret) => ret,
                Err(err) => return Err(err),
            };
        Ok(PromptConfig {
            prompt_line: prompt_line,
            history_size: history_size,
            translate: translate,
            break_enabled: break_enabled,
            break_str: break_str,
            min_duration: min_duration,
            rc_ok: rc_ok,
            rc_err: rc_err,
            git_branch: git_branch,
            git_commit_ref: git_commit_ref,
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
        assert_eq!(config.language, String::from("ru"));
        let prompt_config: PromptConfig = config.prompt_config;
        assert_eq!(prompt_config.prompt_line, String::from("${USER}@${HOSTNAME}:${WRKDIR}$"));
        assert_eq!(prompt_config.break_enabled, false);
        assert_eq!(prompt_config.break_str, String::from("❯"));
        assert_eq!(prompt_config.git_branch, String::from("on "));
        assert_eq!(prompt_config.git_commit_ref, 8);
        assert_eq!(prompt_config.history_size, 256);
        assert_eq!(prompt_config.min_duration, 2000);
        assert_eq!(prompt_config.rc_err, String::from("✖"));
        assert_eq!(prompt_config.rc_ok, String::from("✔"));
        assert_eq!(prompt_config.translate, false);
    }

    #[test]
    fn test_config_file() {
        //Try to parse a configuration file
        let config_file: tempfile::NamedTempFile = write_config_file_en();
        let config_file_path: String = String::from(config_file.path().to_str().unwrap());
        println!("Generated config file: {}", config_file_path);
        assert!(Config::parse_config(config_file_path).is_ok())
    }

    #[test]
    fn test_no_file() {
        assert_eq!(
            Config::parse_config(String::from("config.does.not.exist.yml"))
                .err()
                .unwrap()
                .code,
            ConfigErrorCode::NoSuchFileOrDirectory
        );
    }

    #[cfg(not(target_os = "macos"))]
    #[test]
    fn test_not_accessible() {
        assert_eq!(
            Config::parse_config(String::from("/dev/ttyS0"))
                .err()
                .unwrap()
                .code,
            ConfigErrorCode::CouldNotReadFile
        );
    }

    #[test]
    fn test_config_en_alias() {
        //Try to parse a configuration file
        let config: String =
            String::from("alias:\n  - чд: \"cd\"\n  - пвд: \"pwd\"\n  - уич: \"which\"");
        match Config::parse_config_str(config) {
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
        let config: String = String::from("language: ru\n");
        let config: Config = Config::parse_config_str(config).ok().unwrap();
        assert!(config.get_alias(&String::from("чд")).is_none());
    }

    #[test]
    fn test_alias_not_array() {
        let config: String = String::from("alias: 5\n");
        assert_eq!(
            Config::parse_config_str(config).err().unwrap().code,
            ConfigErrorCode::YamlSyntaxError
        );
    }

    #[test]
    fn test_config_output_config() {
        let config: String =
            String::from("alias:\n  - чд: \"cd\"\n  - пвд: \"pwd\"\n  - уич: \"which\"");
        let config: Config = Config::parse_config_str(config).ok().unwrap();
        assert!(config.output_config.translate_output);
        //Try to parse a configuration file
        let config: String = String::from("output:\n  translate: false\n");
        let config: Config = Config::parse_config_str(config).ok().unwrap();
        assert!(!config.output_config.translate_output);
    }

    #[test]
    fn test_config_bad_output_config() {
        let config: String = String::from("output: 5\n");
        assert_eq!(
            Config::parse_config_str(config).err().unwrap().code,
            ConfigErrorCode::YamlSyntaxError
        );
        let config: String = String::from("output:\n  translate: foobar\n");
        assert_eq!(
            Config::parse_config_str(config).err().unwrap().code,
            ConfigErrorCode::YamlSyntaxError
        );
        let config: String = String::from("output:\n  trsnlate: true\n");
        assert_eq!(
            Config::parse_config_str(config).err().unwrap().code,
            ConfigErrorCode::YamlSyntaxError
        );
    }

    #[test]
    fn test_config_language() {
        let config: String = String::from("language: bg\n");
        let config: Config = Config::parse_config_str(config).ok().unwrap();
        assert_eq!(config.language, String::from("bg"));
    }

    #[test]
    fn test_config_language_missing() {
        let config: String = String::from("output:\n  translate: false\n");
        let config: Config = Config::parse_config_str(config).ok().unwrap();
        assert_eq!(config.language, String::from("ru"));
    }

    #[test]
    #[should_panic]
    fn test_config_language_badvalue() {
        let config: String = String::from("language:\n  name: ru\n");
        assert!(Config::parse_config_str(config).is_ok());
    }

    #[test]
    fn test_config_prompt_default() {
        let config: String = String::from("language:\n  ru\n");
        let config: Config = Config::parse_config_str(config).ok().unwrap();
        let prompt_config: PromptConfig = config.prompt_config;
        assert_eq!(prompt_config.prompt_line, String::from("${USER}@${HOSTNAME}:${WRKDIR}$"));
        assert_eq!(prompt_config.break_enabled, false);
        assert_eq!(prompt_config.break_str, String::from("❯"));
        assert_eq!(prompt_config.git_branch, String::from("on "));
        assert_eq!(prompt_config.git_commit_ref, 8);
        assert_eq!(prompt_config.history_size, 256);
        assert_eq!(prompt_config.min_duration, 2000);
        assert_eq!(prompt_config.rc_err, String::from("✖"));
        assert_eq!(prompt_config.rc_ok, String::from("✔"));
        assert_eq!(prompt_config.translate, false);
    }

    #[test]
    fn test_config_prompt() {
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        let config: Config = Config::parse_config_str(config).ok().unwrap();
        //Verify config parameters
        let prompt_config: PromptConfig = config.prompt_config;
        assert_eq!(prompt_config.prompt_line, String::from("${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}"));
        assert_eq!(prompt_config.break_enabled, false);
        assert_eq!(prompt_config.break_str, String::from(">"));
        assert_eq!(prompt_config.git_branch, String::from("on "));
        assert_eq!(prompt_config.git_commit_ref, 4);
        assert_eq!(prompt_config.history_size, 1024);
        assert_eq!(prompt_config.min_duration, 5000);
        assert_eq!(prompt_config.rc_err, String::from("x_x"));
        assert_eq!(prompt_config.rc_ok, String::from("^_^"));
        assert_eq!(prompt_config.translate, true);
    }

    #[test]
    fn test_config_prompt_bad() {
        let config: String = String::from("prompt:\n  prompt_le: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  histosize: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  trslate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  bak:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    eled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    th: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  dution:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsime: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  r:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    o: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    err: \"x_x\"\n  git:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  gi:\n    branch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    brch: \"on \"\n    commit_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
        let config: String = String::from("prompt:\n  prompt_line: \"${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} (${GIT_COMMIT}) ${CMD_TIME}\"\n  history_size: 1024\n  translate: true\n  break:\n    enabled: false\n    with: \">\"\n  duration:\n    min_elapsed_time: 5000\n  rc:\n    ok: \"^_^\"\n    error: \"x_x\"\n  git:\n    branch: \"on \"\n    com_ref_len: 4\n");
        assert!(Config::parse_config_str(config).is_err());
    }

    #[test]
    fn test_bad_syntax() {
        let config: String = String::from("foobar: 5:\n");
        assert_eq!(
            Config::parse_config_str(config).err().unwrap().code,
            ConfigErrorCode::YamlSyntaxError
        );
    }

    #[test]
    fn test_empty_yaml() {
        let config: String = String::from("\n");
        assert_eq!(
            Config::parse_config_str(config).err().unwrap().code,
            ConfigErrorCode::YamlSyntaxError
        );
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
}
