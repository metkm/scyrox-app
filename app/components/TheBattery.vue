<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { Battery } from '~/types'

const _battery = await invoke<Battery>('get_mouse_battery')

const battery = ref(_battery)
const intervalId = ref()

onMounted(async () => {
  battery.value = await invoke<Battery>('get_mouse_battery')

  intervalId.value = setInterval(async () => {
    battery.value = await invoke<Battery>('get_mouse_battery')
  }, 10_000)
})

onUnmounted(() => {
  clearInterval(intervalId.value)
  intervalId.value = undefined
})
</script>

<template>
  <AppContainer title="Battery">
    <div class="relative flex h-12 -m-6 bg-elevated">
      <div
        class="bg-primary/80"
        :style="{ width: `${battery.level}%` }"
      />

      <div
        class="flex items-center absolute inset-0 px-2"
        :style="{
          paddingLeft: `calc(${battery.level / 2}% + calc(var(--spacing) * 2))`,
        }"
      >
        <p class="text-lg font-medium text-shadow-2xs text-shadow-black -translate-x-1/2">
          {{ `${battery.level}%${battery.charging ? ' (charging)' : ''}` }}
        </p>
      </div>
    </div>
  </AppContainer>
</template>
