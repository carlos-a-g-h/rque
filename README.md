# rQUE

## What is rQUE?

rQUE is an HTTP web server that stores data in memory in the form of List of List of Strings, created with the purpose of dealing with waiting lists and queues

rQUE is written in Rust and it uses the blazingly fast Actix web framework

## Disclaimer

This program stores data in memory but it is meant for temporary uses. If you need a real non-SQL-like database for non-related, persistent and unstructured data, use real-deal database software such as Redis or MongoDB

## How to use from source

This repo is not structured as a cargo project, so

```
$ bash install.sh

$ cd rque

$ cargo run
```

If you want to build from source, you can take a look at the workflow used by this repo [here](.github/workflows/releaser.yml)

- How Linux binaries are built: [here](.github/workflows/releaser.yml#L83)

- How Windows binaries are built: [here](.github/workflows/releaser.yml#L97)

## Documentation

The documentation is in the 'help.html' file. The docs are also embedded in the program

You can read the changelog [here](CHANGELOG.md)

## TODO list

- ~~Adding/Removing multiple items~~

- ~~Basic authentication~~

- ~~Split into separate modules and organize the code~~

- Turn this noob code into pro code
