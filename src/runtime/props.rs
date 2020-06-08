//! ## Props
//!
//! `props` contains the runtime props implementation for Runtime

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

use super::{print_err, resolve_command};

use crate::config::Config;
use crate::shell::Shell;
use crate::shell::proc::ShellState;
use crate::translator::ioprocessor::IOProcessor;
use crate::utils::buffer;
use crate::utils::console::{self, InputEvent};

/// ## RuntimeProps
/// 
/// Runtime Props is a wrapper for all the properties used by the Runtime module
pub(super) struct RuntimeProps {
    pub config: Config,
    pub processor: IOProcessor,
    input_buffer: Vec<char>,
    input_buffer_cursor: usize,
    interactive: bool,
    last_state: ShellState,
    state_changed: bool,
    rev_search: bool,
}

impl RuntimeProps {
    /// ### new
    ///
    /// Instantiates a new RuntimeProps
    pub(super) fn new(interactive: bool, config: Config, processor: IOProcessor) -> RuntimeProps {
        RuntimeProps {
            config: config,
            processor: processor,
            input_buffer: Vec::with_capacity(2048),
            input_buffer_cursor: 0,
            interactive: interactive,
            last_state: ShellState::Unknown,
            state_changed: true,
            rev_search: false
        }
    }

    /// ### clear_buffer
    /// 
    /// Clear buffer and reset cursor to 0
    pub(super) fn clear_buffer(&mut self) {
        self.input_buffer.clear();
        self.input_buffer_cursor = 0;
    }

    /// ### get_state
    /// 
    /// Get Shell State
    pub(super) fn get_last_state(&self) -> ShellState {
        self.last_state
    }

    /// ### get_state_changed
    /// 
    /// Get state changed value
    pub(super) fn get_state_changed(&self) -> bool {
        self.state_changed
    }

    /// ### update_state
    /// 
    /// Update last state
    pub(super) fn update_state(&mut self, new_state: ShellState) {
        self.last_state = new_state;
        self.state_changed = true;
    }

    /// ### state_changed_notified
    /// 
    /// Report that state changed has been notified correctly.
    /// Pratically resets state_changed
    pub(super) fn report_state_changed_notified(&mut self) {
        self.state_changed = false;
    }

    /// ### backspace
    ///
    /// Perform backspace on current console and buffers
    pub(super) fn backspace(&mut self) {
        //Remove from buffer and backspace (if possible)
        if self.input_buffer_cursor > 0 {
            self.input_buffer_cursor -= 1;
            if self.input_buffer.len() > self.input_buffer_cursor {
                self.input_buffer.remove(self.input_buffer_cursor);
            }
            console::backspace();
        }
    }

    /// ### move_left
    ///
    /// Move cursor to left
    pub(super) fn move_left(&mut self) {
        //If possible, move the cursor right
        if self.input_buffer_cursor != 0 {
            self.input_buffer_cursor -= 1;
            console::move_cursor_left();
        }
    }

    /// ### move_right
    ///
    /// Move cursor to right
    pub(super) fn move_right(&mut self) {
        //If possible, move the cursor left
        if self.input_buffer_cursor + 1 <= self.input_buffer.len() {
            self.input_buffer_cursor += 1;
            console::move_cursor_right();
        }
    }

