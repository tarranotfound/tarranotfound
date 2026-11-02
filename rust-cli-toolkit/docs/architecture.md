# Architecture

The application keeps its terminal interface and individual utilities in separate modules.

| Module | Responsibility |
| --- | --- |
| `main` | Owns the main menu loop and session history |
| `menu` | Prints the available menu options |
| `calculator` | Performs arithmetic and records successful calculations |
| `converter` | Converts temperatures between Celsius and Fahrenheit |
| `text_tools` | Reverses and counts text |
| `validation` | Reads terminal input and parses numeric values |
| `history` | Stores and displays calculations for the current session |

The modules communicate through small functions. The calculator receives mutable history explicitly, while the other tools only need the input helpers.