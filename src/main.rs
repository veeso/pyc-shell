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

const PYC_VERSION: &str = "0.1.0";
const PYC_BUILD: &str = "??";

//Crates
extern crate ansi_term;
extern crate dirs;
extern crate getopts;
#[macro_use] extern crate lazy_static;

//External modules
use ansi_term::{Colour, Style};
use dirs::home_dir;
use getopts::Options;
use std::env;

//Internal modules
mod config;
mod runtime;
mod shell;
mod translator;
mod utils;

use translator::ioprocessor::IOProcessor;

/// ### print_usage
///
/// Print usage

fn print_usage(program: &String, opts: Options) {
    let brief = format!("Usage: {} [Options]... [Command]...", program);
    print!("{}", opts.usage(&brief));
}

/// ### str_to_language
///
/// Convert CLI option language string to Language enum

fn str_to_language(lang: String) -> translator::Language {
    match lang.as_str() {
        "ru" | "рус" => translator::Language::Russian,
        _ => {
            eprintln!(
                "{}",
                Colour::Red.paint(format!(
                    "Unknown language: '{}'; Setting language to default: ru",
                    lang
                ))
            );
            translator::Language::Russian
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program: String = args[0].clone();
    //Program CLI options
    let config_file: String;
    let mut shell: Option<String> = None;
    let oneshot: bool;
    let language: Option<translator::Language>;
    let mut opts = Options::new();
    opts.optopt("c", "config", "Specify YAML configuration file", "<config>");
    opts.optopt("l", "lang", "Specify shell language", "<ru|рус>");
    opts.optopt("s", "shell", "Force the shell used for shell mode", "</bin/bash>");
    opts.optflag("v", "version", "");
    opts.optflag("h", "help", "Print this menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", Colour::Red.paint(f.to_string()));
            std::process::exit(255);
        }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(255);
    }
    if matches.opt_present("v") {
        eprintln!(
            "{}",
            Style::new().bold().paint(format!(
                "рус - {} ({}) - Developed by Кристиан Визинтин",
                PYC_VERSION, PYC_BUILD,
            ))
        );
        std::process::exit(255);
    }
    //Get shell
    if let Some(sh) = matches.opt_str("s") {
        shell = Some(sh);
    };
    //Set translator language
    language = match matches.opt_str("l") {
        Some(lang) => Some(str_to_language(lang)),
        None => None,
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
                    "{}",
                    Colour::Red.paint(format!(
                        "{}: {}; {}",
                        String::from("No such file or directory"),
                        config_file,
                        String::from("Using default configuration")
                    ))
                );
                config::Config::default()
            }
            _ => panic!(
                "{}",
                Colour::Red.paint(format!(
                    "{}: '{}'",
                    String::from("Could not parse YAML configuration"),
                    err
                ))
            ),
        },
    };
    //Set language
    let language: translator::Language = match language {
        Some(l) => l,
        None => str_to_language(config.language.clone())
    };
    //Set up processor
    let processor: IOProcessor = IOProcessor::new(language, translator::new_translator(language));
    //Start runtime
    let rc: u8;
    if oneshot {
        //rc = runtime::process_command(processor, &config, argv);
        //TODO: impl oneshot
        rc = 255;
    } else {
        rc = runtime::shell_exec(processor, &config, shell);
    }
    std::process::exit(rc as i32);
}
