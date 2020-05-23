//! # Prompt
//!
//! `prompt` is the module which takes care of processing the shell prompt

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

extern crate regex;

mod cache;
mod modules;

use super::Shell;
use crate::config::PromptConfig;
use crate::translator::ioprocessor::IOProcessor;
use cache::PromptCache;
use modules::*;

use regex::Regex;
use std::io::{self, Write};
use std::time::Duration;

const PROMPT_KEY_REGEX: &str = r"\$\{(.*?)\}";
//Prompt keys
const PROMPT_USER: &str = "${USER}";
const PROMPT_HOSTNAME: &str = "${HOSTNAME}";
const PROMPT_WRKDIR: &str = "${WRKDIR}";
const PROMPT_LANG: &str = "${LANG}";
const PROMPT_CMDTIME: &str = "${CMD_TIME}";
const PROMPT_RC: &str = "${RC}";
//Prompt colors
pub(crate) const PROMPT_KRED: &str = "${KRED}";
pub(crate) const PROMPT_KYEL: &str = "${KYEL}";
pub(crate) const PROMPT_KGRN: &str = "${KGRN}";
pub(crate) const PROMPT_KBLU: &str = "${KBLU}";
pub(crate) const PROMPT_KCYN: &str = "${KCYN}";
pub(crate) const PROMPT_KMAG: &str = "${KMAG}";
pub(crate) const PROMPT_KBLK: &str = "${KBLK}";
pub(crate) const PROMPT_KGRY: &str = "${KGRY}";
pub(crate) const PROMPT_KWHT: &str = "${KWHT}";
pub(crate) const PROMPT_KRST: &str = "${KRST}";
//Git
const PROMPT_GIT_BRANCH: &str = "${GIT_BRANCH}";
const PROMPT_GIT_COMMIT: &str = "${GIT_COMMIT}";

/// ## ShellPrompt
///
/// ShellPrompt is the struct which contains the current shell prompt configuration
pub struct ShellPrompt {
    prompt_line: String,
    translate: bool,
    break_opt: Option<BreakOptions>,
    duration_opt: Option<DurationOptions>,
    rc_opt: Option<RcOptions>,
    git_opt: Option<GitOptions>,
    cache: PromptCache,
}

/// ## ShellPrompt
///
/// ShellPrompt is the struct which contains the current shell prompt configuration
struct BreakOptions {
    pub break_with: String,
}

/// ## DurationOptions
///
/// DurationOptions is the struct which contains the current duration configuration
struct DurationOptions {
    pub minimum: Duration,
}

/// ## RcOptions
///
/// RcOptions is the struct which contains the return code configuration
struct RcOptions {
    pub ok: String,
    pub err: String,
}

/// ## GitOptions
///
/// GitOptions is the struct which contains the current git module configuration
struct GitOptions {
    pub branch: String,
    pub commit_ref_len: usize,
}

impl ShellPrompt {
    /// ### new
    ///
    /// Instantiate a new ShellPrompt with the provided parameters
    pub fn new(prompt_opt: &PromptConfig) -> ShellPrompt {
        let break_opt: Option<BreakOptions> = match prompt_opt.break_enabled {
            true => Some(BreakOptions::new(&prompt_opt.break_str)),
            false => None,
        };
        let duration_opt: Option<DurationOptions> =
            match DurationOptions::should_enable(&prompt_opt.prompt_line) {
                true => Some(DurationOptions::new(prompt_opt.min_duration)),
                false => None,
            };
        let rc_opt: Option<RcOptions> = match RcOptions::should_enable(&prompt_opt.prompt_line) {
            true => Some(RcOptions::new(&prompt_opt.rc_ok, &prompt_opt.rc_err)),
            false => None,
        };
        let git_opt: Option<GitOptions> = match GitOptions::should_enable(&prompt_opt.prompt_line) {
            true => Some(GitOptions::new(
                &prompt_opt.git_branch,
                prompt_opt.git_commit_ref,
            )),
            false => None,
        };
        ShellPrompt {
            prompt_line: prompt_opt.prompt_line.clone(),
            translate: prompt_opt.translate,
            break_opt: break_opt,
            duration_opt: duration_opt,
            rc_opt: rc_opt,
            git_opt: git_opt,
            cache: PromptCache::new(),
        }
    }

    /// ### print
    ///
    /// Print prompt with resolved values
    pub fn print(&mut self, shell_env: &Shell, processor: &IOProcessor) {
        let mut prompt_line: String = self.process_prompt(shell_env, processor);
        //Translate prompt if necessary
        if self.translate {
            prompt_line = processor.text_to_cyrillic(prompt_line);
        }
        //Write prompt
        print!("{} ", prompt_line);
        let _ = io::stdout().flush();
    }

