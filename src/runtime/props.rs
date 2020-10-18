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

use super::{print_err, print_out, console_fmt, resolve_command};

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
    rev_search: Option<String>, // Reverse search match
    rev_search_idx: usize, // Reverse search last match index
    history_index: usize
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
            rev_search: None,
            rev_search_idx: 0,
            history_index: 0
        }
    }

    /// ### clear_buffer
    /// 
    /// Clear buffer and reset cursor to 0
    pub(super) fn clear_buffer(&mut self) {
        self.input_buffer.clear();
        self.input_buffer_cursor = 0;
    }

    fn reset_history_index(&mut self) {
        //Reset history index too
        self.history_index = 0;
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
                if self.interactive && self.last_state == ShellState::Idle {
                    //Get previous element in history
                    self.perform_history_backward(shell);
                } else {
                    //Pass key
                    let _ = shell.write(console::input_event_to_string(ev));
                }
            },
            InputEvent::ArrowUp => {
                if self.interactive && self.last_state == ShellState::Idle {
                    //Get next element in history
                    self.perform_history_forward(shell);
                } else {
                    //Pass key
                    let _ = shell.write(console::input_event_to_string(ev));
                }
            },
            InputEvent::ArrowLeft => {
                if self.interactive && self.last_state == ShellState::Idle {
                    self.move_left();
                } else {
                    //Pass key
                    let _ = shell.write(console::input_event_to_string(ev));
                }
            },
            InputEvent::ArrowRight => {
                if self.interactive && self.last_state == ShellState::Idle {
                    self.move_right();
                } else {
                    //Pass key
                    let _ = shell.write(console::input_event_to_string(ev));
                }
            },
            InputEvent::Backspace => {
                self.backspace();
            },
            InputEvent::CarriageReturn => {
                if self.interactive && self.last_state == ShellState::Idle {
                    console::carriage_return();
                } else {
                    let _ = shell.write(console::input_event_to_string(ev));
                }
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
                            //Reset history index
                            self.reset_history_index();
                            // Unset reverse search
                            self.rev_search = None;
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
                            // exit rev search (and clear buffer)
                            self.rev_search = None;
                            self.rev_search_idx = 0;
                            //Abort input and go to newline
                            self.clear_buffer();
                            console::println(String::new());
                            console::print(format!("{} ", shell.get_promptline(&self.processor)));
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
                            // If reverse search is empty, set reverse search match
                            if self.rev_search.is_none() {
                                // Set reverse search to current input buffer
                                let curr_stdin: String = buffer::chars_to_string(&self.input_buffer);
                                self.rev_search = Some(curr_stdin.clone());
                                // Set index to first element (0)
                                self.rev_search_idx = 0;
                                // Write reverse-i-search prompt
                                console::rewrite(format!("{}`{}':  ", console_fmt(String::from("(reverse-i-search)"), self.config.output_config.translate_output, &self.processor),  curr_stdin), curr_stdin.len());
                            }
                            // Find current input in history starting from bottom
                            if let Some(matched) = self.search_reverse(shell) {
                                // Set matched as current input
                                let prev_length: usize = self.input_buffer.len();
                                self.input_buffer.clear();
                                self.input_buffer = matched.chars().collect();
                                // Set cursor to new length
                                self.input_buffer_cursor = self.input_buffer.len();
                                // Print prompt
                                console::rewrite(matched, prev_length);
                            }
                        },
                        _ => {} //Unhandled
                    }
                } else {
                    //Pass to child
                    //FIXME: doesn't work
                    let _ = shell.write(console::input_event_to_string(ev));
                    //let mut output = String::with_capacity(1);
                    //output.push(sig as char);
                    //let _ = shell.write(output);
                    /*
                    if let Some(sig) = super::shellsignal_to_signal(sig) {
                        if let Err(_) = shell.raise(sig) {
                            print_err(String::from("Could not send signal to shell"), self.config.output_config.translate_output, &self.processor);
                        }
                    }*/
                }
            },
            InputEvent::Key(k) => { //Push key
                //Push k to input buffer
                for ch in k.chars() {
                    self.input_buffer.insert(self.input_buffer_cursor, ch);
                    self.input_buffer_cursor += 1;
                }
                // If rev search, put new input buffer to reverse search
                if self.rev_search.is_some() {
                    // Set reverse search to current input buffer
                    let curr_stdin: String = buffer::chars_to_string(&self.input_buffer);
                    self.rev_search = Some(curr_stdin.clone());
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
        //Reset history index
        self.reset_history_index();
        // Exit reverse search
        self.rev_search = None;
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
            //Process input
            self.process_input_interactive(shell, input);
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

    /// ### process_input_interactive
    /// 
    /// Process input after enter in interactive mode
    fn process_input_interactive(&mut self, shell: &mut Shell, mut input: String) {
        if self.last_state == ShellState::Idle {
            //@! Handle events before anything else
            if input.starts_with("!") {
                //Execute command from history
                //Get index
                let history_index: &str = &input.as_str()[1..input.len() - 1];
                //Convert index to number
                if let Ok(history_index) = history_index.parse::<usize>() {
                    //Check if index is bigger than history lenght
                    if history_index >= shell.history.len() {
                        print_err(format!("!{}: event not found", history_index), self.config.output_config.translate_output, &self.processor);
                        console::print(format!("{} ", shell.get_promptline(&self.processor)));
                        return;
                    }
                    //Reverse index
                    let history_index: usize = shell.history.len() - history_index - 1;
                    match shell.history.at(history_index) {
                        Some(cmd) => { //Event exists, replace input with command
                            //Reverse index
                            input = format!("{}\n", cmd);
                        },
                        None => { //Event doesn't exist
                            print_err(format!("!{}: event not found", history_index), self.config.output_config.translate_output, &self.processor);
                            console::print(format!("{} ", shell.get_promptline(&self.processor)));
                            return;
                        }
                    }
                } else { //Event is Not a number
                    print_err(format!("!{}: event not found", history_index), self.config.output_config.translate_output, &self.processor);
                    console::print(format!("{} ", shell.get_promptline(&self.processor)));
                    return;
                }
            }
            //Push input to history
            shell.history.push(input.clone());
            //Check if clear command
            if input.starts_with("clear") {
                //Clear screen, then write prompt
                console::clear();
                console::print(format!("{} ", shell.get_promptline(&self.processor)));
            } else if input.starts_with("history") {
                //Print history
                let history_lines: Vec<String> = shell.history.dump();
                for (idx, line) in history_lines.iter().enumerate() {
                    print_out(format!("{} {}", self.indent_history_index(idx), line), self.config.output_config.translate_output, &self.processor);
                }
                console::print(format!("{} ", shell.get_promptline(&self.processor)));
            } else { //@! Write input as usual
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

    /// ### perform_history_backward
    /// 
    /// Get previous element in history and put it into the buffer
    fn perform_history_backward(&mut self, shell: &mut Shell) {
        //Match history size
        if self.history_index > 1 {
            //Decrement history index
            self.history_index -= 1;
            //Check if history has index
            if let Some(cmd) = shell.history.at(self.history_index - 1) {
                let prev_len: usize = self.input_buffer.len();
                //Clear buffer
                self.clear_buffer();
                //Push command to buffer
                for ch in cmd.chars() {
                    //Push character
                    self.input_buffer.push(ch);
                    //Increment buffer pointer
                    self.input_buffer_cursor += 1;
                }
                //Rewrite line
                console::rewrite(cmd, prev_len);
            }
        } else if self.history_index == 1 {
            let prev_len: usize = self.input_buffer.len();
            //Put history index to 0
            self.history_index = 0;
            //Clear buffer
            self.clear_buffer();
            console::rewrite(String::from(""), prev_len);
        }
    }

    /// ### perform_history_forward
    /// 
    /// Get next element in history and put it into the buffer
    fn perform_history_forward(&mut self, shell: &mut Shell) {
        //Match history size
        if self.history_index + 1 <= shell.history.len() {
            //Increment history index
            self.history_index += 1;
            //Check if history has index
            if let Some(cmd) = shell.history.at(self.history_index - 1) {
                let prev_len: usize = self.input_buffer.len();
                //Clear buffer
                self.clear_buffer();
                //Push command to buffer
                for ch in cmd.chars() {
                    //Push character
                    self.input_buffer.push(ch);
                    //Increment buffer pointer
                    self.input_buffer_cursor += 1;
                }
                //Rewrite line
                console::rewrite(cmd, prev_len);
            }
        }
    }

    /// ### indent_history_index
    /// 
    /// Format history index to 4 digts
    fn indent_history_index(&self, index: usize) -> String {
        if index < 10 {
            format!("   {}", index)
        } else if index < 100 {
            format!("  {}", index)
        } else if index < 1000 {
            format!(" {}", index)
        } else {
            format!("{}", index)
        }
    }

    /// ### search_reverse
    /// 
    /// Perform reverse search
    /// Returns matched command in history
    
    fn search_reverse(&mut self, shell: &Shell) -> Option<String> {
        let current_match: String = match &self.rev_search {
            Some(s) => s.clone(),
            None => return None
        };
        // Iterate over history
        for i in self.rev_search_idx..shell.history.len() {
            // Check if element at index matches (and is different than previous match)
            if let Some(check_match) = shell.history.at(i) {
                if check_match.contains(current_match.as_str()) {
                    // Update index
                    self.rev_search_idx = i + 1; // i + 1, in order to avoid same result at next cycle
                    // Return match
                    return Some(check_match.clone())
                }
            }
        }
        // Return None if not found
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::Config;
    use crate::translator::ioprocessor::IOProcessor;
    use crate::translator::lang::Language;
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
        assert_eq!(props.rev_search, None);
        assert_eq!(props.rev_search_idx, 0);
        assert_eq!(props.history_index, 0);
    }

    #[test]
    fn test_runtimeprops_clear_buffer() {
        let mut props: RuntimeProps = new_runtime_props(false);
        props.input_buffer = vec!['a', 'b', 'c'];
        props.input_buffer_cursor = 3;
        props.clear_buffer();
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //History index
        props.history_index = 128;
        props.reset_history_index();
        assert_eq!(props.history_index, 0);
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
        props.update_state(ShellState::Idle);
        //Prepare history
        shell.history.push(String::from("pwd"));
        shell.history.push(String::from("ls -l"));
        assert_eq!(props.history_index, 0);
        //Arrow up
        props.handle_input_event(InputEvent::ArrowUp, &mut shell);
        assert_eq!(props.history_index, 1); //History index increased
        assert_eq!(props.input_buffer, vec!['l', 's', ' ', '-', 'l']); //ls -l
        assert_eq!(props.input_buffer_cursor, 5);
        //index 2
        props.handle_input_event(InputEvent::ArrowUp, &mut shell);
        assert_eq!(props.history_index, 2); //History index increased
        assert_eq!(props.input_buffer, vec!['p', 'w', 'd']); //pwd
        assert_eq!(props.input_buffer_cursor, 3);
        //Nothing bad should happen, input buffer won't change, history index won't be increased
        props.handle_input_event(InputEvent::ArrowUp, &mut shell);
        assert_eq!(props.history_index, 2); //History index didn't change
        assert_eq!(props.input_buffer, vec!['p', 'w', 'd']); //pwd
        assert_eq!(props.input_buffer_cursor, 3);
        //Arrow down
        props.handle_input_event(InputEvent::ArrowDown, &mut shell);
        assert_eq!(props.history_index, 1); //History index decreased
        assert_eq!(props.input_buffer, vec!['l', 's', ' ', '-', 'l']); //ls -l
        assert_eq!(props.input_buffer_cursor, 5);
        props.handle_input_event(InputEvent::ArrowDown, &mut shell);
        assert_eq!(props.history_index, 0); //History index decreased
        assert_eq!(props.input_buffer.len(), 0); //Empty
        //Buffer should now be empty
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Another arrow down should change nothing
        props.input_buffer = vec!['l', 's'];
        props.input_buffer_cursor = 2;
        props.handle_input_event(InputEvent::ArrowDown, &mut shell);
        assert_eq!(props.history_index, 0); //History index decreased
        assert_eq!(props.input_buffer.len(), 2); //Empty
        assert_eq!(props.input_buffer_cursor, 2);
        //Arrow left
        //Move cursor to left by 1 position
        props.input_buffer = vec!['l', 's', ' ', '-', 'l'];
        props.input_buffer_cursor = 5;
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
        props.history_index = 255;
        props.handle_input_event(InputEvent::Ctrl(3), &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        assert_eq!(props.history_index, 0); //Reset history index
        //CTRL R ( reverse search; set input buffer to ifc)
        props.input_buffer = vec!['i', 'f', 'c'];
        props.input_buffer_cursor = 3;
        shell.history.push(String::from("ifconfig eth0"));
        props.handle_input_event(InputEvent::Ctrl(18), &mut shell);
        // Input buffer should now be 'ifconfig eth'
        assert_eq!(props.input_buffer, vec!['i', 'f', 'c', 'o', 'n', 'f', 'i', 'g', ' ', 'e', 't', 'h', '0']);
        assert_eq!(props.rev_search, Some(String::from("ifc")));
        assert_eq!(props.rev_search_idx, 1); // 0 + 1
        //CTRL G ( exit rev-search )
        props.handle_input_event(InputEvent::Ctrl(7), &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        assert_eq!(props.rev_search, None);
        assert_eq!(props.rev_search_idx, 0); // 0
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
        assert_eq!(props.history_index, 0);
        //Enter (command)
        props.history_index = 255;
        props.last_state = ShellState::Idle;
        props.input_buffer = vec!['l', 's'];
        props.input_buffer_cursor = 2;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        assert_eq!(props.history_index, 0); //Reset history index
        //@! Check if ls is now in history
        assert_eq!(shell.history.at(0).unwrap(), String::from("ls"));
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
        //Enter (! => Out of range)
        props.last_state = ShellState::Idle;
        props.input_buffer = vec!['!', '4', '0'];
        props.input_buffer_cursor = 3;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Enter (! => Valid)
        props.input_buffer = vec!['!', '1'];
        props.input_buffer_cursor = 2;
        props.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(props.input_buffer.len(), 0);
        assert_eq!(props.input_buffer_cursor, 0);
        //Enter (! => String)
        props.input_buffer = vec!['!', 'f', 'o', 'o'];
        props.input_buffer_cursor = 4;
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
    }

    #[test]
    fn test_runtimeprops_handle_input_event_not_interactive() {
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
        //Arrows
        props.handle_input_event(InputEvent::ArrowDown, &mut shell);
        props.handle_input_event(InputEvent::ArrowLeft, &mut shell);
        props.handle_input_event(InputEvent::ArrowRight, &mut shell);
        props.handle_input_event(InputEvent::ArrowUp, &mut shell);
        //Signal
        props.handle_input_event(InputEvent::Ctrl(3), &mut shell);
        //Stop shell
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
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
    }

    #[test]
    fn test_runtimeprops_indent_history_index() {
        let props: RuntimeProps = new_runtime_props(true);
        assert_eq!(props.indent_history_index(0), String::from("   0"));
        assert_eq!(props.indent_history_index(10), String::from("  10"));
        assert_eq!(props.indent_history_index(100), String::from(" 100"));
        assert_eq!(props.indent_history_index(1000), String::from("1000"));
    }

    #[test]
    fn test_runtimeprops_reverse_search() {
        let mut props: RuntimeProps = new_runtime_props(true);
        let mut shell: Shell = Shell::start(String::from("sh"), Vec::new(), &props.config.prompt_config).unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        props.update_state(ShellState::Idle);
        //Prepare history
        shell.history.push(String::from("pwd"));
        shell.history.push(String::from("ifconfig"));
        shell.history.push(String::from("ls -l"));
        shell.history.push(String::from("ls"));
        shell.history.push(String::from("ls -la"));
        shell.history.push(String::from("lsd")); // Newer ls match
        shell.history.push(String::from("if")); // Newer if match
        // Perform reverse search
        props.rev_search = Some(String::from("ls"));
        props.rev_search_idx = 0;
        assert_eq!(props.search_reverse(&mut shell), Some(String::from("lsd")));
        assert_eq!(props.search_reverse(&mut shell), Some(String::from("ls -la")));
        assert_eq!(props.search_reverse(&mut shell), Some(String::from("ls")));
        assert_eq!(props.search_reverse(&mut shell), Some(String::from("ls -l")));
        assert_eq!(props.search_reverse(&mut shell), None);
        assert_eq!(props.search_reverse(&mut shell), None); // No panic?
    }

    fn new_runtime_props(interactive: bool) -> RuntimeProps {
        RuntimeProps::new(interactive, Config::default(), IOProcessor::new(Language::Russian, new_translator(Language::Russian)))
    }
}
