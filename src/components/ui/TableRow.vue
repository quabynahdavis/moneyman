<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

interface TableRowProps {
  class?: HTMLAttributes["class"]
  selected?: boolean
}
const props = defineProps<TableRowProps>()
const computedClass = computed(() =>
  cn(
    "border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
    props.class,
  ),
)
</script>

<template>
  <tr :class="computedClass" :data-state="selected ? 'selected' : undefined">
    <slot />
  </tr>
</template>
