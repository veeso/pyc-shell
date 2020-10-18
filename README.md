# Pyc

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/Pyc.svg)](https://github.com/ChristianVisintin/Pyc) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/Pyc.svg)](https://github.com/ChristianVisintin/Pyc/issues) [![Crates.io](https://img.shields.io/badge/crates.io-v0.3.0-orange.svg)](https://crates.io/crates/pyc-shell) [![Build](https://api.travis-ci.org/ChristianVisintin/Pyc.svg?branch=master)](https://travis-ci.org/ChristianVisintin/Pyc) [![codecov](https://codecov.io/gh/ChristianVisintin/Pyc/branch/master/graph/badge.svg)](https://codecov.io/gh/ChristianVisintin/Pyc)

~ Use your alphabet with your favourite shell ~  
Developed by Christian Visintin  
Current version: [0.3.0 (??/??/2020)](./CHANGELOG.md#pyc-030)

---

- [Pyc](#pyc)
  - [About Рус](#about-рус)
    - [Why Рус](#why-рус)
    - [Pyc's goal](#pycs-goal)
    - [How it works](#how-it-works)
  - [Features](#features)
  - [Supported alphabets](#supported-alphabets)
    - [Planned alphabets](#planned-alphabets)
  - [Installation](#installation)
    - [Cargo](#cargo)
    - [Deb / Rpm](#deb--rpm)
    - [Usage](#usage)
  - [Configuration](#configuration)
    - [Prompt Line Configuration](#prompt-line-configuration)
      - [General keys](#general-keys)
      - [Colors keys](#colors-keys)
      - [Git keys](#git-keys)
  - [Documentation](#documentation)
  - [Escape text](#escape-text)
  - [Known issues](#known-issues)
    - [Unicode Replacement character while typing (�)](#unicode-replacement-character-while-typing-)
    - [Cd command in oneshot mode doesn't work](#cd-command-in-oneshot-mode-doesnt-work)
    - [Fish doesn't work](#fish-doesnt-work)
    - [Shell alias not working](#shell-alias-not-working)
    - [Text editors dont' work](#text-editors-dont-work)
  - [Upcoming Features and Releases](#upcoming-features-and-releases)
    - [Development Status](#development-status)
    - [Planned releases](#planned-releases)
      - [Pyc 0.3.0](#pyc-030)
      - [Pyc 0.4.0](#pyc-040)
  - [Contributions](#contributions)
  - [Changelog](#changelog)
  - [License](#license)

---

## About Рус

Рус (Pronounced "Rus") is a simple CLI application, written in Rust, which allows you to interface with your favourite shell, giving you the possibility to perform commands in cyrillic and other alphabets, through command and output transliteration.

### Why Рус

Well, basically I started studying russian and to become practical with the cyrillic alphabet I wanted to use it whenever was possible, even while typing on the console; but then I found out that there's not a single console which allow people which use a different alphabet to use it, so I came out with this project.

### Pyc's goal

The goal of this project is to give everybody who uses an alphabet different from latin, to use the computer shell without having to switch the keyboard layout.

### How it works

Basically Рус is a shell interface, which means that it reads the user input, it converts it according to the configured alphabet and then it sends the translated input to the shell which processes it; then the output is read from the shell's stdout and is (if enabled) translated back to the original alphabet and printed to user's output.

## Features

- Different alphabets support
- Possibility to easily implement new translators for other cyrillic alphabets.
- Conversion of both Input and outputs.
- Escaping for latin strings.
- Interactive, oneshot and file modes.
- Customizations and aliases

## Supported alphabets

- ![by](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Belarus.png) Belarusian Cyrillic - According to russian cyrillic [GOST 7.79-2000](https://en.wikipedia.org/wiki/GOST_7.79-2000) with some differences ([See here](./docs/translators/by.md))
- ![bg](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Bulgaria.png) Bulgarian Cyrillic - According to russian cyrillic [GOST 7.79-2000](https://en.wikipedia.org/wiki/GOST_7.79-2000) with some differences ([See here](./docs/translators/ru.md))
- ![ru](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Russia.png) Russian Cyrillic - According to russian cyrillic [GOST 7.79-2000](https://en.wikipedia.org/wiki/GOST_7.79-2000) with some differences ([See here](./docs/translators/ru.md))

### Planned alphabets

- ![in](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/India.png) Devanagari - *Coming soon (0.3.0)*
- ![kr](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/South-Korea.png) Hangŭl - According to [Revised Romanization of Korean](https://en.wikipedia.org/wiki/Revised_Romanization_of_Korean) *Coming soon (0.3.0)*
- ![mk](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Macedonia.png) Macedonian Cyrillic - *TBD*
- ![me](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Montenegro.png) Montenegrin Cyrillic - *TBD*
- ![rs](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Serbia.png)![br](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Bosnia-and-Herzegovina.png) Serbian Cyrillic - *Coming soon (0.3.0)*
- ![ua](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Ukraine.png) Ukrainian Cyrillic - *Coming soon (0.3.0)*

Other alphabets are not planned for the moment.  

---

Can't find yours? Contribute to the project implementing it 😀! [Read More](./CONTRIBUTING.md)

## Installation

If you're considering to install Pyc I want to thank you 💛 ! I hope this project can be useful for you!  
If you want to contribute to this project, don't forget to check out our contribute guide. [Read More](./CONTRIBUTING.md)

### Cargo

```sh
#Install pyc through cargo
cargo install pyc-shell
#Install configuration
mkdir -p $HOME/.config/pyc/
#Copy configuration file from repository
wget -O $HOME/.config/pyc/pyc.yml https://raw.githubusercontent.com/ChristianVisintin/Pyc/master/pyc.yml
```

### Deb / Rpm

Coming soon

### Usage

Pyc can be started with the following options:

- ```-c, --command <command>``` Runs the provided command and return
- ```-C, --config <config>``` Specify Pyc configuration file location.
- ```-l, --lang <ru|рус>``` Specify the language used by Pyc
- ```-s, --shell </bin/bash>``` Specify the shell binary path
- ```-v, --version``` Print version info
- ```-h, --help``` Print help page

## Configuration

Pyc supports a user configuration which adds some features and customization.
The configuration must be stored at ```$HOME/.config/pyc/pyc.yml```. A default configuration is located in the repository in [pyc.yml](./pyc.yml).

Let's see how the configuration is written

```yaml
language: ru
shell:
  exec: "bash"
  args:
    - "-l"
alias:
  - чд: cd
  - пвд: pwd
  - уич: which
output:
  translate: true
prompt:
  prompt_line: "${USER} on ${HOSTNAME} in ${WRKDIR} ${GIT_BRANCH} ${GIT_COMMIT} ${CMD_TIME}"
  history_size: 256
  translate: false
  break:
    enabled: true
    with: "❯"
  duration:
    min_elapsed_time: 2000
  rc:
    ok: "✔"
    error: "✖"
  git:
    branch: "on  "
    commit_ref_len: 8
    commit_prepend: "("
    commit_append: ")"
```

- shell: Shell configuration
  - exec: shell binary (can be absolute or in PATH)
  - args: shell CLI arguments
- alias: list of alias. When the first word of a command is one of the configured alias, it is automatically replaced with the associated latin expression.
- language: Pyc default language (can be overridden with cli options)
  - **Belarusian**: by | бел
  - **Bulgarian**: bg | бг | блг
  - **Russian**: ru | рус
- output: output configuration
  - translate: indicates to pyc whether the output has to be converted to cyrillic or not
- prompt: Prompt configuration (See [Prompt Configuration](#prompt-line-configuration))
  - prompt_line: String describing the prompt line syntax
  - history_size: Pyc history size
  - translate: should the prompt line be translated
  - break: Break line after prompt
    - enabled: should the prompt break or not?
  - duration: command duration configuration
    - enabled: module enabled
    - with: break with provided string
  - rc: return code module
    - ok: string to write in case of successful command
    - error: string to write in case of error
  - git: git module
    - branch: string to write before writing branch name
    - commit_ref_len: length of commit reference
    - commit_prepend: string to prepend to commit ref
    - commit_append: string to append to commit ref

### Prompt Line Configuration

The prompt configuration is used to setup the prompt line when using the interactive mode.
The prompt configuration provides parameters to customize the line printed when interacting with the shell.
In addition to the parameters described before, here the prompt line keys are illustrated.

Each prompt line key must have the following syntax ```${VAR_NAME}```

#### General keys

| Key      | Description                                                              |
|----------|--------------------------------------------------------------------------|
| USER     | Username                                                                 |
| HOSTNAME | Hostname                                                                 |
| WRKDIR   | Current directory                                                        |
| LANG     | The language configured for Pyc in flag colors of the associated country |
| CMD_TIME | Execution time of the last command if >= min_elapsed_time                |
| RC       | Shows the string associated to a successful exitcode or to an error      |

#### Colors keys

| Key      | Description |
|----------|-------------|
| KYEL     | Yellow      |
| KRED     | Red         |
| KBLU     | Blue        |
| KMAG     | Magenta     |
| KGRN     | Green       |
| KWHT     | White       |
| KBLK     | Black       |
| KGRY     | Gray        |
| KRST     | Reset       |

#### Git keys

| Key        | Description                 |
|------------|-----------------------------|
| GIT_BRANCH | The current git branch      |
| GIT_COMMIT | The current git commit  ref |

## Documentation

The developer documentation can be found on Rust Docs at <https://docs.rs/pyc-shell>

The documentation related to translator modules can be instead found here:

- [Belarusian transliteration](docs/by.md)
- [Bulgarian transliteration](docs/bg.md)
- [Russian transliteration](docs/ru.md)

## Escape text

It is possible to escape texts (only when the prompt line is visible, not while a program is running), preventing it from being transliterated to latin.
To do so, just use quotes:

```sh
#Touch foobar.txt
тоуч фообар.ткст
```

Escaped:

```sh
#Touch фообар.мксм
тоуч "фообар.ткст"
```

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

If this didn't solve your problem it's probably an issue of your terminal. Up to now I've found out that some terminals just don't let you type non-ascii characters when executing an application. These are the terminals which I've used or have been reported which **DON'T** work.

- Windows Terminal (Older version I guess, I noticed it has been fixed around april 2020).

### Cd command in oneshot mode doesn't work

Yep. Unfortunately it seems there's no way to make it work in oneshot mode.
If you know a possible way to make it work, please contribute to this project to implement it.

### Fish doesn't work

Uuhm, I don't know why, I need some time to investigate why, maybe it doesn't use stdout to write (?).

### Shell alias not working

I will fix this soon

### Text editors dont' work

I will try to fix this issue

---

## Upcoming Features and Releases

### Development Status

Pyc-shell is an active project, development effort is minimum at the moment due to my spare time and to the little interest from the community. In addition I'm working on other bigger projects.

### Planned releases

No new releases are planned for 2020.

#### Pyc 0.3.0

Planned for December 2020

- translators:
  - devanagari
  - hangul
  - ukrainian
  - serbian
- reverse search
- new configurations keys

#### Pyc 0.4.0

Planned for 2021

- translators:
  - Macedonian
  - Montenegrin
- Fish support
- Text editors support

## Contributions

Contributions are welcome! 😉

If you think you can contribute to Рус, please follow [Рус's contributions guide](./CONTRIBUTING.md)

## Changelog

See the enire changelog [HERE](CHANGELOG.md)

---

## License

Licensed under the GNU GPLv3 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.gnu.org/licenses/gpl-3.0.txt>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

You can read the entire license [HERE](./LICENSE.txt)
