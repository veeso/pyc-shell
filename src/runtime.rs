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
use std::io::Read;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use termion::async_stdin;

use crate::config;
use crate::shellenv::process::ShellProcess;
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
  match config.get_alias(&argv[0]) {
    Some(resolved) => argv[0] = resolved,
    None => {}
  };
  //Join tokens
  let expr: String = match processor.expression_to_latin(argv.join(" ")) {
    Ok(cmd) => cmd,
    Err(err) => {
      println!(
        "{}",
        Colour::Red
          .paint(processor.text_to_cyrillic(String::from(format!("Bad expression: {:?}", err))))
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
  let mut process = match ShellProcess::exec(argv) {
    Ok(p) => p,
    Err(_) => {
      let err = match config.output_config.translate_output {
        true => processor.text_to_cyrillic(String::from(format!("Unknown command {}", command))),
        false => String::from(format!("Unknown command {}", command)),
      };
      println!("{}", Colour::Red.paint(err),);
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
    eprintln!(
      "{}",
      Colour::Red
        .paint(processor.text_to_cyrillic(String::from("Could not start signal listener")))
    )
  }
  //@! Loop until process has terminated
  while process.is_running() {
    //Read user input
    if let Some(Ok(i)) = stdin.next() {
      input_bytes.push(i);
    //TODO: pass characters at each input to stdin?
    } else {
      //Buffer is empty, if len > 0, send input to program, otherwise there's no input
      if input_bytes.len() > 0 {
        //Convert bytes to UTF-8 string
        let input: String = String::from(std::str::from_utf8(input_bytes.as_slice()).unwrap());
        if let Err(err) = process.write(processor.text_to_latin(input)) {
          if config.output_config.translate_output {
            eprintln!(
              "{}",
              Colour::Red.paint(processor.text_to_cyrillic(err.to_string()))
            );
          } else {
            eprintln!("{}", Colour::Red.paint(err.to_string()));
          }
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
        let out: String = if config.output_config.translate_output {
          processor.text_to_cyrillic(out.unwrap())
        } else {
          out.unwrap()
        };
        print!("{}", out);
      }
      if err.is_some() {
        //Convert err to cyrillic
        let err: String = if config.output_config.translate_output {
          processor.text_to_cyrillic(err.unwrap())
        } else {
          err.unwrap()
        };
        eprint!("{}", Colour::Red.paint(err.to_string()));
      }
    }
    //Fetch signals
    match sig_rx.try_recv() {
      Ok(sig) => {
        //Send signals
        if let Err(_) = process.raise(sig) {
          eprintln!(
            "{}",
            Colour::Red.paint(
              processor
                .text_to_cyrillic(String::from("Could not send signal SIGINT to subprocess"))
            )
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
  process.exit_status.unwrap_or(255)
}
