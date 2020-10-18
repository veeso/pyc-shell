# Contributing

Before contributing to this repository, please first discuss the change you wish to make via issue of this repository before making a change.

Please note we have a [code of conduct](./CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

- [Contributing](#contributing)
  - [Preferred contributions](#preferred-contributions)
  - [Pull Request Process](#pull-request-process)
  - [Developer contributions guide](#developer-contributions-guide)
    - [How Pyc Works](#how-pyc-works)
      - [Runtime](#runtime)
      - [Config](#config)
      - [Shell](#shell)
      - [Translators](#translators)
        - [IOProcessor](#ioprocessor)
        - [Translator](#translator)
      - [Utils](#utils)
    - [The prompt](#the-prompt)
    - [Shell IPC](#shell-ipc)
    - [Implement Translators](#implement-translators)
    - [Implement Prompt Modules](#implement-prompt-modules)

## Preferred contributions

At the moment, these kind of contributions are more appreciated and should be preferred:

- Fix for issues described in [Known Issues](./README.md#known-issues)
- New traslators: for further details see [Implement Translators](#implement-translators)
- Improvements to translators: any improvement to transliteration is accepted if makes sense, consider that my implementations could be not 100% correct (and probably they're not), indeed consider that I don't speak all these languages (tbh I only can speak Russian as a language with a different alphabet from latin - and I can't even speak it very well).
- Code optimizations: any optimization to the code is welcome

For any other kind of contribution, especially for new features, please submit an issue first.

## Pull Request Process

Let's make it simple and clear:

1. Write your code.
2. Write a **properly documentation** compliant with **rustdoc** standard.
3. Write tests for your code. **Code coverage for your code must be at least at 90%**.
4. Report changes to the issue you opened.
5. Update the README.md with details of changes to the interface, this includes new line in the changelog, new modules if added, new command line options if added, etc.
6. Request maintainers to merge your changes.

## Developer contributions guide

Welcome to the contributions guide for Pyc. This chapter DOESN'T contain the documentation for Pyc, which can instead be found on Rust Docs at <https://docs.rs/pyc-shell>
This chapter describes how Pyc works and the guide lines to implement stuff such as translators or prompt modules.

### How Pyc Works

Pyc is made up of Runtime and 4 modules; each one has a well defined task.

- [Runtime](#runtime)
- [Config](#config)
- [Shell](#shell)
- [Translator](#translator)
- [Utils](#utils)

#### Runtime

Runtime is the main module and it's executed directly by main. Runtime takes care of the following tasks:

- Read the user input and transliterate it from the alphabet configured to latin.
- Interfacing with the shell module to print the prompt line and write/read from shell's stdin/stdout.
- Transliterate the shell output to the configured alphabet (this can be disabled)

Runtime has 3 different working modes:

- Interactive mode: the classic shell mode. This mode makes the runtime to poll for input and output from the shell until the user stops the shell execution (until state is ```ShellState::Terminated```). When the state changes from ```ShellState::SubprocessRunning``` to ```ShellState::Idle``` the prompt is printed.
- Oneshot mode: executes the command provided as parameter and returns
- File mode: read the file provided as argument, read each line and executes them, then returns.

#### Config

Config is the module which takes care of parsing the YAML configuration file. This module defines the struct which contains the parameters which can be defined in configuration. The configuration is then used by the runtime and by the shell.

#### Shell

Shell is the biggest and the most important module of Pyc.
The main struct of the Shell module, is indeed ```Shell```. ```Shell``` has 3 members:

- process: it is an instance of ```ShellProc```, it's used to interface with the child process (shell).
- prompt: it is an instance of ```ShellPrompt```, it's used to process and print the shell prompt.
- props: it is an instance of ```ShellProps```, it's used to store shell properties, such as username, hostname, exit_status...

The Shell is then, a wrapper for all the different components required to build a shell.
We'll go deeper on how the prompt and the interface with the shell process work later. If you want to skip this chapter and go straight there click [HERE](#the-prompt)  for the prompt and [HERE](#shell-ipc) for the shell IPC.

#### Translators

The translator is the module which takes care of translitering the texts from latin to a certain alphabet and viceversa.
The module is made up of two main components: the **IOProcessor** and the **Translator** itself.

##### IOProcessor

The IOProcessor is a struct which has 4 methods:

- expression_to_latin: converts an expression from your alphabet into latin alphabet.
- expression_to_cyrillic: converts a latin expression to cyrillic (or other alphabet). This method is up to day totally useless and unused, I keep there because it might be useful one day.
- text_to_latin: converts text written in a certain alphabet into latin alphabet.
- text_to_cyrillic: converts text written in latin alphabet into another alphabet.

You may be confused right now, and probably you're wondering: ```What's the difference between expression_to_latin and text_to_latin?``` and ```What if I wanted to implement Hangŭl alphabet? I can see only latin and cyrillic here...```.

Well for the second question, the answer is pretty much simple: Pyc at the beginning was made to convert russian cyrillic to latin, and nothing else, but then I though "Hey, this project could be extended for any other alphabet", so the function names have been kept. Am I going to change them? I'm too lazy for this, and I don't like to change the idea of changing ```text_to_cyrillic``` to ```text_to_something```.

Let's talk instead about text and expressions: the problem here was "What about if the user has a file with cyrillic characters as name?". Yep, in that case the filename would be invalid. So I decided to create a way to escape names, and here expressions came. For further details see [Escape Text](./README.md#escape-text) in the README. But, for you, developer, the only thing you need to know is that expressions are used in interactive mode only and only when you're typing a command, otherwise text is always used.

Let's finish saying that the IOProcessor takes care of taking as parameter a Translator and to use it to convert the text, in the meantime it also takes care of handling expressions, escapes, discarding colors and stuff like that.

##### Translator

Let's finally talk about the Translator, which is probably more interesting, since probably nobody is ever gonna touch the IOProcessor.
The Translator is a Trait:

```rs
pub trait Translator {
  fn to_latin(&self, input: String) -> String;
  fn to_cyrillic(&self, input: String) -> String;
}
```

Nothing particularly complicated, two methods to implement, to_latin and to_cyrillic (to_cyrillic as said before, should be considered as a to_your_alphabet).
You'll learn how to implement a Translator later, if you want to skip the other documentation to know how click [HERE](#implement-translators).

#### Utils

Last but not least (well maybe not so important), the utils module.
This is that kind of module which contains functions of different kinds, but nothing to specific for a module itself.
To keep it clear, I split the utils module into different files with a name which describes clearly what it is about.
Usually utils is used only but Runtime or Main.

### The prompt

Let's talk about the prompt line now. Just a line of text right? Yep, but not so simple. As you've probably already seen the Pyc prompt line is customizable which makes it more complex. Pratically the Prompt has a Print method, which, when invoked, print the prompt line parsing the configured prompt line replacing all the keys. The Prompt keys have a syntax of this kind ```${KEY_NAME}```.
The ShellPrompt is a structure which basically has as properties, the prompt line to process, the different options, a flag which indicates if the prompt line has to be transliterated, and the Cache.

Once ```print()``` is called, the prompt instance calls ```process_prompt()``` which iterates over prompt_line keys and finally ```resolve_key()``` is called, which replace the key with the associated value.

The Prompt, to resolve keys, use both native keys (e.g. ```${USER}```) and extensions keys (which can be found in modules/ directory) which usually have a name like this one ```${$MODULENAME_KEYNAME}``` (e.g. ```${GIT_COMMIT}```). The prompt can be extended adding new modules (See [HERE](#implement-prompt-modules) for further details).

The Prompt uses a cache too (```PromptCache```). The Cache is an object which caches some values. At the moment the cache is invalidated after printing the prompt, maybe in the future it could be kept for a longer time. The use of the cache is just to speed a little up some tasks. For example, since the GIT_BRANCH and GIT_COMMIT have both to fetch for a repository, the cache contains the repository in the current directory, doing so prevents to fetch for a git repository twice. You may probably think it's nothing, but trust me, out there, there are plenty of shells extensions which do these stuffs and DON'T cache anything, and if you use them, you'll notice the performances are not so good 😉.

### Shell IPC

This chapter describes how the communication between Pyc and the shell works.
The first thing you need to know is that all the magic happens in the ```ShellProc``` module, this module indeed forks and the child starts the shell you provided in the configuration using ```nix::unistd::execvp()```. Before doing this, it creates 3 named pipes on your FS (the directory is located in tempfile, and it's created dinamycally) and sets the child process' stdout, stdin and stderr to the pipes.
From now on the parent process, which is pyc, writes and read from the child, which makes it able to read from and write to the shell.

There's though, a little issue with this mechanism, which is how does the parent know when the shell is running a process (e.g. cat) and when is idling (which means the prompt has to be shown to the user). Well, the mechanism is particular, probably you won't like it, but works, and pretty well too.
This mechanism consists in appending an echo command which will report the shell status. This command is appended obviously only when ```write()``` is called and the shell is in Idle state. The echo command will be executed only once all the other commands have already been terminated.

This is the echo command which ShellProc appends:

```sh
echo "\x02$?;`pwd`;{UUID}\x03"
```

Basically, when the shell finishes to run commands, it will output the exit status of the last command, the working directory, the **UUID** (now I'll explain), everything sorrounded by STX and ETX.
The UUID is generated on the constructor of ShellProc and it's used to identify the output as the terminator of shell execution. The UUID guarantees to prevent Pyc from confusing the output from the termination string (I mean, it's pretty hard to imagine to find in any output something like ```9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\x03```, and remember the **UUID must match too**, so YES, it's **unlikely to happen** 😏).
Once the terminator string is found, ShellProc removes it from the output, it gathers the metadata it needs and return to the user the output without the termination string. Does it work? Yes it does 😐, pretty well too. There is also a mechanism which prevents issues due to fragmentation (since up to 8192 bytes are read on each read) if you're asking.

Let's conclude the description on how Shell IPC works saying that, to know if the shell has terminated, ShellProc uses waitpid ```nix::sys::wait::waitpid(nix::unistd::Pid::from_raw(self.pid), Some(nix::sys::wait::WaitPidFlag::WNOHANG))```

### Implement Translators

This chapter describes how to implement and write a documentation for a new translator for Pyc.
As we've seen before, a translator is a struct which implements the ```Translator``` trait and basically just has a lookup table to return the transliteration. Remember that in many cases the preceeding/following character matters.

Let's see the steps to implement the translators, imagine we're going to implement the Emoji alphabet:

1. Define translation table

    Define the translations rules as did for example with [Russian](docs/translators/ru.md) and create a markdown document with exact same syntax and reporting the standard used to create the translator.

    To implement a Translator there are basically 3 rules:

    - If a standard for transliteration exists, please follow it as much it's possible.
    - All characters from source alphabet must be transliterated to latin
    - All latin characters must have a transliteration to source alphabet

2. Define translator

    Go to ```src/translator/lang/mod.rs```

    Add your language to Language enum

    ```rs
    pub enum Language {
      Russian,
      Emoji
    }
    ```

    Implement ToString for it (this is used by the prompt)

    ```rs
    impl ToString for Language {
      fn to_string(&self) -> String {
        match self {
          Language::Russian => String::from("рус"),
          Language::Emoji => String::from("emj")
        }
      }
    }
    ```

    Add it to language module definitions

    ```rs
    //NOTE: languages are listed here
    struct Russian {}
    mod russian;
    struct Emoji {}
    mod emoji;
    ```

3. Add it to translator constructor

    Move to ```src/translator/mod.rs```
    Add it to new_translator

    ```rs
    pub fn new_translator(language: Language) -> Box<dyn Translator> {
      match language {
        Language::Emoji => Box::new(Emoji {}),
        Language::Russian => Box::new(Russian {}),
      }
    }
    ```

4. Implement Translator

    First create a new file ```src/translator/lang/{ALPHABET}.rs```.
    Then following the rules you've defined at point 1, implement it and write tests to cover all its combinations.

    ```rs
    use super::{Emoji, Translator};

    impl Translator for Emoji {
      fn to_latin(&self, input: String) -> String {
        //...
      }
      fn to_cyrillic(&self, input: String) -> String {
        //...
      }
    }
    ```

5. Add it to Prompt Language module

    Move to ```src/shell/prompt/modules/language.rs```

    Add it to ```language_to_str```

    ```rs
    match language {
        Language::Russian => String::from(format!(
            "{}{}{}{}{}{}{}",
            PromptColor::White.to_string(),
            lang_str.chars().nth(0).unwrap(),
            PromptColor::Blue.to_string(),
            lang_str.chars().nth(1).unwrap(),
            PromptColor::Red.to_string(),
            lang_str.chars().nth(2).unwrap(),
            PromptColor::Reset.to_string()
        ))
    }
    ```

6. Add it to str_to_language in main

    Move to ```src/main.rs```

    Add the conversion from configuration String to Language

    ```rs
    match lang.as_str() {
        "ru" | "рус" => translator::Language::Russian,
        "em" | "😂" => translator::Language::Emoji,
        _ => {
            eprintln!(
                "{}",
                Colour::Red.paint(format!(
                    "Unknown language: '{}'; Setting language to default: ru",
                    lang
                ))
            );
            translator::Language::Russian
        }
    }
    ```

7. Write documentation

    Add the alphabet in the README in the ```Supported alphabets```.

    ```md
    - ![br](https://raw.githubusercontent.com/gosquared/flags/master/flags/flags/shiny/24/Emoji.png) Emoji - According to emoji standard [EMOJI-2020](https://en.wikipedia.org/wiki/EMOJI-2020) with some differences ([See here](./docs/translators/emoji.md))
    ```

    Finally review the Emoji doc in ```docs/translators/emoji.md```
    Here you have to write a lookup table which describes how you transliterate the characters from the two alphabets.

### Implement Prompt Modules

This chapter describes how to implement and write a documentation for a new prompt module in Pyc.

1. Define your module's keys

    These keys must use the ${MODULENAME_KEYNAME} name notation.
    Report them in the README in ```Prompt Line Configuration```

    ```md
    #### MYMODULE keys

    | Key         | Description     |
    |-------------|-----------------|
    | MODULE_KEY1 | Key description |
    | MODULE_KEY2 | Key description |
    ```

    Report keys in YAML configuration if you have customizable values

2. Write your module

    Create your module in ```src/shell/prompt/modules/```.
    All you have to do is implement the functions you need to provide a String for the prompt.

    The keys must be reported as const in your module. E.g.

    ```rs
    //Keys
    pub(crate) const PROMPT_GIT_BRANCH: &str = "${GIT_BRANCH}";
    pub(crate) const PROMPT_GIT_COMMIT: &str = "${GIT_COMMIT}";
    ```

    As always, write tests to cover your module.

3. Add your module to mod.rs

    Move to ```src/shell/prompt/modules/mod.rs```
    Here add this line

    ```rs
    pub(crate) mod yourmodule;
    ```

4. Add your keys to ```resolve_key```

    Move to ```src/shell/prompt/mod.rs```.
    Add your key to key resolver, remember you must return a String. Whenever is possible, try to cache intermediate entities.

    ```rs
    match key.as_str() {
      //...
    }
    ```

5. Add a parser for your module configuration (if necessary)

    Move to ```src/config/mod.rs```

    Here add the keys you need to ```PromptConfig```.
    Add parser in ```PromptConfig.parse_config()``` and default values in ```PromptConfig.default()```

---

Thank you for any contribution! 💗  
Christian Visintin
