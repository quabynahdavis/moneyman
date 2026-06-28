<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() => cn("p-4 align-middle [&:has([role=checkbox])]:pr-0", props.class))
</script>

<template>
  <td :class="computedClass">
    <slot />
  </td>
</template>
