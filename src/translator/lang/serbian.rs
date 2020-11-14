//! ### Serbian
//!
//! `serbian` language implementation of Translator trait

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
use super::Serbian;

impl Translator for Serbian {
    /// ### Serbian translator

    /// Converts a string which contains serbian cyrillic characters into a latin string.
    /// Characters between '"' (quotes) are escaped, expressions inside escaped blocks are translitarated anyway
    /// Transliteration according to GOST 7.79-2000
    fn to_latin(&self, input: &String) -> String {
        let mut output = String::new();
        let mut skip_counter: usize = 0;
        for (i, c) in input.chars().enumerate() {
            if skip_counter > 0 {
                //Skip cycles
                skip_counter -= 1; //Decrement skip counter
                continue;
            }
            //Push transliterated character
            let unchanged_str: String;
            output.push_str(match c {
                'А' => "A",
                'а' => "a",
                'Б' => "B",
                'б' => "b",
                'В' => {
                    //If following character is 'В', then is always W
                    match input.chars().nth(i + 1) {
                        Some(ch) => {
                            match ch {
                                'в' | 'В' => {
                                    skip_counter += 1; //Skip character
                                    "W"
                                }
                                _ => "V",
                            }
                        }
                        None => "V",
                    }
                }
                'в' =>
                //If following character is 'В', then is always W
                {
                    match input.chars().nth(i + 1) {
                        Some(ch) => {
                            match ch {
                                'в' | 'В' => {
                                    skip_counter += 1; //Skip character
                                    "w"
                                }
                                _ => "v",
                            }
                        }
                        None => "v",
                    }
                }
                'Г' => "G",
                'г' => "g",
                'Д' => "D",
                'д' => "d",
                'Ђ' => "DJ",
                'ђ' => "dj",
                'Е' => "E",
                'е' => "e",
                'Ж' | 'Ј' => "J",
                'ж' | 'ј' => "j",
                'З' => "Z",
                'з' => "z",
                'И' =>
                //If following character is 'И', then is always Y
                {
                    match input.chars().nth(i + 1) {
                        Some(ch) => {
                            match ch {
                                'и' | 'И' => {
                                    skip_counter += 1; //Skip character
                                    "Y"
                                }
                                _ => "I",
                            }
                        }
                        None => "I",
                    }
                }
                'и' =>
                //If following character is 'И', then is always Y
                {
                    match input.chars().nth(i + 1) {
                        Some(ch) => {
                            match ch {
                                'и' | 'И' => {
                                    skip_counter += 1; //Skip character
                                    "y"
                                }
                                _ => "i",
                            }
                        }
                        None => "i",
                    }
                }
                'Ћ' => "C",
                'ћ' => "c",
                'К' => {
                    match input.chars().nth(i + 1) {
                        //If following character is 'С', then is always X
                        Some(ch) => {
                            match ch {
                                'с' | 'С' => {
                                    skip_counter += 1; //Skip character
                                    "X"
                                }
                                'и' | 'И' => {
                                    // If following characters are 'ИУ', then is always Q
                                    match input.chars().nth(i + 2) {
                                        Some(ch) => {
                                            match ch {
                                                'у' | 'У' => {
                                                    skip_counter += 2; // Skip 2
                                                    "Q"
                                                }
                                                _ => "K",
                                            }
                                        }
                                        None => "K",
                                    }
                                }
                                _ => "K",
                            }
                        }
                        None => "K",
                    }
                }
                'к' => {
                    match input.chars().nth(i + 1) {
                        //If following character is 'С', then is always X
                        Some(ch) => {
                            match ch {
                                'с' | 'С' => {
                                    skip_counter += 1; //Skip character
                                    "x"
                                }
                                'и' | 'И' => {
                                    // If following characters are 'ИУ', then is always Q
                                    match input.chars().nth(i + 2) {
                                        Some(ch) => {
                                            match ch {
                                                'у' | 'У' => {
                                                    skip_counter += 2; // Skip 2
                                                    "q"
                                                }
                                                _ => "k",
                                            }
                                        }
                                        None => "k",
                                    }
                                }
                                _ => "k",
                            }
                        }
                        None => "k",
                    }
                }
                'Л' => "L",
                'л' => "l",
                'Љ' => "LJ",
                'љ' => "lj",
                'М' => "M",
                'м' => "m",
                'Н' => "N",
                'н' => "n",
                'Њ' => "NJ",
                'њ' => "nj",
                'О' => "O",
                'о' => "o",
                'П' => "P",
                'п' => "p",
                'Р' => "R",
                'р' => "r",
                'С' => "S",
                'с' => "s",
                'Т' => "T",
                'т' => "t",
                'Ч' => "CH",
                'ч' => "ch",
                'У' => "U",
                'у' => "u",
                'Ф' => "F",
                'ф' => "f",
                'Х' => "H",
                'х' => "h",
                'Ц' => "TS",
                'ц' => "ts",
                'Џ' => "DZ",
                'џ' => "dz",
                'Ш' => "SH",
                'ш' => "sh",
                _ => {
                    unchanged_str = c.to_string();
                    unchanged_str.as_str()
                }
            });
        }
        output
    }

