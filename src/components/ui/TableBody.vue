<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() => cn("[&_tr:last-child]:border-0", props.class))
</script>

<template>
  <tbody :class="computedClass">
    <slot />
  </tbody>
</template>
