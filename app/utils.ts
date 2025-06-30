export const getCrc = (buffer: Uint8Array<ArrayBuffer> | number[]) => {
  let crc = 0

  for (let i = 0; i < buffer.length - 1; i++) {
    crc += buffer[i] || 0
  }

  crc = crc & 0xff
  crc = 0x55 - crc

  return crc
}

export const bufferToColor = (buffer: number[]) => {
  let result = '#'

  for (let index = 0; index < buffer.length; index++) {
    result += buffer[index]?.toString(16).padStart(2, '0')
  }

  return result
}

const voltages = [
  3050, 3420, 3480, 3540, 3600, 3660, 3720, 3760, 3800, 3840, 3880, 3920, 3940, 3960, 3980, 4000,
  4020, 4040, 4060, 4080, 4110,
]

export const voltageToBatteryLevel = (voltage: number) => {
  let voltageIndex = -1
  let level = 0

  for (let index = 0; index < voltages.length; index++) {
    if (voltage > (voltages[index] || 0)) {
      continue
    }

    voltageIndex = index
    break
  }

  if (voltageIndex === 0) {
    level = 0
  }
  else {
    const interval = (voltages[voltageIndex] || 0 - (voltages[voltageIndex - 1] || 0)) / 5
    level = (voltage - (voltages[voltageIndex] || 0)) / interval + (voltageIndex - 1) * 5
  }

  // level = Math.round(level)
  return level
}

export const sleep = (ms: number) => {
  return new Promise(resolve => setTimeout(resolve, ms))
}
