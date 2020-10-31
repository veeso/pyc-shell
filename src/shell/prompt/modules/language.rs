//! # Language
//!
//! `Language` is the module which resolve the lang prompt token

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

use crate::translator::lang::Language;

use super::colors::PromptColor;

pub(crate) const PROMPT_LANG: &str = "${LANG}";

pub fn language_to_str(language: Language) -> String {
    let mut lang_str: String = language.to_string();
    if lang_str.len() < 3 {
        lang_str.push_str("   ");
    }
    match language {
        Language::Belarusian => String::from(format!(
            "{}{}{}{}{}{}{}",
            PromptColor::Red.to_string(),
            lang_str.chars().nth(0).unwrap_or(' '),
            PromptColor::Green.to_string(),
            lang_str.chars().nth(1).unwrap_or(' '),
            PromptColor::White.to_string(),
            lang_str.chars().nth(2).unwrap_or(' '),
            PromptColor::Reset.to_string()
        )),
        Language::Bulgarian => String::from(format!(
            "{}{}{}{}{}{}{}",
            PromptColor::White.to_string(),
            lang_str.chars().nth(0).unwrap_or(' '),
            PromptColor::Green.to_string(),
            lang_str.chars().nth(1).unwrap_or(' '),
            PromptColor::Red.to_string(),
            lang_str.chars().nth(2).unwrap_or(' '),
            PromptColor::Reset.to_string()
        )),
        Language::Russian => String::from(format!(
            "{}{}{}{}{}{}{}",
            PromptColor::White.to_string(),
            lang_str.chars().nth(0).unwrap_or(' '),
            PromptColor::Blue.to_string(),
            lang_str.chars().nth(1).unwrap_or(' '),
            PromptColor::Red.to_string(),
            lang_str.chars().nth(2).unwrap_or(' '),
            PromptColor::Reset.to_string()
        )),
        Language::Serbian => String::from(format!(
            "{}{}{}{}{}{}{}",
            PromptColor::Red.to_string(),
            lang_str.chars().nth(0).unwrap_or(' '),
            PromptColor::Blue.to_string(),
            lang_str.chars().nth(1).unwrap_or(' '),
            PromptColor::White.to_string(),
            lang_str.chars().nth(2).unwrap_or(' '),
            PromptColor::Reset.to_string()
        )),
        Language::Ukrainian => String::from(format!(
            "{}{}{}{}{}{}{}",
            PromptColor::Cyan.to_string(),
            lang_str.chars().nth(0).unwrap_or(' '),
            PromptColor::Yellow.to_string(),
            lang_str.chars().nth(1).unwrap_or(' '),
            PromptColor::Cyan.to_string(),
            lang_str.chars().nth(2).unwrap_or(' '),
            PromptColor::Reset.to_string()
        ))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_prompt_lang_flag() {
        // Belarusian
        let expected_str = String::from("\x1b[31mб\x1b[32mе\x1b[37mл\x1b[0m");
        println!("{}", language_to_str(Language::Belarusian));
        assert_eq!(language_to_str(Language::Belarusian), expected_str);
        // Bulgarian
        let expected_str = String::from("\x1b[37mб\x1b[32mл\x1b[31mг\x1b[0m");
        println!("{}", language_to_str(Language::Bulgarian));
        assert_eq!(language_to_str(Language::Bulgarian), expected_str);
        // Russian
        let expected_str = String::from("\x1b[37mр\x1b[34mу\x1b[31mс\x1b[0m");
        println!("{}", language_to_str(Language::Russian));
        assert_eq!(language_to_str(Language::Russian), expected_str);
        // Serbian
        let expected_str = String::from("\x1b[31mр\x1b[34mу\x1b[37mс\x1b[0m");
        println!("{}", language_to_str(Language::Serbian));
        assert_eq!(language_to_str(Language::Serbian), expected_str);
        // Ukrainian
        let expected_str = String::from("\x1b[36mу\x1b[33mк\x1b[36mр\x1b[0m");
        println!("{}", language_to_str(Language::Ukrainian));
        assert_eq!(language_to_str(Language::Ukrainian), expected_str);
    }
}
