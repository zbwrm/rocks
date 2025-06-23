# rocks
Terminal dice-roller. `rocks` is currently undergoing a lot of change as I rewrite it to lex, parse, and evaluate a dice expression properly.

## Syntax
- `XdY`: Rolls `X` `Y`-sided dice. If no `X` is present, one die is rolled.
  - `XdY{kh/kl}Z` keeps the highest or lowest `Z` dice, respectively.
- Math operators: `+`, `-`, `*`, `/`.
- Parentheses.
- Equality operators: `=`, `!=`, `<=`, `>=`, `<`, `>`.
