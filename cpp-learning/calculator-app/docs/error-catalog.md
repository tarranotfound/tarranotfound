# Error cases to verify

The program reports invalid operations instead of intentionally terminating the input loop.

| Input | Expected behavior |
|---|---|
| `8 / 0` | Report division by zero |
| `sqrt -9` | Report square root of a negative number |
| `8 % 3` | Report an unsupported operator |
| `hello` | Report invalid expression syntax |
| `sqrt` | Show the square-root usage error |
| `2 + 3 extra` | Reject extra tokens |

Use this list as a manual regression checklist. Automated coverage can be expanded as the parser grows.
