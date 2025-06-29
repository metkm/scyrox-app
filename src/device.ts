import { commands, reportId } from "./constants";
import { getCrc, sleep } from "./utils";

const filters = [
  {
    vendorId: 13652,
    productId: 62967,
  },
  {
    vendorId: 13652,
    productId: 62964,
  },
  {
    vendorId: 13652,
    productId: 62966,
  },
]

export const getDevice = async () => {
  const devices = await navigator.hid.requestDevice({ filters });

  for (const device of devices) {
    for (const collection of device.collections) {
      if (collection.inputReports?.length !== 1 || collection.outputReports?.length !== 1) continue
      if (collection.outputReports[0].reportId != reportId) continue

      return device
    }
  }
}

export const readDeviceFullEeprom = async (device: HIDDevice) => {
  let data = Uint8Array.of(commands.ReadFlashData, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef)
  let add = 0

  do {
    data[2] = add >> 8;
    data[3] = add & 0xFF;
    data[4] = 10

    let crc = getCrc(data);
    data[15] = crc  - reportId;

    await device.sendReport(reportId, data)
    await sleep(200)
    
    add += 10
  } while (add < 0x100)
}

export const readDeviceEeprom = async (device: HIDDevice, address: number, length: number) => {
  let data = Uint8Array.of(commands.ReadFlashData, 0x00, address >> 8, address & 0xFF, length, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xef)

  data[15] = getCrc(data) - reportId
  await device.sendReport(reportId, data)
}
