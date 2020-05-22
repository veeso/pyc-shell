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

//Threads
use std::sync::{Arc, Mutex};
use std::thread;

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
    PipeError(nix::errno::Errno)
}

/// ### ShellProc
/// 
/// Shell Proc represents an instance of the shell process wrapper
#[derive(std::fmt::Debug)]
pub struct ShellProc {
    running: Arc<Mutex<bool>>,              //Running state
    joined: Arc<Mutex<bool>>,               //Tells thread it can terminate
    m_loop: Option<thread::JoinHandle<u8>>, //Returns exitcode
    uuid: String,                           //UUID used for handshake with the shell
    exit_status: u8,                        //Exit status of the subprocess
    pid: usize,                             //Shell pid
    //Pipes
    stdin_pipe: Pipe,
    stdout_pipe: Pipe,
    stderr_pipe: Pipe
}

//TODO: implement fmt pritn for ShellError
