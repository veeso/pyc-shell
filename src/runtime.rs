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
extern crate termion;

use ansi_term::Colour;
use std::env;
use std::io::Read;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use termion::async_stdin;

use crate::config;
use crate::shell::proc::{ShellProc, ShellState};
use crate::shell::{Shell};
use crate::shell::prompt::ShellPrompt;
use crate::translator::ioprocessor::IOProcessor;

/// ### process_command
///
/// Process a shell command, converting it to latin and then letting the user interacting with it
/// the command output is converted back to cyrillic
/// This function is used in oneshot mode only

pub fn process_command(
    processor: IOProcessor,
    config: &config::Config,
    mut argv: Vec<String>,
) -> u8 {
    if argv.len() == 0 {
        //Prevent empty commands
        return 255;
    }
    //Process arg 0
    resolve_command(&mut argv, &config);
    //Join tokens
    let expr: String = match processor.expression_to_latin(argv.join(" ")) {
        Ok(cmd) => cmd,
        Err(err) => {
            print_err(
                String::from(format!("Bad expression: {:?}", err)),
                config.output_config.translate_output,
                &processor,
            );
            return 255;
        }
    };
    //Convert expression back to argv
    let mut argv: Vec<String> = Vec::with_capacity(expr.matches(" ").count() + 1);
    for arg in expr.split_whitespace() {
        argv.push(String::from(arg));
    }
    let command: String = argv[0].clone();
    //Start shell process
    let mut process = match ShellProc::start(argv) {
        Ok(p) => p,
        Err(_) => {
            print_err(
                String::from(format!("Unknown command {}", command)),
                config.output_config.translate_output,
                &processor,
            );
            return 255;
        }
    };
    //Create input stream
    let mut stdin = async_stdin().bytes();
    let mut input_bytes: Vec<u8> = Vec::new();
    let running = Arc::new(Mutex::new(true));
    let (sig_tx, sig_rx) = mpsc::channel::<nix::sys::signal::Signal>();
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
            if let Err(_) = sig_tx.send(nix::sys::signal::Signal::SIGINT) {
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
    //@! Loop until process has terminated
    while process.get_state() != ShellState::Terminated {
        //Read user input
        if let Some(Ok(i)) = stdin.next() {
            input_bytes.push(i);
            //TODO: pass characters at each input to stdin?
        } else {
            //Buffer is empty, if len > 0, send input to program, otherwise there's no input
            if input_bytes.len() > 0 {
                //Convert bytes to UTF-8 string
                let input: String =
                    String::from(std::str::from_utf8(input_bytes.as_slice()).unwrap());
                if let Err(err) = process.write(processor.text_to_latin(input)) {
                    print_err(
                        String::from(err.to_string()),
                        config.output_config.translate_output,
                        &processor,
                    );
                }
                //Reset input buffer
                input_bytes = Vec::new();
            }
        }
        /*
        let mut input: String = String::new();
        stdin.read_to_string(&mut input);
        if input.len() > 0 {
            println!("INPUT: {}", input);
        }
        */
        //Read program stdout
        if let Ok((out, err)) = process.read() {
            if out.is_some() {
                //Convert out to cyrillic
                print_out(
                    out.unwrap(),
                    config.output_config.translate_output,
                    &processor,
                );
            }
            if err.is_some() {
                //Convert err to cyrillic
                print_err(
                    err.unwrap().to_string(),
                    config.output_config.translate_output,
                    &processor,
                );
            }
        }
        //Fetch signals
        match sig_rx.try_recv() {
            Ok(sig) => {
                //Send signals
                if let Err(_) = process.raise(sig) {
                    print_err(
                        String::from("Could not send SIGINT to subprocess"),
                        config.output_config.translate_output,
                        &processor,
                    );
                }
            }
            Err(_) => {}
        }
        sleep(Duration::from_millis(10)); //Sleep for 10ms
    }
    //Terminate sig hnd
    let mut sig_term = running.lock().unwrap();
    *sig_term = true;
    drop(sig_term); //Otherwise the other thread will never read the state
    //Return exitcode
    process.exit_status
}

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
    let mut stdin = async_stdin().bytes();
    let mut input_bytes: Vec<u8> = Vec::new();
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
    let check_interval: Duration = Duration::from_millis(500); //State is checked each 500ms
    let mut last_state_check: Instant = Instant::now();
    let mut last_state: ShellState = ShellState::Idle;
    let mut state_changed: bool = true; //Start with state changed, this determines whether the prompt should be printed
    while shell_env.get_state() != ShellState::Terminated {
        //@! Check if state must be refreshed again
        if last_state_check.elapsed().as_millis() >= check_interval.as_millis() {
            last_state_check = Instant::now(); //Reset last check
            //Refresh state
            shell_env.refresh_env();
            let new_state = shell_env.get_state(); //Force last state to be changed
            if new_state != last_state {
                last_state = new_state;
                state_changed = true;
            }
        }
        //@! Print prompt if state is Idle and state has changed
        if state_changed && last_state == ShellState::Idle {
            //Force shellenv to refresh info
            shell_env.refresh_env();
            //Print prompt
            shell_prompt.print(&shell_env, &processor);
            state_changed = false; //Force state changed to false
        } else if state_changed {
            state_changed = false; //Check has been done, nothing to do
        }
        //@!Read user input
        if let Some(Ok(i)) = stdin.next() {
            input_bytes.push(i);
            //TODO: pass characters at each input to stdin?
        } else {
            //Buffer is empty, if len > 0, send input to program, otherwise there's no input
            if input_bytes.len() > 1 {
                //Convert bytes to UTF-8 string
                let input: String = String::from(std::str::from_utf8(input_bytes.as_slice()).unwrap());
                //Get shell env state
                let new_state = shell_env.get_state(); //Force last state to be changed
                if new_state != last_state {
                    last_state = new_state;
                    state_changed = true;
                }
                //Reset last check
                last_state_check = Instant::now();
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
                        //State must be set to Subprocess
                        last_state = ShellState::SubprocessRunning;
                        state_changed = true;
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
                //Reset input buffer
                input_bytes = Vec::new();
            } else if input_bytes.len() == 1 {
                //If length is 1, there is only a new line
                input_bytes = Vec::new();
                //In this case, prompt must be printed if shell state is Idle
                if last_state == ShellState::Idle {
                    shell_prompt.print(&shell_env, &processor);
                }
            }
        }
        //Read program stdout
        if let Ok((out, err)) = shell_env.read() {
            if out.is_some() {
                //Convert out to cyrillic
                print_out(
                    out.unwrap(),
                    config.output_config.translate_output,
                    &processor,
                );
            }
            if err.is_some() {
                //Convert err to cyrillic
                print_err(
                    err.unwrap().to_string(),
                    config.output_config.translate_output,
                    &processor,
                );
            }
        }
        //Fetch signals
        match sig_rx.try_recv() {
            Ok(()) => {
                //Send signals
                if let Err(_) = shell_env.sigint() {
                    print_err(
                        String::from("Could not send SIGINT to subprocess"),
                        config.output_config.translate_output,
                        &processor,
                    );
                }
            }
            Err(_) => {}
        }
        sleep(Duration::from_millis(10)); //Sleep for 10ms
    }
    //Return shell exitcode
    match shell_env.get_exitcode() {
        Some(rc) => rc,
        None => 255
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
