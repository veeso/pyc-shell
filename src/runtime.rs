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
extern crate ctrlc;
extern crate nix;

use ansi_term::Colour;
use std::env;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration};

use crate::config;
use crate::shell::proc::ShellState;
use crate::shell::{Shell};
use crate::shell::prompt::ShellPrompt;
use crate::translator::ioprocessor::IOProcessor;
use crate::utils::async_stdin;

/// ### shell_exec
///
/// Run pyc in shell mode

pub fn shell_exec(processor: IOProcessor, config: &config::Config, shell: Option<String>) -> u8 {
    //Determine the shell to use
    //TODO: add shell from config
    let shell: String = match shell {
        Some(sh) => sh,
        None => match get_shell_from_env() {
            Ok(sh) => sh,
            Err(()) => {
                print_err(
                    String::from("Could not determine the shell to use"),
                    config.output_config.translate_output,
                    &processor,
                );
                return 255;
            }
        }
    };
    //Intantiate and start a new shell
    let mut shell_env: Shell = match Shell::start(shell) {
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
    //Create input stream
    /*
    let mut stdin = async_stdin().bytes();
    let mut input_bytes: Vec<u8> = Vec::new();
    */
    let running = Arc::new(Mutex::new(true));
    let (sig_tx, sig_rx) = mpsc::channel::<()>();
    let sig_running = Arc::clone(&running);
    //Start signal handler
    if let Err(_) = ctrlc::set_handler(move || {
        let mut terminate: bool = false;
        while !terminate {
            {
                //Inside this block, otherwise does never go out of scope
                let current_state = sig_running.lock().unwrap();
                if *current_state == false {
                    terminate = true;
                }
            }
            if let Err(_) = sig_tx.send(()) {
                break;
            }
            sleep(Duration::from_millis(50));
        }
    }) {
        print_err(
            String::from("Could not start signal listener"),
            config.output_config.translate_output,
            &processor,
        );
    }
    //Instantiate shell prompt
    let mut shell_prompt: ShellPrompt = ShellPrompt::new(&config.prompt_config);
    //@! Main loop
    let mut last_state: ShellState = ShellState::Idle;
    let mut state_changed: bool = true; //Start with state changed, this determines whether the prompt should be printed
    while shell_env.get_state() != ShellState::Terminated {
        //@! Print prompt if state is Idle and state has changed
        if state_changed && shell_env.get_state() == ShellState::Idle {
            //Force shellenv to refresh info
            shell_env.refresh_env();
            //Print prompt
            shell_prompt.print(&shell_env, &processor);
            state_changed = false; //Force state changed to false
        } else if state_changed {
            state_changed = false; //Check has been done, nothing to do
        }
        //@! Read user input
        if async_stdin::is_ready() { //Check if stdin is ready to be read
            let input: String = async_stdin::read();
            //If input is empty, print prompt (if state is IDLE)
            if input.trim().len() == 0 {
                if last_state == ShellState::Idle {
                    shell_prompt.print(&shell_env, &processor);
                }
            } else {
                //Treat input
                //If state is Idle, convert expression, otherwise convert text
                let input: String = match last_state {
                    ShellState::Idle => {
                        //Resolve alias
                        let mut argv: Vec<String> = Vec::with_capacity(input.matches(" ").count() + 1);
                        for arg in input.split_whitespace() {
                            argv.push(String::from(arg));
                        }
                        //Process arg 0
                        resolve_command(&mut argv, &config);
                        //Rejoin arguments
                        let input: String = argv.join(" ") + "\n";
                        match processor.expression_to_latin(input) {
                            Ok(ex) => ex,
                            Err(err) => {
                                print_err(String::from(format!("Input error: {:?}", err)), config.output_config.translate_output, &processor);
                                continue;
                            }
                        }
                    },
                    ShellState::SubprocessRunning => processor.text_to_latin(input),
                    ShellState::Terminated => continue
                };
                if let Err(err) = shell_env.write(input) {
                    print_err(
                        String::from(err.to_string()),
                        config.output_config.translate_output,
                        &processor,
                    );
                }
                //Update state after write
                let new_state = shell_env.get_state(); //Force last state to be changed
                if new_state != last_state {
                    last_state = new_state;
                    state_changed = true;
                }
            }
        }
        /*
        if let Some(Ok(i)) = stdin.next() {
            input_bytes.push(i);
        } else {
            //Buffer is empty, if len > 0, send input to program, otherwise there's no input
            if input_bytes.len() > 1 {
                //Convert bytes to UTF-8 string
                let input: String = String::from(std::str::from_utf8(input_bytes.as_slice()).unwrap());
                //Prevent empty strings
                if input.trim().len() == 0 {
                    input_bytes.clear();
                    if last_state == ShellState::Idle {
                        shell_prompt.print(&shell_env, &processor);
                    }
                    continue;
                }
                //Get shell env state
                let new_state = shell_env.get_state(); //Force last state to be changed
                if new_state != last_state {
                    last_state = new_state;
                    state_changed = true;
                }
                //If state is Idle, convert expression, otherwise convert text
                let input: String = match last_state {
                    ShellState::Idle => {
                        //Resolve alias
                        let mut argv: Vec<String> = Vec::with_capacity(input.matches(" ").count() + 1);
                        for arg in input.split_whitespace() {
                            argv.push(String::from(arg));
                        }
                        //Process arg 0
                        resolve_command(&mut argv, &config);
                        //Rejoin arguments
                        let input: String = argv.join(" ") + "\n";
                        match processor.expression_to_latin(input) {
                            Ok(ex) => ex,
                            Err(err) => {
                                print_err(String::from(format!("Input error: {:?}", err)), config.output_config.translate_output, &processor);
                                continue;
                            }
                        }
                    },
                    ShellState::SubprocessRunning => processor.text_to_latin(input),
                    ShellState::Terminated => continue
                };
                if let Err(err) = shell_env.write(input) {
                    print_err(
                        String::from(err.to_string()),
                        config.output_config.translate_output,
                        &processor,
                    );
                }
                //Update state after write
                let new_state = shell_env.get_state(); //Force last state to be changed
                if new_state != last_state {
                    last_state = new_state;
                    state_changed = true;
                }
                //Reset input buffer
                input_bytes.clear()
            } else if input_bytes.len() == 1 {
                //If length is 1, there is only a new line
                input_bytes.clear();
                //In this case, prompt must be printed if shell state is Idle
                if last_state == ShellState::Idle {
                    shell_prompt.print(&shell_env, &processor);
                }
            }
        }
        */
        //@! Read Shell stdout
        if let Ok((out, err)) = shell_env.read() {
            if out.is_some() {
                //Convert out to cyrillic
                print_out(out.unwrap(), config.output_config.translate_output, &processor);
            }
            if err.is_some() {
                //Convert err to cyrillic
                print_err(err.unwrap().to_string(), config.output_config.translate_output, &processor);
            }
        }
        //Fetch signals
        match sig_rx.try_recv() {
            Ok(()) => {
                //Send signals
                if let Err(_) = shell_env.sigint() {
                    print_err(String::from("Could not send SIGINT to subprocess"), config.output_config.translate_output, &processor);
                }
            }
            Err(_) => {}
        }
        sleep(Duration::from_nanos(100)); //Sleep for 100ns
    } //@! End of loop
    //Return shell exitcode
    match shell_env.stop() {
        Ok(rc) => rc,
        Err(err) => {
            print_err(format!("Could not stop shell: {}", err), config.output_config.translate_output, &processor);
            255
        }
    }
}

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
        true => eprintln!("{}", Colour::Red.paint(processor.text_to_cyrillic(err))),
        false => eprintln!("{}", Colour::Red.paint(err)),
    };
}

/// ### print_out
///
/// print normal message; the message is may converted to cyrillic if translate config is true

fn print_out(out: String, to_cyrillic: bool, processor: &IOProcessor) {
    match to_cyrillic {
        true => print!("{}", processor.text_to_cyrillic(out)),
        false => print!("{}", out),
    };
}
