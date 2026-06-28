<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() => cn("text-2xl font-semibold leading-none tracking-tight", props.class))
</script>

<template>
  <h3 :class="computedClass">
    <slot />
  </h3>
</template>
