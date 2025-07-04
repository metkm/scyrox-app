<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'

const _minimizeToTray = await invoke<boolean>('get_minimize_to_tray')

const minimizeToTray = ref(_minimizeToTray)

const handleMinimizeUpdate = async () => {
  await invoke('set_minimize_to_tray', {
    value: minimizeToTray.value,
  })
}
</script>

<template>
  <USwitch
    v-model="minimizeToTray"
    label="Minimize to tray when closed"
    @change="handleMinimizeUpdate"
  />
</template>
