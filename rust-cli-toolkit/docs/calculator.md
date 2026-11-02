# Calculator

The calculator accepts two floating-point values and one operator.

## Supported operators

| Operator | Operation |
| --- | --- |
| `+` | Addition |
| `-` | Subtraction |
| `*` | Multiplication |
| `/` | Division |

The `calculate` function returns a `Result`: successful operations contain the numeric result, while unsupported operators and division by zero return an error message.

Successful calculations are appended to the session history. Failed calculations are not recorded.

## Example

```text
First number: 12
Operator: *
Second number: 3
Result: 36
```