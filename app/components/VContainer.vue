<script setup lang="ts">
defineProps<{
  title?: string
}>()
</script>

<template>
  <section class="flex flex-col gap-4 bg-elevated/50 p-4 rounded overflow-hidden">
    <h1
      v-if="title"
      class="font-medium text-sm"
    >
      {{ title }}
    </h1>

    <slot />
  </section>
</template>
