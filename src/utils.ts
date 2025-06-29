export const getCrc = (buffer: Uint8Array<ArrayBuffer>) => {
  let crc = 0;

  for (let i = 0; i < buffer.length - 1; i++) {
    crc += buffer[i];
  }

  crc = crc & 0xFF;
  crc = 0x55 - crc;

  return crc;
}

export const bufferToColor = (buffer: number[]) => {
  let result = '#'

  for (let index = 0; index < buffer.length; index++) {
    result += buffer[index].toString(16).padStart(2, '0')
  }

  return result
}

export const sleep = (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms))
}
