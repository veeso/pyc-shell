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

use ansi_term::Colour;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration};

//Config
use crate::config;
//Shell
use crate::shell::proc::ShellState;
use crate::shell::{Shell};
use crate::shell::unixsignal::UnixSignal;
//Translator
use crate::translator::ioprocessor::IOProcessor;
//Utils
use crate::utils::buffer;
use crate::utils::console::{self, InputEvent};
use crate::utils::file;

/// ## RuntimeProps
/// 
/// Runtime Props is a wrapper for all the properties used by the Runtime module
pub(self) struct RuntimeProps {
    input_buffer: Vec<char>,
    input_buffer_cursor: usize,
    interactive: bool,
    last_state: ShellState
}

/// ### run_interactive
///
/// Run pyc in interactive mode

pub fn run_interactive(processor: IOProcessor, config: &config::Config, shell: Option<String>) -> u8 {
    //Determine the shell to use
    let (shell, args): (String, Vec<String>) = resolve_shell(&config, shell);
    //Intantiate and start a new shell
    let mut shell: Shell = match Shell::start(shell, args, &config.prompt_config) {
        Ok(sh) => sh,
        Err(err) => {
            print_err(
                String::from(format!("Could not start shell: {}", err)),
                config.output_config.translate_output,
                &processor,
            );
            return 255;
        }
    };
    //@! Main loop
    let mut last_state: ShellState = ShellState::Unknown;
    let mut state_changed: bool = true; //Start with state changed, this determines whether the prompt should be printed
    let mut input_buffer: Vec<char> = Vec::with_capacity(2048); //Expected as the maximum amount of bytes
    let mut input_buffer_cursor: usize = 0;
    while last_state != ShellState::Terminated {
        //@! Print prompt if state is Idle and state has changed
        let current_state: ShellState = shell.get_state();
        if current_state != last_state {
            state_changed = true;
            last_state = current_state;
        }
        if state_changed && current_state == ShellState::Idle {
            //Force shellenv to refresh info
            shell.refresh_env();
            //Print prompt
            console::print(format!("{} ", shell.get_promptline(&processor)));
            state_changed = false; //Force state changed to false
        } else if state_changed {
            state_changed = false; //Check has been done, nothing to do
        }
        //@! Read user input
        if let Some(ev) = console::read() {
            //Match input event
            match ev {
                InputEvent::ArrowDown => {
                    //TODO: history next
                },
                InputEvent::ArrowUp => {
                    //TODO: history prev
                },
                InputEvent::ArrowLeft => {
                    move_left(&mut input_buffer_cursor);
                },
                InputEvent::ArrowRight => {
                    move_right(&mut input_buffer_cursor, input_buffer.len());
                },
                InputEvent::Backspace => {
                    backspace(&mut input_buffer, &mut input_buffer_cursor);
                },
                InputEvent::CarriageReturn => {
                    console::carriage_return();
                },
                InputEvent::Ctrl(sig) => {
                    //Check running state 
                    //if running state is Idle, it will be handled by the console,
                    //otherwise by the shell process
                    if last_state == ShellState::Idle {
                        match sig {
                            1 => { //CTRL + A
                                //We must return at the beginning of the string
                                for _ in 0..input_buffer_cursor {
                                    //Move left
                                    console::move_cursor_left();
                                }
                                input_buffer_cursor = 0; //Reset cursor
                            }, 
                            2 => { //CTRL + B
                                move_left(&mut input_buffer_cursor);
                            },
                            3 => { //CTRL + C
                                //Abort input and go to newline
                                input_buffer.clear();
                                input_buffer_cursor = 0;
                                console::println(String::new());
                                console::print(format!("{} ", shell.get_promptline(&processor)));
                            },
                            4 => { //CTRL + D
                                backspace(&mut input_buffer, &mut input_buffer_cursor);
                            },
                            5 => { //CTRL + E
                                for _ in input_buffer_cursor..input_buffer.len() {
                                    console::move_cursor_right();
                                }
                                input_buffer_cursor = input_buffer.len();
                            },
                            6 => { //CTRL + F
                                move_right(&mut input_buffer_cursor, input_buffer.len());
                            },
                            7 => { //CTRL + G
                                //TODO: exit rev search
                            },
                            8 => { //CTRL + H
                                backspace(&mut input_buffer, &mut input_buffer_cursor);
                            },
                            11 => { // CTRL + K
                                //Delete all characters after cursor
                                while input_buffer_cursor < input_buffer.len() {
                                    let _ = input_buffer.pop();
                                }
                            },
                            12 => { // CTRL + L
                                //Clear, but doesn't reset input
                                console::clear();
                                console::print(format!("{} {}", shell.get_promptline(&processor), buffer::chars_to_string(&input_buffer)));
                            },
                            18 => { // CTRL + R
                                //TODO: rev search
                            },
                            _ => {} //Unhandled
                        }
                    } else {
                        //Pass to child
                        //FIXME: doesn't work
                        //let mut output = String::with_capacity(1);
                        //output.push(sig as char);
                        //let _ = shell.write(output);
                        if let Some(sig) = shellsignal_to_signal(sig) {
                            if let Err(_) = shell.raise(sig) {
                                print_err(String::from("Could not send signal to shell"), config.output_config.translate_output, &processor);
                            }
                        }
                    }
                },
                InputEvent::Key(k) => { //Push key
                    //Push k to input buffer
                    for ch in k.chars() {
                        input_buffer.insert(input_buffer_cursor, ch);
                        input_buffer_cursor += 1;
                    }
                    //Print key
                    console::print(k);
                },
                InputEvent::Enter => { //@! Send input
                    //@! Handle enter...
                    //Newline first
                    console::println(String::new());
                    //Convert input buffer to string
                    let stdin_input: String = buffer::chars_to_string(&input_buffer);
                    //If input is empty, print prompt (if state is IDLE)
                    if stdin_input.trim().len() == 0 {
                        if last_state == ShellState::Idle {
                            console::print(format!("{} ", shell.get_promptline(&processor)));
                        }
                    } else {
                        //Treat input
                        //If state is Idle, convert expression, otherwise convert text
                        let input: String = match last_state {
                            ShellState::Idle => {
                                //Resolve alias
                                let mut argv: Vec<String> = Vec::with_capacity(stdin_input.matches(" ").count() + 1);
                                for arg in stdin_input.split_whitespace() {
                                    argv.push(String::from(arg));
                                }
                                //Process arg 0
                                resolve_command(&mut argv, &config);
                                //Rejoin arguments
                                let input: String = argv.join(" ") + "\n";
                                match processor.expression_to_latin(&input) {
                                    Ok(ex) => ex,
                                    Err(err) => {
                                        print_err(String::from(format!("Input error: {:?}", err)), config.output_config.translate_output, &processor);
                                        //Clear input buffer
                                        input_buffer.clear();
                                        input_buffer_cursor = 0;
                                        continue;
                                    }
                                }
                            },
                            ShellState::SubprocessRunning => processor.text_to_latin(&buffer::chars_to_string(&input_buffer)),
                            _ => continue
                        };
                        //Clear input buffer
                        input_buffer.clear();
                        input_buffer_cursor = 0;
                        if last_state == ShellState::Idle {
                            //Check if clear command
                            if input.starts_with("clear") {
                                //Clear screen, then write prompt
                                console::clear();
                                console::print(format!("{} ", shell.get_promptline(&processor)));
                            } else if input.starts_with("history") {
                                //TODO: print history
                            } else if input.starts_with("!") {
                                //TODO: command from history
                            } else { //Write input as usual
                                if let Err(err) = shell.write(input) {
                                    print_err(
                                        String::from(err.to_string()),
                                        config.output_config.translate_output,
                                        &processor,
                                    );
                                }
                            }
                        } else { //Write input as usual
                            if let Err(err) = shell.write(input) {
                                print_err(
                                    String::from(err.to_string()),
                                    config.output_config.translate_output,
                                    &processor,
                                );
                            }
                        }
                        //Update state after write
                        let new_state = shell.get_state(); //Force last state to be changed
                        if new_state != last_state {
                            last_state = new_state;
                            state_changed = true;
                        }
                    }
                }
            }
        };
        //@! Read Shell stdout
        read_from_shell(&mut shell, &config, &processor);
        //Check if shell has terminated
        sleep(Duration::from_nanos(100)); //Sleep for 100ns
    } //@! End of loop
    //Return shell exitcode
    match shell.stop() {
        Ok(rc) => rc,
        Err(err) => {
            print_err(format!("Could not stop shell: {}", err), config.output_config.translate_output, &processor);
            255
        }
    }
}

