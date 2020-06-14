//! ## Translator
//!
//! `translator` is the module which takes care of translating latin to russian cyrillic and viceversa

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

pub mod ioprocessor;
pub mod lang;

use lang::Language;

/// ### Translator
///
/// Struct used to convert form cyrillic script to latin script and viceversa
pub trait Translator {
  /// ### to_latin
  ///
  /// Converts a string which contains russian cyrillic characters into a latin string.
  /// Characters between '"' (quotes) are escaped, expressions inside escaped blocks are translitarated anyway
  fn to_latin(&self, input: &String) -> String;

  /// ### to_cyrillic
  ///
  /// Converts a string which contains latin characters into a russian cyrillic string.
  /// Characters between quotes are escapes
  fn to_cyrillic(&self, input: &String) -> String;
}

/// ### new_translator
///
/// instantiates a new Translator with the provided language,
/// associating the correct conversion functions
pub fn new_translator(language: Language) -> Box<dyn Translator> {
  match language {
    Language::Belarusian => Box::new(lang::Belarusian {}),
    Language::Russian => Box::new(lang::Russian {}),
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_translator_new() {
    //Just don't panic
    let _ = new_translator(Language::Russian);
  }

}
