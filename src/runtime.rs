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
use std::env;
use std::path::Path;
use std::thread::sleep;
use std::time::{Duration};

use crate::config;
use crate::shell::proc::ShellState;
use crate::shell::{Shell};
use crate::translator::ioprocessor::IOProcessor;
use crate::utils::console::{self, InputEvent};
use crate::utils::file;

/// ### run_interactive
///
/// Run pyc in interactive mode

pub fn run_interactive(processor: IOProcessor, config: &config::Config, shell: Option<String>) -> u8 {
    //Determine the shell to use
    let (shell, args): (String, Vec<String>) = match shell {
        Some(sh) => (sh, vec![]),
        None => (config.shell_config.exec.clone(), config.shell_config.args.clone()) //Get shell from config
    };
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
    let mut input_buffer: String = String::new();
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
                InputEvent::Backspace => {
                    //Pop from buffer and backspace (only if was Some)
                    if let Some(_) = input_buffer.pop() {
                        console::backspace();
                    }
                },
                InputEvent::CarriageReturn => {
                    console::carriage_return();
                },
                InputEvent::Ctrl(sig) => {
                    //TODO: match signal
                },
                InputEvent::Key(k) => {
                    //Push k to input buffer
                    input_buffer.push_str(k.as_str());
                    //Print key
                    console::print(k);
                },
                InputEvent::Enter => {
                    //Newline first
                    console::println(String::new());
                    //Handle enter...
                    //If input is empty, print prompt (if state is IDLE)
                    if input_buffer.trim().len() == 0 {
                        if last_state == ShellState::Idle {
                            console::print(format!("{} ", shell.get_promptline(&processor)));
                        }
                    } else {
                        //Treat input
                        //If state is Idle, convert expression, otherwise convert text
                        let input: String = match last_state {
                            ShellState::Idle => {
                                //Resolve alias
                                let mut argv: Vec<String> = Vec::with_capacity(input_buffer.matches(" ").count() + 1);
                                for arg in input_buffer.split_whitespace() {
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
                                        continue;
                                    }
                                }
                            },
                            ShellState::SubprocessRunning => processor.text_to_latin(&input_buffer),
                            _ => continue
                        };
                        //Clear input buffer
                        input_buffer.clear();
                        if last_state == ShellState::Idle {
                            //Check if clear command
                            if input.starts_with("clear") {
                                //Clear screen, then write prompt
                                console::clear();
                                console::print(format!("{} ", shell.get_promptline(&processor)));
                            } else if input.starts_with("history") {
                                //TODO: print history
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
                },
                _ => {}
            }
        };
        //@! Read Shell stdout
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
        sleep(Duration::from_nanos(10)); //Sleep for 10ns
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
    let (shell, args): (String, Vec<String>) = match shell {
        Some(sh) => (sh, vec![]),
        None => (config.shell_config.exec.clone(), config.shell_config.args.clone()) //Get shell from config
    };
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
    //Write command
    while command.ends_with('\n') {
        command.pop();
    }
    while command.ends_with(';') {
        command.pop();
    }
    //FIXME: handle fish $status
    command.push_str("; exit $?\n");
    if let Err(err) = shell.write(command) {
        print_err(
            String::from(format!("Could not start shell: {}", err)),
            config.output_config.translate_output,
            &processor,
        );
        return 255;
    }
    let _ = shell.write(String::from("\n"));
    let mut input_buffer: String = String::new();
    //@! Main loop
    loop { //Check state after reading/writing, since program could have already terminate
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
                InputEvent::Backspace => {
                    //Pop from buffer and backspace
                    let _ = input_buffer.pop();
                    console::backspace();
                },
                InputEvent::CarriageReturn => {
                    console::carriage_return();
                },
                InputEvent::Ctrl(sig) => {
                    //TODO: match signal
                },
                InputEvent::Key(k) => {
                    //Push k to input buffer
                    input_buffer.push_str(k.as_str());
                    //Print key
                    console::print(k);
                },
                InputEvent::Enter => {
                    //Handle enter...
                    //If input is empty, ignore it
                    if input_buffer.trim().len() > 0 {
                        //Treat input
                        //Convert text
                        let input: String = processor.text_to_latin(&input_buffer);
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
                },
                _ => {}
            }
        };
        //@! Read Shell stdout
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
    let mut command: String = String::new();
    for line in lines.iter() {
        if line.starts_with("#") {
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        command.push_str(line);
        command.push(';');
    }
    //Execute command
    run_command(command, processor, config, shell)
}

#[allow(dead_code)]
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
        true => print!("{}", processor.text_to_cyrillic(&out)),
        false => print!("{}", out),
    };
}