/// ### run_command
/// 
/// Run command in shell and return
pub fn run_command(mut command: String, processor: IOProcessor, config: &config::Config, shell: Option<String>) -> u8 {
    //Determine the shell to use
    let (shell, args): (String, Vec<String>) = resolve_shell(&config, shell);
    //Intantiate and start a new shell
    let mut shell: Shell = match Shell::start(shell, args, &config.prompt_config) {
        Ok(sh) => sh,
        Err(err) => {
            print_err(
                String::from(format!("Could not start shell: {}", err)),
                config.output_config.translate_output,
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
            config.output_config.translate_output,
            &processor,
        );
        return 255;
    }
    let _ = shell.write(String::from("\n"));
    let mut input_buffer: Vec<char> = Vec::with_capacity(2048);
    let mut input_buffer_cursor: usize = 0;
    //@! Main loop
    loop { //Check state after reading/writing, since program could have already terminate
        //@! Read user input
        if let Some(ev) = console::read() {
            //Match input event
            match ev {
                InputEvent::CarriageReturn => {
                    console::carriage_return();
                },
                InputEvent::ArrowLeft => {
                    move_left(&mut input_buffer_cursor);
                },
                InputEvent::ArrowRight => {
                    move_right(&mut input_buffer_cursor, input_buffer.len());
                },
                InputEvent::Backspace => {
                    backspace(&mut input_buffer, &mut input_buffer_cursor);
                },
                InputEvent::Ctrl(sig) => {
                    //Send signal
                    if let Some(sig) = shellsignal_to_signal(sig) {
                        if let Err(_) = shell.raise(sig) {
                            print_err(String::from("Could not send signal to shell"), config.output_config.translate_output, &processor);
                        }
                    }
                },
                InputEvent::Key(k) => {
                    //Push k to input buffer
                    for ch in k.chars() {
                        input_buffer.insert(input_buffer_cursor, ch);
                        input_buffer_cursor += 1;
                    }
                    //Print key
                    console::print(k);
                },
                InputEvent::Enter => {
                    //@! Handle enter...
                    let stdin_input: String = buffer::chars_to_string(&input_buffer);
                    //If input is empty, ignore it
                    if stdin_input.trim().len() > 0 {
                        //Treat input
                        //Convert text
                        let input: String = processor.text_to_latin(&stdin_input);
                        if let Err(err) = shell.write(input) {
                            print_err(
                                String::from(err.to_string()),
                                config.output_config.translate_output,
                                &processor,
                            );
                        }
                    }
                    //Clear input buffer
                    input_buffer.clear();
                    input_buffer_cursor = 0;
                },
                _ => {}
            }
        };
        //@! Read Shell stdout
        read_from_shell(&mut shell, &config, &processor);
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
            print_err(format!("Could not stop shell: {}", err), config.output_config.translate_output, &processor);
            255
        }
    }
}

