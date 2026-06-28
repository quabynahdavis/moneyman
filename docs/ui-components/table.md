# Table

Six components that compose a semantic HTML table:

- `Table` — `<table>` wrapper
- `TableHeader` — `<thead>`
- `TableBody` — `<tbody>`
- `TableRow` — `<tr>` (supports `selected` prop for highlighting)
- `TableHead` — `<th>` with muted text styling
- `TableCell` — `<td>`

## Usage

```vue
<Table>
  <TableHeader>
    <TableRow>
      <TableHead>Date</TableHead>
      <TableHead>Payee</TableHead>
      <TableHead class="text-right">Amount</TableHead>
    </TableRow>
  </TableHeader>
  <TableBody>
    <TableRow v-for="row in rows" :key="row.id">
      <TableCell>{{ row.date }}</TableCell>
      <TableCell>{{ row.payee }}</TableCell>
      <TableCell class="text-right">{{ row.amount }}</TableCell>
    </TableRow>
  </TableBody>
</Table>
```

## Row Selection

`TableRow` accepts a `selected` boolean prop that applies `data-[state=selected]:bg-muted`.
