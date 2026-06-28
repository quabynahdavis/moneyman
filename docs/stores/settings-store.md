# Settings Store (`useSettingsStore`)

Application preferences persisted to `localStorage`.

## Defaults

```ts
{
  baseCurrency: "USD",
  defaultAccountId: null,
  reconcileDefaultAccountId: null,
  autoSaveInterval: 60,
  showScheduledReminders: true,
  priceQuoteProvider: "manual",
  priceQuoteApiKey: "",
  dateFormat: "YYYY-MM-DD",
  numberFormat: "1,234.56",
  showInactiveAccounts: false,
  enableMultiCurrency: true,
  enableInvestments: true,
  enableInvoicing: true,
}
```

## Methods

| Method | Description |
|---|---|
| `updateSetting(key, value)` | Update single key |
| `updateSettings(partial)` | Merge partial object |
| `resetSettings()` | Restore defaults |
