# Input

A styled text input with `v-model` support.

## Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `modelValue` | `string` | — | Bound value |
| `class` | `string` | — | Additional classes |

## Events

| Event | Payload | Description |
|---|---|---|
| `update:modelValue` | `string` | Emitted on input |

## Usage

```vue
<Input v-model="searchQuery" placeholder="Search..." />
```
