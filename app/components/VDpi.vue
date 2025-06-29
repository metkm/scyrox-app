<script setup lang="ts">
import type { DpiValue } from '~/constants'
import { setDpiIndex } from '~/device'
import { useDevice } from '~/hooks/useDevice'

const props = defineProps<{
  currentDpiIndex: number
  dpiValues: DpiValue[]
  maxDpiIndex: number
}>()

const { device } = useDevice()

const dpiValue = ref<DpiValue>(
  props.dpiValues[props.currentDpiIndex] || { color: '#FFFFFF', value: 1000 },
)

const values = computed(() => props.dpiValues.slice(0, props.maxDpiIndex))

const updateDpiIndex = async (dpi: DpiValue, index: number) => {
  if (!device.value) return

  dpiValue.value = dpi
  await setDpiIndex(device.value, index)
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <USlider
      v-model="dpiValue.value"
      :default-value="values[currentDpiIndex]?.value"
      :min="0"
      :max="values[maxDpiIndex - 1]?.value"
      tooltip
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
  </div>
</template>
