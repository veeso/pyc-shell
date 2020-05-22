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
        let uuid: String = Uuid::new_v4().to_hyphenated().to_string();
        let mut running = Arc::new(Mutex::new(true));
        //Create pipes
        let tmpdir: tempfile::TempDir = tempfile::TempDir::new().unwrap();
        let stdin_pipe: Pipe = match Pipe::open(&tmpdir.path().join("/stdin.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        let stderr_pipe: Pipe = match Pipe::open(&tmpdir.path().join("/stderr.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        let stdout_pipe: Pipe = match Pipe::open(&tmpdir.path().join("/stdout.fifo")) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };
        //Start thread which guards the shell process
        let running_rc = Arc::clone(&running);
        let m_loop: Option<thread::JoinHandle<u8>> = Some(thread::spawn(move || {
            ShellProc::run(argv, stdin_pipe.fd, stderr_pipe.fd, stdout_pipe.fd, running_rc)
        }));
        //Return Shell Proc
        Ok(ShellProc {
            state: ShellState::Idle,
            running: running,
            m_loop: m_loop,
            uuid: uuid,
            exit_status: 0,
            wrkdir: String::from("/"),
            pid: 0,
            stdout_cache: None,
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
    pub fn read(&mut self) -> Result<(Option<String>, Option<String>), ShellError> {
        let stdout: Option<String> = match self.stdin_pipe.read(50) {
            Ok(stdout) => self.parse_stdout(stdout),
            Err(err) => return Err(err)
        };
        let stderr: Option<String> = match self.stderr_pipe.read(50) {
            Ok(stderr) => match stderr {
                None => None,
                Some(stderr) => Some(stderr)
            },
            Err(err) => return Err(err)
        };
        Ok((stdout, stderr))
    }
    //TODO: write (add 'echo "\x02$$;$?;`pwd`;UUID\x03"')

    /// ### run
    /// 
    /// Run method for thread
    fn run(argv: Vec<String>, stdin: RawFd, stderr: RawFd, stdout: RawFd, running: Arc<Mutex<bool>>) -> u8 {
        //TODO: implement
    }

    /// ### parse_stdout
    /// 
    /// Parse stdout received from shell process
    fn parse_stdout(&mut self, stdout: Option<String>) -> Option<String> {
        match stdout {
            None => None,
            Some(stdout) => {
                //Treat stdout
                let termination_string: String = format!("{}\x03", self.uuid);
                //Check if ends with this (\x02${$};${?};${PWD};${UUID}\x03)
                //Create check string (cache + stdout)
                let check_string: String = match self.stdout_cache {
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
                    let metadata: String = String::from(&check_string[stx_index..]);
                    //Get stdout
                    let stx_index_stdout: usize = stx_index - match self.stdout_cache {
                        Some(s) => s.len(),
                        None => 0
                    };
                    let stdout: String = String::from(&stdout[..stx_index_stdout - 1]);
                    //get metadata
                    self.get_metadata(metadata);
                    //Set state to Idle
                    self.state = ShellState::Idle;
                    //Clear cache
                    self.stdout_cache = None;
                    Some(stdout)
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

    /// ### get_metadata
    /// 
    /// Get metadata from string
    fn get_metadata(&mut self, metadata: String) {
        for (index, token) in metadata.split(";").enumerate() {
            match index {
                0 => self.pid = token.parse::<u64>().unwrap_or(0),
                1 => self.exit_status = token.parse::<u8>().unwrap_or(255),
                2 => self.wrkdir = String::from(token),
                _ => continue
            }
        }
    }
}