    /// ### process_prompt
    ///
    /// Process prompt keys and resolve prompt line
    /// Returns the processed prompt line
    /// This function is optimized to try to cache the previous values
    fn process_prompt(&mut self, shell_env: &Shell, processor: &IOProcessor) -> String {
        let mut prompt_line: String = self.prompt_line.clone();
        //Iterate over keys through regex ```\${(.*?)}```
        lazy_static! {
            static ref RE: Regex = Regex::new(PROMPT_KEY_REGEX).unwrap();
        }
        for regex_match in RE.captures_iter(prompt_line.clone().as_str()) {
            let mtch: String = String::from(&regex_match[0]);
            let replace_with: String = self.resolve_key(shell_env, processor, &mtch);
            prompt_line = prompt_line.replace(mtch.as_str(), replace_with.as_str());
        }
        //Trim prompt line
        prompt_line = String::from(prompt_line.trim());
        //If break, break line
        if let Some(brkopt) = &self.break_opt {
            prompt_line += "\n";
            prompt_line += brkopt.break_with.trim();
        }
        //Invalidate cache
        self.cache.invalidate();
        //Return prompt line
        prompt_line
    }

    /// ### resolve_key
    ///
    /// Replace the provided key with the resolved value
    fn resolve_key(
        &mut self,
        shell_env: &Shell,
        processor: &IOProcessor,
        key: &String,
    ) -> String {
        match key.as_str() {
            PROMPT_CMDTIME => {
                let elapsed_time: Duration = shell_env.elapsed_time();
                match &self.duration_opt {
                    Some(opt) => {
                        if elapsed_time.as_millis() >= opt.minimum.as_millis() {
                            let millis: u128 = elapsed_time.as_millis();
                            let secs: f64 = (millis as f64 / 1000 as f64) as f64;
                            String::from(format!("took {:.1}s", secs))
                        } else {
                            String::from("")
                        }
                    }
                    None => String::from(""),
                }
            }
            PROMPT_GIT_BRANCH => {
                if self.git_opt.is_none() {
                    return String::from("");
                }
                //If repository is not cached, find repository
                if self.cache.get_cached_git().is_none() {
                    let repo_opt = git::find_repository(&shell_env.wrkdir());
                    match repo_opt {
                        Some(repo) => self.cache.cache_git(repo),
                        None => return String::from(""),
                    };
                }
                //Get branch (unwrap without fear; can't be None here)
                let branch: String = match git::get_branch(self.cache.get_cached_git().unwrap()) {
                    Some(branch) => branch,
                    None => return String::from(""),
                };
                //Format branch
                String::from(format!(
                    "{}{}",
                    self.git_opt.as_ref().unwrap().branch.clone(),
                    branch
                ))
            }
            PROMPT_GIT_COMMIT => {
                if self.git_opt.is_none() {
                    return String::from("");
                }
                //If repository is not cached, find repository
                if self.cache.get_cached_git().is_none() {
                    let repo_opt = git::find_repository(&shell_env.wrkdir());
                    match repo_opt {
                        Some(repo) => self.cache.cache_git(repo),
                        None => return String::from(""),
                    };
                }
                //Get commit (unwrap without fear; can't be None here)
                match git::get_commit(
                    self.cache.get_cached_git().unwrap(),
                    self.git_opt.as_ref().unwrap().commit_ref_len,
                ) {
                    Some(commit) => commit,
                    None => String::from(""),
                }
            }
            PROMPT_HOSTNAME => shell_env.hostname.clone(),
            PROMPT_KBLK | PROMPT_KBLU | PROMPT_KCYN | PROMPT_KGRN | PROMPT_KGRY | PROMPT_KMAG | PROMPT_KRED | PROMPT_KRST | PROMPT_KWHT | PROMPT_KYEL => colors::PromptColor::from_key(key.as_str()).to_string(),
            PROMPT_LANG => language::language_to_str(processor.language),
            PROMPT_RC => match &self.rc_opt {
                Some(opt) => match shell_env.exit_status() {
                    0 => opt.ok.clone(),
                    _ => opt.err.clone(),
                },
                None => String::from(""),
            },
            PROMPT_USER => shell_env.username.clone(),
            PROMPT_WRKDIR => shell_env.wrkdir().as_path().display().to_string(),
            _ => key.clone(), //Keep unresolved keys
        }
    }
}

