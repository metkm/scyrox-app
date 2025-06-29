<script setup lang="ts">
import { getCurrentWindow } from '@tauri-apps/api/window'
import { onMounted, reactive, ref } from 'vue'
import { getDevice, readDeviceEeprom, readDeviceFullEeprom } from './device'
import { commands, mouseEepromAddr, reportId } from './constants'
import { bufferToColor, sleep, voltageToBatteryLevel } from './utils'

const flashData: number[] = []

const reading = ref(false)

const deviceData = reactive({
  version: '',
  batteryLevel: 0,
  batteryCharging: false,
  batteryVoltage: 0,
  dpiValues: [] as { value: number, color: string }[],
  currentDpiIndex: 0,
  maxDpi: 0,
  reportRate: 1000,
  dpiEffect: {
    mode: 0,
    brightness: 0,
    speed: 0,
    state: 0,
  },
  lightEffect: {
    mode: 0,
    color: 0,
    speed: 0,
    brightness: 0,
    state: 0,
    time: 0,
  },
  debounceTime: 0,
  sensor: {
    motionSync: 0,
    sleepTime: 0,
    angle: 0,
    ripple: 0,
    performance: 0,
    mode: 0,
  },
  movingOffLight: 0,
})

onMounted(() => {
  const window = getCurrentWindow()
  window.show()
})

const handleInputReport = async (event: HIDInputReportEvent) => {
  if (event.reportId !== reportId) return

  const data = new Uint8Array(event.data.buffer)
  const slice = data.slice(5, 15)

  switch (data[0]) {
    case commands.BatteryLevel:
      deviceData.batteryCharging = slice[1] === 1
      deviceData.batteryVoltage = ((slice[2] || 0) << 8) + (slice[3] || 0)
      // deviceData.batteryLevel = slice[0]
      deviceData.batteryLevel = voltageToBatteryLevel(deviceData.batteryVoltage)
      break
    case commands.GetDongleVersion:
      deviceData.version = `${slice[0]}.${slice[1]?.toString(16).padStart(2, '0')}`
      break
    case commands.ReadFlashData: {
      const addr = (data[2] || 0 << 8) + (data[3] || 0)

      for (let index = 0; index < (data[4] || 0); index++) {
        flashData[addr + index] = slice[index] || 0
      }
    }
      break
    default:
      break
  }
}

const parseReadDeviceEeprom = () => {
  deviceData.maxDpi = flashData[mouseEepromAddr.MaxDPI] || 0
  deviceData.currentDpiIndex = flashData[mouseEepromAddr.CurrentDPI] || 0
  deviceData.reportRate = (flashData[0] || 0) > 0x10 ? ((flashData[0] || 0) / 0x10) * 2000 : 1000 / (flashData[0] || 0)

  // fill dpi values
  for (let index = 0; index < 8; index++) {
    const dpiAddr = index * 4 + mouseEepromAddr.DPIValue

    const lowBits = flashData[dpiAddr] || 0
    const highBits = flashData[dpiAddr + 2] || 0

    const masked = highBits * mouseEepromAddr.DPIValue
    const high = masked >> 2

    let value = lowBits + (high << 8)
    value = (value + 1) * 50

    deviceData.dpiValues[index] = {
      value,
      color: bufferToColor(flashData.slice(dpiAddr + mouseEepromAddr.DPIColor, dpiAddr + mouseEepromAddr.DPIColor + 3)),
    }
  }
  // fill dpi values end

  deviceData.dpiEffect.mode = flashData[mouseEepromAddr.DPIEffectMode] || 0
  deviceData.dpiEffect.brightness = flashData[mouseEepromAddr.DPIEffectBrightness] || 0
  deviceData.dpiEffect.speed = flashData[mouseEepromAddr.DPIEffectSpeed] || 0
  deviceData.dpiEffect.state = flashData[mouseEepromAddr.DPIEffectState] || 0

  deviceData.lightEffect.mode = flashData[mouseEepromAddr.LightEffectMode] || 0
  deviceData.lightEffect.brightness = flashData[mouseEepromAddr.LightEffectBrightness] || 0
  deviceData.lightEffect.speed = flashData[mouseEepromAddr.LightEffectSpeed] || 0
  deviceData.lightEffect.state = flashData[mouseEepromAddr.LightEffectState] || 0
  deviceData.lightEffect.time = flashData[mouseEepromAddr.LightEffectTime] || 0
  deviceData.lightEffect.color = flashData[mouseEepromAddr.LightEffectColor] || 0

  deviceData.debounceTime = flashData[mouseEepromAddr.DebounceTime] || 0
  deviceData.sensor.motionSync = flashData[mouseEepromAddr.MotionSync] || 0
  deviceData.sensor.sleepTime = flashData[mouseEepromAddr.SleepTime] || 0
  deviceData.sensor.angle = flashData[mouseEepromAddr.Angle] || 0
  deviceData.sensor.ripple = flashData[mouseEepromAddr.Ripple] || 0
  deviceData.sensor.performance = flashData[mouseEepromAddr.Ripple] || 0
  deviceData.sensor.mode = flashData[mouseEepromAddr.SensorMode] || 0

  deviceData.movingOffLight = flashData[mouseEepromAddr.MovingOffLight] || 0
}

const requestDevice = async () => {
  const device = await getDevice()
  if (!device) return

  if (!device.opened) {
    await device.open()
  }

  device.oninputreport = handleInputReport
  reading.value = true

  await readDeviceEeprom(device, commands.PCDriverStatus, 0, [1])
  await readDeviceEeprom(device, commands.GetDongleVersion, 0, [])
  await readDeviceEeprom(device, commands.BatteryLevel, 0, [])

  await sleep(50)
  await readDeviceFullEeprom(device)
  parseReadDeviceEeprom()

  reading.value = false
}
</script>

<template>
  <main class="p-4">
    <UButton
      :loading="reading"
      @click="requestDevice"
    >
      Request Device
    </UButton>

    <pre>
      {{ JSON.stringify(deviceData, undefined, 2) }}
    </pre>
  </main>
</template>

<style>
@import "tailwindcss";
@import "@nuxt/ui";
</style>
