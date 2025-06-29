import type { CommandValues } from './constants'
import { commands, mouseEepromAddr, reportId } from './constants'
import { getCrc, sleep } from './utils'

export const readDeviceFullEeprom = async (device: HIDDevice) => {
  const data = Uint8Array.of(
    commands.ReadFlashData,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xef,
  )
  let add = 0

  do {
    data[2] = add >> 8
    data[3] = add & 0xff
    data[4] = 10

    const crc = getCrc(data)
    data[15] = crc - reportId

    await device.sendReport(reportId, data)
    await sleep(50)

    add += 10
  } while (add < 0x100)
}

export const writeDeviceEeprom = async <T extends keyof typeof commands>(
  device: HIDDevice,
  command: CommandValues<T>,
  address: number,
  value: number[],
  length: number = value.length,
) => {
  const data = Uint8Array.of(
    command,
    0x00,
    address >> 8,
    address & 0xff,
    length,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xef,
  )

  for (let index = 0; index < value.length; index++) {
    data[5 + index] = value[index] || 0
  }

  data[15] = getCrc(data) - reportId
  await device.sendReport(reportId, data)
}

export const setDpiIndex = (device: HIDDevice, index: number) => {
  return writeDeviceEeprom(device, commands.WriteFlashData, mouseEepromAddr.CurrentDPI, [index, 0x55 - index], 2)
}
