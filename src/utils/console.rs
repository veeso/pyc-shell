//! ## Console
//!
//! `Console` module provides an API for the terminal console

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
extern crate nix;
extern crate termios;

use std::io::{self, Read, Write};
use std::os::unix::io::RawFd;

const STDIN_FILENO: RawFd = 0;

/// ## InputEvent
/// 
/// InputEvent enum represents an Input Event got from user on a read call
#[derive(std::fmt::Debug)]
pub enum InputEvent {
    Key(String),
    Ctrl(u8),
    Enter,
    CarriageReturn,
    Backspace,
    ArrowUp,
    ArrowLeft,
    ArrowRight,
    ArrowDown
}


/// ### backspace
/// 
/// Remove last typed character from prompt
pub fn backspace() {
    //To backspace we have to go back of 1 position, print blank and go back again
    print(String::from("\x08 \x08"));
}

pub fn move_cursor_right() {
    print(String::from("\x1b[1C"));
}

pub fn move_cursor_left() {
    print(String::from("\x1b[1D"));
}

/// ### carriage_return
/// 
/// Return to the beginning of the line
pub fn carriage_return() {
    print(String::from("\r"));
}

/// ### clear
/// 
/// Clear console
pub fn clear() {
    print(String::from("\x1b[H\x1b[2J"));
}

/// ### read
/// 
/// Read user input and returns an individual InputEvent (or None)
pub fn read() -> Option<InputEvent> {
    //Configure terminal
    match input_ready() {
        false => None,
        true => {
            //Configure input
            prepare_termios();
            //Read
            let mut buf: Vec<u8> = vec![0u8; 1];
            let _ = io::stdin().read_exact(&mut buf);
            //Handle input
            let key: u8 = *buf.get(0).unwrap_or(&0);
            let ev: InputEvent = match key {
                8 | 127 => InputEvent::Backspace,
                10 => InputEvent::Enter,
                13 => InputEvent::CarriageReturn,
                0..=26 => InputEvent::Ctrl(key), //CTRL key (exclude 8, 10, 13)
                27 => { //Is Arrow Key
                    //Read twice
                    let _ = io::stdin().read_exact(&mut buf);
                    let _ = io::stdin().read_exact(&mut buf);
                    let direction: char = *buf.get(0).unwrap_or(&0) as char;
                    match direction {
                        'A' => InputEvent::ArrowUp,
                        'B' => InputEvent::ArrowDown,
                        'C' => InputEvent::ArrowRight,
                        'D' => InputEvent::ArrowLeft,
                        _ => return None //Unknown event
                    }
                },
                _ => { //Handle normal key
                    //@! Read until it's a valid UTF8 string
                    //NOTE: 4 is the maximum amount of bytes used by a UTF-8
                    let mut utfbuffer: [u8; 4] = [0; 4];
                    let mut buff_index: usize = 0;
                    let mut keystr: Option<String> = None;
                    loop {
                        //Copy last character into utf buffer
                        if buff_index >= 4 { //Overflow
                            break
                        }
                        utfbuffer[buff_index] = *buf.get(0).unwrap_or(&0);
                        buff_index += 1;
                        //Check if utf buffer is a valid utf8 string
                        match std::str::from_utf8(&utfbuffer[0..buff_index]) { //If buffer is a valid
                            Ok(key) => {
                                keystr = Some(String::from(key));
                                break
                            },
                            Err(_) => { //If not valid...
                                if let Err(_) = io::stdin().read_exact(&mut buf) {
                                    break
                                }
                                continue
                            }
                        };
                    }
                    match keystr {
                        Some(s) => InputEvent::Key(s),
                        None => return None //Unknown key
                    }
                }
            };
            //Restore settings
            reset_termios();
            Some(ev)
        }
    }
}

/// ### rewrite
/// 
/// Rewrite current stdout line
pub fn rewrite(row: String) {
    print!("\r\x1b[K");
    print(row);
}

/// ### print
/// 
/// print on this line without newline
pub fn print(row: String) {
    print!("{}", row);
    let _ = io::stdout().flush();
}

/// ### println
/// 
/// Print line and go to new line
pub fn println(row: String) {
    println!("{}", row);
}

/// ### input_ready
/// 
/// Returns whether stdin is ready to be read
fn input_ready() -> bool {
    prepare_termios();
    let mut poll_fds: [nix::poll::PollFd; 1] = [nix::poll::PollFd::new(STDIN_FILENO, nix::poll::PollFlags::POLLIN | nix::poll::PollFlags::POLLRDBAND | nix::poll::PollFlags::POLLHUP)];
    let ready: bool = match nix::poll::poll(&mut poll_fds, 10) {
        Ok(ret) => {
            if ret > 0 && poll_fds[0].revents().is_some() { //Stdin is available to be read
                let event: nix::poll::PollFlags = poll_fds[0].revents().unwrap();
                if event.intersects(nix::poll::PollFlags::POLLIN) || event.intersects(nix::poll::PollFlags::POLLRDBAND) {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        },
        Err(_) => false
    };
    reset_termios();
    ready
}

/// ### prepare_termios
/// 
/// Prepare termios for console
fn prepare_termios() {
    let mut term = termios::Termios::from_fd(STDIN_FILENO).unwrap();
    let _ = termios::tcgetattr(STDIN_FILENO, &mut term);
    term.c_lflag &= !termios::ICANON;
    term.c_lflag &= !termios::ECHO;
    let _ = termios::tcsetattr(STDIN_FILENO, termios::TCSANOW, &term);
}

/// ### reset_termios
/// 
/// Restore previous termios configuration
fn reset_termios() {
    let mut term = termios::Termios::from_fd(STDIN_FILENO).unwrap();
    let _ = termios::tcgetattr(STDIN_FILENO, &mut term);
    term.c_lflag |= termios::ICANON;
    term.c_lflag &= termios::ECHO;
    let _ = termios::tcsetattr(STDIN_FILENO, termios::TCSADRAIN, &term);
}
