//! ## File
//!
//! `File` module implements some utilities related to files

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

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// ### read_lines
/// 
/// Read lines from file
pub fn read_lines<P>(filename: P) -> io::Result<Vec<String>> where P: AsRef<Path>, {
    let file: File = File::open(filename)?;
    let reader = io::BufReader::new(file).lines();
    let mut lines: Vec<String> = Vec::new();
    for line in reader {
        if let Ok(line) = line {
            lines.push(line);
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_utils_file_read_lines() {
        let sample_file: tempfile::NamedTempFile = write_sample_file();
        let res: io::Result<Vec<String>> = read_lines(sample_file.path());
        assert!(res.is_ok());
        let lines: Vec<String> = res.unwrap();
        assert_eq!(lines.len(), 3);
        assert_eq!(*lines.get(0).unwrap(), String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit."));
        assert_eq!(*lines.get(1).unwrap(), String::from("Mauris ultricies consequat eros,"));
        assert_eq!(*lines.get(2).unwrap(), String::from("nec scelerisque magna imperdiet metus."));
    }

    #[test]
    fn test_utils_file_read_lines_no_file() {
        assert!(read_lines(Path::new("/sample.file123123.txt")).is_err());
    }

    /// ### write_sample_file
    /// Write a sample file
    fn write_sample_file() -> tempfile::NamedTempFile {
        // Write
        let mut tmpfile: tempfile::NamedTempFile = tempfile::NamedTempFile::new().unwrap();
        write!(
            tmpfile,
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nMauris ultricies consequat eros,\nnec scelerisque magna imperdiet metus.\n"
        )
        .unwrap();
        tmpfile
    }
}
