# Button

A polymorphic button component based on `reka-ui`'s `Primitive`.

## Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `variant` | `"default" \| "destructive" \| "outline" \| "secondary" \| "ghost" \| "link"` | `"default"` | Visual style |
| `size` | `"default" \| "sm" \| "lg" \| "icon"` | `"default"` | Size preset |
| `as` | `string` | `"button"` | Rendered element (`"a"`, `"button"`, `"div"`, etc.) |
| `asChild` | `boolean` | `false` | Delegate rendering to child element |
| `class` | `string` | — | Additional Tailwind classes |

## Usage

```vue
<Button variant="outline" size="sm" @click="handleClick">
  Click Me
</Button>

<Button as="a" href="https://example.com" variant="link">
  Link Button
</Button>
```
