<script setup lang="ts">
import type { DropdownMenuItem } from '@nuxt/ui'
import { multimediaKeys } from '~/constants'
import { setKeyFunction, setMultimedia } from '~/device'
import { useDevice } from '~/hooks/useDevice'

const props = defineProps<{
  keyIndex: number
}>()

const { device } = useDevice()

const selectedLabel = ref()

const handleKeyUpdate = async (label: string, value: number[]) => {
  selectedLabel.value = label

  await setKeyFunction(device.value, props.keyIndex, value)
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
          await setMultimedia(device.value, props.keyIndex, multiKey)
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
    class="absolute -translate-x-4 top-24 right-0"
  >
    <slot :selected-label="selectedLabel" />
  </UDropdownMenu>
</template>
