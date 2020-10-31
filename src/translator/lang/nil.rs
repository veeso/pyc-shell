//! ### Nil
//!
//! `nil` language implementation of Translator trait
//! This translator is just for test purposes, and it just doesn't translate anything.
//! Indeed, it returns the same string as the input

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

use super::super::Translator;
use super::Nil;

impl Translator for Nil {
    /// ### Nil translator

    /// ### to_latin
    /// Just returns input
    fn to_latin(&self, input: &String) -> String {
        input.clone()
    }
    /// ### to_cyrillic
    /// Converts a string which contains latin characters into a ukrainian cyrillic string.
    /// Characters between quotes are escapes
    fn to_cyrillic(&self, input: &String) -> String {
        input.clone()
    }
}

//@! Tests

#[cfg(test)]
mod tests {

    use super::*;
    use crate::translator::{new_translator, Language};

    #[test]
    fn test_translator_lang_nil_to_latin() {
        let translator: Box<dyn Translator> = new_translator(Language::Nil);
        assert_eq!(translator.to_latin(&String::from("HELLO WORLD")), String::from("HELLO WORLD"));
    }

    #[test]
    fn test_translator_lang_nil_to_cyrillic() {
        let translator: Box<dyn Translator> = new_translator(Language::Nil);
        assert_eq!(translator.to_cyrillic(&String::from("HELLO WORLD")), String::from("HELLO WORLD"));
    }
}
