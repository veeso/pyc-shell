# Pyc

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/Pyc.svg)](https://github.com/ChristianVisintin/Pyc) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/Pyc.svg)](https://github.com/ChristianVisintin/Pyc/issues) [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/pyc) [![Build](https://api.travis-ci.org/ChristianVisintin/Pyc.svg?branch=master)](https://travis-ci.org/ChristianVisintin/Pyc) [![codecov](https://codecov.io/gh/ChristianVisintin/Pyc/branch/master/graph/badge.svg)](https://codecov.io/gh/ChristianVisintin/Pyc)

~ Shell По-Русски ~  
Developed by Christian Visintin  
Current version: 0.1.0 (??/??/2020)

---

- [Pyc](#pyc)
  - [About Рус](#about-%d0%a0%d1%83%d1%81)
    - [The reasons](#the-reasons)
  - [Features](#features)
  - [Usage](#usage)
  - [Configuration](#configuration)
  - [Documentation](#documentation)
    - [Cyrillic to latin](#cyrillic-to-latin)
  - [Known issues](#known-issues)
    - [Unicode Replacement character while typing (�)](#unicode-replacement-character-while-typing-%ef%bf%bd)
  - [Contributions](#contributions)
  - [Changelog](#changelog)
  - [License](#license)

---

## About Рус

Pyc is a simple CLI application, written in Rust, which allows you to perform shell commands in russian cyrillic, through command and output transliteration.

### The reasons

Well, basically I started studying russian and to become practical with the cyrillic alphabet a started to use it for even when I was writing in my own language, but I couldn’t use it at work on the shell, so I thought it would have been funny to create Pyc.

## Features

- Transliteration of every word according to russian [GOST 7.79-2000](https://en.wikipedia.org/wiki/GOST_7.79-2000) with some differences ([See here](#cyrillic-to-latin))
- Conversion of both Input and outputs.
- Escaping for latin strings.
- Oneshot and interactive modes
- Customizations for interactive mode and aliases

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
```

- аляс (or alias): list of alias. When the first word of a command is one of the configured alias, it is automatically replaced with the associated latin expression.

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

---

## Contributions

TODO

## Changelog

## License

Licensed under the GNU GPLv3 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.gnu.org/licenses/gpl-3.0.txt>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

You can read the entire license [HERE](./LICENSE.txt)
