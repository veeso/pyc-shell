//! ## Process
//!
//! `process` is the module which takes care of executing processes and handling the process execution

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
extern crate subprocess;

//Fmt
use std::fmt;
//I/O
use std::io::{Read, Write};
//UNIX stuff
use nix::sys::select;
use nix::sys::signal;
use nix::sys::time::TimeVal;
use nix::sys::time::TimeValLike;
use nix::unistd::Pid;
use std::os::unix::io::IntoRawFd;
use std::os::unix::io::RawFd;
//Subprocess
use subprocess::{ExitStatus, Popen, PopenConfig, Redirection};

//TODO: re-implement

/// ### ShellState
///
/// ShellState represents the current shell state
#[derive(Copy, Clone, PartialEq, fmt::Debug)]
pub enum ShellState {
    Idle,
    SubprocessRunning,
    Terminated,
}
