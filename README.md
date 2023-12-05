# Bscript

This project has the main purpose to improve my personal rust skills.
It has a simple syntax with very limited functionality. It is designed just to play
around a little bit with it. But maybe I will find a suitable purpose in real world application in it.

## Usage

```shell
bscript compile test.bs
```

## Standard library

| Keyword | description                | Usage                          |
|---------|----------------------------|--------------------------------|
| `print` | Prints data to stdout      | `print << "Hello World!"`      |
| `init`  | Initializes a new variable | `init << test << int << 69`    |
| `add`   | Adds two numbers           | `add << result << a << b`      |
| `sub`   | Substracts two numbers     | `sub << result << a << b`      |
| `mul`   | Multiplys two numbers      | `mul << result << a << b`      |
| `div`   | Divides two numbers        | `div << result << a << b`      |
| `store` | Reinitializes a value      | `store << variable << "Test!"` |
| `pow`   | Pows two number            | `pow << result << a << b`      |
| `cmp`   | Compares two values        | `cmp << result << a << b`      |