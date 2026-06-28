import Decimal from "decimal.js"

Decimal.set({ precision: 40, rounding: Decimal.ROUND_HALF_UP })

export function toDecimal(value: string | number | Decimal): Decimal {
  return new Decimal(value)
}

export function toCents(amount: string | number): string {
  return toDecimal(amount).times(100).toFixed(0)
}

export function fromCents(cents: string | number): string {
  return toDecimal(cents).div(100).toFixed(2)
}

export function formatMoney(
  amount: string | number,
  currencySymbol = "$",
  decimals = 2,
): string {
  const d = toDecimal(amount)
  const negative = d.isNegative()
  const abs = d.abs()
  const formatted = abs.toFixed(decimals)
  const parts = formatted.split(".")
  parts[0] = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, ",")
  return `${negative ? "-" : ""}${currencySymbol}${parts.join(".")}`
}

export function add(a: string | number, b: string | number): string {
  return toDecimal(a).plus(b).toString()
}

export function sub(a: string | number, b: string | number): string {
  return toDecimal(a).minus(b).toString()
}

export function mul(a: string | number, b: string | number): string {
  return toDecimal(a).times(b).toString()
}

export function div(a: string | number, b: string | number): string {
  return toDecimal(a).div(b).toString()
}

export function abs(a: string | number): string {
  return toDecimal(a).abs().toString()
}

export function neg(a: string | number): string {
  return toDecimal(a).negated().toString()
}

export function isZero(a: string | number): boolean {
  return toDecimal(a).isZero()
}

export function isPositive(a: string | number): boolean {
  return toDecimal(a).isPositive()
}

export function isNegative(a: string | number): boolean {
  return toDecimal(a).isNegative()
}

export function compare(a: string | number, b: string | number): number {
  return toDecimal(a).comparedTo(b)
}

export function gt(a: string | number, b: string | number): boolean {
  return toDecimal(a).greaterThan(b)
}

export function gte(a: string | number, b: string | number): boolean {
  return toDecimal(a).greaterThanOrEqualTo(b)
}

export function lt(a: string | number, b: string | number): boolean {
  return toDecimal(a).lessThan(b)
}

export function lte(a: string | number, b: string | number): boolean {
  return toDecimal(a).lessThanOrEqualTo(b)
}

export { Decimal }