/// ### run_file
/// 
/// Run shell reading commands from file
pub fn run_file(file: String, processor: IOProcessor, config: &config::Config, shell: Option<String>) -> u8 {
    let file_path: &Path = Path::new(file.as_str());
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
    run_command(command, processor, config, shell)
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

/// ### backspace
/// 
/// Perform backspace on current console and buffers
fn backspace(input_buffer: &mut Vec<char>, cursor: &mut usize) {
    //Remove from buffer and backspace (if possible)
    if *cursor > 0 {
        *cursor -= 1;
        if input_buffer.len() > *cursor {
            input_buffer.remove(*cursor);
        }
        console::backspace();
    }
}

/// ### move_left
/// 
/// Move cursor to left
fn move_left(cursor: &mut usize) {
    //If possible, move the cursor right
    if *cursor != 0 {
        *cursor -= 1;
        console::move_cursor_left();
    }
}

/// ### move_right
/// 
/// Move cursor to right
fn move_right(cursor: &mut usize, buflen: usize) {
     //If possible, move the cursor left
     if *cursor + 1 <= buflen {
        *cursor += 1;
        console::move_cursor_right();
    }
}

/// ### shellsignal_to_signal
/// 
/// Converts a signal received on prompt to a UnixSignal
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
    use crate::translator::Language;

    use std::collections::HashMap;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_runtime_read_from_shell() {
        let mut cfg: Config = Config::default();
        cfg.output_config.translate_output = true;
        let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
        let mut shell: Shell = Shell::start(String::from("sh"), vec![], &cfg.prompt_config).unwrap();
        sleep(Duration::from_millis(100));
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
        assert!(shell.stop().is_ok());
        sleep(Duration::from_millis(250));
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
    fn test_runtime_backspace() {
        let mut input_buffer: Vec<char> = vec!['a', 'b', 'c'];
        let mut cursor: usize = 0;
        //If cursor is 0, cursor and input buffer won't change
        backspace(&mut input_buffer, &mut cursor);
        assert_eq!(cursor, 0);
        assert_eq!(input_buffer.len(), 3);
        cursor = 3;
        //Backspace from end of buffer
        backspace(&mut input_buffer, &mut cursor);
        assert_eq!(cursor, 2);
        assert_eq!(input_buffer, vec!['a', 'b']);
        //Set cursor to 1 and backspace from the middle
        cursor = 1;
        backspace(&mut input_buffer, &mut cursor);
        assert_eq!(cursor, 0);
        assert_eq!(input_buffer, vec!['b']);
        //Try to delete with cursor out of range
        let mut input_buffer: Vec<char> = vec!['a', 'b', 'c'];
        let mut cursor: usize = 4;
        backspace(&mut input_buffer, &mut cursor);
        assert_eq!(cursor, 3);
        assert_eq!(input_buffer.len(), 3);
    }

    #[test]
    fn test_runtime_move_cursor() {
        let mut cursor: usize = 12;
        move_left(&mut cursor);
        assert_eq!(cursor, 11);
        cursor = 0;
        move_left(&mut cursor);
        assert_eq!(cursor, 0);
        let buflen: usize = 255;
        move_right(&mut cursor, buflen);
        assert_eq!(cursor, 1);
        let buflen: usize = 1;
        move_right(&mut cursor, buflen);
        assert_eq!(cursor, 1);
    }

    #[test]
    fn test_runtime_shellsignal() {
        assert_eq!(shellsignal_to_signal(3).unwrap(), UnixSignal::Sigint);
        assert_eq!(shellsignal_to_signal(26).unwrap(), UnixSignal::Sigstop);
        assert!(shellsignal_to_signal(255).is_none());
    }

}
