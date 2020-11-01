# Lev

Developed and designed by Christian Visintin.
Part of the Pyc-shell Project.

- [Lev](#lev)
  - [Introduction](#introduction)
  - [Features](#features)
  - [Usage](#usage)
  - [Commands](#commands)
  - [License](#license)

---

## Introduction

Lev text editor is the built-in text editor in Pyc-shell. Pratically is a program inside another program. It has its option parser, it receives its arguments, etc...
The reason Lev is necessary for Pyc, is the fact that Pyc as it is implemented, doesn't allow you to use a traditional text editor, such as vim, nano or emacs.
Lev text editor is designed to provide something in between vim and nano; so not too simple, but not even too complex. It supports in my opinion anything you need in a terminal text editor.

And if you ask, yes, Lev because of Tolstòj.

## Features

- Simple text editor functionalities
  - open/close
  - save/save as
  - revert changes
  - delete row
- Syntax hightlightning with [Syntect](https://github.com/trishume/syntect)
- Customizations
  - Syntax hightlightning theme

## Usage

TODO: complete

## Commands

Commands are performed through keyboard shortcuts:

- ```Esc```: Quit
- ```Ctrl+A```: Go at the beginning of the current line
- ```Ctrl+B```:
- ```Ctrl+C```: Copy current line
- ```Ctrl+D```: Delete current line
- ```Ctrl+E```: Go at the end of the current line
- ```Ctrl+F```:
- ```Ctrl+G```:
- ```Ctrl+H```: Show help
- ```Ctrl+I```:
- ```Ctrl+J```:
- ```Ctrl+K```:
- ```Ctrl+L```:
- ```Ctrl+M```:
- ```Ctrl+N```:
- ```Ctrl+O```:
- ```Ctrl+P```:
- ```Ctrl+Q```: Quit (same as esc)
- ```Ctrl+R```:
- ```Ctrl+S```: Save file
- ```Ctrl+T```: Enable/Disable input/output transliteration
- ```Ctrl+U```:
- ```Ctrl+V```: Paste previously copied row
- ```Ctrl+W```: Save file with another name
- ```Ctrl+X```:
- ```Ctrl+Y```:
- ```Ctrl+Z```:

## License

Licensed under the GNU GPLv3 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at

<http://www.gnu.org/licenses/gpl-3.0.txt>

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

You can read the entire license [HERE](./LICENSE.txt)
