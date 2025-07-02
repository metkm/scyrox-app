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
  <VContainer title="Battery">
    <p class="font-medium text-primary">
      {{ `${battery.level}%${battery.charging ? ' (charging)' : ''}` }}
    </p>

    <div class="-mb-4 -mx-4">
      <div
        class="h-0.5 bg-primary "
        :style="{ width: `${battery.level}%` }"
      />
    </div>
  </VContainer>
</template>
