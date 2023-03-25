# rQUE

## Disclaimer

This is not a database software, if you need a real non-SQL-like database for non-related unstructured data, use real database software such as Redis or MongoDB

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

You can also access the help doing a GET to the '/help' route while the server is running

## TODO list

- Basic authentication
