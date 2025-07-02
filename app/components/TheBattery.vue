<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import type { Battery } from '~/types'

const battery = await invoke<Battery>('get_mouse_battery')
</script>

<template>
  <VContainer title="Battery">
    <p class="font-medium text-primary">
      {{ `${battery.level}%${battery.charging ? ' (charging)' : ''}` }}

      <!-- {{ battery.level.toFixed(2) }}% -->
    </p>

    <div class="-mb-4 -mx-4">
      <div
        class="h-0.5 bg-primary "
        :style="{ width: `${battery.level}%` }"
      />
    </div>
  </VContainer>
</template>
