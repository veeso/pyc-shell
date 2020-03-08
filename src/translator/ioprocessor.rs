//! ## IOProcessor
//!
//! `IoProcessor` is the module which takes care of handling different kind of inputs to convert them in the desidered alphabet

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

extern crate regex;

use regex::Regex;
use std::fmt;

use super::Language;
use super::Translator;

const COLORS_ESCAPE_REGEX: &str = "\x1b\\[[0-9]{1,2}m";

pub struct IOProcessor {
  translator: Box<dyn Translator>,
  pub language: Language,
  escape_colors_regex: Regex, //Escape regex as struct member to increase speed up to 500%
}

/// ### ExpressionParserError
///
/// Parser Error represents an error while parsing an expression

#[derive(Copy, Clone, PartialEq, fmt::Debug)]
pub enum ExpressionParserError {
  MissingToken,
}

/// ### ExpressionParserStates
///
/// Expression Parser states is a struct which represents the current state in converting an expressions into a text

struct ExpressionParserStates {
  text: String,                                        //Current converted expression text
  expression_token: String,                            //Current expression token
  escape_block: bool, //Check if we're inside an escaped block (hey, keep out for expressions though)
  backslash: bool,    //Check if backslash is active
  in_expression: bool, //Check is we're inside an expression
  previous_state: Option<Box<ExpressionParserStates>>, //Reference to previous state
}

/// ### ExpressionConversion
///
/// Expression Conversion indicates the type of conversion to perform on the expression

#[allow(dead_code)]
enum ExpressionConversion {
  ToLatin,
  ToCyrillic,
}

impl IOProcessor {
  /// ### new
  ///
  /// Instantiates a new IOProcessor with the provided translator
  pub fn new(language: Language, translator: Box<dyn Translator>) -> IOProcessor {
    let this_lang_regex: String =
      String::from(translator.to_cyrillic(String::from(COLORS_ESCAPE_REGEX)));
    let re: Regex = Regex::new(this_lang_regex.as_str()).unwrap();
    IOProcessor {
      translator: translator,
      language: language,
      escape_colors_regex: re,
    }
  }

  /// ### expression_to_latin
  ///
  /// Converts a cyrillic expression into a latin string ready to be performed as a shell process
  /// An expression must care of backslashes, escapes and inner expressions '(...)'
  pub fn expression_to_latin(&self, expression: String) -> Result<String, ExpressionParserError> {
    self.translate_expression(expression, ExpressionConversion::ToLatin)
  }

  #[allow(dead_code)]
  pub fn expression_to_cyrillic(
    &self,
    expression: String,
  ) -> Result<String, ExpressionParserError> {
    self.translate_expression(expression, ExpressionConversion::ToCyrillic)
  }

  /// ### text_to_latin
  ///
  /// Converts a cyrillic text into latin using the provided translator
  pub fn text_to_latin(&self, text: String) -> String {
    self.translator.to_latin(text)
  }

  /// ### text_to_cyrillic
  ///
  /// Converts a latin text into cyrillic using the provided translator
  pub fn text_to_cyrillic(&self, text: String) -> String {
    self.escape_cyrillic(self.translator.to_cyrillic(text))
  }

