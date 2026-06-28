<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

interface TableProps {
  class?: HTMLAttributes["class"]
}
const props = defineProps<TableProps>()
const computedClass = computed(() => cn("w-full caption-bottom text-sm", props.class))
</script>

<template>
  <table :class="computedClass">
    <slot />
  </table>
</template>
