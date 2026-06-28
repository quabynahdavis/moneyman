# Select

A styled dropdown built on `reka-ui`'s `SelectRoot`.

## Components

- `Select` — Root wrapper (renders trigger + content)
- `SelectItem` — Individual option

## Select Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `modelValue` | `string` | — | Currently selected value |
| `placeholder` | `string` | `"Select..."` | Placeholder text |
| `class` | `string` | — | Additional classes on content |

## SelectItem Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `value` | `string` | (required) | Option value |
| `disabled` | `boolean` | `false` | Disabled state |

## Usage

```vue
<Select v-model="selected">
  <template #label>Account Type</template>
  <SelectItem value="ASSET">Asset</SelectItem>
  <SelectItem value="LIABILITY">Liability</SelectItem>
  <SelectItem value="INCOME">Income</SelectItem>
</Select>
```