impl BreakOptions {
    /// ### new
    ///
    /// Instantiate a new BreakOptions with the provided parameters
    pub fn new(break_with: &String) -> BreakOptions {
        BreakOptions {
            break_with: break_with.clone(),
        }
    }
}

impl DurationOptions {
    /// ### should_enable
    ///
    /// helper which says if duration module should be enabled
    pub fn should_enable(prompt_line: &String) -> bool {
        prompt_line.contains(PROMPT_CMDTIME)
    }

    /// ### new
    ///
    /// Instantiate a new DurationOptions with the provided parameters
    pub fn new(min_duration: usize) -> DurationOptions {
        DurationOptions {
            minimum: Duration::from_millis(min_duration as u64),
        }
    }
}

impl RcOptions {
    /// ### should_enable
    ///
    /// helper which says if rc module should be enabled
    pub fn should_enable(prompt_line: &String) -> bool {
        prompt_line.contains(PROMPT_RC)
    }

    /// ### new
    ///
    /// Instantiate a new RcOptions with the provided parameters
    pub fn new(ok_str: &String, err_str: &String) -> RcOptions {
        RcOptions {
            ok: ok_str.clone(),
            err: err_str.clone(),
        }
    }
}

impl GitOptions {
    /// ### should_enable
    ///
    /// helper which says if git module should be enabled
    pub fn should_enable(prompt_line: &String) -> bool {
        prompt_line.contains(PROMPT_GIT_BRANCH) || prompt_line.contains(PROMPT_GIT_COMMIT)
    }

