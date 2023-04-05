# rQUE

## What is rQUE?

A simple REST-like in-memory data storage server. Made specifically for working with waitlists and queues

## Features

- HTTP web server [rQUE](_) written in blazingly FAF Rust and using the Actix Web framework
- Python client module [aiorque](aiorque) that uses aiohttp.Client under the hood + some examples

## Disclaimer

This program stores data in memory but for temporary uses, it is not database oriented. If you need a real non-SQL-like database for non-related, persistent and unstructured data, use real-deal database software such as Redis or MongoDB

## How to use from source

This repo is not structured as a cargo project, so

```
$ bash install.sh

$ cd rque

$ cargo run
```

If you want to build from source, you can take a look at the workflow used by this repo [here](.github/workflows/releaser.yml)

- Linux builds [here](.github/workflows/releaser.yml#L62)

- Windows builds [here](.github/workflows/releaser.yml#L76)

## Documentation

The documentation is [here](help.md)

You can also read the help in your browser while the server is running by accessing the '/help' route

You can read the changelog [here](CHANGELOG.md)

## TODO list

- ~~Adding/Removing multiple items~~

- ~~Basic authentication~~

- Split into separate modules and organize the code
