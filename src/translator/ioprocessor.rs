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

use std::fmt;

#[derive(Copy, Clone, PartialEq, fmt::Debug)]
pub enum ParserError {
  MissingToken,
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

/* To reimplment in IO processor
//Iterate over string
    let mut states: ParserStates = ParserStates::new(None);
    for (i, c) in input.chars().enumerate() {
      if states.skip_counter > 0 {
        //Skip cycles
        states.skip_counter -= 1; //Decrement skip counter
        continue;
      }
      //If character is '(' an expression block starts (if backlsash is disabled)
      if c == '(' && !states.backslash {
        //If previous character is ₽, then change it into $
        if output.chars().last().unwrap() == '₽' {
          output.pop();
          output.push('$');
        }
        //Set escape to false
        states.escape_block = false;
        //Create new state
        states = ParserStates::new(Some(states));
        states.in_expression = true;
        output.push(c);
        continue;
      }
      //If backslash, enable backslash and push character
      if c == '\\' {
        states.backslash = true;
        output.push(c);
        continue;
      } else {
        states.backslash = false; //No more in backslash state
      }
      //If character is ')' an expression ends (if backslash is disabled)
      if c == ')' && !states.backslash {
        states.in_expression = false;
        //Restore previous state
        states = match states.previous_state {
          Some(_) => states.restore_previous_state(),
          None => return Err(ParserError::MissingToken),
        };
        output.push(c);
        continue;
      }
      //Check if escape (and previous character is not backslash or we're inside an expression)
      if c == '"' && (!states.backslash || states.in_expression) {
        states.escape_block = !states.escape_block;
        output.push(c);
        continue;
      }
      //If in escaped block, just push character
      if states.escape_block {
        output.push(c);
        continue;
      }

      ...
      if states.backslash || states.in_expression || states.previous_state.is_some() {
      //Check if expression has been completely closed
      return Err(ParserError::MissingToken);
    }
*/


  /*
  #[test]
  fn test_russian_to_latin_syntax_error() {
    let translator: Box<dyn Translator> = new_translator(Language::Russian);
    //Missing expression token
    let input: String = String::from("лс ₽(пвьд");
    let res: Result<String, ParserError> = translator.to_latin(input.clone());
    println!("Missing token result: {:?}", res);
    assert!(res.is_err()); //it must be error
    assert_eq!(res.err().unwrap(), ParserError::MissingToken); //Must be missing token
                                                               //Closed expression, but never started one
    let input: String = String::from("лс пвьд)");
    let res: Result<String, ParserError> = translator.to_latin(input.clone());
    println!("Missing token result: {:?}", res);
    assert!(res.is_err()); //it must be error
    assert_eq!(res.err().unwrap(), ParserError::MissingToken); //Must be missing token
  }
  
  //Try escapes
    let input: String = String::from("кат \"Привет.ткст\"");
    let output = translator.to_latin(input.clone());
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "cat \"Привет.ткст\"");
    //Escapes with expressions
    let input: String = String::from("экхо \"хостнамэ: ₽(хостнамэ)\""); //Stuff inside quotes, won't be translated, but content inside expression () will
    let output = translator.to_latin(input.clone());
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "echo \"хостнамэ: $(hostname)\"");
    let input: String = String::from("экхо \"Намэ: ₽(экхо \\\"кристиан\\\")\""); //Double escape block
    let output = translator.to_latin(input.clone());
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "echo \"Намэ: $(echo \\\"кристиан\\\")\"");
  */