//! # Colors
//!
//! `Colors` is the module which provides terminal colors

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

//Colors
const KRED: &str = "\x1b[31m";
const KGRN: &str = "\x1b[32m";
const KYEL: &str = "\x1b[33m";
const KBLU: &str = "\x1b[34m";
const KCYN: &str = "\x1b[36m";
const KMAG: &str = "\x1b[35m";
const KGRY: &str = "\x1b[90m";
const KBLK: &str = "\x1b[30m";
const KWHT: &str = "\x1b[37m";
const KRST: &str = "\x1b[0m";

use super::super::{
    PROMPT_KBLK, PROMPT_KBLU, PROMPT_KCYN, PROMPT_KGRN, PROMPT_KGRY, PROMPT_KMAG, PROMPT_KRED,
    PROMPT_KRST, PROMPT_KWHT, PROMPT_KYEL,
};

#[derive(Copy, Clone, PartialEq, std::fmt::Debug)]
pub enum PromptColor {
    Red,
    Green,
    Yellow,
    Blue,
    Cyan,
    Magenta,
    Black,
    Gray,
    White,
    Reset,
}

impl ToString for PromptColor {
    fn to_string(&self) -> String {
        match self {
            PromptColor::Red => String::from(KRED),
            PromptColor::Green => String::from(KGRN),
            PromptColor::Yellow => String::from(KYEL),
            PromptColor::Blue => String::from(KBLU),
            PromptColor::Cyan => String::from(KCYN),
            PromptColor::Magenta => String::from(KMAG),
            PromptColor::Black => String::from(KBLK),
            PromptColor::Gray => String::from(KGRY),
            PromptColor::White => String::from(KWHT),
            PromptColor::Reset => String::from(KRST),
        }
    }
}

impl PromptColor {
    fn from_key(key: &str) -> PromptColor {
        match key {
            PROMPT_KRED => PromptColor::Red,
            PROMPT_KYEL => PromptColor::Yellow,
            PROMPT_KGRN => PromptColor::Green,
            PROMPT_KBLU => PromptColor::Blue,
            PROMPT_KCYN => PromptColor::Cyan,
            PROMPT_KGRY => PromptColor::Gray,
            PROMPT_KMAG => PromptColor::Magenta,
            PROMPT_KBLK => PromptColor::Black,
            PROMPT_KWHT => PromptColor::White,
            PROMPT_KRST => PromptColor::Reset,
            _ => PromptColor::Reset,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_color_from_str() {
        assert_eq!(PromptColor::from_key(PROMPT_KRED), PromptColor::Red);
        assert_eq!(PromptColor::from_key(PROMPT_KYEL), PromptColor::Yellow);
        assert_eq!(PromptColor::from_key(PROMPT_KGRN), PromptColor::Green);
        assert_eq!(PromptColor::from_key(PROMPT_KBLU), PromptColor::Blue);
        assert_eq!(PromptColor::from_key(PROMPT_KCYN), PromptColor::Cyan);
        assert_eq!(PromptColor::from_key(PROMPT_KMAG), PromptColor::Magenta);
        assert_eq!(PromptColor::from_key(PROMPT_KGRY), PromptColor::Gray);
        assert_eq!(PromptColor::from_key(PROMPT_KWHT), PromptColor::White);
        assert_eq!(PromptColor::from_key(PROMPT_KBLK), PromptColor::Black);
        assert_eq!(PromptColor::from_key(PROMPT_KRST), PromptColor::Reset);
        assert_eq!(PromptColor::from_key("UnknownColor"), PromptColor::Reset);
    }

    #[test]
    fn test_color_print() {
        assert_eq!(PromptColor::Red.to_string(), KRED);
        println!("{}Red", PromptColor::Red.to_string());
        assert_eq!(PromptColor::Yellow.to_string(), KYEL);
        println!("{}Yellow", PromptColor::Yellow.to_string());
        assert_eq!(PromptColor::Green.to_string(), KGRN);
        println!("{}Green", PromptColor::Green.to_string());
        assert_eq!(PromptColor::Blue.to_string(), KBLU);
        println!("{}Blue", PromptColor::Blue.to_string());
        assert_eq!(PromptColor::Cyan.to_string(), KCYN);
        println!("{}Cyan", PromptColor::Cyan.to_string());
        assert_eq!(PromptColor::Magenta.to_string(), KMAG);
        println!("{}Magenta", PromptColor::Magenta.to_string());
        assert_eq!(PromptColor::Gray.to_string(), KGRY);
        println!("{}Gray", PromptColor::Gray.to_string());
        assert_eq!(PromptColor::White.to_string(), KWHT);
        println!("{}White", PromptColor::White.to_string());
        assert_eq!(PromptColor::Black.to_string(), KBLK);
        println!("{}Black", PromptColor::Black.to_string());
        assert_eq!(PromptColor::Reset.to_string(), KRST);
        println!("{}Reset", PromptColor::Reset.to_string());
    }
}