    /// Converts a string which contains latin characters into a serbian cyrillic string.
    /// Characters between quotes are escapes
    fn to_cyrillic(&self, input: &String) -> String {
        let mut output: String = String::new();
        let mut skip_cycles: usize = 0;
        for (i, c) in input.chars().enumerate() {
            if skip_cycles > 0 {
                skip_cycles -= 1;
                continue;
            }
            let unchanged_str: String;
            output.push_str(match c {
                'A' => "А",
                'a' => "а",
                'B' => "Б",
                'b' => "б",
                'C' => match input.chars().nth(i + 1) {
                    Some(ch) => match ch {
                        'h' | 'H' => {
                            skip_cycles += 1;
                            "Ч"
                        }
                        _ => "К",
                    },
                    None => "К",
                },
                'c' => match input.chars().nth(i + 1) {
                    Some(ch) => match ch {
                        'h' | 'H' => {
                            skip_cycles += 1;
                            "ч"
                        }
                        _ => "к",
                    },
                    None => "к",
                },
                'D' => match input.chars().nth(i + 1) {
                    // If 'J' follows => Ђ; if 'Z' follows => Џ
                    Some(ch) => match ch {
                        'J' | 'j' => {
                            skip_cycles += 1;
                            "Ђ"
                        }
                        'Z' | 'z' => {
                            skip_cycles += 1;
                            "Џ"
                        }
                        _ => "Д",
                    },
                    None => "Д",
                },
                'd' => match input.chars().nth(i + 1) {
                    // If 'J' follows => Ђ; if 'Z' follows => Џ
                    Some(ch) => match ch {
                        'J' | 'j' => {
                            skip_cycles += 1;
                            "ђ"
                        }
                        'Z' | 'z' => {
                            skip_cycles += 1;
                            "џ"
                        }
                        _ => "д",
                    },
                    None => "д",
                },
                'E' => "Е",
                'e' => "е",
                'F' => "Ф",
                'f' => "ф",
                'G' => match input.chars().nth(i + 1) {
                    Some(ch) => match ch {
                        'y' | 'Y' | 'e' | 'E' | 'i' | 'I' => "ДЖ",
                        _ => "Г",
                    },
                    None => "Г",
                },
                'g' => match input.chars().nth(i + 1) {
                    Some(ch) => match ch {
                        'y' | 'Y' | 'e' | 'E' | 'i' | 'I' => "дж",
                        _ => "г",
                    },
                    None => "г",
                },
                'H' => "Х",
                'h' => "х",
                'I' => "И",
                'i' => "и",
                'J' => "Ј",
                'j' => "ј",
                'K' => "К",
                'k' => "к",
                'L' => match input.chars().nth(i + 1) {
                    // If 'J' follows => Љ
                    Some(ch) => match ch {
                        'J' | 'j' => {
                            skip_cycles += 1;
                            "Љ"
                        }
                        _ => "Л",
                    },
                    None => "Л",
                },
                'l' => match input.chars().nth(i + 1) {
                    // If 'J' follows => Љ
                    Some(ch) => match ch {
                        'J' | 'j' => {
                            skip_cycles += 1;
                            "љ"
                        }
                        _ => "л",
                    },
                    None => "л",
                },
                'M' => "М",
                'm' => "м",
                'N' => match input.chars().nth(i + 1) {
                    // If 'J' follows => Њ
                    Some(ch) => match ch {
                        'J' | 'j' => {
                            skip_cycles += 1;
                            "Њ"
                        }
                        _ => "Н",
                    },
                    None => "Н",
                },
                'n' => match input.chars().nth(i + 1) {
                    // If 'J' follows => Њ
                    Some(ch) => match ch {
                        'J' | 'j' => {
                            skip_cycles += 1;
                            "њ"
                        }
                        _ => "н",
                    },
                    None => "н",
                },
                'O' => "О",
                'o' => "о",
                'P' => "П",
                'p' => "п",
                'Q' => "КУ",
                'q' => "ку",
                'R' => "Р",
                'r' => "р",
                'S' => match input.chars().nth(i + 1) {
                    Some(ch) => match ch {
                        'h' | 'H' => {
                            skip_cycles += 1;
                            "Ш"
                        }
                        _ => "С",
                    },
                    None => "С",
                },
                's' => match input.chars().nth(i + 1) {
                    Some(ch) => match ch {
                        'h' | 'H' => {
                            skip_cycles += 1;
                            "ш"
                        }
                        _ => "с",
                    },
                    None => "с",
                },
                'T' => match input.chars().nth(i + 1) {
                    Some(ch) => match ch {
                        's' | 'S' => {
                            skip_cycles += 1;
                            "Ц"
                        }
                        _ => "Т",
                    },
                    None => "Т",
                },
                't' => match input.chars().nth(i + 1) {
                    Some(ch) => match ch {
                        's' | 'T' => {
                            skip_cycles += 1;
                            "ц"
                        }
                        _ => "т",
                    },
                    None => "т",
                },
                'U' => "У",
                'u' => "у",
                'V' => "В",
                'v' => "в",
                'W' => "В",
                'w' => "в",
                'X' => "КС",
                'x' => "кс",
                'Y' => "И",
                'y' => "и",
                'Z' => "З",
                'z' => "з",
                _ => {
                    unchanged_str = c.to_string();
                    unchanged_str.as_str()
                }
            });
        }
        output
    }
}

