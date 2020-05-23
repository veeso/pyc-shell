//! ## Process
//!
//! `Process` contains the implementation for ShellProc

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
extern crate tempfile;
extern crate uuid;

use super::{ShellError, ShellProc, ShellState};
use super::pipe::Pipe;

use std::ffi::{CStr, CString};
use std::os::unix::io::RawFd;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use uuid::Uuid;

impl ShellProc {

    /// ### start
    /// 
    /// Start a process
    pub fn start(argv: Vec<String>) -> Result<ShellProc, ShellError> {
        if argv.len() == 0 {
            return Err(ShellError::CouldNotStartProcess)
        }
        //Generate UUID - NOTE: UUID is used to notice process that shell subprocess has terminated
        let uuid: String = Uuid::new_v4().to_hyphenated().to_string();
        //Create pipes
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let stdin_pipe: Pipe = match Pipe::open(&tmpdir.path().join("stdin.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        let stderr_pipe: Pipe = match Pipe::open(&tmpdir.path().join("stderr.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        let stdout_pipe: Pipe = match Pipe::open(&tmpdir.path().join("stdout.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        //Fork process
        match nix::unistd::fork() {
            Ok(nix::unistd::ForkResult::Parent { child, .. }) => {
                //Prepare echo command
                let echo_command: String = format!("echo \"\x02$?;`pwd`;{}\x03\"\n", uuid);
                let wrkdir: PathBuf = match std::env::current_dir() {
                    Err(_) => PathBuf::from("/"),
                    Ok(path) => PathBuf::from(path.as_path())
                };
                //Return Shell Proc
                Ok(ShellProc {
                    state: ShellState::Idle,
                    uuid: uuid,
                    exit_status: 0,
                    exec_time: Duration::from_millis(0),
                    wrkdir: wrkdir,
                    pid: child.as_raw(),
                    rc: 255,
                    stdout_cache: None,
                    start_time: Instant::now(),
                    echo_command: echo_command,
                    stdin_pipe: stdin_pipe,
                    stderr_pipe: stderr_pipe,
                    stdout_pipe: stdout_pipe
                })
            },
            Ok(nix::unistd::ForkResult::Child) => {
                std::process::exit(ShellProc::run(argv, stdin_pipe.fd, stderr_pipe.fd, stdout_pipe.fd));
            },
            Err(_) => {
                return Err(ShellError::CouldNotStartProcess)
            }
        }
    }

    /// ### cleanup
    /// 
    /// cleanup shell once exited. Returns the shell exit code
    pub fn cleanup(&mut self) -> Result<u8, ShellError> {
        if self.update_state() != ShellState::Terminated {
            return Err(ShellError::ShellRunning)
        }
        //Close pipes
        let _ = self.stdin_pipe.close();
        let _ = self.stdout_pipe.close();
        let _ = self.stderr_pipe.close();
        Ok(self.rc)
    }

    /// ### raise
    /// 
    /// Send signal to shell
    pub fn raise(&self, signal: nix::sys::signal::Signal) -> Result<(), ShellError> {
        match nix::sys::signal::kill(nix::unistd::Pid::from_raw(self.pid), signal) {
            Ok(_) => Ok(()),
            Err(_) => Err(ShellError::CouldNotKill)
        }
    }

    /// ### kill
    /// 
    /// Kill shell sending SIGKILL
    pub fn kill(&self) -> Result<(), ShellError> {
        self.raise(nix::sys::signal::Signal::SIGKILL)
    }
    
    /// ### read
    /// 
    /// Read from child pipes
    pub fn read(&mut self) -> Result<(Option<String>, Option<String>), ShellError> {
        if self.update_state() == ShellState::Terminated {
            return Err(ShellError::ShellTerminated)
        }
        let stdout: Option<String> = match self.stdout_pipe.read(100) {
            Ok(stdout) => self.parse_stdout(stdout),
            Err(err) => return Err(err)
        };
        let stderr: Option<String> = match self.stderr_pipe.read(100) {
            Ok(stderr) => match stderr {
                None => None,
                Some(stderr) => Some(stderr)
            },
            Err(err) => return Err(err)
        };
        Ok((stdout, stderr))
    }

    /// ### write
    /// 
    /// Write to child process stdin
    pub fn write(&mut self, mut data: String) -> Result<(), ShellError> {
        if self.update_state() == ShellState::Terminated {
            return Err(ShellError::ShellTerminated)
        }
        //Add echo command to data if shell state is Idle
        if self.state == ShellState::Idle {
            //Append echo command to data
            data.push_str(self.echo_command.as_str());
            //Set state to running
            self.set_state_running();
        }
        self.stdin_pipe.write(data, 5000)
    }

    /// ### run
    /// 
    /// Run method for thread
    fn run(argv: Vec<String>, stdin: RawFd, stderr: RawFd, stdout: RawFd) -> i32 {
        //Set child process stdout/stdin/stderr
        if let Err(_) = nix::unistd::dup2(stdin, 0) {
            return 255
        }
        if let Err(_) = nix::unistd::dup2(stdout, 1) {
            return 255
        }
        if let Err(_) = nix::unistd::dup2(stderr, 2) {
            return 255
        }
        //Prepare arguments
        let mut c_argv: Vec<CString> = Vec::with_capacity(argv.len());
        for arg in argv.iter() {
            c_argv.push(CString::new(arg.as_str()).unwrap());
        }
        let mut c_argv_refs: Vec<&CStr> = Vec::with_capacity(c_argv.len());
        for arg in c_argv.iter() {
            c_argv_refs.push(arg);
        }
        //Exec process
        if let Err(_) = nix::unistd::execvp(c_argv_refs.get(0).unwrap(), c_argv_refs.as_slice()) {
            return 255
        }
        return 0
    }

    /// ### update_state
    /// 
    /// Update shell running state checking if the other thread has terminated
    fn update_state(&mut self) -> ShellState {
        //Wait pid (NO HANG)
        match nix::sys::wait::waitpid(nix::unistd::Pid::from_raw(self.pid), Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
            Err(_) => {}, //Could not get information
            Ok(status) => match status {
                nix::sys::wait::WaitStatus::Exited(_, rc) => {
                    self.state = ShellState::Terminated;
                    self.rc = rc as u8;
                },
                nix::sys::wait::WaitStatus::Signaled(_, signal, _) => {
                    self.state = ShellState::Terminated;
                    self.rc = signal as u8;
                },
                _ => {}, //Still running
            }
        };
        self.state
    }

    /// ### parse_stdout
    /// 
    /// Parse stdout received from shell process
    fn parse_stdout(&mut self, stdout: Option<String>) -> Option<String> {
        match stdout {
            None => None,
            Some(stdout) => {
                //Treat stdout
                let termination_string: String = format!("{}\x03\n", self.uuid);
                //Check if ends with this (\x02${?};${PWD};${UUID}\x03\n)
                //Create check string (cache + stdout)
                let check_string: String = match &self.stdout_cache {
                    None => stdout.clone(),
                    Some(cache) => {
                        let mut s: String = String::with_capacity(stdout.len() + cache.len());
                        s.push_str(cache.as_str());
                        s.push_str(stdout.as_str());
                        s
                    }
                };
                //Check if string ends with termination string
                if check_string.ends_with(termination_string.as_str()) {
                    //It's the end of shell execution, split string in output and METADATA
                    //Let's find the index of \x02
                    let mut stx_index: usize = check_string.len();
                    for c in check_string.chars().rev() {
                        if c == '\x02' {
                            break;
                        }
                        stx_index -= 1; //Decrement STX index
                    }
                    let metadata: String = String::from(&check_string[stx_index..check_string.len() - 2]);
                    //Get stdout
                    let stx_index_stdout: usize = stx_index - match &self.stdout_cache {
                        Some(s) => s.len(),
                        None => 0
                    };
                    let stdout: String = String::from(&stdout[..stx_index_stdout - 1]);
                    //get metadata
                    self.set_state_idle(metadata);
                    //Clear cache
                    self.stdout_cache = None;
                    match stdout.len() {
                        0 => None,
                        _ => Some(stdout)
                    }
                } else {
                    //Not a termination
                    //Push stdout to cache
                    self.stdout_cache = Some(stdout.clone());
                    //Return stdout
                    Some(stdout)
                }
            }
        }
    }

    /// ### set_state_idle
    /// 
    /// Parse metadata string and set state back to idle
    fn set_state_idle(&mut self, metadata: String) {
        for (index, token) in metadata.split(";").enumerate() {
            match index {
                0 => self.exit_status = token.parse::<u8>().unwrap_or(255),
                1 => self.wrkdir = PathBuf::from(token),
                _ => continue
            }
        }
        self.exec_time = self.start_time.elapsed();
        self.state = ShellState::Idle;
    }

    /// ### set_state_running
    /// 
    /// Set state to running
    fn set_state_running(&mut self) {
        self.start_time = Instant::now();
        self.state = ShellState::SubprocessRunning;
    }
}

impl Drop for ShellProc {
    fn drop(&mut self) {
        if let Err(_) = self.cleanup() {
            let _ = self.kill(); //Force to terminate
            let _ = self.cleanup(); //Then finally clean up
        }
    }
}

//@! Test module

#[cfg(test)]
mod tests {

    use super::*;

    use nix::NixPath;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_process_start_stop() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("sh")]).unwrap();
        println!("A new shell started with PID {}", shell_proc.pid);
        //Check shell parameters
        assert_eq!(shell_proc.state, ShellState::Idle);
        assert_eq!(shell_proc.exit_status, 0);
        assert_ne!(shell_proc.pid, 0);
        assert_ne!(shell_proc.wrkdir.len(), 0);
        assert_eq!(shell_proc.exec_time, Duration::from_millis(0));
        assert_eq!(shell_proc.rc, 255);
        assert_ne!(shell_proc.uuid.len(), 0);
        assert!(shell_proc.stdout_cache.is_none());
        assert_eq!(shell_proc.echo_command, format!("echo \"\x02$?;`pwd`;{}\x03\"\n", shell_proc.uuid));
        //Verify shell is still running
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.update_state(), ShellState::Idle);
        //Stop process
        assert!(shell_proc.kill().is_ok());
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.update_state(), ShellState::Terminated);
        //Rc should be set to 9
        assert_eq!(shell_proc.state, ShellState::Terminated);
        assert_eq!(shell_proc.rc, 9);
        //Cleanup
        assert!(shell_proc.cleanup().is_ok());
    }

    #[test]
    fn test_process_start_error() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("piroporopero")]).unwrap();
        println!("A new shell started with PID {}", shell_proc.pid);
        //Shell should have died
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.update_state(), ShellState::Terminated);
        assert_eq!(shell_proc.rc, 255);
    }

    #[test]
    fn test_process_raise() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("sh")]).unwrap();
        println!("A new shell started with PID {}", shell_proc.pid);
        //Verify shell is still running
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.update_state(), ShellState::Idle);
        //Send SIGINT
        assert!(shell_proc.raise(nix::sys::signal::Signal::SIGINT).is_ok());
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.update_state(), ShellState::Terminated);
        assert_eq!(shell_proc.rc, 2);
    }

    #[test]
    fn test_process_parse_metadata() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("sh")]).unwrap();
        println!("A new shell started with PID {}", shell_proc.pid);
        //Parse metadata
        let metadata: String = String::from("128;/home;ee9ec814-a751-4329-850f-6d54d12c8a5c");
        shell_proc.state = ShellState::SubprocessRunning;
        shell_proc.set_state_idle(metadata);
        //Verify metadata have been parsed successfully
        assert_eq!(shell_proc.exit_status, 128);
        assert_eq!(shell_proc.wrkdir, PathBuf::from("/home"));
        assert_eq!(shell_proc.state, ShellState::Idle);
        //Kill
        assert!(shell_proc.kill().is_ok());
    }

    #[test]
    fn test_process_parse_stdout() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("sh")]).unwrap();
        println!("A new shell started with PID {}", shell_proc.pid);
        //Parse stdout when empty
        assert!(shell_proc.parse_stdout(None).is_none());
        //Parse stdout with metadata only (and parse theme)
        shell_proc.state = ShellState::SubprocessRunning;
        assert!(shell_proc.parse_stdout(Some(format!("\x02128;/home;{}\x03\n", shell_proc.uuid))).is_none());
        assert_eq!(shell_proc.exit_status, 128);
        assert_eq!(shell_proc.wrkdir, PathBuf::from("/home"));
        assert_eq!(shell_proc.state, ShellState::Idle);
        //Parse stdout with output only
        shell_proc.state = ShellState::SubprocessRunning;
        assert_eq!(shell_proc.parse_stdout(Some(String::from("HELLO\n"))).unwrap(), String::from("HELLO\n"));
        assert_eq!(shell_proc.state, ShellState::SubprocessRunning); //State unchanged
        assert_eq!(*shell_proc.stdout_cache.as_ref().unwrap(), String::from("HELLO\n"));
        //Parse stdout with everything
        shell_proc.state = ShellState::SubprocessRunning;
        assert_eq!(shell_proc.parse_stdout(Some(format!("HELLO\n\x022;/tmp;{}\x03\n", shell_proc.uuid))).unwrap(), String::from("HELLO\n"));
        assert_eq!(shell_proc.exit_status, 2);
        assert_eq!(shell_proc.wrkdir, PathBuf::from("/tmp"));
        assert_eq!(shell_proc.state, ShellState::Idle);
        assert!(shell_proc.stdout_cache.is_none());
        //Kill
        assert!(shell_proc.kill().is_ok());
    }

    #[test]
    fn test_process_command() {
        let mut shell_proc: ShellProc = ShellProc::start(vec![String::from("sh")]).unwrap();
        println!("A new shell started with PID {}", shell_proc.pid);
        //Send a cd command
        assert!(shell_proc.write(String::from("cd /tmp\n")).is_ok());
        //State should have changed to subprocess
        assert_eq!(shell_proc.state, ShellState::SubprocessRunning);
        //Then read response
        sleep(Duration::from_millis(50));
        let (stdout, stderr) = shell_proc.read().unwrap();
        //Output should be empty
        assert!(stdout.is_none());
        assert!(stderr.is_none());
        //Verify shell is still running
        sleep(Duration::from_millis(100));
        assert_eq!(shell_proc.update_state(), ShellState::Idle);
        //Verify wrkdir is now /tmp/
        assert_eq!(shell_proc.wrkdir, PathBuf::from("/tmp"));
        //Verify exit status
        assert_eq!(shell_proc.exit_status, 0);
        //Verify execution time
        assert_ne!(shell_proc.exec_time.as_nanos(), 0);
        //Stop process
        assert!(shell_proc.kill().is_ok());
        sleep(Duration::from_millis(500));
        assert_eq!(shell_proc.update_state(), ShellState::Terminated);
        //Rc should be set to 9
        assert_eq!(shell_proc.state, ShellState::Terminated);
        assert_eq!(shell_proc.rc, 9);
        //Cleanup
        assert!(shell_proc.cleanup().is_ok());
    }

}
