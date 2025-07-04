<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import type { MouseConfig } from './types'

const mouseConfig = shallowRef<MouseConfig>()

onMounted(async () => {
  invoke<MouseConfig>('read_mouse_config')
    .then(config => mouseConfig.value = config)
    .catch(err => console.log(err))

  getCurrentWindow().show()
})

// const parseReadDeviceEeprom = () => {
//   deviceData.maxDpi = flashData[mouseEepromAddr.MaxDPI] || 0
//   deviceData.currentDpiIndex = flashData[mouseEepromAddr.CurrentDPI] || 0
//   deviceData.reportRate
//     = (flashData[0] || 0) > 0x10 ? ((flashData[0] || 0) / 0x10) * 2000 : 1000 / (flashData[0] || 0)

//   // deviceData.dpiEffect.mode = flashData[mouseEepromAddr.DPIEffectMode] || 0
//   // deviceData.dpiEffect.brightness = flashData[mouseEepromAddr.DPIEffectBrightness] || 0
//   // deviceData.dpiEffect.speed = flashData[mouseEepromAddr.DPIEffectSpeed] || 0
//   // deviceData.dpiEffect.state = flashData[mouseEepromAddr.DPIEffectState] || 0

//   // deviceData.lightEffect.mode = flashData[mouseEepromAddr.LightEffectMode] || 0
//   // deviceData.lightEffect.brightness = flashData[mouseEepromAddr.LightEffectBrightness] || 0
//   // deviceData.lightEffect.speed = flashData[mouseEepromAddr.LightEffectSpeed] || 0
//   // deviceData.lightEffect.state = flashData[mouseEepromAddr.LightEffectState] || 0
//   // deviceData.lightEffect.time = flashData[mouseEepromAddr.LightEffectTime] || 0
//   // deviceData.lightEffect.color = flashData[mouseEepromAddr.LightEffectColor] || 0

//   // deviceData.debounceTime = flashData[mouseEepromAddr.DebounceTime] || 0
//   // deviceData.sensor.motionSync = flashData[mouseEepromAddr.MotionSync] || 0
//   // deviceData.sensor.sleepTime = flashData[mouseEepromAddr.SleepTime] || 0
//   // deviceData.sensor.angle = flashData[mouseEepromAddr.Angle] || 0
//   // deviceData.sensor.ripple = flashData[mouseEepromAddr.Ripple] || 0
//   // deviceData.sensor.performance = flashData[mouseEepromAddr.Ripple] || 0
//   // deviceData.sensor.mode = flashData[mouseEepromAddr.SensorMode] || 0
//   // deviceData.sensor.lod = flashData[mouseEepromAddr.LOD] || 0

//   // deviceData.movingOffLight = flashData[mouseEepromAddr.MovingOffLight] || 0
// }
</script>

<template>
  <UApp>
    <main class="flex flex-col gap-4 p-4 min-h-screen h-screen max-h-screen overflow-hidden">
      <AppSuspense>
        <AppSettingsMinimizeToTray />
      </AppSuspense>

      <div
        v-if="mouseConfig"
        class="grow flex items-center justify-center gap-14 overflow-hidden max-w-4xl w-full mx-auto"
      >
        <TheKeys />

        <div class="flex flex-col gap-4 flex-1 basis-1/4">
          <AppSuspense>
            <TheBattery />
          </AppSuspense>

          <TheDpiSelect
            :current-dpi-index="mouseConfig.current_dpi_index"
            :dpi-values="mouseConfig.dpi_values"
            :max-dpi-index="mouseConfig.max_dpi_index"
            :tooltip="{
              open: true,
            }"
          />

          <TheSleepTime :sleep-time="mouseConfig.sleep_time" />
        </div>
      </div>
    </main>
  </UApp>
</template>
