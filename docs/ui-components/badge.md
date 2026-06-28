# Badge

A small label/tag component.

## Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `variant` | `"default" \| "secondary" \| "destructive" \| "outline" \| "success" \| "warning"` | `"default"` | Color variant |
| `class` | `string` | — | Additional classes |

## Usage

```vue
<Badge>Default</Badge>
<Badge variant="success">Reconciled</Badge>
<Badge variant="warning">Cleared</Badge>
<Badge variant="destructive">Void</Badge>
```