//@! Tests

#[cfg(test)]
mod tests {

    use super::*;
    use crate::translator::{new_translator, Language};

    #[test]
    fn test_translator_lang_serbian_to_latin() {
        // Serbian translator
        let translator: Box<dyn Translator> = new_translator(Language::Serbian);
        // All characters
        assert_eq!(translator.to_latin(&String::from("АБВВВГДЂЕЖЈЗИИИЋККСКИУЛЉМНЊОПРСТЧУФХЦЏШ")), String::from("ABWVGDDJEJJZYICKXQLLJMNNJOPRSTCHUFHTSDZSH"));
        assert_eq!(translator.to_latin(&String::from("абвввгдђежјзииићккскиулљмнњопрстчуфхцџш")), String::from("abwvgddjejjzyickxqlljmnnjoprstchufhtsdzsh"));
        // Simple commands (lower)
        assert_eq!(translator.to_latin(&String::from("лс поотис/")), String::from("ls pootis/"));
        assert_eq!(translator.to_latin(&String::from("ексећ зш")), String::from("exec zsh"));
        assert_eq!(translator.to_latin(&String::from("ћд тесц/")), String::from("cd tests/"));
        // Simple commands (upper)
        assert_eq!(translator.to_latin(&String::from("ЛС ПООТИС/")), String::from("LS POOTIS/"));
        assert_eq!(translator.to_latin(&String::from("ЕКСЕЋ ЗШ")), String::from("EXEC ZSH"));
        assert_eq!(translator.to_latin(&String::from("ЋД ТЕСЦ/")), String::from("CD TESTS/"));
        // With next char special at the end of word (lower)
        assert_eq!(translator.to_latin(&String::from("лс в")), String::from("ls v"));
        assert_eq!(translator.to_latin(&String::from("лс сими")), String::from("ls simi"));
        assert_eq!(translator.to_latin(&String::from("лс кек")), String::from("ls kek"));
        assert_eq!(translator.to_latin(&String::from("лс ки")), String::from("ls ki"));
        // With next char special at the end of word (upper)
        assert_eq!(translator.to_latin(&String::from("ЛС В")), String::from("LS V"));
        assert_eq!(translator.to_latin(&String::from("ЛС СИМИ")), String::from("LS SIMI"));
        assert_eq!(translator.to_latin(&String::from("ЛС КЕК")), String::from("LS KEK"));
        assert_eq!(translator.to_latin(&String::from("ЛС КИ")), String::from("LS KI"));
    }

