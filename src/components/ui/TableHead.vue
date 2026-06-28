<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() =>
  cn(
    "h-12 px-4 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0",
    props.class,
  ),
)
</script>

<template>
  <th :class="computedClass">
    <slot />
  </th>
</template>
