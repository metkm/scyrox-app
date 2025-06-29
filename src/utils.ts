import { reportId } from "./constants";

export const getCrc = (buffer: Uint8Array<ArrayBuffer>) => {
  let crc = 0;

  for (let i = 0; i < buffer.length - 1; i++) {
    crc += buffer[i];
  }

  crc = (crc & 0xFF);
  crc = 0x55 - crc;

  return crc;
}

export const writeCommand = async (device: HIDDevice, command: number, value: number[]) => {
  const buffer = Uint8Array.of(command, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef);
  buffer[4] = value.length

  for (let index = 0; index < value.length; index++) {
    buffer[5 + index] = value[index]
  }

  buffer[15] = getCrc(buffer) - reportId
  return device.sendReport(reportId, buffer)
}

export const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
