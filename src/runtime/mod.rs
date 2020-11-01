//! ## Runtime
//!
//! `runtime` contains the runtime functions for pyc core

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

//Deps
extern crate ansi_term;
extern crate nix;

// Runtime modules
mod props;
mod imiop;

use ansi_term::Colour;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration};

//Config
use crate::config;
//Props
use props::RuntimeProps;
//Shell
use crate::shell::{Shell, ShellState};
use crate::shell::unixsignal::UnixSignal;
// Translator
use crate::translator::ioprocessor::IOProcessor;
use crate::translator::lang::Language;
use crate::translator::new_translator;
//Utils
use crate::utils::console;
use crate::utils::file;

//@! Runners

/// ### run_interactive
///
/// Run pyc in interactive mode

pub fn run_interactive(language: Language, config: config::Config, shell: Option<String>, history_file: Option<PathBuf>) -> u8 {
    //Instantiate Runtime Props
    let mut props: RuntimeProps = RuntimeProps::new(true, config, language);
    let processor: IOProcessor = IOProcessor::new(language, new_translator(language));
    //Determine the shell to use
    let (shell, args): (String, Vec<String>) = resolve_shell(&props.config, shell);
    //Intantiate and start a new shell
    let mut shell: Shell = match Shell::start(shell, args, &props.config.prompt_config) {
        Ok(sh) => sh,
        Err(err) => {
            print_err(
                String::from(format!("Could not start shell: {}", err)),
                props.config.output_config.translate_output,
                &processor,
            );
            return 255;
        }
    };
    //If history file is set, load history
    if let Some(history_file) = history_file.clone() {
        match file::read_lines(history_file.clone()) {
            Ok(lines) => shell.history.load(lines),
            Err(err) => print_err(
                String::from(format!("Could not load history from '{}': {}", history_file.display(), err)),
                props.config.output_config.translate_output,
                &processor,
            )
        }
    };
    //@! Main loop
    while props.get_last_state() != ShellState::Terminated {
        //@! Print prompt if state is Idle and state has changed
        let current_state: ShellState = shell.get_state();
        if current_state != props.get_last_state() {
            props.update_state(current_state);
        }
        if props.get_state_changed() && current_state == ShellState::Shell {
            //Force shellenv to refresh info
            shell.refresh_env();
            //Print prompt
            console::print(format!("{} ", shell.get_promptline(&processor)));
            props.report_state_changed_notified(); //Force state changed to false
        } else if props.get_state_changed() {
            props.report_state_changed_notified(); //Check has been done, nothing to do
        }
        //@! Read user input
        if let Some(ev) = console::read() {
            props.handle_input_event(ev, &mut shell);
        };
        //Update state after write
        let new_state = shell.get_state(); //Force last state to be changed
        if new_state != props.get_last_state() {
            props.update_state(new_state);
        }
        //@! Read Shell stdout
        read_from_shell(&mut shell, &props.config, &processor);
        //Check if shell has terminated
        sleep(Duration::from_nanos(100)); //Sleep for 100ns
    } //@! End of loop
    //Write history back to file
    if let Some(history_file) = history_file {
        let lines: Vec<String> = shell.history.dump();
        if let Err(err) = file::write_lines(history_file.clone(), lines) {
            print_err(
                String::from(format!("Could not write history to '{}': {}", history_file.display(), err)),
                props.config.output_config.translate_output,
                &processor,
            );
        }
    };
    //Return shell exitcode
    match shell.stop() {
        Ok(rc) => rc,
        Err(err) => {
            print_err(format!("Could not stop shell: {}", err), props.config.output_config.translate_output, &processor);
            255
        }
    }
}

