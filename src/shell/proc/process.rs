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

use std::os::unix::io::RawFd;
use std::sync::{Arc, Mutex};
use std::thread;
use uuid::Uuid;

impl ShellProc {

    /// ### start
    /// 
    /// Start a process
    pub fn start(argv: Vec<String>) -> Result<ShellProc, ShellError> {
        //Generate UUID - NOTE: UUID is used to notice process that shell subprocess has terminated
        let uuid: String = Uuid::new_v4().map(|uuid| uuid.to_hyphenated().to_string());
        let mut running = Arc::new(Mutex::new(true));
        //Create pipes
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let stdin_pipe: Pipe = match Pipe::open(tmpdir.path().join("/stdin.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        let stderr_pipe: Pipe = match Pipe::open(tmpdir.path().join("/stderr.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        let stdout_pipe: Pipe = match Pipe::open(tmpdir.path().join("/stdout.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        //Start thread which guards the shell process
        let running_rc = Arc::clone(&running);
        let m_loop: Option<thread::JoinHandle<u8>> = Some(thread::spawn(move || {
            ShellProc::run(argv, stdin_pipe.fd, stderr_pipe.fd, stdout_pipe.fd running_rc)
        }));
        //Return Shell Proc
        Ok(ShellProc {
            running: running,
            m_loop: m_loop,
            uuid: uuid,
            exit_status: 0,
            pid: 0,
            stdin_pipe: stdin_pipe,
            stderr_pipe: stderr_pipe,
            stdout_pipe: stdout_pipe
        })
    }

    //TODO: stop
    //TODO: get_pid
    pub fn pid(&self) -> u64 {
        self.pid
    }
    //TODO: get ret code
    //TODO: raise
    //TODO: kill
    
    /// ### read
    /// 
    /// Read from child pipes
    pub fn read(&self) -> Result<Option<String>, Option<String>, ShellError> {
        let stdout: Option<String> = match self.stdin_pipe.read(50) {
            Ok(stdout) => match stdout {
                None => None,
                Some(stdout) => {
                    //Treat stdout
                    let termination_string: String = format!("{}\x03", self.uuid);
                    //TODO: check if ends with this (\x02${$};${?};${PWD};${UUID}\x03)
                }
            },
            Err(err) => return Err(err)
        };
        let stderr: Option<String> = match self.stderr_pipe.read(50) {
            Ok(stderr) => match stderr {
                None => None,
                Some(stderr) => stderr
            },
            Err(err) => return Err(err)
        };
        Ok(stdout, stderr)
    }
    //TODO: write (add 'echo "\x02$$;$?;`pwd`;UUID\x03"')

    /// ### run
    /// 
    /// Run method for thread
    fn run(argv: Vec<String>, stdin: RawFd, stderr: RawFd, stdout: RawFd, running: Arc<Mutex<bool>>) -> u8 {
    }

}
