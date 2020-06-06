//! ## Buffer
//!
//! `buffer` contains utilities for console buffers

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

/// ### chars_to_string
/// 
/// Converts a characters vector to string
pub fn chars_to_string(buff: &Vec<char>) -> String {
    buff.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils_buffer_chars_to_string() {
        assert_eq!(chars_to_string(&vec!['a', 'b', 'c', 'л']), String::from("abcл"));
    }
}
