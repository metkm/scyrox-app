<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted, reactive, ref } from "vue";
import { getDevice, readDeviceEeprom, readDeviceFullEeprom } from "./device";
import { commands, mouseEepromAddr, reportId } from "./constants";

const flashData: number[] = []

const reading = ref(false)

const deviceData = reactive({
  version: '',
  batteryLevel: 0,
  batteryCharging: false,
  batteryChargingVoltage: 0,
  dpiValues: [] as { value: number, color: string }[]
})

onMounted(() => {
  const window = getCurrentWindow();
  window.show();
});

const handleInputReport = async (event: HIDInputReportEvent) => {
  if (event.reportId !== reportId) return

  const data = new Uint8Array(event.data.buffer)
  const slice = data.slice(5, 15)

  switch (data[0]) {
    case commands.BatteryLevel:
      deviceData.batteryLevel = slice[0]
      deviceData.batteryCharging = slice[1] === 1
      deviceData.batteryChargingVoltage = slice[2] << 8 + slice[3]
      break;
    case commands.GetDongleVersion:
      deviceData.version = `${slice[0]}.${slice[1].toString(16).padStart(2, '0')}`
      break;
    case commands.ReadFlashData:
      const addr = (data[2] << 8) + data[3]
      
      for (let index = 0; index < data[4]; index++) {
        flashData[addr + index] = slice[index]
      }

      // if (addr === mouseEepromAddr.CurrentDPI) {
      //   for (let index = 0; index < 8; index++) {
      //     const dpiAddr = index * 4 + mouseEepromAddr.DPIValue

      //     const lowBits = flashData[dpiAddr]
      //     const highBits = flashData[dpiAddr + 2]

      //     const masked = highBits * mouseEepromAddr.DPIValue
      //     const high = masked >> 2

      //     let value = lowBits + (high << 8)
      //     value = (value + 1) * 50

      //     deviceData.dpiValues[index] = {
      //       value,
      //       color: ''
      //     }
      //   }
      // }

      break;
    default:
      break;
  }
}

const parseReadDeviceEeprom = () => {
  // fill dpi values
  for (let index = 0; index < 8; index++) {
    const dpiAddr = index * 4 + mouseEepromAddr.DPIValue

    const lowBits = flashData[dpiAddr]
    const highBits = flashData[dpiAddr + 2]

    const masked = highBits * mouseEepromAddr.DPIValue
    const high = masked >> 2

    let value = lowBits + (high << 8)
    value = (value + 1) * 50

    deviceData.dpiValues[index] = {
      value,
      color: ''
    }
  }
  // fill dpi values end
}

const requestDevice = async () => {
  const device = await getDevice()
  if (!device) return

  // device.removeEventListener('inputreport', handleInputReport)
  // device.addEventListener('inputreport', handleInputReport)
  if (!device.opened) {
    await device.open()
  }

  device.oninputreport = handleInputReport

  // await writeCommand(device, commands.PCDriverStatus, [1])
  // await writeCommand(device, commands.GetDongleVersion, [])
  // await writeCommand(device, commands.BatteryLevel, [])

  reading.value = true

  await readDeviceFullEeprom(device)
  parseReadDeviceEeprom()

  // await readDeviceEeprom(device, mouseEepromAddr.CurrentDPI, 2)

  reading.value = false
}
</script>

<template>
  <main class="container">
    <button @click="requestDevice">request device</button>

    <p>reading: {{ reading }}</p>
    <p>{{ deviceData }}</p>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
