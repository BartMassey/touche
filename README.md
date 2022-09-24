# touche: touch command in Rust
Bart Massey 2022

This is a Rust implementation of the `touch` utility. This
version will make any needed directories in the path to the
target file, then touch or create the target file as needed.

Usage is `touche [<path> ...]`. Install with

    cargo install --path .

This program was inspired by
<https://github.com/elliot40404/bonk>, but is effectively
written from scratch by me.

This work is made available under the "MIT License". Please
see the file `LICENSE.txt` in this distribution for license
terms.