    /// ### new
    ///
    /// Instantiate a new GitOptions with the provided parameters
    pub fn new(branch: &String, commit: usize) -> GitOptions {
        GitOptions {
            branch: branch.clone(),
            commit_ref_len: commit,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::config::PromptConfig;
    use crate::shell::{Shell};
    use crate::translator::ioprocessor::IOProcessor;
    use crate::translator::new_translator;
    use crate::translator::Language;
    use colors::PromptColor;

    use git2::Repository;
    use std::path::PathBuf;
    use std::time::Duration;

    #[test]
    fn test_prompt_simple() {
        let prompt_config_default = PromptConfig::default();
        let mut prompt: ShellPrompt = ShellPrompt::new(&prompt_config_default);
        let iop: IOProcessor = get_ioprocessor();
        let mut shellenv: Shell = get_shellenv();
        //Print first in latin
        prompt.print(&shellenv, &iop);
        prompt.translate = true;
        //Then in cyrillic
        prompt.print(&shellenv, &iop);
        //Get prompt line
        let prompt_line: String = prompt.process_prompt(&shellenv, &iop);
        let expected_prompt_line = String::from(format!(
            "{}@{}:{}$",
            shellenv.username.clone(),
            shellenv.hostname.clone(),
            shellenv.wrkdir().display()
        ));
        assert_eq!(prompt_line, expected_prompt_line);
        //Terminate shell at the end of a test
        terminate_shell(&mut shellenv);
        println!("\n");
    }

    #[test]
    fn test_prompt_colors() {
        let mut prompt_config_default = PromptConfig::default();
        //Update prompt line
        prompt_config_default.prompt_line = String::from("${KRED}RED${KYEL}YEL${KBLU}BLU${KGRN}GRN${KWHT}WHT${KGRY}GRY${KBLK}BLK${KMAG}MAG${KCYN}CYN${KRST}");
        let mut prompt: ShellPrompt = ShellPrompt::new(&prompt_config_default);
        let iop: IOProcessor = get_ioprocessor();
        let mut shellenv: Shell = get_shellenv();
        //Print first in latin
        prompt.print(&shellenv, &iop);
        prompt.translate = true;
        //Then in cyrillic
        prompt.print(&shellenv, &iop);
        //Get prompt line
        let prompt_line: String = prompt.process_prompt(&shellenv, &iop);
        let expected_prompt_line = String::from(format!(
            "{}RED{}YEL{}BLU{}GRN{}WHT{}GRY{}BLK{}MAG{}CYN{}",
            PromptColor::Red.to_string(),
            PromptColor::Yellow.to_string(),
            PromptColor::Blue.to_string(),
            PromptColor::Green.to_string(),
            PromptColor::White.to_string(),
            PromptColor::Gray.to_string(),
            PromptColor::Black.to_string(),
            PromptColor::Magenta.to_string(),
            PromptColor::Cyan.to_string(),
            PromptColor::Reset.to_string()
        ));
        assert_eq!(prompt_line, expected_prompt_line);
        //Terminate shell at the end of a test
        terminate_shell(&mut shellenv);
        println!("\n");
    }

    #[test]
    fn test_prompt_lang_time_with_break() {
        let mut prompt_config_default = PromptConfig::default();
        //Update prompt line
        prompt_config_default.prompt_line = String::from("${LANG} ~ ${KYEL}${USER}${KRST} on ${KGRN}${HOSTNAME}${KRST} in ${KCYN}${WRKDIR}${KRST} ${KYEL}${CMD_TIME}${KRST}");
        prompt_config_default.break_enabled = true;
        let mut prompt: ShellPrompt = ShellPrompt::new(&prompt_config_default);
        let iop: IOProcessor = get_ioprocessor();
        let mut shellenv: Shell = get_shellenv();
        shellenv.process.exec_time = Duration::from_millis(5100);
        shellenv.process.wrkdir = PathBuf::from("/tmp/");
        //Print first in latin
        prompt.print(&shellenv, &iop);
        prompt.translate = true;
        //Then in cyrillic
        prompt.print(&shellenv, &iop);
        //Get prompt line
        let prompt_line: String = prompt.process_prompt(&shellenv, &iop);
        let expected_prompt_line = String::from(format!(
            "{} ~ {}{}{} on {}{}{} in {}{}{} {}took 5.1s{}\n❯",
            language::language_to_str(Language::Russian),
            PromptColor::Yellow.to_string(),
            shellenv.username.clone(),
            PromptColor::Reset.to_string(),
            PromptColor::Green.to_string(),
            shellenv.hostname.clone(),
            PromptColor::Reset.to_string(),
            PromptColor::Cyan.to_string(),
            shellenv.wrkdir().display(),
            PromptColor::Reset.to_string(),
            PromptColor::Yellow.to_string(),
            PromptColor::Reset.to_string()
        ));
        assert_eq!(prompt_line, expected_prompt_line);
        //Terminate shell at the end of a test
        terminate_shell(&mut shellenv);
        println!("\n");
    }

    #[test]
    fn test_prompt_git() {
        //Get current git info
        //Initialize module
        let repo: Repository = git::find_repository(&PathBuf::from("./")).unwrap();
        //Branch should be none
        let branch: String = git::get_branch(&repo).unwrap();
        let commit: String = git::get_commit(&repo, 8).unwrap();
        let mut prompt_config_default = PromptConfig::default();
        //Update prompt line
        prompt_config_default.prompt_line =
            String::from("${USER}@${HOSTNAME}:${WRKDIR} ${GIT_BRANCH}:${GIT_COMMIT}");
        let mut prompt: ShellPrompt = ShellPrompt::new(&prompt_config_default);
        let iop: IOProcessor = get_ioprocessor();
        let mut shellenv: Shell = get_shellenv();
        shellenv.process.exec_time = Duration::from_millis(5100);
        shellenv.process.wrkdir = PathBuf::from("./");
        //Print first in latin
        prompt.print(&shellenv, &iop);
        prompt.translate = true;
        //Then in cyrillic
        prompt.print(&shellenv, &iop);
        //Get prompt line
        let prompt_line: String = prompt.process_prompt(&shellenv, &iop);
        let expected_prompt_line = String::from(format!(
            "{}@{}:{} on {}:{}",
            shellenv.username.clone(),
            shellenv.hostname.clone(),
            shellenv.wrkdir().display(),
            branch,
            commit
        ));
        assert_eq!(prompt_line, expected_prompt_line);
        //Terminate shell at the end of a test
        terminate_shell(&mut shellenv);
        println!("\n");
    }

    #[test]
    fn test_prompt_git_not_in_repo() {
        let mut prompt_config_default = PromptConfig::default();
        //Update prompt line
        prompt_config_default.prompt_line =
            String::from("${USER}@${HOSTNAME}:${WRKDIR} ${GIT_BRANCH} ${GIT_COMMIT}");
        let mut prompt: ShellPrompt = ShellPrompt::new(&prompt_config_default);
        let iop: IOProcessor = get_ioprocessor();
        let mut shellenv: Shell = get_shellenv();
        shellenv.process.exec_time = Duration::from_millis(5100);
        shellenv.process.wrkdir = PathBuf::from("/");
        //Print first in latin
        prompt.print(&shellenv, &iop);
        prompt.translate = true;
        //Then in cyrillic
        prompt.print(&shellenv, &iop);
        //Get prompt line
        let prompt_line: String = prompt.process_prompt(&shellenv, &iop);
        let expected_prompt_line = String::from(format!(
            "{}@{}:{}",
            shellenv.username.clone(),
            shellenv.hostname.clone(),
            shellenv.wrkdir().display()
        ));
        assert_eq!(prompt_line, expected_prompt_line);
        //Terminate shell at the end of a test
        terminate_shell(&mut shellenv);
        println!("\n");
    }

    #[test]
    fn test_prompt_rc_ok() {
        let mut prompt_config_default = PromptConfig::default();
        //Update prompt line
        prompt_config_default.prompt_line = String::from("${RC} ${USER}@${HOSTNAME}:${WRKDIR}");
        let mut prompt: ShellPrompt = ShellPrompt::new(&prompt_config_default);
        let iop: IOProcessor = get_ioprocessor();
        let mut shellenv: Shell = get_shellenv();
        shellenv.process.exec_time = Duration::from_millis(5100);
        shellenv.process.wrkdir = PathBuf::from("/");
        //Print first in latin
        prompt.print(&shellenv, &iop);
        prompt.translate = true;
        //Then in cyrillic
        prompt.print(&shellenv, &iop);
        //Get prompt line
        let prompt_line: String = prompt.process_prompt(&shellenv, &iop);
        let expected_prompt_line = String::from(format!(
            "✔ {}@{}:{}",
            shellenv.username.clone(),
            shellenv.hostname.clone(),
            shellenv.wrkdir().display()
        ));
        assert_eq!(prompt_line, expected_prompt_line);
        //Terminate shell at the end of a test
        terminate_shell(&mut shellenv);
        println!("\n");
    }

    #[test]
    fn test_prompt_rc_error() {
        let mut prompt_config_default = PromptConfig::default();
        //Update prompt line
        prompt_config_default.prompt_line = String::from("${RC} ${USER}@${HOSTNAME}:${WRKDIR}");
        let mut prompt: ShellPrompt = ShellPrompt::new(&prompt_config_default);
        let iop: IOProcessor = get_ioprocessor();
        let mut shellenv: Shell = get_shellenv();
        shellenv.process.exec_time = Duration::from_millis(5100);
        shellenv.process.wrkdir = PathBuf::from("/");
        shellenv.process.exit_status = 255;
        //Print first in latin
        prompt.print(&shellenv, &iop);
        prompt.translate = true;
        //Then in cyrillic
        prompt.print(&shellenv, &iop);
        //Get prompt line
        let prompt_line: String = prompt.process_prompt(&shellenv, &iop);
        let expected_prompt_line = String::from(format!(
            "✖ {}@{}:{}",
            shellenv.username.clone(),
            shellenv.hostname.clone(),
            shellenv.wrkdir().display()
        ));
        assert_eq!(prompt_line, expected_prompt_line);
        //Terminate shell at the end of a test
        terminate_shell(&mut shellenv);
        println!("\n");
    }

    #[test]
    fn test_prompt_unresolved() {
        let mut prompt_config_default = PromptConfig::default();
        //Update prompt line
        prompt_config_default.prompt_line = String::from("${USER}@${HOSTNAME}:${WRKDIR} ${FOOBAR}");
        let mut prompt: ShellPrompt = ShellPrompt::new(&prompt_config_default);
        let iop: IOProcessor = get_ioprocessor();
        let mut shellenv: Shell = get_shellenv();
        shellenv.process.exec_time = Duration::from_millis(5100);
        shellenv.process.wrkdir = PathBuf::from("/");
        shellenv.process.exit_status = 255;
        //Print first in latin
        prompt.print(&shellenv, &iop);
        prompt.translate = true;
        //Then in cyrillic
        prompt.print(&shellenv, &iop);
        //Get prompt line
        let prompt_line: String = prompt.process_prompt(&shellenv, &iop);
        let expected_prompt_line = String::from(format!(
            "{}@{}:{} {}",
            shellenv.username.clone(),
            shellenv.hostname.clone(),
            shellenv.wrkdir().display(),
            "${FOOBAR}"
        ));
        assert_eq!(prompt_line, expected_prompt_line);
        //Terminate shell at the end of a test
        terminate_shell(&mut shellenv);
        println!("\n");
    }

    fn get_ioprocessor() -> IOProcessor {
        IOProcessor::new(Language::Russian, new_translator(Language::Russian))
    }

    fn get_shellenv() -> Shell {
        Shell::start(String::from("/bin/sh")).unwrap()
    }

    fn terminate_shell(shell: &mut Shell) {
        assert!(shell.write(String::from("exit 0\n")).is_ok());
        let _ = shell.stop();
    }
}
