# Normal Balances

Account types have a "normal balance" side:

| Account Type | Normal Balance |
|---|---|
| ASSET, BANK, CASH, INVESTMENT, STOCK, MUTUAL_FUND, RECEIVABLE, EXPENSE | Debit |
| LIABILITY, CREDIT_CARD, PAYABLE, EQUITY, INCOME, ROOT | Credit |

The `getAccountNormalBalance()` type guard returns the expected side. This is used by `computeRunningBalance()` to determine whether changes are added or subtracted from the running total.
