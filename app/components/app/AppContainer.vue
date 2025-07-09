<script setup lang="ts">
defineProps<{
  title?: string
  icon?: string
}>()
</script>

<template>
  <section class="flex flex-col gap-4 p-4 border border-(--ui-border) rounded bg-elevated/10">
    <div class="flex items-center gap-2">
      <UIcon
        v-if="icon"
        :name="icon"
        class="size-5"
      />

      <h1 class="font-medium text-sm">
        {{ title }}
      </h1>
    </div>

    <div>
      <slot />
    </div>
  </section>
</template>
