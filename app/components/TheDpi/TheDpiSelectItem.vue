<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { DpiValue } from '~/types'

const props = defineProps<{
  dpi: DpiValue
  index: number
  selected: boolean
}>()

const emit = defineEmits<{
  click: []
}>()

const dpiValue = ref(props.dpi.value)
const dpiColor = ref(props.dpi.color)

const open = ref(false)

const updateDpiIndex = async () => {
  emit('click')
  await invoke('set_current_dpi_index', { index: props.index })
}

const updateDpiValue = async () => {
  await invoke('update_dpi_value', {
    index: props.index,
    value: dpiValue.value,
  })
}
</script>

<template>
  <div class="flex flex-col gap-2">
    <UButton
      :variant="selected ? 'solid' : 'soft'"
      block
      @click="updateDpiIndex"
      @dblclick="open = true"
    >
      {{ dpiValue }}
    </UButton>

    <div
      class="h-1 w-full rounded-full"
      :style="{ backgroundColor: dpiColor }"
    />

    <UModal
      v-model:open="open"
      title="Change Dpi"
      description="updates the dpi value in this index"
      close
      @after:leave="updateDpiValue"
    >
      <template #body>
        <UFormField
          label="Dpi Value"
          help="Specify number of attempts"
          required
        >
          <UInputNumber
            v-model="dpiValue"
            placeholder="Enter retries"
          />
        </UFormField>
      </template>
    </UModal>
  </div>
</template>
