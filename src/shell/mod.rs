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

extern crate nix;
extern crate sysinfo;
extern crate whoami;

use nix::sys::signal;
use proc::{ShellError, ShellProc, ShellState};
use std::fmt;
use std::time::{Duration, Instant};
use sysinfo::{ProcessExt, RefreshKind, Signal, System, SystemExt};

/// ### Shell
///
/// Shell represents the current user shell configuration
pub struct Shell {
    process: ShellProc,
    pub username: String,
    pub hostname: String,
    pub wrkdir: String,
    pub rc: u8, //Return code of last process
    pub elapsed_time: Duration, //Duration of last process
    started_time: Instant //Instant the process was started
}

impl Shell {
    /// ### start
    ///  
    /// Start a new shell instance and instantiates a new Shell struct
    pub fn start(shell: String) -> Result<Shell, ShellError> {
        //Start shell
        let argv: Vec<String> = vec![shell];
        let shell_process: ShellProc = match ShellProc::exec(argv) {
            Ok(p) => p,
            Err(err) => return Err(err),
        };
        //Get process PID
        let pid = match shell_process.pid() {
            Some(p) => p,
            None => return Err(ShellError::CouldNotStartProcess),
        };
        //Get process info
        let refresh_kind: RefreshKind = RefreshKind::new();
        let refresh_kind: RefreshKind = refresh_kind.with_processes();
        //Get system information
        let system = System::new_with_specifics(refresh_kind);
        let wrkdir: String;
        //Get process username
        let user: String = whoami::username();
        //Get hostname
        let hostname: String = whoami::host();
        //Get workdir
        match system.get_process(pid as i32) {
            Some(p) => {
                wrkdir = String::from(p.cwd().to_str().unwrap());
            },
            None => return Err(ShellError::CouldNotStartProcess)
        };
        Ok(Shell {
            process: shell_process,
            username: user,
            hostname: hostname,
            wrkdir: wrkdir,
            rc: 0,
            elapsed_time: Duration::from_millis(0),
            started_time: Instant::now()
        })
    }

    /// ### get_state
    ///
    /// Returns the current Shell state
    pub fn get_state(&mut self) -> ShellState {
        self.process.get_state()
    }

    /// ### get_exitcode
    ///
    /// Returns the shell exit status when terminated
    pub fn get_exitcode(&self) -> Option<u8> {
        if self.get_state() == ShellState::Terminated {
            self.process.exit_status
        } else {
            None
        }
    }

    /// ### read
    ///
    /// Mirrors ShellProc read
    pub fn read(&mut self) -> std::io::Result<(Option<String>, Option<String>)> {
        self.process.read()
    }

    /// ### write
    ///
    /// Mirrors ShellProc write
    pub fn write(&mut self, input: String) -> std::io::Result<()> {
        self.process.write(input)
    }

    /// ### sigint
    ///
    /// Send SIGINT to process. The signal is sent to shell or to subprocess (based on current execution state)
    pub fn sigint(&mut self) -> Result<(), ()> {
        self.process.raise(signal::SIGINT)
    }

    /// ### refresh_env
    /// 
    /// Refresh Shell Environment information
    pub fn refresh_env(&mut self) {
        let system = self.get_processes();
        if let Some(p) = system.get_process(self.process.get_pid() as i32) {
            //Get working directory
            self.wrkdir = String::from(p.cwd().to_str().unwrap());
            //TODO: fix cwd doesn't work
            //TODO: get exitcode for previous process
        };
        //Get process username
        self.username = whoami::username();
        //Get hostname
        self.hostname = whoami::host();
    }

    /// ### set_elapsed_time
    /// 
    /// Set elapsed time
    fn set_elapsed_time(&mut self) {
        self.elapsed_time = self.started_time.elapsed();
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
        let mut shell_env: Shell = Shell::start(shell).ok().unwrap();
        //Verify PID
        assert_ne!(shell_env.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Get username etc
        println!("Username: {}", shell_env.username);
        println!("Hostname: {}", shell_env.hostname);
        println!("Working directory: {}", shell_env.wrkdir);
        assert!(shell_env.username.len() > 0);
        assert!(shell_env.hostname.len() > 0);
        assert!(shell_env.wrkdir.len() > 0);
        //Refresh environment
        shell_env.refresh_env();
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
        let mut shell_env: Shell = Shell::start(shell).ok().unwrap();
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
        let mut shell_env: Shell = Shell::start(shell).ok().unwrap();
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
        Shell::start(shell).ok().unwrap(); //Should panic
    }

    #[test]
    fn shell_sigint() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell).ok().unwrap();
        assert!(shell_env.sigint().is_ok());
        //Wait shell to terminate
        sleep(Duration::from_millis(100));
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        //Verify exitcode to be 0
        assert_eq!(shell_env.get_exitcode().unwrap(), 2);
    }
}
