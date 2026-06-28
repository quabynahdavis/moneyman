# Decimal Utilities

All financial math uses `decimal.js` with `precision: 40` and `ROUND_HALF_UP`.

## Helpers

| Function | Description |
|---|---|
| `toDecimal(value)` | Convert string/number/Decimal → Decimal |
| `toCents(amount)` | Multiply by 100, return integer string |
| `fromCents(cents)` | Divide by 100, return 2-decimal string |
| `formatMoney(amount, symbol, decimals)` | Comma-separated thousands, fixed decimals |
| `add(a, b)` | a + b |
| `sub(a, b)` | a − b |
| `mul(a, b)` | a × b |
| `div(a, b)` | a ÷ b |
| `abs(a)` | Absolute value |
| `neg(a)` | Negate |
| `isZero(a)` | Check equality to zero |
| `isPositive(a)` | a > 0 |
| `isNegative(a)` | a < 0 |
| `compare(a, b)` | -1, 0, or 1 |
| `gt(a, b) / gte(a, b) / lt(a, b) / lte(a, b)` | Comparison shorthands |

## Why Not Number?

JavaScript's `Number` type uses IEEE 754 double-precision floating point, which cannot represent decimal fractions like `0.10` exactly. Over thousands of transactions, rounding errors accumulate to real penny discrepancies. `decimal.js` uses base-10 arithmetic that matches how financial systems actually work.
