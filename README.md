## flagup

<p align="center">
<img src="https://raw.githubusercontent.com/cthogg/flagup/main/logo.svg" width=320>
</p>

- Have a French friend and want to have a present with their flag but do not know what the French flag is?
- Struggle in the flag round of your local pub quiz?
- Ever at the Olympics, a French Athelete wins and asks to you to find their flag (but are too embarassed to say you do not know what the flag of France is)?

If you answered Yes to any of these questions, then flagup is for you.

Simply type in the name of the country (in English) and intelligent alogorithm will show the correct flag for you!

## Packages

- cli: the command line application.
- libflagup: the library package. Also available from crates.io.

## Installation (mac-os)

- `brew tap cthogg/flagup`
- `brew install flagup`

## Generate package for homebrew

Follow the instructions [here](https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/) or run the command.

```
cargo test && cargo build --release && cd target/release && tar -czf flagup-mac.tar.gz flagup && shasum -a 256 flagup-mac.tar.gz >256-flagup.txt
```

## Features

### Done

1. Create a hashmap of two countries: Germany 🇩🇪 and France 🇫🇷 .
1. When the user types in Germany 🇩🇪 they get back 🇩🇪
1. When the user types in France they get back 🇫🇷.
1. Case insensitive e.g. running flagup
1. Get it running as a homebrew package using https://federicoterzi.com/blog/how-to-publish-your-rust-project-on-homebrew/
1. Has a test for France and Germany.
1. Has a test for when the country does not exist => adds a 🤷‍♂️
1. Use other countries as well (e.g. Andorra)
1. Split up the packages to cli and lib like https://github.com/mitsuhiko/when/commit/194eab88d9b05dc4799166720dff42cdaaabf810
1. Publish to a crate on crates.io with one public function of flagup("Germany") -> 🇩🇪 See https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html
1. Make all tests doc tests.

### TODO

1.
1. Need to change the git commit user name to cthogg and not the christopher-luminovo one
1. Support countries with spaces e.g. (Ascension Island) -> see comment by colleague
1. Use multiple lanugages i.e. Deutschland -> 🇩🇪.
1. Bug: on my vscode terminal runnig it returns a box and not an emoji. Think more of a vscode bug though.
1. Get a webstite running wasm with something similar to https://github.com/mitsuhiko/when.
1. Get a reverse search e.g. 🇩🇪 -> Germany
1. Get a description parse the information to https://en.wikipedia.org/wiki/Flag_of_Germany. (e.g. country with red gold and black flag)
1. Add suggestions => e.g. typing Ger brings up "did you mean Germany"?
1. Automatically publishes to crates.io and homebrew.
