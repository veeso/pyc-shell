# Changelog

- [Changelog](#changelog)
  - [Pyc 0.3.0](#pyc-030)
  - [Pyc 0.2.0](#pyc-020)

## Pyc 0.3.0

Released on 27.06.2020

- Reverse search for prompt
  - KeyBinding: CTRL+R (enter reverse search)
  - KeyBinding: CTRL+G (exit reverse search)
- Updated dependencies
  - nix: 0.19.0
  - dirs: 3.0.1
  - whoami: 0.9.0
  - libgit2: 0.13.12

## Pyc 0.2.0

Released on 27.06.2020

- Prompt improvements
  - Added left/right arrows handler to move cursor
  - Added bash shortcuts with ctrl key
  - Added history
    - Navigate through history with arrow UP and arrow DOWN
    - Perform a previously executed command with ```!{index}``` syntax
    - History command support ```history```
    - History will be now saved in ```$HOME/.config/pyc/pyc_history```
  - Added clear command ```clear```
- New translators:
  - 🇧🇾 Belarusian
  - 🇧🇬 Bulgarian
- Improved code coverage
- General performance improvement
