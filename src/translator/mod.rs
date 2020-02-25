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

use std::fmt;

#[derive(Copy, Clone, PartialEq, fmt::Debug)]
pub enum ParserError {
  MissingToken,
}

/// ### Language
///
/// Cyrillic alphabet language
#[derive(Copy, Clone, PartialEq, fmt::Debug)]
pub enum Language {
  Russian,
}

struct ParserStates {
  escape_block: bool, //Check if we're inside an escaped block (hey, keep out for expressions though)
  backslash: bool,    //Check if backslash is active
  in_expression: bool, //Check is we're inside an expression
  skip_counter: usize, //The amount of cycles to skip
  previous_state: Option<Box<ParserStates>>, //Reference to previous state
}

impl ParserStates {
  fn new(previous_state: Option<ParserStates>) -> ParserStates {
    ParserStates {
      escape_block: false,
      backslash: false,
      in_expression: false,
      skip_counter: 0,
      previous_state: match previous_state {
        None => None,
        Some(prev_state) => Some(Box::new(prev_state)),
      },
    }
  }

  fn clone(strref: &ParserStates) -> ParserStates {
    ParserStates {
      escape_block: strref.escape_block,
      backslash: strref.backslash,
      in_expression: strref.in_expression,
      skip_counter: strref.skip_counter,
      previous_state: match &strref.previous_state {
        //Recursive clone
        None => None,
        Some(state_box) => Some(Box::new(ParserStates::clone(state_box.as_ref()))),
      },
    }
  }

  fn restore_previous_state(&mut self) -> ParserStates {
    match &self.previous_state {
      None => panic!("ParserState has no previous state"),
      Some(prev_state) => ParserStates::clone(prev_state.as_ref()),
    }
  }
}

/// ### Translator
///
/// Struct used to convert form cyrillic script to latin script and viceversa
pub trait Translator {
  /// ### to_latin
  ///
  /// Converts a string which contains russian cyrillic characters into a latin string.
  /// Characters between '"' (quotes) are escaped, expressions inside escaped blocks are translitarated anyway
  fn to_latin(&self, input: String) -> Result<String, ParserError>;

  /// ### to_cyrillic
  ///
  /// Converts a string which contains latin characters into a russian cyrillic string.
  /// Characters between quotes are escapes
  fn to_cyrillic(&self, input: String) -> String;
}

/// ### new_translator
///
/// instantiates a new Translator with the provided language,
/// associating the correct conversion functions
pub fn new_translator(language: Language) -> Box<dyn Translator> {
  match language {
    Language::Russian => Box::new(Russian {}),
  }
}

/// ## Languages
///
/// Languages are empty structs which must implement the Translator trait

struct Russian {}

mod russian;