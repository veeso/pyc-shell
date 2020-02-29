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

use std::collections::HashMap;

/// ### ShellEnvironment
/// 
/// ShellEnvironment represents the current user shell environment configuration

pub struct ShellEnvironment {
    user_shell: String,
    user_alias: HashMap<String, String>
}