/// ### run_command
/// 
/// Run command in shell and return
pub fn run_command(mut command: String, language: Language, config: config::Config, shell: Option<String>) -> u8 {
    //Instantiate Runtime Props
    let mut props: RuntimeProps = RuntimeProps::new(false, config, language);
    let processor: IOProcessor = IOProcessor::new(language, new_translator(language));
    //Determine the shell to use
    let (shell, args): (String, Vec<String>) = resolve_shell(&props.config, shell);
    //Intantiate and start a new shell
    let mut shell: Shell = match Shell::start(shell, args, &props.config.prompt_config) {
        Ok(sh) => sh,
        Err(err) => {
            print_err(
                String::from(format!("Could not start shell: {}", err)),
                props.config.output_config.translate_output,
                &processor,
            );
            return 255;
        }
    };
    //Prepare command
    while command.ends_with('\n') {
        command.pop();
    }
    while command.ends_with(';') {
        command.pop();
    }
    //FIXME: handle fish $status
    command.push_str("; exit $?\n");
    //Write command
    if let Err(err) = shell.write(command) {
        print_err(
            String::from(format!("Could not start shell: {}", err)),
            props.config.output_config.translate_output,
            &processor,
        );
        return 255;
    }
    let _ = shell.write(String::from("\n"));
    //@! Main loop
    loop { //Check state after reading/writing, since program could have already terminate
        //@! Read user input
        if let Some(ev) = console::read() {
            props.handle_input_event(ev, &mut shell);
        };
        //@! Read Shell stdout
        read_from_shell(&mut shell, &props.config, &processor);
        //Check if shell has terminated
        if shell.get_state() == ShellState::Terminated {
            break;
        }
        sleep(Duration::from_nanos(100)); //Sleep for 100ns
    } //@! End of main loop
    //Return shell exitcode
    match shell.stop() {
        Ok(rc) => rc,
        Err(err) => {
            print_err(format!("Could not stop shell: {}", err), props.config.output_config.translate_output, &processor);
            255
        }
    }
}

/// ### run_file
/// 
/// Run shell reading commands from file
pub fn run_file(file: String, language: Language, config: config::Config, shell: Option<String>) -> u8 {
    let file_path: &Path = Path::new(file.as_str());
    let processor: IOProcessor = IOProcessor::new(language, new_translator(language));
    let lines: Vec<String> = match file::read_lines(file_path) {
        Ok(lines) => lines,
        Err(_) => {
            print_err(format!("{}: No such file or directory", file), config.output_config.translate_output, &processor);
            return 255
        }
    };
    //Join lines in a single command
    let command: String = script_lines_to_string(&lines);
    //Execute command
    run_command(command, language, config, shell)
}

//@! Shell functions

/// ### read_from_shell
/// 
/// Read from shell stderr and stdout
fn read_from_shell(shell: &mut Shell, config: &config::Config, processor: &IOProcessor) {
    if let Ok((out, err)) = shell.read() {
        if out.is_some() {
            //Convert out to cyrillic
            print_out(out.unwrap(), config.output_config.translate_output, &processor);
        }
        if err.is_some() {
            //Convert err to cyrillic
            print_err(err.unwrap().to_string(), config.output_config.translate_output, &processor);
        }
    }
}

/// ### resolve_shell
/// 
/// Resolve shell to use from configuration and arguments
fn resolve_shell(config: &config::Config, shellopt: Option<String>) -> (String, Vec<String>) {
    match shellopt {
        Some(sh) => (sh, vec![]),
        None => (config.shell_config.exec.clone(), config.shell_config.args.clone()) //Get shell from config
    }
}

