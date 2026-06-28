# Card

The Card component is split into 6 sub-components for maximum flexibility:

- `Card` — Root container with border, background, and shadow
- `CardHeader` — Top padding container (usually holds `CardTitle` and `CardDescription`)
- `CardTitle` — `<h3>` heading
- `CardDescription` — Muted `<p>` subtitle
- `CardContent` — Main content area
- `CardFooter` — Bottom action area

## Usage

```vue
<Card>
  <CardHeader>
    <CardTitle>Account Balance</CardTitle>
    <CardDescription>Current checking account</CardDescription>
  </CardHeader>
  <CardContent>
    <p class="text-2xl font-bold">$12,450.89</p>
  </CardContent>
  <CardFooter>
    <Button>View Transactions</Button>
  </CardFooter>
</Card>
```

All sub-components accept a `class` prop for additional styling.
