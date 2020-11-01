//! ## subprociop
//!
//! `subprociop`, or Sub Process I/O Processor, is the implementation of the IMIOP trait to use
//! when in standard SubProcessRunning state

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

use super::Imiop;
use crate::config::Config;
use crate::runtime::print_err;
use crate::shell::Shell;
use crate::translator::ioprocessor::IOProcessor;
use crate::utils::buffer;
use crate::utils::console::{self, InputEvent};

pub(crate) struct SubProcIop {
    input_buffer: Vec<char>,
    input_buffer_cursor: usize,
    config: Config,
    processor: IOProcessor,
}

impl SubProcIop {
    /// ### new
    ///
    /// Instantiate a new `SubProcIop`
    pub fn new(config: Config, processor: IOProcessor) -> SubProcIop {
        SubProcIop {
            input_buffer: Vec::with_capacity(2048),
            input_buffer_cursor: 0,
            config: config,
            processor: processor,
        }
    }

    /// ### clear_buffer
    ///
    /// Clear buffer and reset cursor to 0
    fn clear_buffer(&mut self) {
        self.input_buffer.clear();
        self.input_buffer_cursor = 0;
    }

    /// ### backspace
    ///
    /// Perform backspace on current console and buffers
    fn backspace(&mut self) {
        //Remove from buffer and backspace (if possible)
        if self.input_buffer_cursor > 0 {
            self.input_buffer_cursor -= 1;
            if self.input_buffer.len() > self.input_buffer_cursor {
                self.input_buffer.remove(self.input_buffer_cursor);
            }
            console::backspace();
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
                print_err(
                    String::from(err.to_string()),
                    self.config.output_config.translate_output,
                    &self.processor,
                );
            }
        }
        self.clear_buffer();
    }
}

impl Imiop for SubProcIop {
    /// ### handle_input_event
    ///
    /// Handle input event received from stdin
    fn handle_input_event(&mut self, ev: InputEvent, shell: &mut Shell) {
        match ev {
            InputEvent::ArrowDown => {
                //Pass key
                let _ = shell.write(console::input_event_to_string(ev));
            }
            InputEvent::ArrowUp => {
                //Pass key
                let _ = shell.write(console::input_event_to_string(ev));
            }
            InputEvent::ArrowLeft => {
                //Pass key
                let _ = shell.write(console::input_event_to_string(ev));
            }
            InputEvent::ArrowRight => {
                //Pass key
                let _ = shell.write(console::input_event_to_string(ev));
            }
            InputEvent::Backspace => {
                self.backspace();
            }
            InputEvent::CarriageReturn => {
                let _ = shell.write(console::input_event_to_string(ev));
            }
            InputEvent::Ctrl(_) => {
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
            InputEvent::Key(k) => {
                //Push key
                //Push k to input buffer
                for ch in k.chars() {
                    self.input_buffer.insert(self.input_buffer_cursor, ch);
                    self.input_buffer_cursor += 1;
                }
                //Print key
                console::print(k);
            }
            InputEvent::Enter => {
                //@! Send input
                self.perform_enter(shell);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::Config;
    use crate::translator::ioprocessor::IOProcessor;
    use crate::translator::lang::Language;
    use crate::translator::new_translator;

    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_runtimeprops_new() {
        let processor = new_subprociop();
        assert!(processor.config.get_alias(&String::from("ll")).is_none());
        assert_eq!(processor.processor.language, Language::Russian);
        assert_eq!(processor.input_buffer.capacity(), 2048);
        assert_eq!(processor.input_buffer_cursor, 0);
    }

    #[test]
    fn test_runtimeprops_backspace() {
        let mut processor = new_subprociop();
        processor.input_buffer = vec!['a', 'b', 'c'];
        //If cursor is 0, cursor and input buffer won't change
        processor.backspace();
        assert_eq!(processor.input_buffer_cursor, 0);
        assert_eq!(processor.input_buffer.len(), 3);
        processor.input_buffer_cursor = 3;
        //Backspace from end of buffer
        processor.backspace();
        assert_eq!(processor.input_buffer_cursor, 2);
        assert_eq!(processor.input_buffer, vec!['a', 'b']);
        //Set cursor to 1 and backspace from the middle
        processor.input_buffer_cursor = 1;
        processor.backspace();
        assert_eq!(processor.input_buffer_cursor, 0);
        assert_eq!(processor.input_buffer, vec!['b']);
        //Try to delete with cursor out of range
        processor.input_buffer = vec!['a', 'b', 'c'];
        processor.input_buffer_cursor = 4;
        processor.backspace();
        assert_eq!(processor.input_buffer_cursor, 3);
        assert_eq!(processor.input_buffer.len(), 3);
    }

    #[test]
    fn test_runtimeprops_handle_input_event_not_interactive() {
        //Non interactive shell enter
        let mut processor = new_subprociop();
        let mut shell: Shell = Shell::start(
            String::from("sh"),
            Vec::new(),
            &processor.config.prompt_config,
        )
        .unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        processor.input_buffer = vec!['l', 's'];
        processor.input_buffer_cursor = 2;
        processor.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(processor.input_buffer.len(), 0);
        assert_eq!(processor.input_buffer_cursor, 0);
        //Enter with empty buffer
        processor.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(processor.input_buffer.len(), 0);
        assert_eq!(processor.input_buffer_cursor, 0);
        //Arrows
        processor.handle_input_event(InputEvent::ArrowDown, &mut shell);
        processor.handle_input_event(InputEvent::ArrowLeft, &mut shell);
        processor.handle_input_event(InputEvent::ArrowRight, &mut shell);
        processor.handle_input_event(InputEvent::ArrowUp, &mut shell);
        //Signal
        processor.handle_input_event(InputEvent::Ctrl(3), &mut shell);
        //Stop shell
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        let _ = shell.stop();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
                                           //Send signal once has terminated
        processor.handle_input_event(InputEvent::Ctrl(2), &mut shell);
        //Enter when process has terminated
        processor.input_buffer = vec!['l', 's'];
        processor.input_buffer_cursor = 2;
        processor.handle_input_event(InputEvent::Enter, &mut shell);
        assert_eq!(processor.input_buffer.len(), 0);
        assert_eq!(processor.input_buffer_cursor, 0);
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
    }

    fn new_subprociop() -> SubProcIop {
        SubProcIop::new(
            Config::default(),
            IOProcessor::new(Language::Russian, new_translator(Language::Russian)),
        )
    }
}
