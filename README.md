# Pyc

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/Pyc.svg)](https://github.com/ChristianVisintin/Pyc) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/Pyc.svg)](https://github.com/ChristianVisintin/Pyc/issues) [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/pyc) [![Build](https://api.travis-ci.org/ChristianVisintin/Pyc.svg?branch=master)](https://travis-ci.org/ChristianVisintin/Pyc) [![codecov](https://codecov.io/gh/ChristianVisintin/Pyc/branch/master/graph/badge.svg)](https://codecov.io/gh/ChristianVisintin/Pyc)

~ Shell По-Русски ~  
Developed by Christian Visintin  
Current version: 0.1.0 (??/??/2020) **STILL UNDER DEVELOPMENT**

---

- [Pyc](#pyc)
  - [About Рус](#about-%d0%a0%d1%83%d1%81)
    - [The reasons](#the-reasons)
  - [Features](#features)
  - [Usage](#usage)
  - [Configuration](#configuration)
  - [Documentation](#documentation)
    - [Cyrillic to latin](#cyrillic-to-latin)
    - [Latin to Cyrillic](#latin-to-cyrillic)
  - [Known issues](#known-issues)
    - [Unicode Replacement character while typing (�)](#unicode-replacement-character-while-typing-%ef%bf%bd)
    - [Cd command in oneshot mode doesn't work](#cd-command-in-oneshot-mode-doesnt-work)
  - [Contributions](#contributions)
  - [Changelog](#changelog)
  - [License](#license)

---

## About Рус

Pyc is a simple CLI application, written in Rust, which allows you to perform shell commands in russian cyrillic, through command and output transliteration.

### The reasons

Well, basically I started studying russian and to become practical with the cyrillic alphabet a started to use it for even when I was writing in my own language, but I couldn’t use it at work on the shell, so I thought it would have been funny to create Pyc.

## Features

- Transliteration of every word according into russian cyrillic [GOST 7.79-2000](https://en.wikipedia.org/wiki/GOST_7.79-2000) with some differences ([See here](#cyrillic-to-latin))
- Possibility to implement new translators for other cyrillic alphabets.
- Conversion of both Input and outputs.
- Escaping for latin strings.
- Oneshot and shell modes
- Customizations and aliases

## Usage

TODO

## Configuration

Pyc supports a user configuration which adds some features and customization.
The configuration must be stored at ```$HOME/.config/pyc/pyc.yml```. A default configuration is located in the repository at ```/pyc.yml```.

Let's see how the configuration is written

```yaml
аляс:
  - чд: "cd"
  - пвд: "pwd"
  - уич: "which"
оутпут:
  транслатэ: true
```

- аляс (or alias): list of alias. When the first word of a command is one of the configured alias, it is automatically replaced with the associated latin expression.
- оутпут (or output): output configuration
  - транслатэ (or translate): indicates to pyc whether the output has to be converted to cyrillic or not

## Documentation

### Cyrillic to latin

The conversion from cyrillic to latin follows the [GOST 7.79-2000](https://en.wikipedia.org/wiki/GOST_7.79-2000) standard with some differences. The entire conversion table is illustrated here below:

| Russian | Latin | Notes                                                                                                                                                  |
|---------|-------|--------------------------------------------------------------------------------------------------------------------------------------------------------|
| А       | A     |                                                                                                                                                        |
| Б       | B     |                                                                                                                                                        |
| К       | C     | K is translated into C, only when not followed ```'Е','Э','И','Й','Ы','ъ'```, or it is preceeded by ```'К','А','И','О'```. You can force a 'C' using ```'Кь'```  |
| Ч       | CH    |                                                                                                                                                        |
| Д       | D     |                                                                                                                                                        |
| Э       | E     |                                                                                                                                                        |
| Ф       | F     |                                                                                                                                                        |
| Г       | G     |                                                                                                                                                        |
| Х       | H     |                                                                                                                                                        |
| И       | I     |                                                                                                                                                        |
| Ж       | J     |                                                                                                                                                        |
| Й       | J     |                                                                                                                                                        |
| Ё       | JO    |                                                                                                                                                        |
| К       | K     | K is converted to latin K only when followed by ```'Е','Э','И','Й','Ы','ъ'``` ,or it is NOT preceeded by ```'К','А','И','О'``` .You can force a K using ```'КЪ'``` |
| Л       | L     |                                                                                                                                                        |
| М       | M     |                                                                                                                                                        |
| Н       | N     |                                                                                                                                                        |
| О       | O     |                                                                                                                                                        |
| П       | P     |                                                                                                                                                        |
| Кю      | Q     |                                                                                                                                                        |
| Р       | R     |                                                                                                                                                        |
| С       | S     |                                                                                                                                                        |
| Т       | T     |                                                                                                                                                        |
| У       | U     |                                                                                                                                                        |
| В       | V     |                                                                                                                                                        |
| Вь      | W     |                                                                                                                                                        |
| КС      | X     |                                                                                                                                                        |
| Ы       | Y     |                                                                                                                                                        |
| Я       | YA    |                                                                                                                                                        |
| Е       | YE    |                                                                                                                                                        |
| Ю       | YU    |                                                                                                                                                        |
| З       | Z     |                                                                                                                                                        |
| ₽       | $     |                                                                                                                                                        |
| Ъ       | '     |                                                                                                                                                        |
| Ь       | `     |                                                                                                                                                        |
| №       | #     |                                                                                                                                                        |

### Latin to Cyrillic

| Latin | Russian | Notes                         |
|-------|---------|-------------------------------|
| А     | A       |                               |
| B     | Б       |                               |
| C     | К       | Unless if followed by H       |
| CH    | Ч       |                               |
| Ч     | CH      |                               |
| D     | Д       |                               |
| E     | Э       |                               |
| F     | Ф       |                               |
| G     | Г       |                               |
| G     | ДЖ      | If g is followed by Y, E, I   |
| H     | Х       |                               |
| I     | И       | Unless if followed be U, A, O |
| IU    | Ю       |                               |
| IA    | Я       |                               |
| IO    | Ё       |                               |
| J     | Ж       |                               |
| K     | К       |                               |
| L     | Л       |                               |
| M     | М       |                               |
| N     | Н       |                               |
| O     | О       |                               |
| P     | П       |                               |
| Q     | КЮ      |                               |
| R     | Р       |                               |
| S     | С       | Unless if followed by H       |
| Sh    | Ш       |                               |
| T     | Т       |                               |
| TS    | Ц       | Unless if followed by S       |
| U     | У       |                               |
| V     | В       |                               |
| W     | В       |                               |
| X     | КС      |                               |
| Y     | Ы       | Unless if followed by E       |
| YE    | E       |                               |
| Z     | З       |                               |

---

## Known issues

### Unicode Replacement character while typing (�)

If you see this character ```�``` when you're typing cyrillic characters, these steps may fix the problem:

Reconfigure locales:

```sh
sudo dpkg-reconfigure locales
```

Select all the locales you need from the list, but most important select **ru_RU.UTF-8**.

Regenerate locales:

```sh
sudo locale-gen
```

### Cd command in oneshot mode doesn't work

Yep. Unfortunately it seems there's no way to make it work in oneshot mode.
If you know a possible way to make it work, please contribute to this project to implement it.

---

## Contributions

TODO

## Changelog

## License

Licensed under the GNU GPLv3 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.gnu.org/licenses/gpl-3.0.txt>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

You can read the entire license [HERE](./LICENSE.txt)