  /// ### translate_expression
  ///
  /// Converts an expression and translate unescaped texts using the desidered translate function
  fn translate_expression(
    &self,
    expression: String,
    conversion: ExpressionConversion,
  ) -> Result<String, ExpressionParserError> {
    //Instantiate a new Parser State
    let mut states: ExpressionParserStates = ExpressionParserStates::new(None);
    //Iterate over input
    for c in expression.chars() {
      //If character is '(' an expression block starts (if backlsash is disabled)
      if c == '(' && !states.backslash {
        //Set escape to false
        states.escape_block = false;
        //Convert current expression to latin and push it to text
        states.text.push_str(
          match conversion {
            ExpressionConversion::ToLatin => self.translator.to_latin(states.expression_token),
            ExpressionConversion::ToCyrillic => {
              self.translator.to_cyrillic(states.expression_token)
            }
          }.as_str(),
        );
        //Expression token is reinitialized
        states.expression_token = String::new();
        //@! Create new state
        states = ExpressionParserStates::new(Some(states));
        states.in_expression = true;
        //Push '(' to new expression
        states.expression_token.push(c);
        continue;
      }
      //If character is ')' an expression ends (if backslash is disabled)
      if c == ')' && !states.backslash {
        states.in_expression = false;
        //Push ')' to current expression
        states.expression_token.push(c);
        //Convert current expression to latin and push it to text
        states.text.push_str(
          match conversion {
            ExpressionConversion::ToLatin => {
              self.translator.to_latin(states.expression_token.clone())
            }
            ExpressionConversion::ToCyrillic => {
              self.translator.to_cyrillic(states.expression_token.clone())
            }
          }.as_str(),
        );
        //Save text into a tmp variable
        let expression_output: String = states.text.clone();
        //If there are still active states, return error 'missing token'
        if states.backslash || states.in_expression || states.escape_block {
          //Check if expression has been completely closed
          return Err(ExpressionParserError::MissingToken);
        }
        //@! Restore previous state
        states = match states.previous_state {
          Some(_) => states.restore_previous_state(),
          None => return Err(ExpressionParserError::MissingToken),
        };
        //Push converted expression to previous state's text
        states.text.push_str(expression_output.as_str());
        continue;
      } //@! End of expression closed
        //Handle quotes
        //Check if escape (and previous character is not backslash)
      if c == '"' && !states.backslash {
        if states.escape_block {
          //Escape block ends, push current token to text WITHOUT CONVERTING IT
          //Push quote to expression token
          states.expression_token.push(c);
          //Push expression token to text without converting it
          states.text.push_str(states.expression_token.as_str());
          //Reset expression token
          states.expression_token = String::new();
        } else {
          //Escape block starts
          //Convert and then Push current expression token to text
          states.text.push_str(
            match conversion {
              ExpressionConversion::ToLatin => self.translator.to_latin(states.expression_token),
              ExpressionConversion::ToCyrillic => {
                self.translator.to_cyrillic(states.expression_token)
              }
            }.as_str(),
          );
          //Reset expression token
          states.expression_token = String::new();
          //Push quote to expression token
          states.expression_token.push(c);
        }
        //Invert escape block value
        states.escape_block = !states.escape_block;
        continue;
      }
      //If backslash, enable backslash and push character
      //NOTE: it's very important this statement is after every other
      if c == '\\' {
        states.backslash = true;
        states.expression_token.push(c);
        continue;
      } else {
        states.backslash = false; //No more in backslash state
      }
      //Otheriwse, If it's just a character, Push it to the current expression
      states.expression_token.push(c);
    } //@! End of character iterator
      //Push last expression token to text
    states.text.push_str(
      match conversion {
        ExpressionConversion::ToLatin => self.translator.to_latin(states.expression_token),
        ExpressionConversion::ToCyrillic => self.translator.to_cyrillic(states.expression_token),
      }.as_str(),
    );
    //If there are still active states, return error 'missing token'
    if states.backslash || states.in_expression || states.escape_block || states.previous_state.is_some() {
      //Check if expression has been completely closed
      return Err(ExpressionParserError::MissingToken);
    }
    Ok(states.text)
  }

  /// ### escape_cyrillic
  ///
  /// Apply different escapes to escape cyrillic texts
  fn escape_cyrillic(&self, cyrillic_text: String) -> String {
    self.escape_colors(cyrillic_text)
  }

  /// ### escape_colors
  ///
  /// Since colors sequences have latin characters, the translator will translate these ascii characters too
  /// this functions has been implemented to redefine color sequences
  fn escape_colors(&self, cyrillic_text: String) -> String {
    let mut res: String = cyrillic_text.clone();
    for regex_match in self.escape_colors_regex.captures_iter(cyrillic_text.clone().as_str()) {
      let mtch: String = String::from(&regex_match[0]);
      let replace_with: String = self.text_to_latin(mtch.clone());
      res = res.replace(mtch.as_str(), replace_with.as_str());
    }
    res
  }
}

impl ExpressionParserStates {
  fn new(previous_state: Option<ExpressionParserStates>) -> ExpressionParserStates {
    ExpressionParserStates {
      text: String::new(),
      expression_token: String::new(),
      escape_block: false,
      backslash: false,
      in_expression: false,
      previous_state: match previous_state {
        None => None,
        Some(prev_state) => Some(Box::new(prev_state)),
      },
    }
  }

  fn clone(strref: &ExpressionParserStates) -> ExpressionParserStates {
    ExpressionParserStates {
      text: strref.text.clone(),                         //Text is restored
      expression_token: strref.expression_token.clone(), //Expression token is restored
      escape_block: strref.escape_block,
      backslash: strref.backslash,
      in_expression: strref.in_expression,
      previous_state: match &strref.previous_state {
        //Recursive clone
        None => None,
        Some(state_box) => Some(Box::new(ExpressionParserStates::clone(state_box.as_ref()))),
      },
    }
  }

  fn restore_previous_state(&mut self) -> ExpressionParserStates {
    match &self.previous_state {
      None => panic!("ParserState has no previous state"),
      Some(prev_state) => ExpressionParserStates::clone(prev_state.as_ref()),
    }
  }
}

//@! Tests

#[cfg(test)]
mod tests {

  use super::*;
  use crate::translator::{new_translator, Language};

