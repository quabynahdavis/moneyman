<script setup lang="ts">
import { ref, computed } from "vue"
import { Check, ChevronsUpDown } from "@lucide/vue"
import { Button } from "@/components/ui"
import { DialogRoot as Dialog, DialogContent, DialogOverlay, DialogPortal } from "reka-ui"
import {
  Command,
  CommandInput,
  CommandList,
  CommandGroup,
  CommandItem,
  CommandEmpty,
} from "@/components/ui/command"

interface ComboboxItem {
  value: string
  label: string
}

const props = withDefaults(defineProps<{
  modelValue: string
  items: ComboboxItem[]
  placeholder?: string
  searchPlaceholder?: string
  emptyText?: string
}>(), {
  placeholder: "Select...",
  searchPlaceholder: "Search...",
  emptyText: "No results found",
})

const emit = defineEmits<{
  "update:modelValue": [value: string]
}>()

const open = ref(false)

const selectedLabel = computed(() => {
  const item = props.items.find((i) => i.value === props.modelValue)
  return item?.label ?? props.placeholder
})

function onSelect(value: string) {
  emit("update:modelValue", value)
  open.value = false
}
</script>

<template>
  <Dialog :open="open" @update:open="open = $event">
    <Button
      variant="outline"
      role="combobox"
      :aria-expanded="open"
      class="w-full justify-between font-normal"
      @click="open = true"
    >
      {{ selectedLabel }}
      <ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
    </Button>
    <DialogPortal>
      <DialogOverlay class="fixed inset-0 z-50 bg-black/80" />
      <DialogContent
        class="fixed left-1/2 top-1/2 z-50 w-full max-w-sm -translate-x-1/2 -translate-y-1/2 rounded-lg border bg-popover p-0 shadow-lg"
        @escape-key-down="open = false"
        @pointer-down-outside="open = false"
      >
        <Command
          :model-value="modelValue"
          @update:model-value="onSelect($event as string)"
        >
          <div class="p-2 pb-0">
            <CommandInput :placeholder="searchPlaceholder" />
          </div>
          <CommandList>
            <CommandEmpty>{{ emptyText }}</CommandEmpty>
            <CommandGroup>
              <CommandItem
                v-for="item in items"
                :key="item.value"
                :value="item.value"
              >
                <span>{{ item.label }}</span>
                <Check
                  v-if="modelValue === item.value"
                  class="ml-auto h-4 w-4 shrink-0"
                />
              </CommandItem>
            </CommandGroup>
          </CommandList>
        </Command>
      </DialogContent>
    </DialogPortal>
  </Dialog>
</template>
