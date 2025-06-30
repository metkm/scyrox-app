<script setup lang="ts">
import type { DropdownMenuItem } from '@nuxt/ui'
import { setKeyFunction } from '~/device'
import { useDevice } from '~/hooks/useDevice'

const props = defineProps<{
  keyIndex: number
}>()

const { device } = useDevice()

const selectedLabel = ref()

const handleKeyUpdate = async (label: string, value: number[]) => {
  selectedLabel.value = label

  // multimedia key
  if (value[0] === 5) {
    console.log('multimedia key')
  }
  else {
    await setKeyFunction(device.value, props.keyIndex, value)
  }
}

const buttonLabels = ['Left button', 'Right button', 'Middle button', 'Back button', 'Forward button']
const multimediaLabels = ['Next', 'Previous', 'Play / pause', 'Stop', 'Volume+', 'Volume-']

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
  return multimediaLabels.map((label) => {
    return {
      label,
      onSelect: () => handleKeyUpdate(label, [5, 0, 0]),
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
