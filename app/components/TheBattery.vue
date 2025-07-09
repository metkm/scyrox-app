<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { clearInterval, setInterval } from 'worker-timers'
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
  <AppContainer
    title="Battery"
    icon="i-lucide-battery"
  >
    <div class="flex flex-col gap-2">
      <p class="font-medium text-sm">
        {{ battery.level }}%
      </p>

      <div class="bg-elevated rounded-full overflow-hidden">
        <div
          class="h-2 bg-primary/80 transition-all"
          :style="{ width: `${battery.level}%` }"
        />
      </div>
    </div>
  </AppContainer>
</template>
