//! ## History
//!
//! `History` provides an API for the shell History

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

use std::collections::VecDeque;

pub struct ShellHistory {
    history: VecDeque<String>
}

impl ShellHistory {

    /// ### new
    /// 
    /// Instantiate a new ShellHistory
    pub fn new() -> ShellHistory {
        ShellHistory {
            history: VecDeque::with_capacity(2048)
        }
    }

    /// ### at
    /// 
    /// Get the command at a certain index of the history
    /// None is returned in case index is out of range
    pub fn at(&self, index: usize) -> Option<String> {
        match self.history.get(index) {
            Some(s) => Some(s.clone()),
            None => None
        }
    }

    /// ### clear
    /// 
    /// Clear history
    pub fn clear(&mut self) {
        self.history.clear();
    }

    /// ### dump
    /// 
    /// Dump history
    pub fn dump(&mut self) -> Vec<String> {
        let mut history: Vec<String> = Vec::with_capacity(self.history.len());
        for entry in self.history.iter() {
            history.push(entry.clone());
        }
        history
    }

    /// ### len
    /// 
    /// Returns history len
    pub fn len(&self) -> usize {
        self.history.len()
    }

    /// ### load
    /// 
    /// Load history
    /// NOTE: the maximum history size will still be the size provided at constructor
    pub fn load(&mut self, lines: Vec<String>) {
        //Clear current history
        self.clear();
        //Parse file
        for line in lines.iter() {
            self.push(line.clone());
        }
    }

    /// ### push
    /// 
    /// Push a new entry to the history.
    /// The entry is stored at the front of the history. The first the newest
    pub fn push(&mut self, line: String) {
        //Check if history overflows the size
        let size: usize = (self.history.capacity() + 1) / 2;
        if self.history.len() + 1 > size {
            self.history.pop_back();
        }
        self.history.push_front(line);
    }

}

//@! Test module

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_shell_history() {
        let mut history: ShellHistory = ShellHistory::new();
        assert_eq!(history.history.capacity(), (2048 * 2 - 1)); //2048 * 2 - 1
        //Load history
        history.load(vec![String::from("ls"), String::from("cd /tmp/")]);
        assert_eq!(history.len(), 2);
        //History at
        assert_eq!(history.at(0).unwrap(), String::from("cd /tmp/"));
        assert_eq!(history.at(1).unwrap(), String::from("ls"));
        assert!(history.at(2).is_none());
        //Push element
        history.push(String::from("pwd"));
        assert_eq!(history.len(), 3);
        assert_eq!(history.at(0).unwrap(), String::from("pwd"));
        //Fill history with 2048 elements
        let mut history_vec: Vec<String> = Vec::with_capacity(2048);
        for i in 0..2048 {
            history_vec.push(format!("echo {}", i));
        }
        history.load(history_vec);
        assert_eq!(history.len(), 2048);
        assert_eq!(history.at(0).unwrap(), String::from("echo 2047"));
        assert_eq!(history.at(2047).unwrap(), String::from("echo 0"));
        //Push element
        history.push(String::from("echo 2048"));
        assert_eq!(history.len(), 2048);
        assert_eq!(history.at(0).unwrap(), String::from("echo 2048"));
        assert_eq!(history.at(2047).unwrap(), String::from("echo 1"));
        //Clear
        history.clear();
        assert_eq!(history.len(), 0);
        //Push element
        history.push(String::from("ls -l"));
        history.push(String::from("cd /tmp/"));
        //Dump history
        let dump: Vec<String> = history.dump();
        assert_eq!(dump.len(), 2);
        assert_eq!(*dump.get(0).unwrap(), String::from("cd /tmp/"));
        assert_eq!(*dump.get(1).unwrap(), String::from("ls -l"));
    }

}
