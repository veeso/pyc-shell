//! ## imiop
//!
//! `imiop`, or Interactive Mode I/O Processor, provides the data types and methods for indeed
//! I/O processors in interactive modes. These processors are modules which handled the
//! user input in different shell states (Idle, Running, TextEditor, ...)
//! All processors must implement handle_input_event function, which is called by the Runtime

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

use crate::shell::Shell;
use crate::utils::console::InputEvent;

// List of Imiop
pub(crate) mod shiop;
pub(crate) mod subprociop;

/// ## Imiop
/// 
/// Imiop (interactive mode I/O processor) defines the methods an Imiop has to implement
pub(crate) trait Imiop {

    /// ### handle_input_event
    /// 
    /// Handle input event received from stdin
    fn handle_input_event(&mut self, ev: InputEvent, shell: &mut Shell);

}

// TODO: add factory for imiop
