<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { invoke } from '@tauri-apps/api/core'
import type { MouseConfig } from './types'

const mouseConfig = shallowRef<MouseConfig>()

onMounted(async () => {
  invoke<MouseConfig>('read_mouse_config')
    .then(config => mouseConfig.value = config)
    .catch(err => console.log(err))

  getCurrentWindow()
    .show()
})

// onMounted(async () => {
//   const window = getCurrentWindow()
//   window.show()

//   try {
//     const mouseConfig = await invoke('read_mouse_config')
//     console.log(mouseConfig)

//     const battery = await invoke<{ charging: boolean, level: number }>('get_mouse_battery')
//     console.log(battery)

//     const version = await invoke<string>('get_dongle_version')
//     console.log(version)
//   }
//   catch (err) {
//     console.log(err)
//   }
// })

// const handleInputReport = async (event: HIDInputReportEvent) => {
//   if (event.reportId !== reportId) return

//   const data = new Uint8Array(event.data.buffer)
//   const slice = data.slice(5, 15)

//   switch (data[0]) {
//     case commands.BatteryLevel:
//       deviceData.batteryCharging = slice[1] === 1
//       deviceData.batteryVoltage = ((slice[2] || 0) << 8) + (slice[3] || 0)
//       // deviceData.batteryLevel = slice[0
//       deviceData.batteryLevel = voltageToBatteryLevel(deviceData.batteryVoltage)
//       break
//     case commands.GetDongleVersion:
//       deviceData.version = `${slice[0]}.${slice[1]?.toString(16).padStart(2, '0')}`
//       console.log(slice)
//       break
//     case commands.ReadFlashData:
//       {
//         const addr = (data[2] || 0 << 8) + (data[3] || 0)

//         for (let index = 0; index < (data[4] || 0); index++) {
//           flashData[addr + index] = slice[index] || 0
//         }
//       }
//       break
//     default:
//       break
//   }
// }

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

// const getDevice = async () => {
//   // invoke('set_current_dpi_index', { index: 1 })

//   await requestDevice()
//   if (!device.value) return

//   device.value.oninputreport = handleInputReport
//   // // reading.value = true

//   // // await writeDeviceEeprom(device.value, commands.PCDriverStatus, 0, [1])
//   // await writeDeviceEeprom(device.value, commands.GetDongleVersion, 0, [])
//   // // await writeDeviceEeprom(device.value, commands.BatteryLevel, 0, [])

//   // // await sleep(50)
//   // await readDeviceFullEeprom(device.value)
//   // parseReadDeviceEeprom()

//   // reading.value = false
//   read.value = true
// }
</script>

<template>
  <UApp>
    <main class="flex flex-col p-4 space-y-4 min-h-screen">
      <div
        v-if="mouseConfig"
        class="grow flex items-center gap-4 justify-around"
      >
        <TheKeys />

        <div class="flex flex-col gap-4">
          <VSuspense>
            <TheBattery />
          </VSuspense>

          <TheDpiSelect
            :current-dpi-index="mouseConfig.current_dpi_index"
            :dpi-values="mouseConfig.dpi_values"
            :max-dpi-index="mouseConfig.max_dpi_index"
            :tooltip="{
              open: true,
            }"
          />
        </div>
      </div>
    </main>
  </UApp>
</template>