  #[test]
  fn to_cyrillic_simple() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    let input: String = String::from("Привет Мир!");
    assert_eq!(iop.text_to_latin(input), String::from("Privyet Mir!"));
  }

  #[test]
  fn to_cyrillic_expressions() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    //Simple command
    let input: String = String::from("экхо фообар");
    assert_eq!(
      iop.expression_to_latin(input).unwrap(),
      String::from("echo foobar")
    );
    let input: String = String::from("echo foobar");
    assert_eq!(
      iop.expression_to_latin(input).unwrap(),
      String::from("echo foobar")
    );
    //With escape
    let input: String = String::from("экхо \"привет\"");
    assert_eq!(
      iop.expression_to_latin(input).unwrap(),
      String::from("echo \"привет\"")
    );
    //With escape + backslash
    let input: String = String::from("экхо \\\"привет\\\"");
    assert_eq!(
      iop.expression_to_latin(input).unwrap(),
      String::from("echo \\\"privyet\\\"")
    );
    //With expressions
    let input: String = String::from("экхо ₽(хостнамэ)");
    assert_eq!(
      iop.expression_to_latin(input).unwrap(),
      String::from("echo $(hostname)")
    );
    //With expressions + escapes
    let input: String = String::from("экхо ₽(кат \"/tmp/РЭАДМЭ.ткст\")");
    assert_eq!(
      iop.expression_to_latin(input).unwrap(),
      String::from("echo $(cat \"/tmp/РЭАДМЭ.ткст\")")
    );
    //With expressions + escapes + backslash
    let input: String = String::from("экхо ₽(кат \"/tmp/Ивана_\\(дочка\\).ткст\")");
    assert_eq!(
      iop.expression_to_latin(input).unwrap(),
      String::from("echo $(cat \"/tmp/Ивана_\\(дочка\\).ткст\")")
    );
    //Nested expressions
    let input: String = String::from("экхо ₽(хостнамэ) ₽(экхо ₽(хомэ)/₽(вьхоами))");
    assert_eq!(
      iop.expression_to_latin(input).unwrap(),
      String::from("echo $(hostname) $(echo $(home)/$(whoami))")
    );
  }

  #[test]
  #[should_panic]
  fn to_cyrillic_missing_token_parenthesis() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    //Bad expression
    let input: String = String::from("экхо ₽(хостнамэ");
    assert!(iop.expression_to_latin(input).is_ok());
  }

  #[test]
  #[should_panic]
  fn to_cyrillic_missing_token_quotes() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    //Bad expression
    let input: String = String::from("экхо \"привет");
    assert!(iop.expression_to_latin(input).is_ok());
  }

  #[test]
  #[should_panic]
  fn to_cyrillic_missing_token_backslash() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    //Bad expression
    let input: String = String::from("экхо \"привет\\");
    assert!(iop.expression_to_latin(input).is_ok());
  }

  #[test]
  fn to_latin_simple() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    let input: String = String::from("Hello World!");
    assert_eq!(iop.text_to_cyrillic(input), String::from("Хэлло Уорлд!"));
  }

  #[test]
  fn to_latin_expressions() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    //Simple command
    let input: String = String::from("echo foobar");
    assert_eq!(
      iop.expression_to_cyrillic(input).unwrap(),
      String::from("эчо фообар")
    );
    //With escape
    let input: String = String::from("echo \"hello world\"");
    assert_eq!(
      iop.expression_to_cyrillic(input).unwrap(),
      String::from("эчо \"hello world\"")
    );
    //With escape + backslash
    let input: String = String::from("echo \\\"privyet\\\"");
    assert_eq!(
      iop.expression_to_cyrillic(input).unwrap(),
      String::from("эчо \\\"привет\\\"")
    );
    //With expressions
    let input: String = String::from("echo $(hostname)");
    assert_eq!(
      iop.expression_to_cyrillic(input).unwrap(),
      String::from("эчо $(хостнамэ)")
    );
    //With expressions + escapes
    let input: String = String::from("echo $(cat \"/tmp/README.txt\")");
    assert_eq!(
      iop.expression_to_cyrillic(input).unwrap(),
      String::from("эчо $(кат \"/tmp/README.txt\")")
    );
    //With expressions + escapes + backslash
    let input: String = String::from("echo $(cat \"/tmp/john\\(that_guy\\).txt\")");
    assert_eq!(
      iop.expression_to_cyrillic(input).unwrap(),
      String::from("эчо $(кат \"/tmp/john\\(that_guy\\).txt\")")
    );
    //Nested expressions
    let input: String = String::from("echo $(hostname) $(echo $(home)/$(whoami))");
    assert_eq!(
      iop.expression_to_cyrillic(input).unwrap(),
      String::from("эчо $(хостнамэ) $(эчо $(хомэ)/$(ухоами))")
    );
  }

  #[test]
  #[should_panic]
  fn to_latin_missing_token_parenthesis() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    //Bad expression
    let input: String = String::from("echo $(hostname");
    assert!(iop.expression_to_latin(input).is_ok());
  }

  #[test]
  #[should_panic]
  fn to_latin_missing_token_quotes() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    //Bad expression
    let input: String = String::from("echo \"hello");
    assert!(iop.expression_to_latin(input).is_ok());
  }

  #[test]
  #[should_panic]
  fn to_latin_missing_token_backslash() {
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    //Bad expression
    let input: String = String::from("echo \"hello\\");
    assert!(iop.expression_to_latin(input).is_ok());
  }

  #[test]
  fn test_escapes() {
    let latin_text: String = String::from("\x1b[31mRED\x1b[0m");
    //Instantiate IOProcessor
    let iop: IOProcessor = IOProcessor::new(Language::Russian, new_translator(Language::Russian));
    assert_eq!(iop.language, Language::Russian);
    assert_eq!(iop.text_to_cyrillic(latin_text), String::from("\x1b[31mРЭД\x1b[0m"));
  }
}
