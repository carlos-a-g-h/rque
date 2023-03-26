# rQUE

## Features

- HTTP web server [rQUE](_) written in blazingly fast AF Rust using the Actix Web framework
- Python client [aiorque](aiorque) that uses aiohttp.Client

## Disclaimer

This does not count as an actual database software, if you need a real non-SQL-like database for non-related unstructured data, use real-deal database software such as Redis or MongoDB

## Running the server

You can specify a port

```
./rque {PORT}
```

Example: Run in the default port (8080)

```
./rque
```

Example: Run in a specific port

```
./rque 28376
```

If the argument for the port is NaN, the program will fallback to the default port

## API usage and data storage

API usage and data storage reference is explained in the [help.html](help.html) file

You can also read the help in your browser while the server is running by accessing '/help'

## TODO list

- ~~Adding/Removing multiple items~~

- Basic authentication

- Split into separate modules
