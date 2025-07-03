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
  <UCard>
    <template #header>
      <p class="font-medium text-primary">
        {{ `${battery.level}%${battery.charging ? ' (charging)' : ''}` }}
      </p>
    </template>

    <div class="bg-elevated rounded-full overflow-hidden">
      <div
        class="h-1 rounded-full bg-primary"
        :style="{ width: `${battery.level}%` }"
      />
    </div>
  </UCard>
</template>
