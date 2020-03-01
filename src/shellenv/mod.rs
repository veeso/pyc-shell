//! ## Shellenv
//!
//! `shellenv` is the module which takes care of processing the shell environment and the process execution

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

pub mod process;

extern crate nix;
extern crate sysinfo;

use nix::sys::signal;
use process::{ProcessError, ShellProcess};
use std::fmt;
use sysinfo::{ProcessExt, RefreshKind, Signal, System, SystemExt};

/// ### ShellEnvironment
///
/// ShellEnvironment represents the current user shell environment configuration
pub struct ShellEnvironment {
    state: ShellState,
    pid: u32,
    process: ShellProcess,
    child_pid: Option<u32>, //PID of child process
}

/// ### ShellState
///
/// ShellState represents the current shell state
#[derive(Copy, Clone, PartialEq, fmt::Debug)]
pub enum ShellState {
    Idle,
    SubprocessRunning,
    Terminated,
}

impl ShellEnvironment {
    /// ### start
    ///  
    /// Start a new shell instance and instantiates a new ShellEnvironment struct
    pub fn start(shell: String) -> Result<ShellEnvironment, ProcessError> {
        //Start shell
        let argv: Vec<String> = vec![shell];
        let shell_process: ShellProcess = match ShellProcess::exec(argv) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        //Get process PID
        let pid = match shell_process.pid() {
            Some(p) => p,
            None => return Err(ProcessError::CouldNotStartProcess),
        };
        Ok(ShellEnvironment {
            state: ShellState::Idle,
            pid: pid,
            process: shell_process,
            child_pid: None,
        })
    }

    /// ### get_state
    ///
    /// Returns the current Shell state (the state is internally updated when the function is called)
    pub fn get_state(&mut self) -> ShellState {
        //Check if self is running
        if !self.is_running() {
            //Process is no more running
            self.state = ShellState::Terminated;
        } else if self.is_child_running() {
            //Child is running
            self.state = ShellState::SubprocessRunning;
        } else {
            //Shell is in idle state
            self.state = ShellState::Idle;
        }
        self.state
    }

    /// ### is_running
    /// 
    /// Fast check to verify if the shell is still running; this method should be preferred to get_state for fast and iterative checks
    pub fn is_running(&mut self) -> bool {
        match self.process.is_running() {
            true => true,
            false => {
                self.state = ShellState::Terminated;
                false
            }
        }
    }

    /// ### get_exitcode
    ///
    /// Returns the shell exit status when terminated
    pub fn get_exitcode(&self) -> Option<u8> {
        if self.state == ShellState::Terminated {
            self.process.exit_status
        } else {
            None
        }
    }

    /// ### read
    ///
    /// Mirrors ShellProcess read
    pub fn read(&mut self) -> std::io::Result<(Option<String>, Option<String>)> {
        self.process.read()
    }

    /// ### write
    ///
    /// Mirrors ShellProcess write
    pub fn write(&mut self, input: String) -> std::io::Result<()> {
        self.process.write(input)
    }

    /// ### sigint
    ///
    /// Send SIGINT to process. The signal is sent to shell or to subprocess (based on current execution state)
    pub fn sigint(&mut self) -> Result<(), ()> {
        match self.is_child_running() {
            true => {
                let system = self.get_processes();
                match system.get_process(self.child_pid.unwrap() as i32) {
                    Some(p) => match p.kill(Signal::Interrupt) {
                        true => Ok(()),
                        false => Err(()),
                    },
                    None => Err(()),
                }
            }
            false => self.process.raise(signal::SIGINT),
        }
    }

    /// ### is_child_running
    ///
    /// checks whether there is at least a child process running
    fn is_child_running(&mut self) -> bool {
        let system = self.get_processes();
        //Iterate over active processes
        for (pid, proc) in system.get_processes() {
            let parent_pid = match proc.parent() {
                Some(p) => p,
                None => continue,
            };
            if parent_pid as u32 == self.pid {
                //Set child pid
                self.child_pid = Some(*pid as u32);
                return true;
            }
        }
        self.child_pid = None; //Set child pid to None
        false
    }

    /// ### get_processes
    ///
    /// get current running processes in your system
    fn get_processes(&self) -> System {
        let refresh_kind: RefreshKind = RefreshKind::new();
        let refresh_kind: RefreshKind = refresh_kind.with_processes();
        //Get system information
        System::new_with_specifics(refresh_kind)
    }
}

//@! Test module

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn shell_start() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: ShellEnvironment = ShellEnvironment::start(shell).ok().unwrap();
        //Verify PID
        assert_ne!(shell_env.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Verify exitcode UNSET
        assert!(shell_env.get_exitcode().is_none());
        //Terminate shell
        assert!(shell_env.process.kill().is_ok());
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        //Verify exit code
        assert_eq!(shell_env.get_exitcode().unwrap(), 9); //Exitcode will be 9
    }

    #[test]
    fn shell_exec() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: ShellEnvironment = ShellEnvironment::start(shell).ok().unwrap();
        //Verify PID
        assert_ne!(shell_env.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Try to start a blocking process (e.g. cat)
        let command: String = String::from("cat\n");
        assert!(shell_env.write(command).is_ok());
        sleep(Duration::from_millis(100));
        //Check if status is SubprocessRunning
        assert_eq!(shell_env.get_state(), ShellState::SubprocessRunning);
        let stdin: String = String::from("foobar\n");
        assert!(shell_env.write(stdin.clone()).is_ok());
        //Wait 100ms
        sleep(Duration::from_millis(100));
        //Try to read stdout
        let (stdout, stderr) = shell_env.read().ok().unwrap();
        assert_eq!(stdout.unwrap(), stdin);
        assert!(stderr.is_none());
        //Verify shell status again
        assert_eq!(shell_env.get_state(), ShellState::SubprocessRunning);
        //Okay, send SIGINT now
        assert!(shell_env.sigint().is_ok());
        //Status must be Idle again now
        sleep(Duration::from_millis(100));
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Okay, terminate shell now
        assert!(shell_env.process.kill().is_ok());
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
    }

    #[test]
    fn shell_terminate_gracefully() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: ShellEnvironment = ShellEnvironment::start(shell).ok().unwrap();
        //Verify PID
        assert_ne!(shell_env.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Terminate the shell gracefully
        let command: String = String::from("exit\n");
        assert!(shell_env.write(command).is_ok());
        //Wait shell to terminate
        sleep(Duration::from_millis(100));
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        //Verify exitcode to be 0
        assert_eq!(shell_env.get_exitcode().unwrap(), 0);
    }

    #[test]
    #[should_panic]
    fn shell_start_failed() {
        //Use fictional shell
        let shell: String = String::from("pipponbash");
        //Instantiate and start a shell
        ShellEnvironment::start(shell).ok().unwrap(); //Should panic
    }

    #[test]
    fn shell_sigint() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: ShellEnvironment = ShellEnvironment::start(shell).ok().unwrap();
        assert!(shell_env.sigint().is_ok());
        //Wait shell to terminate
        sleep(Duration::from_millis(100));
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        //Verify exitcode to be 0
        assert_eq!(shell_env.get_exitcode().unwrap(), 2);
    }
}
