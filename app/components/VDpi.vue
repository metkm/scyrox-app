<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { DpiValue } from '~/constants'
import { setDpiIndex, setDpiValue } from '~/device'
import { useDevice } from '~/hooks/useDevice'

const props = defineProps<{
  currentDpiIndex: number
  dpiValues: DpiValue[]
  maxDpiIndex: number
}>()

const { device } = useDevice()

const dpiValue = ref<DpiValue>(props.dpiValues[props.currentDpiIndex] || { color: '#FFFFFF', value: 1000 })
const dpiIndex = ref(props.currentDpiIndex || 0)

const values = computed(() => props.dpiValues.slice(0, props.maxDpiIndex))

const updateDpiIndex = async (dpi: DpiValue, index: number) => {
  if (!device.value) return

  dpiValue.value = dpi
  dpiIndex.value = index

  await invoke('set_current_dpi_index', { index })
  // await setDpiIndex(device.value, index)
}

const handleChange = async () => {
  await setDpiValue(device.value, dpiIndex.value, dpiValue.value.value)
}
</script>

<template>
  <VContainer title="Dpi value">
    <USlider
      v-model="dpiValue.value"
      :default-value="values[currentDpiIndex]?.value"
      :min="0"
      :max="values[maxDpiIndex - 1]?.value"
      tooltip
      @change="handleChange"
    />

    <ol class="flex items-center gap-4">
      <li
        v-for="(dpi, index) in values"
        :key="dpi.value"
      >
        <UButton
          :variant="dpi.value === dpiValue?.value ? 'solid' : 'soft'"
          size="lg"
          @click="updateDpiIndex(dpi, index)"
        >
          {{ dpi.value }}
        </UButton>
      </li>
    </ol>
  </VContainer>
</template>
