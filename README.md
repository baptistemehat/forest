# Forest - terminal-based project manager

> [!IMPORTANT]  
> This project is in an early development stage and is a work in progress.


## Overview
`forest` is a terminal-based project manager.

Its main focus is to help you organise your work and your time on your projects.
As such, it provides a neat cli to help you:
* manage tasks and priorities accross different projects
* track the time you spend on each task or project
* analyse your time-tracking records to better understand where you spend your time
* take notes on your work to save your thoughts and ideas

## Install
Forest is still a work in progres and does not provide pre-built packages yet.
To install `forest`, you will need to build from source using the rust toolchain:

1) Install the rust toolchain via `rustup`:

follow the instructions presented here: [https://rustup.rs/](https://rustup.rs/)

2) Clone this repo:
```sh
git clone https://github.com/baptistemehat/forest.git
```

3) Build and install forest:
```sh
cd forest
cargo install --path .
```

4) Check that `forest` is properly installed:
```
forest --version
```

## Status
This project is in an early development stage and is a work in progress.

## Motivations
I started this project for three main resons:

* I'm currently learning Rust and I wanted to practice this new language on a cli project, which Rust is supposed to be good at.
* I've tested a few tools to organise my tasks, track my time and take notes for my personal projects but I haven't yet found a single tool that suits all my needs at once. So, why not create my own!
* I simply like the terminal as a user interface and wanted to explore the process of designing modern cli apps.



## Take notes
- `forest note`
- `forest diary`
