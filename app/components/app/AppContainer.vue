<script setup lang="ts">
defineProps<{
  title?: string
}>()
</script>

<template>
  <UCard>
    <template
      v-if="title"
      #header
    >
      <h1 class="font-medium">
        {{ title }}
      </h1>
    </template>

    <slot />
  </UCard>
</template>
