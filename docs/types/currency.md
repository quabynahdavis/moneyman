# Currency Types

## `Currency`

```ts
interface Currency {
  code: string        // ISO 4217
  name: string
  symbol: string
  decimalPlaces: number
}
```

## `ExchangeRate`

```ts
interface ExchangeRate {
  id?: number
  fromCurrency: string
  toCurrency: string
  rate: string        // Decimal string
  date: string
  source: "manual" | "api_fetch"
}
```

## Default Currencies

The `DEFAULT_CURRENCIES` constant includes USD, EUR, GBP, JPY, CHF, CAD, AUD, INR, BRL, and CNY with their correct decimal places (JPY = 0, all others = 2).
