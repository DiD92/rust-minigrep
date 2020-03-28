# minigrep - Rust grep parser tutorial tool

The original source for the tutorial can be found [here](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

This version has an additional enhancement, it accepts an extra parameter to control wherever we ignore casing or not. Use it like this:

`cargo run <search-query> <file> [I|K]`

For example:

`cargo run to poem.txt I`

If the casing parameter is not present, `mingrep` will search for an environment variable called `IGNORE_CASE`, the default behaviour is to keep the casing information on search.