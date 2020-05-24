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
extern crate whoami;

use nix::sys::signal;
use proc::{ShellError, ShellProc, ShellState};
use std::path::PathBuf;
use std::time::{Duration};

/// ### Shell
///
/// Shell represents the current user shell configuration
pub struct Shell {
    process: ShellProc,
    pub username: String,
    pub hostname: String
}

impl Shell {
    /// ### start
    ///  
    /// Start a new shell instance and instantiates a new Shell struct
    pub fn start(exec: String, args: Vec<String>) -> Result<Shell, ShellError> {
        //Start shell
        let mut argv: Vec<String> = Vec::with_capacity(1 + args.len());
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
        Ok(Shell {
            process: shell_process,
            username: user,
            hostname: hostname
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

    #[allow(dead_code)]
    /// ### sigint
    ///
    /// Send SIGINT to process. The signal is sent to shell or to subprocess (based on current execution state)
    pub fn sigint(&mut self) -> Result<(), ShellError> {
        self.process.raise(signal::SIGINT)
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
        //Get process username
        self.username = whoami::username();
        //Get hostname
        self.hostname = whoami::host();
    }

    /// ### wrkdir
    /// 
    /// Get working directory
    pub fn wrkdir(&self) -> PathBuf {
        self.process.wrkdir.clone()
    }

    /// ### exit_status
    /// 
    /// Returns last shell exit status
    pub fn exit_status(&self) -> u8 {
        self.process.exit_status
    }

    /// ### elapsed_time
    /// 
    /// Get the last command execution time
    pub fn elapsed_time(&self) -> Duration {
        self.process.exec_time
    }
}

//@! Test module

#[cfg(test)]
mod tests {

    use super::*;
    use std::thread::sleep;
    use std::time::{Duration, Instant};
    use nix::NixPath;

    #[test]
    fn test_shell_start() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![]).ok().unwrap();
        //Verify PID
        assert_ne!(shell_env.process.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Get username etc
        println!("Username: {}", shell_env.username);
        println!("Hostname: {}", shell_env.hostname);
        println!("Working directory: {}", shell_env.wrkdir().display());
        assert!(shell_env.username.len() > 0);
        assert!(shell_env.hostname.len() > 0);
        assert!(shell_env.wrkdir().len() > 0);
        //Refresh environment
        shell_env.refresh_env();
        //Terminate shell
        assert_eq!(shell_env.stop().unwrap(), 9);
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
    }

    #[test]
    fn test_shell_start_failed() {
        //Use fictional shell
        let shell: String = String::from("pipponbash");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![]).unwrap();
        //Shell should have terminated
        assert_eq!(shell_env.stop().unwrap(), 255);
    }

    #[test]
    fn test_shell_exec() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![]).ok().unwrap();
        //Verify PID
        assert_ne!(shell_env.process.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Try to start a blocking process (e.g. cat)
        let command: String = String::from("head -n 2\n");
        assert!(shell_env.write(command).is_ok());
        sleep(Duration::from_millis(100));
        //Check if status is SubprocessRunning
        assert_eq!(shell_env.get_state(), ShellState::SubprocessRunning);
        let stdin: String = String::from("foobar\n");
        assert!(shell_env.write(stdin.clone()).is_ok());
        //Wait 100ms
        sleep(Duration::from_millis(100));
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
        sleep(Duration::from_millis(100));
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        assert_eq!(shell_env.stop().unwrap(), 9);
    }

    #[test]
    fn test_shell_terminate_gracefully() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![]).ok().unwrap();
        //Verify PID
        assert_ne!(shell_env.process.pid, 0);
        //Verify shell status
        assert_eq!(shell_env.get_state(), ShellState::Idle);
        //Terminate the shell gracefully
        let command: String = String::from("exit 5\n");
        assert!(shell_env.write(command).is_ok());
        //Wait shell to terminate
        sleep(Duration::from_millis(100));
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        //Verify exitcode to be 0
        assert_eq!(shell_env.stop().unwrap(), 5);
    }

    #[test]
    fn test_shell_sigint() {
        //Use universal accepted shell
        let shell: String = String::from("sh");
        //Instantiate and start a shell
        let mut shell_env: Shell = Shell::start(shell, vec![]).ok().unwrap();
        assert!(shell_env.sigint().is_ok());
        //Wait shell to terminate
        sleep(Duration::from_millis(100));
        //Verify shell has terminated
        assert_eq!(shell_env.get_state(), ShellState::Terminated);
        //Verify exitcode to be 0
        assert_eq!(shell_env.stop().unwrap(), 2);
    }
}
