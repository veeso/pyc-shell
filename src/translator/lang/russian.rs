//! ### Russian
//!
//! Russian language implementation of Translator trait

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

use super::Russian;
use super::super::Translator;

impl Translator for Russian {
  /// ### Russian translator

  /// Converts a string which contains russian cyrillic characters into a latin string.
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
          //If following character is 'ь', then is always W
          match input.chars().nth(i + 1) {
            Some(ch) => {
              match ch {
                'ь' | 'Ь' => {
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
        //If following character is 'ь', then is always W
        {
          match input.chars().nth(i + 1) {
            Some(ch) => {
              match ch {
                'ь' | 'Ь' => {
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
        'Е' => "YE",
        'е' => "ye",
        'Э' => "E",
        'э' => "e",
        'Ё' => "YO",
        'ё' => "yo",
        'Ж' => "J",
        'ж' => "j",
        'З' => "Z",
        'з' => "z",
        'И' => "I",
        'и' => "i",
        'Й' => "J",
        'й' => "j",
        'К' => {
          //K is very complex, sometimes it is C, sometimes is K or even Q or X
          //If following letter is in (E, I, Y), then is K
          //If following character is 'Ъ', then is always K
          //If following character is 'ь', then is always C
          //If following character is 'y', then is always Q
          //If follwing character is 'с', then is always X
          match input.chars().nth(i + 1) {
            Some(ch) => {
              //Check following character
              match ch {
                'Е' | 'Э' | 'И' | 'Й' | 'Ы' | 'е' | 'э' | 'и' | 'й' | 'ы' => "K",
                ' ' => {
                  //Check previous character
                  match i {
                    0 => "K",
                    _ => match input.chars().nth(i - 1) {
                      Some(ch) => match ch {
                        'К' | 'А' | 'И' | 'О' | 'к' | 'а' | 'и' | 'о' | ' ' => "K",
                        _ => "C",
                      },
                      None => "K",
                    },
                  }
                }
                'Ю' | 'ю' => {
                  skip_counter += 1;
                  "Q"
                }
                'с' | 'С' => {
                  skip_counter += 1;
                  "X"
                }
                'ъ' | 'Ъ' => {
                  skip_counter += 1; //Skip next character
                  "K"
                }
                'ь' | 'Ь' => {
                  skip_counter += 1; //Skip character
                  "C"
                }
                _ => "C",
              }
            }
            None => {
              //Check previous character
              match i {
                0 => "K",
                _ => match input.chars().nth(i - 1) {
                  //Check previous character
                  Some(ch) => match ch {
                    'К' | 'А' | 'И' | 'О' | 'У' | 'к' | 'а' | 'и' | 'о' | 'у' | ' ' => {
                      "K"
                    }
                    _ => "C",
                  },
                  None => "K",
                },
              }
            }
          }
        }
        'к' => {
          //K is very complex, sometimes it is C and sometimes is K
          //If following letter is in (E, I, Y), then is K
          match input.chars().nth(i + 1) {
            Some(ch) => {
              //Check following character
              match ch {
                'Е' | 'Э' | 'И' | 'Й' | 'Ы' | 'е' | 'э' | 'и' | 'й' | 'ы' => "k",
                ' ' => {
                  match i {
                    0 => "k",
                    _ => match input.chars().nth(i - 1) {
                      //Check previous character
                      Some(ch) => match ch {
                        'К' | 'А' | 'И' | 'О' | 'к' | 'а' | 'и' | 'о' | ' ' => "k",
                        _ => "c",
                      },
                      None => "k",
                    },
                  }
                }
                'Ю' | 'ю' => {
                  skip_counter += 1;
                  "q"
                }
                'с' | 'С' => {
                  skip_counter += 1;
                  "x"
                }
                'ъ' | 'Ъ' => {
                  skip_counter += 1; //Skip next character
                  "k"
                }
                'ь' | 'Ь' => {
                  skip_counter += 1; //Skip character
                  "c"
                }
                _ => "c",
              }
            }
            None => {
              //Check previous character
              match i {
                0 => "k",
                _ => match input.chars().nth(i - 1) {
                  Some(ch) => match ch {
                    'К' | 'А' | 'И' | 'О' | 'У' | 'к' | 'а' | 'и' | 'о' | 'у' | ' ' => {
                      "k"
                    }
                    _ => "c",
                  },
                  None => "k",
                },
              }
            }
          }
        }
        'Л' => "L",
        'л' => "l",
        'М' => "M",
        'м' => "m",
        'Н' => "N",
        'н' => "n",
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
        'У' => "U",
        'у' => "u",
        'Ф' => "F",
        'ф' => "f",
        'Х' => "H",
        'х' => "h",
        'Ч' => "CH",
        'ч' => "ch",
        'Ш' => "SH",
        'ш' => "sh",
        'Щ' => "SHH",
        'щ' => "shh",
        'Ъ' => "'",
        'ъ' => "'",
        'Ы' => "Y",
        'ы' => "y",
        'Ь' => "`",
        'ь' => "`",
        'Ю' => "YU",
        'ю' => "yu",
        'Я' => "YA",
        'я' => "ya",
        'Ц' => "Z",
        'ц' => "z",
        '№' => "#",
        '₽' => "$",
        _ => {
          unchanged_str = c.to_string();
          unchanged_str.as_str()
        }
      });
    }
    output
  }

  /// Converts a string which contains latin characters into a russian cyrillic string.
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
        'D' => "Д",
        'd' => "д",
        'E' => "Э",
        'e' => "э",
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
        'I' => match input.chars().nth(i + 1) {
          Some(ch) => match ch {
            'u' | 'U' => {
              skip_cycles += 1;
              "Ю"
            }
            'a' | 'A' => {
              skip_cycles += 1;
              "Я"
            }
            'o' | 'O' => {
              skip_cycles += 1;
              "Ё"
            }
            _ => "И",
          },
          None => "И",
        },
        'i' => match input.chars().nth(i + 1) {
          Some(ch) => match ch {
            'u' | 'U' => {
              skip_cycles += 1;
              "ю"
            }
            'a' | 'A' => {
              skip_cycles += 1;
              "я"
            }
            'o' | 'O' => {
              skip_cycles += 1;
              "ё"
            }
            _ => "и",
          },
          None => "и",
        },
        'J' => "Ж",
        'j' => "ж",
        'K' => "К",
        'k' => "к",
        'L' => "Л",
        'l' => "л",
        'M' => "М",
        'm' => "м",
        'N' => "Н",
        'n' => "н",
        'O' => "О",
        'o' => "о",
        'P' => "П",
        'p' => "п",
        'Q' => "КЮ",
        'q' => "кю",
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
        'W' => "У",
        'w' => "у",
        'X' => "КС",
        'x' => "кс",
        'Y' => match input.chars().nth(i + 1) {
          Some(ch) => match ch {
            'e' | 'E' => {
              skip_cycles += 1;
              "Е"
            }
            _ => "Ы",
          },
          None => "Ы",
        },
        'y' => match input.chars().nth(i + 1) {
          Some(ch) => match ch {
            'e' | 'E' => {
              skip_cycles += 1;
              "е"
            }
            _ => "ы",
          },
          None => "ы",
        },
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
  fn test_translator_lang_russian_to_latin() {
    //Simple commands
    let translator: Box<dyn Translator> = new_translator(Language::Russian);
    //ls -l
    let input: String = String::from("лс -л");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ls -l");
    //Echo hello
    let input: String = String::from("экхо хэлло");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "echo hello");
    //K vs C
    let input: String = String::from("ифконфиг этх0 аддрэсс 192.168.1.30 нэтмаскъ 255.255.255.0"); //Use твёрдый знак to force k in netmask
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(
      output,
      "ifconfig eth0 address 192.168.1.30 netmask 255.255.255.0"
    );
    let input: String = String::from("кат РЭАДМЭ.мд");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "cat README.md");
    //Test all letters (Lowercase)
    let input: String = String::from("абкьдэфгхижйкълмнопкюрстуввьксызшщёюячц");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "abcdefghijjklmnopqrstuvwxyzshshhyoyuyachz");
    //Test all letters (Uppercase)
    let input: String = String::from("АБКЬДЭФГХИЖЙКЪЛМНОПКЮРСТУВВЬКСЫЗШЩЁЮЯЧЦ");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ABCDEFGHIJJKLMNOPQRSTUVWXYZSHSHHYOYUYACHZ");
    //Special cases 'Q'
    let input: String = String::from("москюуитто_пуб");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "mosquitto_pub");
    let input: String = String::from("МОСКЮУИТТО_ПУБ");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "MOSQUITTO_PUB");
    //Special case: В as last character
    let input: String = String::from("срв");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "srv");
    let input: String = String::from("СРВ");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "SRV");
    //Special case: Ye
    let input: String = String::from("елл");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "yell");
    let input: String = String::from("ЕЛЛ");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "YELL");
    //Special case: ck
    let input: String = String::from("чэкк чэкк");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "check check");
    let input: String = String::from("ЧЭКК ЧЭКК");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "CHECK CHECK");
    //Special case: k as last character which becomes 'c'
    let input: String = String::from("рэк к к");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "rec k k");
    let input: String = String::from("РЭК К К");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "REC K K");
    //Special case: k as last character which becomes 'k'
    let input: String = String::from("ок ок");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ok ok");
    let input: String = String::from("ОК ОК");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "OK OK");
    //Special case: k as first character
    let input: String = String::from("к о");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "k o");
    let input: String = String::from("К О");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "K O");
    //Special case: k as last character, but preceeded by 'к' | 'а' | 'и' | 'о'
    let input: String = String::from("как бар");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "cak bar");
    let input: String = String::from("КАК БАР");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "CAK BAR");
    let input: String = String::from("как");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "cak");
    let input: String = String::from("КАК");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "CAK");
    //Special case: k out of matches
    let input: String = String::from("кд");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "cd");
    let input: String = String::from("КД");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "CD");
    //Backtick and quote
    let input: String = String::from("ъьЪЬ");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "'`'`");
    //Symbols
    let input: String = String::from("№ ₽");
    let output = translator.to_latin(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "# $");
  }

  #[test]
  fn test_translator_lang_russian_to_cyrillic() {
    let translator: Box<dyn Translator> = new_translator(Language::Russian);
    //Test all
    let input: String = String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(
      output,
      "а б к д э ф г х и ж к л м н о п кю р с т у в у кс ы з"
    );
    let input: String = String::from("A B C D E F G H I J K L M N O P Q R S T U V W X Y Z");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(
      output,
      "А Б К Д Э Ф Г Х И Ж К Л М Н О П КЮ Р С Т У В У КС Ы З"
    );
    //Test particular case (sh)
    let input: String = String::from("shell");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "шэлл");
    let input: String = String::from("SHELL");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ШЭЛЛ");
    //Test particular case (jo)
    let input: String = String::from("Option");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "Оптён");
    let input: String = String::from("OPTION");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ОПТЁН");
    //Test particular case (ts)
    let input: String = String::from("tsunami");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "цунами");
    let input: String = String::from("TSUNAMI");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ЦУНАМИ");
    //Test particular case (g)
    let input: String = String::from("gin and games");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "джин анд гамэс");
    let input: String = String::from("GIN AND GAMES");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ДЖИН АНД ГАМЭС");
    //Test particular case (iu)
    let input: String = String::from("iuta");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "юта");
    let input: String = String::from("IUTA");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ЮТА");
    //Test particular case (ye)
    let input: String = String::from("yellow");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "еллоу");
    let input: String = String::from("YELLOW");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ЕЛЛОУ");
    //Test particular case (giu) + (ia)
    let input: String = String::from("giulia");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "джюля");
    let input: String = String::from("GIULIA");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ДЖЮЛЯ");
    //Test case 'ch'
    let input: String = String::from("channel");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "чаннэл");
    let input: String = String::from("CHANNEL");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ЧАННЭЛ");
    //Test some words
    let input: String = String::from("Usage: cat [OPTION]... [FILE]...");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "Усаджэ: кат [ОПТЁН]... [ФИЛЭ]...");
    //Special cases: last character is 'c'
    let input: String = String::from("chic");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "чик");
    let input: String = String::from("CHIC");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ЧИК");
    //Special cases: last character is 'п'
    let input: String = String::from("gag");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "гаг");
    let input: String = String::from("GAG");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ГАГ");
    //Special cases: last character is 'i'
    let input: String = String::from("vi");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ви");
    let input: String = String::from("VI");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ВИ");
    //Special cases: last character is 's'
    let input: String = String::from("less");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "лэсс");
    let input: String = String::from("LESS");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ЛЭСС");
    //Special cases: last character is 't'
    let input: String = String::from("cat");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "кат");
    let input: String = String::from("CAT");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "КАТ");
    //Special cases: y
    let input: String = String::from("yacc");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ыакк");
    let input: String = String::from("YACC");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ЫАКК");
    //Special cases: y part 2
    let input: String = String::from("y");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "ы");
    let input: String = String::from("Y");
    let output = translator.to_cyrillic(&input);
    println!("\"{}\" => \"{}\"", input, output);
    assert_eq!(output, "Ы");
  }
}
