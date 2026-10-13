# Numeric behavior

The application currently uses `double` for calculation and prints results with four digits after the decimal point.

Things to keep in mind when extending it:

- Floating-point values are approximations; exact decimal equality is not generally safe.
- Division by zero is checked explicitly.
- Square root rejects negative inputs.
- Power operations can produce non-finite results for some inputs; handling these consistently should be added with tests.
- Formatting is presentation only and should not be reused as the internal numeric value.
