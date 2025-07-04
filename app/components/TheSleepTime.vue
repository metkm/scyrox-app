<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

const props = defineProps<{
  sleepTime: number
}>()

const sleepDurations = [
  10,
  30,
  60,
  300, // 5 min
  600, // 10 min
  1800, // 30 min
]

const selectedDuration = ref(props.sleepTime * 10)

const setPerformanceTime = async (_value: number) => {
  const value = _value / 10

  await invoke('set_performance_time', {
    value,
  })

  selectedDuration.value = _value
}

const secondsToFormat = (value: number) => {
  if (value < 60) {
    return `${value}s`
  }

  const minutes = Math.floor(value / 60)
  const seconds = value - minutes * 60

  let result = `${minutes}m`

  if (seconds > 0) {
    result += ` ${seconds}s`
  }

  return result
}
</script>

<template>
  <AppContainer title="Sleep time">
    <ol class="flex items-center gap-4">
      <li
        v-for="duration in sleepDurations"
        :key="duration"
        class="flex-1"
      >
        <UButton
          :variant="duration === selectedDuration ? 'solid' : 'soft'"
          block
          @click="setPerformanceTime(duration)"
        >
          {{ secondsToFormat(320) }}
        </UButton>
      </li>
    </ol>
  </AppContainer>
</template>
