<script setup lang="ts">
import type { DropdownMenuItem } from '@nuxt/ui'

const items: DropdownMenuItem[] = [
  {
    label: 'Buttons',
    children: [
      {
        label: 'Left button',
      },
      {
        label: 'Right button',
      },
      {
        label: 'Middle button',
      },
      {
        label: 'Back button',
      },
      {
        label: 'Forward button',
      },
    ],
  },
  {
    label: 'Media key',
    children: [
      {
        label: 'Next',
      },
      {
        label: 'Previous',
      },
      {
        label: 'Play / pause',
      },
      {
        label: 'Stop',
      },
      {
        label: 'Mute',
      },
      {
        label: 'Volume+',
      },
      {
        label: 'Volume-',
      },
    ],
  },
]
</script>

<template>
  <div class="background bg-no-repeat w-[295px] aspect-[295/557] relative">
    <UDropdownMenu
      :items="items"
      :ui="{
        content: 'w-48',
      }"
      class="absolute -translate-x-4 top-24 right-0"
    >
      <UButton>
        Right button
      </UButton>
    </UDropdownMenu>
  </div>
</template>

<style>
.background {
  background-image: url('~/assets/mouse-top.png');
}
</style>
