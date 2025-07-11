<script setup lang="ts">
import type { DropdownMenuItem } from '@nuxt/ui'
import { invoke } from '@tauri-apps/api/core'
import { multimediaKeys } from '~/constants'

const props = defineProps<{
  keyIndex: number
}>()

const selectedLabel = ref()

const handleKeyUpdate = async (label: string, value: number[]) => {
  selectedLabel.value = label

  await invoke('set_key', {
    index: props.keyIndex,
    value,
  })
}

const buttonLabels = ['Left button', 'Right button', 'Middle button', 'Back button', 'Forward button']
const multimediaLabels = ['Play / Pause', 'Next', 'Previous', 'Stop', 'Volume+', 'Volume-']

const buttonItems = computed(() => {
  return buttonLabels.map((label, index) => {
    return {
      label,
      onSelect: () => handleKeyUpdate(label, [1, 1 << index, 0]),
      checked: selectedLabel.value === label,
      type: 'checkbox',
    }
  }) as DropdownMenuItem[]
})

const multimediaItems = computed(() => {
  return multimediaLabels.map((label, index) => {
    return {
      label,
      onSelect: async () => {
        const multiKey = multimediaKeys[index]

        if (multiKey) {
          await invoke('set_key_multimedia', {
            index: props.keyIndex,
            value: multiKey,
          })
        }

        handleKeyUpdate(label, [5, 0, 0])
      },
      checked: selectedLabel.value === label,
      type: 'checkbox',
    }
  }) as DropdownMenuItem[]
})

const items = computed(() => {
  return [
    {
      label: 'Buttons',
      children: buttonItems.value,
    },
    {
      label: 'Media key',
      children: multimediaItems.value,
    },
    {
      label: 'Disabled',
      onSelect: () => handleKeyUpdate('Disabled', [0, 0, 0]),
      checked: selectedLabel.value === 'Disabled',
      type: 'checkbox',
    },
  ] satisfies DropdownMenuItem[]
})
</script>

<template>
  <UDropdownMenu
    :items="items"
    class="absolute"
  >
    <slot :selected-label="selectedLabel" />
  </UDropdownMenu>
</template>
