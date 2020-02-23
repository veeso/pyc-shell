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

//TODO: shell format function
//TODO: handle 'cd' command with cwd (since it is not an executable)

const PYC_VERSION: &str = "0.1.0";
const PYC_BUILD: &str = "??";

//Crates
extern crate dirs;
extern crate getopts;
extern crate nix;
extern crate termion;

//External modules
use dirs::home_dir;
use getopts::Options;
use signal_hook::{iterator::Signals, SIGINT};
use std::env;
use std::io::Read;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use termion::{async_stdin, color, style};

//Internal modules
mod config;
mod shellenv;
mod translator;

/// ### print_usage
///
/// Print usage

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [OPTIONS]... [COMMAND]...", program);
    print!("{}", opts.usage(&brief));
}

/// ### str_to_language
///
/// Convert CLI option language string to Language enum

fn str_to_language(lang: String) -> translator::Language {
    match lang.as_str() {
        "ru" | "рус" => translator::Language::Russian,
        _ => {
            eprintln!("{}Unknown language: '{}'; Defaulting to russian", color::Fg(color::Red), lang);
            translator::Language::Russian
        }
    }
}

fn int_to_signal(sig: i32) -> nix::sys::signal::Signal {
    match sig {
        SIGINT => nix::sys::signal::Signal::SIGINT,
        _ => nix::sys::signal::Signal::SIGINT,
    }
}

/// ### process_command
///
/// Process a shell command, converting it to latin and then letting the user interacting with it
/// the command output is converted back to cyrillic

fn process_command(
    translator: &translator::Translator,
    config: &config::Config,
    mut argv: Vec<String>,
) -> u8 {
    if argv.len() == 0 {
        //Prevent empty commands
        return 255;
    }
    //Process arg 0
    match config.get_alias(&argv[0]) {
        Some(resolved) => argv[0] = resolved,
        None => {}
    };
    //Join tokens
    let expr: String = argv.join(" ");
    let expr: String = match (translator.to_latin)(expr) {
        Ok(s) => s,
        Err(err) => {
            println!("{}Syntax error: {:?}", color::Fg(color::Red), err);
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
    let mut process = match shellenv::ShellProcess::exec(argv) {
        Ok(p) => p,
        Err(_) => {
            println!("{}Unknown command '{}'", color::Fg(color::Red), command);
            return 255;
        }
    };
    //Create input stream
    let mut stdin = async_stdin().bytes();
    let mut input: String = String::new();
    //Catch signals
    let signals: Signals = match Signals::new(&[SIGINT]) {
        Ok(s) => s,
        Err(err) => {
            eprintln!(
                "{}Could not start signal listeners: {}",
                color::Fg(color::Red),
                err
            );
            return 255;
        }
    };
    let running = Arc::new(Mutex::new(true));
    let (sig_tx, sig_rx) = mpsc::channel::<i32>();
    let sig_running = Arc::clone(&running);
    let sig_join_hnd = thread::spawn(move || {
        let mut terminate: bool = false;
        while !terminate {
            let current_state = sig_running.lock().unwrap();
            if *current_state == false {
                terminate = true;
            }
            for sig in signals.forever() {
                if let Err(_) = sig_tx.send(sig) {
                    terminate = true;
                    break;
                }
            }
            sleep(Duration::from_millis(50));
        }
    });
    //Loop until process has terminated
    while process.is_running() {
        //Read user input
        if let Some(Ok(i)) = stdin.next() {
            input.push(i as char);
        } else {
            //Buffer is empty, if len > 0, send input to program, otherwise there's no input
            if input.len() > 0 {
                if let Err(err) = process.write(input) {
                    eprintln!("{}", err);
                }
                //Reset input buffer
                input = String::new();
            }
        }
        //Read program stdout
        if let Ok((out, err)) = process.read() {
            if out.is_some() {
                //Convert out to cyrillic
                let out: String = (translator.to_cyrillic)(out.unwrap());
                print!("{}", out);
            }
            if err.is_some() {
                //Convert err to cyrillic
                let err: String = (translator.to_cyrillic)(err.unwrap());
                eprint!("{}", err);
            }
        }
        //Fetch signals
        match sig_rx.try_recv() {
            Ok(sig) => {
                //Send signals
                if let Err(_) = process.raise(int_to_signal(sig)) {
                    eprintln!(
                        "{}Could not send signal {} to subprocess!",
                        color::Fg(color::Red),
                        sig
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
    if let Err(err) = sig_join_hnd.join() {
        eprintln!("{}Child process panicked: {:?}", color::Fg(color::Red), err);
    }
    //Return exitcode
    process.exit_status.unwrap_or(255)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();
    //Program CLI options
    let config_file: String;
    //let shell: String;
    let oneshot: bool;
    let language: translator::Language;
    let mut opts = Options::new();
    opts.optopt(
        "c",
        "config",
        "Specify the configuration YAML file",
        "<config_yaml>",
    );
    opts.optopt("l", "lang", "Specify the cyrillic language", "<ru|рус>");
    opts.optflag("", "ссср", "");
    opts.optflag("v", "version", "");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("v") {
        eprintln!(
            "{}рус - {} ({}) - Developed by Кристиан Визинтин{}",
            style::Bold,
            PYC_VERSION,
            PYC_BUILD,
            style::Reset
        );
        return;
    }
    //Set translator language
    language = match matches.opt_str("l") {
        Some(lang) => str_to_language(lang),
        None => translator::Language::Russian,
    };
    //Set config file to '-c' file or to default file
    config_file = match matches.opt_str("c") {
        Some(cfg_override) => cfg_override,
        None => {
            //Default path
            let home: String = match home_dir() {
                Some(path) => String::from(path.to_str().unwrap()),
                None => String::from("~"),
            };
            String::from(home + "/.config/pyc/pyc.yml")
        }
    };
    //Check if oneshot and get args
    let argv: Vec<String> = matches.free.clone();
    oneshot = argv.len() > 0;
    //Parse configuration
    let config: config::Config = match config::Config::parse_config(config_file.clone()) {
        Ok(cfg) => cfg,
        Err(err) => match err.code {
            config::ConfigErrorCode::NoSuchFileOrDirectory => {
                eprintln!(
                    "{}No such file or directory {}; using default configuration",
                    color::Fg(color::Red),
                    config_file
                );
                config::Config::default()
            }
            _ => panic!(
                "{}Could not parse YAML configuration: {}",
                color::Fg(color::Red),
                err
            ),
        },
    };
    //Set up translator
    let translator = translator::Translator::new(language);
    let mut rc: u8 = 0;
    if oneshot {
        rc = process_command(&translator, &config, argv);
    } else {
        //TODO: implement loop
        //TODO: catch signals
    }
    std::process::exit(rc as i32);
}
