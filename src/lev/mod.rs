//! ## Editor
//!
//! `editor` is the module which contains the data types and the implementation of the Lev text editor

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

// Pyc Library
use crate::translator::ioprocessor::IOProcessor;

// Standard library
use std::fs::File;

pub struct LevEditor {
    file: Option<File>, // File opened
    lines: Vec<String>, // Buffer content
    row: usize,         // Row pointed by cursor
    col: usize,         // Column pointed by cursor
    // TODO: states
    iop: IOProcessor
}

impl LevEditor {

    /// ### main
    /// 
    /// LevEditor entry point
    pub fn main(argv: Vec<String>) -> u8 {
        // parse arguments...
        0
    }

}
