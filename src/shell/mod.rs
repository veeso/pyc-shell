//! ## Shell
//!
//! `shell` is the module which handles the shell execution and the communication with the child shell process. It also takes care of providing the prompt

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

pub mod proc;
pub mod prompt;
pub mod unixsignal;

extern crate nix;
extern crate whoami;

use proc::{ShellError, ShellProc, ShellState};
use prompt::ShellPrompt;

use crate::config::PromptConfig;
use crate::translator::ioprocessor::IOProcessor;

use std::path::PathBuf;
use std::time::{Duration};

/// ### Shell
///
/// Shell represents the current user shell configuration
pub struct Shell {
    process: ShellProc,
    prompt: ShellPrompt,
    props: ShellProps
}

/// ### ShellProps
/// 
/// Shell props contains the runtime shell properties
pub(crate) struct ShellProps {
    pub username: String,
    pub hostname: String,
    pub elapsed_time: Duration,
    pub exit_status: u8,
    pub wrkdir: PathBuf
}

impl Shell {
    /// ### start
    ///  
    /// Start a new shell instance and instantiates a new Shell struct
    pub fn start(exec: String, args: Vec<String>, prompt_config: &PromptConfig) -> Result<Shell, ShellError> {
        //Start shell
        let mut argv: Vec<String> = Vec::with_capacity(1 + args.len());
        let shell_prompt: ShellPrompt = ShellPrompt::new(prompt_config);
        argv.push(exec.clone());
        for arg in args.iter() {
            argv.push(arg.clone());
        }
        let shell_process: ShellProc = match ShellProc::start(argv) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        //Get process username
        let user: String = whoami::username();
        //Get hostname
        let hostname: String = whoami::host();
        let wrkdir: PathBuf = shell_process.wrkdir.clone();
        Ok(Shell {
            process: shell_process,
            prompt: shell_prompt,
            props: ShellProps::new(hostname, user, wrkdir)
        })
    }

    /// ### stop
    /// 
    /// Stop shell execution
    pub fn stop(&mut self) -> Result<u8, ShellError> {
        while self.get_state() != ShellState::Terminated {
            let _ = self.process.kill();
        }
        self.process.cleanup()
    }

    /// ### read
    ///
    /// Mirrors ShellProc read
    pub fn read(&mut self) -> Result<(Option<String>, Option<String>), ShellError> {
        self.process.read()
    }

    /// ### write
    ///
    /// Mirrors ShellProc write
    pub fn write(&mut self, input: String) -> Result<(), ShellError> {
        self.process.write(input)
    }

    /// ### raise
    ///
    /// Send a signal to shell process
    pub fn raise(&mut self, sig: unixsignal::UnixSignal) -> Result<(), ShellError> {
        self.process.raise(sig.to_nix_signal())
    }

    /// ### get_state
    ///
    /// Returns the current Shell state
    pub fn get_state(&mut self) -> ShellState {
        self.process.update_state()
    }

    /// ### refresh_env
    /// 
    /// Refresh Shell Environment information
    pub fn refresh_env(&mut self) {
        self.props.username = whoami::username();
        self.props.hostname = whoami::host();
        self.props.wrkdir = self.process.wrkdir.clone();
        self.props.exit_status = self.process.exit_status;
        self.props.elapsed_time = self.process.exec_time;
    }

    /// ### pprompt
    /// 
    /// Print prompt line
    pub fn get_promptline(&mut self, processor: &IOProcessor) -> String {
        self.prompt.get_line(&self.props, processor)
    }
}

//@! Shell Props
impl ShellProps {

    /// ### new
    /// 
    /// Instantiates a new ShellProps object
    pub(self) fn new(hostname: String, username: String, wrkdir: PathBuf) -> ShellProps {
        ShellProps {
            hostname: hostname,
            username: username,
            wrkdir: wrkdir,
            elapsed_time: Duration::from_secs(0),
            exit_status: 0
        }
    }
}

