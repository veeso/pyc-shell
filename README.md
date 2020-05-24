# Pyc

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![Stars](https://img.shields.io/github/stars/ChristianVisintin/Pyc.svg)](https://github.com/ChristianVisintin/Pyc) [![Issues](https://img.shields.io/github/issues/ChristianVisintin/Pyc.svg)](https://github.com/ChristianVisintin/Pyc/issues) [![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/crates/pyc) [![Build](https://api.travis-ci.org/ChristianVisintin/Pyc.svg?branch=master)](https://travis-ci.org/ChristianVisintin/Pyc) [![codecov](https://codecov.io/gh/ChristianVisintin/Pyc/branch/master/graph/badge.svg)](https://codecov.io/gh/ChristianVisintin/Pyc)

~ Use your alphabet with your favourite shell ~  
Developed by Christian Visintin  
Current version: 0.1.0 (24/05/2020)

---

- [Pyc](#pyc)
  - [About Рус](#about-%d0%a0%d1%83%d1%81)
    - [The reasons](#the-reasons)
    - [How it works](#how-it-works)
  - [Features](#features)
  - [Supported alphabets](#supported-alphabets)
  - [Usage](#usage)
  - [Configuration](#configuration)
    - [Prompt Line Configuration](#prompt-line-configuration)
  - [Documentation](#documentation)
  - [Known issues](#known-issues)
    - [Unicode Replacement character while typing (�)](#unicode-replacement-character-while-typing-%ef%bf%bd)
    - [Cd command in oneshot mode doesn't work](#cd-command-in-oneshot-mode-doesnt-work)
    - [Fish doesn't work](#fish-doesnt-work)
    - [Bash alias not working](#bash-alias-not-working)
  - [Contributions](#contributions)
  - [Changelog](#changelog)
  - [License](#license)

---

## About Рус

Рус (Pronounced "Rus") is a simple CLI application, written in Rust, which allows you to interface with your favourite shell, giving you the possibility to perform commands in cyrillic and other alphabets, through command and output transliteration.

### The reasons

Well, basically I started studying russian and to become practical with the cyrillic alphabet I wanted to use it whenever was possible, even while typing on the console; but then I found out that there's not a single console which allow people which use a different alphabet to use it, so I came out with this project.

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

- 🇷🇺 Russian Cyrillic 🇷🇺 - According to russian cyrillic [GOST 7.79-2000](https://en.wikipedia.org/wiki/GOST_7.79-2000) with some differences ([See here](./docs/rus.md))
- 🇧🇾 Belarusian Cyrillic 🇧🇾 - *Coming soon*
- 🇧🇬 Bulgarian Cyrillic 🇧🇬 - *Coming soon* (for the moment, you can use Russian)
- 🇰🇷🇰🇵 Hangŭl 🇰🇵🇰🇷 - *TBD*
- 🇯🇵 Hiragana 🇯🇵 - *Coming soon*
- 🇲🇰 Macedonian Cyrillic 🇲🇰 - *TBD*
- 🇲🇪 Montenegrin Cyrillic 🇲🇪 - *TBD*
- 🇷🇸🇧🇦 Serbian Cyrillic 🇧🇦🇷🇸 - *TBD*
- 🇺🇦 Ukrainian Cyrillic 🇺🇦 - *TBD*

Other alphabets are not planned for the moment.
Can't find yours? Contribute to the project implementing it! [Read More](./CONTRIBUTING.md)

## Usage

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
```

- shell: Shell configuration
  - exec: shell binary (can be absolute or in PATH)
  - args: shell CLI arguments
- alias: list of alias. When the first word of a command is one of the configured alias, it is automatically replaced with the associated latin expression.
- language: Pyc default language (can be overridden with cli options)
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

### Prompt Line Configuration

The prompt configuration is used to setup the prompt line when using the interactive mode.
The prompt configuration provides parameters to customize the line printed when interacting with the shell.
In addition to the parameters described before, here the prompt line keys are illustrated.

Each prompt line key must have the following syntax ```${VAR_NAME}```

**General** keys

| Key      | Description                                                              |
|----------|--------------------------------------------------------------------------|
| USER     | Username                                                                 |
| HOSTNAME | Hostname                                                                 |
| WRKDIR   | Current directory                                                        |
| LANG     | The language configured for Pyc in flag colors of the associated country |
| CMD_TIME | Execution time of the last command if >= min_elapsed_time                |
| RC       | Shows the string associated to a successful exitcode or to an error      |

**Colors** keys

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

**Git** keys

| Key        | Description                 |
|------------|-----------------------------|
| GIT_BRANCH | The current git branch      |
| GIT_COMMIT | The current git commit  ref |

## Documentation

These are the documents related to Pyc documentation:

- [Russian transliteration](docs/rus.md)

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

- Windows Terminal (Maybe fixed in latest updates; need further tests)

### Cd command in oneshot mode doesn't work

Yep. Unfortunately it seems there's no way to make it work in oneshot mode.
If you know a possible way to make it work, please contribute to this project to implement it.

### Fish doesn't work

Uuhm, I don't know why, I need some time to investigate why, maybe it doesn't use stdout to write (?).

### Bash alias not working

I will fix this soon

---

## Contributions

Contributions are welcome, in particular regarding:

- translators
- generic improvements

If you think you can contribute to Рус, please follow [Рус's contributions guide](./CONTRIBUTING.md)

## Changelog

## License

Licensed under the GNU GPLv3 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.gnu.org/licenses/gpl-3.0.txt>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

You can read the entire license [HERE](./LICENSE.txt)
