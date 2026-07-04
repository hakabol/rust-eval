# math-eval

A simple mathematical expression evaluator written in Rust.

## Features

- Addition (`+`)
- Subtraction (`-`)
- Multiplication (`*`)
- Division (`/`)
- Exponentiation (`^`)
- Parentheses
- `sin()`
- `cos()`
- `tan()`
- `ln()`
- `log()` (base 10)

## Installation

```toml
[dependencies]
math-eval = "0.1.0"
```

## Example

```rust
use math_eval::evaluate;

fn main() {
    let result = evaluate("2 + 3 * 4");
    println!("{result}");
}
```

## Supported Expressions

```text
2 + 3 * 4
(2 + 3) * 4
2^10
sin(1)
cos(1)
tan(1)
ln(2)
log(100)
```

## Limitations

- Decimal numbers does not work if you wish to use them you must use fractions.
- Angles for trigonometric functions are in radians.
- Invalid expressions return an error (or panic, depending on your implementation).

## License

MIT
