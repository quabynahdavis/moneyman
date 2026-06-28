<script setup lang="ts">
import { cn } from "@/lib/utils"
import { computed, type HTMLAttributes } from "vue"

const props = defineProps<{ class?: HTMLAttributes["class"] }>()
const computedClass = computed(() => cn("[&_tr]:border-b", props.class))
</script>

<template>
  <thead :class="computedClass">
    <slot />
  </thead>
</template>
