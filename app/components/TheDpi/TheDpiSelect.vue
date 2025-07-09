<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import type { DpiValue } from '~/types'

const props = defineProps<{
  currentDpiIndex: number
  dpiValues: DpiValue[]
  maxDpiIndex: number
}>()

const dpi = ref<DpiValue>(props.dpiValues[props.currentDpiIndex]!)
const dpiIndex = ref<number>(props.currentDpiIndex || 1)

const dpiList = computed(() => props.dpiValues.slice(0, props.maxDpiIndex))

const updateDpi = async (_dpi: DpiValue, index: number) => {
  dpi.value = _dpi
  dpiIndex.value = index
}

const handleChange = async () => {
  await invoke('update_dpi_value', {
    index: dpiIndex.value,
    value: dpi.value.value,
  })
}

onMounted(() => {
  listen<number>('status-change', async (event) => {
    if (event.payload === 1) {
      dpiIndex.value = await invoke<number>('get_current_dpi_index')
    }
  })
})
</script>

<template>
  <AppContainer
    title="Dpi value"
    icon="i-lucide-circle-ellipsis"
  >
    <div class="flex flex-col gap-4">
      <USlider
        v-model="dpi.value"
        :default-value="dpiList[currentDpiIndex]?.value"
        :min="0"
        :max="dpiList[maxDpiIndex - 1]?.value"
        tooltip
        @change="handleChange"
      />

      <ol class="flex flex-wrap items-center gap-4">
        <li
          v-for="(_dpi, index) in dpiList"
          :key="_dpi.value"
          class="flex-1"
        >
          <TheDpiSelectItem
            :dpi="_dpi"
            :index="index"
            :selected="dpiIndex === index"
            @click="updateDpi(dpi, index)"
          />
        </li>
      </ol>
    </div>
  </AppContainer>
</template>
