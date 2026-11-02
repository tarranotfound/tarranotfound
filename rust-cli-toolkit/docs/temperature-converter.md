# Temperature converter

The converter supports both directions:

- Celsius to Fahrenheit: `F = C × 9 / 5 + 32`
- Fahrenheit to Celsius: `C = (F - 32) × 5 / 9`

The conversion functions accept and return `f64`, so they can be reused independently of the interactive menu. The terminal interface formats the displayed values to two decimal places.

Choose option `1` for Celsius to Fahrenheit or option `2` for Fahrenheit to Celsius.