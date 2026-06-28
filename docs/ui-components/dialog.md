# Dialog

A modal dialog built on `reka-ui`'s `DialogRoot`.

## Props

| Prop | Type | Default | Description |
|---|---|---|---|
| `showClose` | `boolean` | `true` | Show the close (X) button |
| `class` | `string` | — | Additional classes on the content panel |

## Slots

| Slot | Description |
|---|---|
| `trigger` (named) | Element that opens the dialog |
| `default` | Dialog body content |

## Usage

```vue
<Dialog>
  <template #trigger>
    <Button>Open Dialog</Button>
  </template>
  <h2 class="text-lg font-semibold">Dialog Title</h2>
  <p class="text-sm text-muted-foreground">Dialog content here...</p>
</Dialog>
```
