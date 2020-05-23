//! ## Proc
//!
//! `Proc` is the module which takes care of executing processes and handling the process execution

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

mod pipe;
pub mod process;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use pipe::Pipe;

//Proc has a thread which runs the subprocess of the shell and 3 pipes (stdout, stdin, stderr). It must provides the function to write and to read

/// ### ShellState
///
/// ShellState represents the current shell state
#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum ShellState {
    Idle,
    SubprocessRunning,
    Terminated,
}

/// ### ShellError
///
/// ShellError represents an error caused by shell module
#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum ShellError {
    CouldNotStartProcess,
    InvalidData,
    IoTimeout,
    ShellRunning,
    ShellTerminated,
    CouldNotKill,
    PipeError(nix::errno::Errno)
}

/// ### ShellProc
/// 
/// Shell Proc represents an instance of the shell process wrapper
#[derive(std::fmt::Debug)]
pub struct ShellProc {
    pub state: ShellState,                  //Shell process state
    pub exit_status: u8,                    //Exit status of the subprocess (child of shell)
    pub pid: i32,                           //Shell pid
    pub wrkdir: PathBuf,                    //Working directory
    pub exec_time: Duration,                //Execution time of the last command
    //Private
    rc: u8,                                 //Return code of the shell process
    uuid: String,                           //UUID used for handshake with the shell
    start_time: Instant,                    //Instant when the last command was started
    stdout_cache: Option<String>,           //Used to prevent buffer fragmentation
    echo_command: String,                   //Echo command
    //Pipes
    stdin_pipe: Pipe,
    stdout_pipe: Pipe,
    stderr_pipe: Pipe
}

impl std::fmt::Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let code_str: String = match self {
            ShellError::CouldNotStartProcess => String::from("Could not start process"),
            ShellError::InvalidData => String::from("Invalid data from process"),
            ShellError::IoTimeout => String::from("I/O timeout"),
            ShellError::ShellTerminated => String::from("Shell has terminated"),
            ShellError::ShellRunning => String::from("Tried to clean shell up while still running"),
            ShellError::CouldNotKill => String::from("Could not send signal to shell process"),
            ShellError::PipeError(errno) => format!("Pipe error: {}", errno),
        };
        write!(f, "{}", code_str)
    }
}

//@! Test module

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_proc_fmt_shell_error() {
        assert_eq!(format!("{}", ShellError::CouldNotStartProcess), String::from("Could not start process"));
        assert_eq!(format!("{}", ShellError::InvalidData), String::from("Invalid data from process"));
        assert_eq!(format!("{}", ShellError::IoTimeout), String::from("I/O timeout"));
        assert_eq!(format!("{}", ShellError::ShellTerminated), String::from("Shell has terminated"));
        assert_eq!(format!("{}", ShellError::ShellRunning), String::from("Tried to clean shell up while still running"));
        assert_eq!(format!("{}", ShellError::CouldNotKill), String::from("Could not send signal to shell process"));
        assert_eq!(format!("{}", ShellError::PipeError(nix::errno::Errno::EACCES)), format!("Pipe error: {}", nix::errno::Errno::EACCES));
    }

}