    /// ### handle_input_event
    /// 
    /// Handle input event received from stdin
    pub(super) fn handle_input_event(&mut self, ev: InputEvent, shell: &mut Shell) {
        match ev {
            InputEvent::ArrowDown => {
                //TODO: history next
            },
            InputEvent::ArrowUp => {
                //TODO: history prev
            },
            InputEvent::ArrowLeft => {
                self.move_left();
            },
            InputEvent::ArrowRight => {
                self.move_right();
            },
            InputEvent::Backspace => {
                self.backspace();
            },
            InputEvent::CarriageReturn => {
                console::carriage_return();
            },
            InputEvent::Ctrl(sig) => {
                //Check running state 
                //if running state is Idle, it will be handled by the console,
                //otherwise by the shell process
                if self.last_state == ShellState::Idle && self.interactive {
                    match sig {
                        1 => { //CTRL + A
                            //We must return at the beginning of the string
                            for _ in 0..self.input_buffer_cursor {
                                //Move left
                                console::move_cursor_left();
                            }
                            self.input_buffer_cursor = 0; //Reset cursor
                        }, 
                        2 => { //CTRL + B
                            self.move_left();
                        },
                        3 => { //CTRL + C
                            //Abort input and go to newline
                            self.clear_buffer();
                            console::println(String::new());
                            console::print(format!("{} ", shell.get_promptline(&self.processor)));
                        },
                        4 => { //CTRL + D
                            self.backspace();
                        },
                        5 => { //CTRL + E
                            for _ in self.input_buffer_cursor..self.input_buffer.len() {
                                console::move_cursor_right();
                            }
                            self.input_buffer_cursor = self.input_buffer.len();
                        },
                        6 => { //CTRL + F
                            self.move_right();
                        },
                        7 => { //CTRL + G
                            //TODO: exit rev search
                        },
                        8 => { //CTRL + H
                            self.backspace();
                        },
                        11 => { // CTRL + K
                            //Delete all characters after cursor
                            while self.input_buffer_cursor < self.input_buffer.len() {
                                let _ = self.input_buffer.pop();
                            }
                        },
                        12 => { // CTRL + L
                            //Clear, but doesn't reset input
                            console::clear();
                            console::print(format!("{} {}", shell.get_promptline(&self.processor), buffer::chars_to_string(&self.input_buffer)));
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
                    if let Some(sig) = super::shellsignal_to_signal(sig) {
                        if let Err(_) = shell.raise(sig) {
                            print_err(String::from("Could not send signal to shell"), self.config.output_config.translate_output, &self.processor);
                        }
                    }
                }
            },
            InputEvent::Key(k) => { //Push key
                //Push k to input buffer
                for ch in k.chars() {
                    self.input_buffer.insert(self.input_buffer_cursor, ch);
                    self.input_buffer_cursor += 1;
                }
                //Print key
                console::print(k);
            },
            InputEvent::Enter => { //@! Send input
                //@! Handle enter...
                if self.interactive { //@! Interactive shell
                    self.perform_interactive_enter(shell);
                } else { //@! Non interactive shell
                    self.perform_enter(shell);
                }
            }
        }
    }

    /// ### perform_interactive_enter
    /// 
    /// Perform enter in interactive shell mode
    fn perform_interactive_enter(&mut self, shell: &mut Shell) {
        //Newline first
        console::println(String::new());
        //Convert input buffer to string
        let stdin_input: String = buffer::chars_to_string(&self.input_buffer);
        //If input is empty, print prompt (if state is IDLE)
        if stdin_input.trim().len() == 0 {
            if self.last_state == ShellState::Idle {
                console::print(format!("{} ", shell.get_promptline(&self.processor)));
            }
            self.clear_buffer();
        } else {
            //Treat input
            //If state is Idle, convert expression, otherwise convert text
            let input: String = match self.last_state {
                ShellState::Idle => {
                    //Resolve alias
                    let mut argv: Vec<String> = Vec::with_capacity(stdin_input.matches(" ").count() + 1);
                    for arg in stdin_input.split_whitespace() {
                        argv.push(String::from(arg));
                    }
                    //Process arg 0
                    resolve_command(&mut argv, &self.config);
                    //Rejoin arguments
                    let input: String = argv.join(" ") + "\n";
                    match self.processor.expression_to_latin(&input) {
                        Ok(ex) => ex,
                        Err(err) => {
                            print_err(String::from(format!("Input error: {:?}", err)), self.config.output_config.translate_output, &self.processor);
                            //Clear input buffer
                            self.clear_buffer();
                            return;
                        }
                    }
                },
                ShellState::SubprocessRunning => self.processor.text_to_latin(&buffer::chars_to_string(&self.input_buffer)),
                _ => {
                    self.clear_buffer();
                    return;
                }
            };
            //Clear input buffer
            self.clear_buffer();
            if self.last_state == ShellState::Idle {
                //Check if clear command
                if input.starts_with("clear") {
                    //Clear screen, then write prompt
                    console::clear();
                    console::print(format!("{} ", shell.get_promptline(&self.processor)));
                } else if input.starts_with("history") {
                    //TODO: print history
                } else if input.starts_with("!") {
                    //TODO: command from history
                } else { //Write input as usual
                    if let Err(err) = shell.write(input) {
                        print_err(String::from(err.to_string()), self.config.output_config.translate_output, &self.processor);
                    }
                }
            } else { //Write input as usual
                if let Err(err) = shell.write(input) {
                    print_err(String::from(err.to_string()), self.config.output_config.translate_output, &self.processor);
                }
            }
        }
    }

    /// ### perform_enter
    /// 
    /// Perform enter in non interactive shell
    fn perform_enter(&mut self, shell: &mut Shell) {
        //@! Handle enter...
        let stdin_input: String = buffer::chars_to_string(&self.input_buffer);
        //If input is empty, ignore it
        if stdin_input.trim().len() > 0 {
            //Treat input
            //Convert text
            let input: String = self.processor.text_to_latin(&stdin_input);
            if let Err(err) = shell.write(input) {
                print_err(String::from(err.to_string()), self.config.output_config.translate_output, &self.processor);
            }
        }
        self.clear_buffer();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::Config;
    use crate::translator::ioprocessor::IOProcessor;
    use crate::translator::Language;
    use crate::translator::new_translator;

    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_runtimeprops_new() {
        let props: RuntimeProps = new_runtime_props(true);
        assert!(props.config.get_alias(&String::from("ll")).is_none());
        assert_eq!(props.processor.language, Language::Russian);
        assert_eq!(props.input_buffer.capacity(), 2048);
        assert_eq!(props.input_buffer_cursor, 0);
        assert_eq!(props.interactive, true);
        assert_eq!(props.last_state, ShellState::Unknown);
        assert_eq!(props.state_changed, true);
        assert_eq!(props.rev_search, false);
    }

    #[test]
    fn test_runtimeprops_clear_buffer() {
        let mut props: RuntimeProps = new_runtime_props(false);
        props.input_buffer = vec!['a', 'b', 'c'];
        props.input_buffer_cursor = 3;
        props.clear_buffer();
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
    }

    #[test]
    fn test_runtimeprops_update_state() {
        let mut props: RuntimeProps = new_runtime_props(true);
        assert_eq!(props.get_last_state(), ShellState::Unknown);
        assert_eq!(props.get_state_changed(), true);
        props.report_state_changed_notified();
        assert_eq!(props.get_state_changed(), false);
        props.update_state(ShellState::Idle);
        assert_eq!(props.get_last_state(), ShellState::Idle);
        assert_eq!(props.get_state_changed(), true);
    }

    #[test]
    fn test_runtimeprops_backspace() {
        let mut props: RuntimeProps = new_runtime_props(true);
        props.input_buffer = vec!['a', 'b', 'c'];
        //If cursor is 0, cursor and input buffer won't change
        props.backspace();
        assert_eq!(props.input_buffer_cursor, 0);
        assert_eq!(props.input_buffer.len(), 3);
        props.input_buffer_cursor = 3;
        //Backspace from end of buffer
        props.backspace();
        assert_eq!(props.input_buffer_cursor, 2);
        assert_eq!(props.input_buffer, vec!['a', 'b']);
        //Set cursor to 1 and backspace from the middle
        props.input_buffer_cursor = 1;
        props.backspace();
        assert_eq!(props.input_buffer_cursor, 0);
        assert_eq!(props.input_buffer, vec!['b']);
        //Try to delete with cursor out of range
        props.input_buffer = vec!['a', 'b', 'c'];
        props.input_buffer_cursor = 4;
        props.backspace();
        assert_eq!(props.input_buffer_cursor, 3);
        assert_eq!(props.input_buffer.len(), 3);
    }

    #[test]
    fn test_runtimeprops_move_cursor() {
        let mut props: RuntimeProps = new_runtime_props(true);
        props.input_buffer = vec!['a', 'b', 'c', 'd', 'e'];
        //Move left
        props.input_buffer_cursor = 5;
        props.move_left();
        assert_eq!(props.input_buffer_cursor, 4);
        //Try to move left when is at 0
        props.input_buffer_cursor = 0;
        props.move_left();
        assert_eq!(props.input_buffer_cursor, 0);
        //Move right
        props.move_right();
        assert_eq!(props.input_buffer_cursor, 1);
        //Move out of bounds
        props.input_buffer = vec!['a'];
        props.move_right();
        assert_eq!(props.input_buffer_cursor, 1);
    }

    #[test]
    fn test_runtimeprops_handle_input_event() {
        let mut props: RuntimeProps = new_runtime_props(true);
        let mut shell: Shell = Shell::start(String::from("sh"), Vec::new(), &props.config.prompt_config).unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        props.input_buffer = vec!['l', 's', ' ', '-', 'l'];
        props.input_buffer_cursor = 5;
        //TODO: arrow up
        props.handle_input_event(InputEvent::ArrowUp, &mut shell);
        //TODO: arrow down
        props.handle_input_event(InputEvent::ArrowDown, &mut shell);
        //Move cursor to left by 1 position
        props.handle_input_event(InputEvent::ArrowLeft, &mut shell);
        assert_eq!(props.input_buffer_cursor, 4);
        //Move cursor to right by 1 position
        props.handle_input_event(InputEvent::ArrowRight, &mut shell);
        assert_eq!(props.input_buffer_cursor, 5);
        //Backspace
        props.handle_input_event(InputEvent::Backspace, &mut shell);
        assert_eq!(props.input_buffer, vec!['l', 's', ' ', '-']);
        assert_eq!(props.input_buffer_cursor, 4);
        //Carriage return
        props.handle_input_event(InputEvent::CarriageReturn, &mut shell);
        //Ctrl (interactive mode)
        props.last_state = ShellState::Idle;
        //CTRL A
        props.handle_input_event(InputEvent::Ctrl(1), &mut shell);
        assert_eq!(props.input_buffer_cursor, 0);
        //CTRL B
        props.input_buffer_cursor = 2;
        props.handle_input_event(InputEvent::Ctrl(2), &mut shell);
        assert_eq!(props.input_buffer_cursor, 1);
        //CTRL C
        props.handle_input_event(InputEvent::Ctrl(3), &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //CTRL D
        props.input_buffer = vec!['l', 's', ' ', '-', 'l'];
        props.input_buffer_cursor = 5;
        props.handle_input_event(InputEvent::Ctrl(4), &mut shell);
        assert_eq!(props.input_buffer, vec!['l', 's', ' ', '-']);
        assert_eq!(props.input_buffer_cursor, 4);
        //CTRL E
        props.input_buffer_cursor = 1;
        props.handle_input_event(InputEvent::Ctrl(5), &mut shell);
        assert_eq!(props.input_buffer_cursor, 4);
        //CTRL F
        props.input_buffer_cursor = 1;
        props.handle_input_event(InputEvent::Ctrl(6), &mut shell);
        assert_eq!(props.input_buffer_cursor, 2);
        //CTRL G TODO: rev search
        props.handle_input_event(InputEvent::Ctrl(7), &mut shell);
        //CTRL H
        props.handle_input_event(InputEvent::Ctrl(8), &mut shell);
        assert_eq!(props.input_buffer, vec!['l', ' ', '-']);
        assert_eq!(props.input_buffer_cursor, 1);
        //CTRL K
        props.handle_input_event(InputEvent::Ctrl(11), &mut shell);
        assert_eq!(props.input_buffer, vec!['l']);
        assert_eq!(props.input_buffer_cursor, 1);
        //CTRL L
        props.handle_input_event(InputEvent::Ctrl(12), &mut shell);
        assert_eq!(props.input_buffer, vec!['l']);
        assert_eq!(props.input_buffer_cursor, 1);
        //CTRL R TODO: rev search
        props.handle_input_event(InputEvent::Ctrl(18), &mut shell);
        //Unhandled ctrl key
        props.handle_input_event(InputEvent::Ctrl(255), &mut shell);
        assert_eq!(props.input_buffer, vec!['l']);
        assert_eq!(props.input_buffer_cursor, 1);
        //Key
        props.clear_buffer();
        props.handle_input_event(InputEvent::Key(String::from("l")), &mut shell);
        assert_eq!(props.input_buffer, vec!['l']);
        assert_eq!(props.input_buffer_cursor, 1);
        //Try UTF8 character
        props.handle_input_event(InputEvent::Key(String::from("л")), &mut shell);
        assert_eq!(props.input_buffer, vec!['l', 'л']);
        assert_eq!(props.input_buffer_cursor, 2);
        //Add character one position behind
        props.move_left();
        props.handle_input_event(InputEvent::Key(String::from("s")), &mut shell);
        assert_eq!(props.input_buffer, vec!['l', 's', 'л']);
        assert_eq!(props.input_buffer_cursor, 2);
        //Enter (empty buffer)
        props.last_state = ShellState::Idle;
        props.input_buffer = Vec::new();
        props.input_buffer_cursor = 0;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Enter (command)
        props.last_state = ShellState::Idle;
        props.input_buffer = vec!['l', 's'];
        props.input_buffer_cursor = 2;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Enter (clear)
        props.last_state = ShellState::Idle;
        props.input_buffer = vec!['c', 'l', 'e', 'a', 'r'];
        props.input_buffer_cursor = 5;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Enter (history)
        props.last_state = ShellState::Idle;
        props.input_buffer = vec!['h', 'i', 's', 't', 'o', 'r', 'y'];
        props.input_buffer_cursor = 7;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Enter (!)
        props.last_state = ShellState::Idle;
        props.input_buffer = vec!['!', '4', '0'];
        props.input_buffer_cursor = 3;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Enter once has terminated
        props.last_state = ShellState::Terminated;
        props.input_buffer = vec!['l', 's'];
        props.input_buffer_cursor = 2;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Write as text
        props.last_state = ShellState::SubprocessRunning;
        props.input_buffer = vec!['h', 'e', 'l', 'l', 'o'];
        props.input_buffer_cursor = 5;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //CTRL key non interactive (or shell state not Idle)
        //SIGINT
        props.handle_input_event(InputEvent::Ctrl(2), &mut shell);
        //Unhandled signal
        props.handle_input_event(InputEvent::Ctrl(1), &mut shell);
        //Terminate shell
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        let _ = shell.stop();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP

        //Non interactive shell enter
        let mut props: RuntimeProps = new_runtime_props(false);
        let mut shell: Shell = Shell::start(String::from("sh"), Vec::new(), &props.config.prompt_config).unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        props.input_buffer = vec!['l', 's'];
        props.input_buffer_cursor = 2;
        props.last_state = ShellState::SubprocessRunning;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Enter with empty buffer
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        let _ = shell.stop();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        assert_eq!(shell.get_state(), ShellState::Terminated);
        //Send signal once has terminated
        props.last_state = ShellState::SubprocessRunning;
        props.handle_input_event(InputEvent::Ctrl(2), &mut shell);
        //Enter when process has terminated
        props.input_buffer = vec!['l', 's'];
        props.input_buffer_cursor = 2;
        props.last_state = ShellState::SubprocessRunning;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
    }

    fn new_runtime_props(interactive: bool) -> RuntimeProps {
        RuntimeProps::new(interactive, Config::default(), IOProcessor::new(Language::Russian, new_translator(Language::Russian)))
    }
}