/// ### script_lines_to_string
/// 
/// Converts script lines to a single command as string
fn script_lines_to_string(lines: &Vec<String>) -> String {
    let mut command: String = String::new();
    for line in lines.iter() {
        if line.starts_with("#") {
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        command.push_str(line);
        //Don't add multiple semicolons
        if ! line.ends_with(";") {
            command.push(';');
        }
    }
    command
}

/// ### resolve_command
///
/// resolve command according to configured alias

fn resolve_command(argv: &mut Vec<String>, config: &config::Config) {
    //Process arg 0
    match config.get_alias(&argv[0]) {
        Some(resolved) => argv[0] = resolved,
        None => {}
    };
}

/*
/// ### get_shell_from_env
///
/// Try to get the shell path from SHELL environment variable
fn get_shell_from_env() -> Result<String, ()> {
    if let Ok(val) = env::var("SHELL") {
        Ok(val)
    } else {
        Err(())
    }
}
*/

//@! Prompt functions

/// ### print_err
/// 
/// print error message; the message is may converted to cyrillic if translate config is true

fn print_err(err: String, to_cyrillic: bool, processor: &IOProcessor) {
    match to_cyrillic {
        true => eprintln!("{}", Colour::Red.paint(processor.text_to_cyrillic(&err))),
        false => eprintln!("{}", Colour::Red.paint(err)),
    };
}

/// ### print_out
///
/// print normal message; the message is may converted to cyrillic if translate config is true

fn print_out(out: String, to_cyrillic: bool, processor: &IOProcessor) {
    match to_cyrillic {
        true => console::println(format!("{}", processor.text_to_cyrillic(&out))),
        false => console::println(format!("{}", out)),
    };
}

/// ### console_fmt
/// 
/// Format console message

fn console_fmt(out: String, to_cyrillic: bool, processor: &IOProcessor) -> String {
    match to_cyrillic {
        true => format!("{}", processor.text_to_cyrillic(&out)),
        false => format!("{}", out)
    }
}

/// ### shellsignal_to_signal
/// 
/// Converts a signal received on prompt to a UnixSignal
#[allow(dead_code)]
fn shellsignal_to_signal(sig: u8) -> Option<UnixSignal> {
    match sig {
        3 => Some(UnixSignal::Sigint),
        26 => Some(UnixSignal::Sigstop),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::Config;

    use crate::translator::ioprocessor::IOProcessor;
    use crate::translator::new_translator;
    use crate::translator::lang::Language;

    use std::collections::HashMap;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_runtime_read_from_shell() {
        let mut cfg: Config = Config::default();
        cfg.output_config.translate_output = true;
        let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
        let mut shell: Shell = Shell::start(String::from("sh"), vec![], &cfg.prompt_config).unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        //Write
        let _ = shell.write(String::from("echo 4\n"));
        sleep(Duration::from_millis(100));
        //Read
        read_from_shell(&mut shell, &cfg, &iop);
        //Don't translate
        cfg.output_config.translate_output = false;
        let _ = shell.write(String::from("echo 5\n"));
        sleep(Duration::from_millis(100));
        read_from_shell(&mut shell, &cfg, &iop);
        //Try stderr
        cfg.output_config.translate_output = true;
        let _ = shell.write(String::from("poropero\n"));
        sleep(Duration::from_millis(100));
        read_from_shell(&mut shell, &cfg, &iop);
        //Try stderr not translated
        cfg.output_config.translate_output = false;
        let _ = shell.write(String::from("poropero\n"));
        sleep(Duration::from_millis(100));
        read_from_shell(&mut shell, &cfg, &iop);
        //Terminate shell
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        assert!(shell.stop().is_ok());
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
    }

    #[test]
    fn test_runtime_resolve_shell() {
        let mut cfg: Config = Config::default();
        cfg.shell_config.args = vec![String::from("-i")];
        //Resolve shell without cli option
        assert_eq!(resolve_shell(&cfg, None), (String::from("bash"), vec![String::from("-i")]));
        //Resolve shell with cli option
        assert_eq!(resolve_shell(&cfg, Some(String::from("fish"))), (String::from("fish"), vec![]));
    }

    #[test]
    fn test_runtime_script_lines_to_command() {
        let lines: Vec<String> = vec![String::from("#!/bin/bash"), String::from(""), String::from("echo 4"), String::from("#this is a comment"), String::from("cat /tmp/output;")];
        assert_eq!(script_lines_to_string(&lines), String::from("echo 4;cat /tmp/output;"));
    }

    #[test]
    fn test_runtime_resolve_command() {
        let mut alias_cfg: HashMap<String, String> = HashMap::new();
        alias_cfg.insert(String::from("ll"), String::from("ls -l"));
        let cfg: Config = Config {
            language: String::from(""),
            shell_config: config::ShellConfig::default(),
            alias: alias_cfg,
            output_config: config::OutputConfig::default(),
            prompt_config: config::PromptConfig::default()
        };
        //Resolve command
        let mut argv: Vec<String> = vec![String::from("ll"), String::from("/tmp/")];
        resolve_command(&mut argv, &cfg);
        assert_eq!(*argv.get(0).unwrap(), String::from("ls -l"));

        //Unresolved command
        let mut argv: Vec<String> = vec![String::from("du"), String::from("-hs")];
        resolve_command(&mut argv, &cfg);
        assert_eq!(*argv.get(0).unwrap(), String::from("du"));
    }

    #[test]
    fn test_runtime_print() {
        let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
        //Out
        print_out(String::from("Hello"), true, &iop);
        print_out(String::from("Hello"), false, &iop);
        //Err
        print_err(String::from("Hello"), true, &iop);
        print_err(String::from("Hello"), false, &iop);
    }

    #[test]
    fn test_runtime_console_fmt() {
        let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
        //Out
        assert_eq!(console_fmt(String::from("Hello"), true, &iop), String::from("Хэлло"));
        assert_eq!(console_fmt(String::from("Hello"), false, &iop), String::from("Hello"));
    }

    #[test]
    fn test_runtime_shellsignal() {
        assert_eq!(shellsignal_to_signal(3).unwrap(), UnixSignal::Sigint);
        assert_eq!(shellsignal_to_signal(26).unwrap(), UnixSignal::Sigstop);
        assert!(shellsignal_to_signal(255).is_none());
    }

}
