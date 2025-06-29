import { reportId } from '~/constants'

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

export const useDevice = () => {
  const device = useState<HIDDevice>('hid-device')

  const requestDevice = async () => {
    const devices = await navigator.hid.requestDevice({ filters })

    for (const _device of devices) {
      for (const collection of _device.collections) {
        if (collection.inputReports?.length !== 1 || collection.outputReports?.length !== 1) continue
        if (collection.outputReports[0]?.reportId != reportId) continue

        device.value = _device

        if (!_device.opened) {
          await _device.open()
        }

        break
      }
    }
  }

  return {
    device,
    requestDevice,
  }
}