    #[test]
    fn test_translator_lang_serbian_to_cyrillic() {
        // Serbian translator
        let translator: Box<dyn Translator> = new_translator(Language::Serbian);
        // All characters
        assert_eq!(translator.to_cyrillic(&String::from("ABCCHDDJDZEFGGEHIJKLLJMNNJOPQRSSHTTSUVWXYZ")), String::from("АБКЧДЂЏЕФГДЖЕХИЈКЛЉМНЊОПКУРСШТЦУВВКСИЗ"));
        assert_eq!(translator.to_cyrillic(&String::from("abcchddjdzefggehijklljmnnjopqrsshttsuvwxyz")), String::from("абкчдђџефгджехијклљмнњопкурсштцуввксиз"));
        // Test particular case (sh)
        assert_eq!(translator.to_cyrillic(&String::from("shell sis")), "шелл сис");
        assert_eq!(translator.to_cyrillic(&String::from("SHELL SIS")), "ШЕЛЛ СИС");
        // Test particular case (ts)
        assert_eq!(translator.to_cyrillic(&String::from("tsunami")), "цунами");
        assert_eq!(translator.to_cyrillic(&String::from("TSUNAMI")), "ЦУНАМИ");
        // Test particular case (g)
        assert_eq!(translator.to_cyrillic(&String::from("gin and games")), "джин анд гамес");
        assert_eq!(translator.to_cyrillic(&String::from("GIN AND GAMES")), "ДЖИН АНД ГАМЕС");
        // Test particular case (ch)
        assert_eq!(translator.to_cyrillic(&String::from("channel")), "чаннел");
        assert_eq!(translator.to_cyrillic(&String::from("CHANNEL")), "ЧАННЕЛ");
        // Test particular case (ts)
        assert_eq!(translator.to_cyrillic(&String::from("tsunami")), "цунами");
        assert_eq!(translator.to_cyrillic(&String::from("TSUNAMI")), "ЦУНАМИ");
        // Test particular case (last character is C)
        assert_eq!(translator.to_cyrillic(&String::from("cac")), "как");
        assert_eq!(translator.to_cyrillic(&String::from("CAC")), "КАК");
        // Test particular case (last char is G)
        assert_eq!(translator.to_cyrillic(&String::from("gag")), "гаг");
        assert_eq!(translator.to_cyrillic(&String::from("GAG")), "ГАГ");
        // Test particular case (LJ)
        assert_eq!(translator.to_cyrillic(&String::from("ljubljana l")), "љубљана л");
        assert_eq!(translator.to_cyrillic(&String::from("LJUBLJANA L")), "ЉУБЉАНА Л");
        // Test particular case (NJ)
        assert_eq!(translator.to_cyrillic(&String::from("new jersey is abbreviated with nj non")), "нев јерсеи ис аббревиатед витх њ нон");
        assert_eq!(translator.to_cyrillic(&String::from("NEW JERSEY IS ABBREVIATED WITH NJ NON")), "НЕВ ЈЕРСЕИ ИС АББРЕВИАТЕД ВИТХ Њ НОН");
        // Test particular case (TS)
        assert_eq!(translator.to_cyrillic(&String::from("typescript extension is .ts tot")), "типескрипт екстенсион ис .ц тот");
        assert_eq!(translator.to_cyrillic(&String::from("TYPESCRIPT EXTENSION IS .TS TOT")), "ТИПЕСКРИПТ ЕКСТЕНСИОН ИС .Ц ТОТ");
    }
}