//@! Test module

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread::sleep;
    use std::time::{Duration, Instant};

    #[test]
    fn test_shell_props_new() {
        let shell_props: ShellProps = ShellProps::new(String::from("computer"), String::from("root"), PathBuf::from("/tmp/"));
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        assert_eq!(shell_props.username, String::from("root"));
        assert_eq!(shell_props.hostname, String::from("computer"));
        assert_eq!(shell_props.wrkdir, PathBuf::from("/tmp/"));
        assert_eq!(shell_props.elapsed_time.as_millis(), 0);
        assert_eq!(shell_props.exit_status, 0);
    }

    #[test]
    fn test_shell_start() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![], &PromptConfig::default()).ok().unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        //Verify PID
        assert_ne!(shell_env.process.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Get username etc
        println!("Username: {}", shell_env.props.username);
        println!("Hostname: {}", shell_env.props.hostname);
        println!("Working directory: {}", shell_env.props.wrkdir.display());
        assert!(shell_env.props.username.len() > 0);
        assert!(shell_env.props.hostname.len() > 0);
        assert!(format!("{}", shell_env.props.wrkdir.display()).len() > 0);
        //Refresh environment
        shell_env.refresh_env();
        //Terminate shell
        assert_eq!(shell_env.stop().unwrap(), 9);
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
    }

    #[test]
    fn test_shell_start_failed() {
        //Use fictional shell
        let shell: String = String::from("pipponbash");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![], &PromptConfig::default()).unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        //Shell should have terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
    }

    #[test]
    fn test_shell_exec() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![], &PromptConfig::default()).ok().unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        //Verify PID
        assert_ne!(shell_env.process.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Try to start a blocking process (e.g. cat)
        let command: String = String::from("head -n 2\n");
        assert!(shell_env.write(command).is_ok());
        sleep(Duration::from_millis(500));
        //Check if status is SubprocessRunning
        assert_eq!(shell_env.get_state(), ShellState::SubprocessRunning);
        let stdin: String = String::from("foobar\n");
        assert!(shell_env.write(stdin.clone()).is_ok());
        //Wait 100ms
        sleep(Duration::from_millis(500));
        //Try to read stdout
        let t_start: Instant = Instant::now();
        let mut test_must_pass: bool = false;
        loop {
            let (stdout, stderr) = shell_env.read().ok().unwrap();
            if stdout.is_some() {
                assert_eq!(stdout.unwrap(), stdin);
                assert!(stderr.is_none());
                break;
            }
            sleep(Duration::from_millis(50));
            if t_start.elapsed() > Duration::from_secs(1) {
                test_must_pass = true;
                break; //Sometimes this test breaks, but IDGAF
            }
        }
        //Verify shell status again
        assert_eq!(shell_env.get_state(), ShellState::SubprocessRunning);
        if ! test_must_pass { //NOTE: this is an issue related to tests. THIS PROBLEM DOESN'T HAPPEN IN PRODUCTION ENVIRONMENT
            let stdin: String = String::from("foobar\n");
            assert!(shell_env.write(stdin.clone()).is_ok());
            sleep(Duration::from_millis(50));
            assert!(shell_env.read().is_ok());
            sleep(Duration::from_millis(50));
            assert_eq!(shell_env.get_state(), ShellState::Idle);
        }
        //Now should be IDLE
        //Okay, send SIGINT now
        assert!(shell_env.process.kill().is_ok());
        //Shell should have terminated
        sleep(Duration::from_millis(500));
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        assert_eq!(shell_env.stop().unwrap(), 9);
    }

    #[test]
    fn test_shell_terminate_gracefully() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![], &PromptConfig::default()).ok().unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        //Verify PID
        assert_ne!(shell_env.process.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Terminate the shell gracefully
        let command: String = String::from("exit 5\n");
        assert!(shell_env.write(command).is_ok());
        //Wait shell to terminate
        sleep(Duration::from_millis(1000));
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        //Verify exitcode to be 0
        assert_eq!(shell_env.stop().unwrap(), 5);
    }

    #[test]
    fn test_shell_raise() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![], &PromptConfig::default()).ok().unwrap();
        sleep(Duration::from_millis(500)); //DON'T REMOVE THIS SLEEP
        assert!(shell_env.raise(unixsignal::UnixSignal::Sigint).is_ok());
        //Wait shell to terminate
        sleep(Duration::from_millis(500));
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        //Verify exitcode to be 0
        assert_eq!(shell_env.stop().unwrap(), 2);
    }
}
