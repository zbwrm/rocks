# rocks
Terminal dice-roller.

## Syntax
`rocks` is built on operators and numbers. A valid dice sequence consists of alternating numbers and operators, beginning and ending with numbers (such that there will always be one more number than operators).

For example, `2d10-4>6` decomposes into `2, d, 10, -, 4, >, 6`.

Valid control characters are:
- `d`
- `+`
- `-` (note: underflow will be capped at 0.)
- `>` (note: this character must be escaped so in practice it's `\>`.)
- `<`
- `=`
