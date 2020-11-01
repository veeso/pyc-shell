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

use super::imiop::{self, Imiop};

use crate::config::Config;
use crate::shell::proc::ShellState;
use crate::shell::Shell;
use crate::translator::ioprocessor::IOProcessor;
use crate::translator::lang::Language;
use crate::translator::new_translator;
use crate::utils::console::InputEvent;

// TODO: convert interactive boolean to Enum ShellEnvMode

/// ## RuntimeProps
///
/// Runtime Props is a wrapper for all the properties used by the Runtime module
pub(super) struct RuntimeProps {
    pub config: Config,
    language: Language,
    last_state: ShellState,
    state_changed: bool,
    imiop: Box<dyn Imiop>,
}

impl RuntimeProps {
    /// ### new
    ///
    /// Instantiates a new RuntimeProps
    pub(super) fn new(interactive: bool, config: Config, language: Language) -> RuntimeProps {
        RuntimeProps {
            config: config.clone(),
            language: language,
            last_state: ShellState::Unknown,
            state_changed: true,
            imiop: RuntimeProps::init_imiop(interactive, &config, language),
        }
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

    /// ### handle_input_event
    ///
    /// Handle input event received from stdin
    pub(super) fn handle_input_event(&mut self, ev: InputEvent, shell: &mut Shell) {
        // Check if IMIOP has to be changed
        self.switch_imiop();
        // Call handle input event for current IMIOP
        self.imiop.handle_input_event(ev, shell);
    }

    /// ### init_imiop
    ///
    /// Instantiate the first IMIOP at first launch of props

    fn init_imiop(interactive: bool, config: &Config, language: Language) -> Box<dyn Imiop> {
        match interactive {
            true => Box::new(imiop::shiop::ShIop::new(
                config.clone(),
                IOProcessor::new(language, new_translator(language)),
            )),
            false => Box::new(imiop::subprociop::SubProcIop::new(
                config.clone(),
                IOProcessor::new(language, new_translator(language)),
            )),
        }
    }

    /// ### switch_imiop
    ///
    /// Change current imiop based on states
    fn switch_imiop(&mut self) {
        // Change if last state changed
        if self.get_state_changed() {
            // TODO: check environmental shell state
            // If in standard state...
            // Check last_state
            self.imiop = match self.get_last_state() {
                ShellState::Idle => Box::new(imiop::shiop::ShIop::new(
                    self.config.clone(),
                    IOProcessor::new(self.language, new_translator(self.language)),
                )),
                ShellState::SubprocessRunning => Box::new(imiop::subprociop::SubProcIop::new(
                    self.config.clone(),
                    IOProcessor::new(self.language, new_translator(self.language)),
                )),
                _ => Box::new(imiop::shiop::ShIop::new(
                    self.config.clone(),
                    IOProcessor::new(self.language, new_translator(self.language)),
                )),
            };
            // Reset state changed
            self.report_state_changed_notified();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::Config;
    use crate::translator::lang::Language;

    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_runtimeprops_new() {
        let props: RuntimeProps = new_runtime_props(true);
        assert!(props.config.get_alias(&String::from("ll")).is_none());
        assert_eq!(props.language, Language::Russian);
        assert_eq!(props.last_state, ShellState::Unknown);
        assert_eq!(props.state_changed, true);
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
    fn test_runtimeprops_switch_imiop() {
        let mut props: RuntimeProps = new_runtime_props(true);
        // State hasn't changed
        props.state_changed = false;
        props.last_state = ShellState::Idle;
        props.switch_imiop();
        // Change state
        props.state_changed = true;
        props.last_state = ShellState::SubprocessRunning;
        props.switch_imiop();
        // Change back to Idle
        props.state_changed = true;
        props.last_state = ShellState::Idle;
        props.switch_imiop();
        // Change to unhandled state
        props.state_changed = true;
        props.last_state = ShellState::Unknown;
        props.switch_imiop();
    }

    #[test]
    fn test_runtimeprops_handle_input_event() {
        let mut props: RuntimeProps = new_runtime_props(true);
        let config: Config = Config::default();
        let mut shell: Shell = Shell::start(
            String::from("sh"),
            Vec::new(),
            &config.prompt_config,
        )
        .unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        props.handle_input_event(InputEvent::Enter, &mut shell);
        //Signal
        props.handle_input_event(InputEvent::Ctrl(3), &mut shell);
        //Stop shell
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        let _ = shell.stop();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
    }

    #[test]
    fn test_runtimeprops_handle_input_event_not_interactive() {
        let mut props: RuntimeProps = new_runtime_props(false);
        let config: Config = Config::default();
        let mut shell: Shell = Shell::start(
            String::from("sh"),
            Vec::new(),
            &config.prompt_config,
        )
        .unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        props.handle_input_event(InputEvent::Enter, &mut shell);
        //Signal
        props.handle_input_event(InputEvent::Ctrl(3), &mut shell);
        //Stop shell
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        let _ = shell.stop();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
    }

    fn new_runtime_props(interactive: bool) -> RuntimeProps {
        RuntimeProps::new(interactive, Config::default(), Language::Russian)
    }
}
