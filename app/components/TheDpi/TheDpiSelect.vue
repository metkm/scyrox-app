<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { DpiValue } from '~/types'

const props = defineProps<{
  currentDpiIndex: number
  dpiValues: DpiValue[]
  maxDpiIndex: number
}>()

const dpiValue = ref<DpiValue>(props.dpiValues[props.currentDpiIndex] || { color: '#FFFFFF', value: 1000 })
const dpiIndex = ref(props.currentDpiIndex || 0)

const values = computed(() => props.dpiValues.slice(0, props.maxDpiIndex))

const updateDpiIndex = async (dpi: DpiValue, index: number) => {
  dpiValue.value = dpi
  dpiIndex.value = index
}

const handleChange = async () => {
  await invoke('update_dpi_value', {
    index: dpiIndex.value,
    value: dpiValue.value.value,
  })
}
</script>

<template>
  <UCard>
    <div class="flex flex-col gap-4">
      <USlider
        v-model="dpiValue.value"
        :default-value="values[currentDpiIndex]?.value"
        :min="0"
        :max="values[maxDpiIndex - 1]?.value"
        tooltip
        @change="handleChange"
      />

      <ol class="flex flex-wrap items-center gap-4">
        <li
          v-for="(dpi, index) in values"
          :key="dpi.value"
          class="grow"
        >
          <TheDpiSelectItem
            :dpi="dpi"
            :index="index"
            :selected="dpi.value === dpiValue.value"
            @click="updateDpiIndex(dpi, index)"
          />
        </li>
      </ol>
    </div>
  </UCard>
</template>
