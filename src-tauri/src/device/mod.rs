pub mod constants;
pub mod hid;
pub mod utils;

use hidapi::HidDevice;

struct Filter {
    pub vendor_id: u16,
    pub product_id: u16
}

const FILTERS: [Filter; 3] = [
    Filter { vendor_id: 13652, product_id: 62967 },
    Filter { vendor_id: 13652, product_id: 62964 },
    Filter { vendor_id: 13652, product_id: 62966 },
];

pub fn get_device() -> Option<HidDevice> {
    let api = hidapi::HidApi::new().unwrap();
    let devices  = api
        .device_list()
        .filter(|device| {
            FILTERS
                .iter()
                .find(|filter| device.vendor_id() == filter.vendor_id && device.product_id() == filter.product_id)
                .is_some()
        });

    for device_info in devices {
        let Ok(device) = device_info.open_device(&api) else {
            continue;
        };

        let mut buffer = [0x00; 512];
        let Ok(read_descriptor_count) = device.get_report_descriptor(&mut buffer) else {
            continue;
        };

        let Ok(report_descriptor) = hidparser::parse_report_descriptor(&buffer[0..read_descriptor_count]) else {
            continue
        };

        if report_descriptor.input_reports.len() != 1 || report_descriptor.output_reports.len() != 1 {
            continue;
        }

        if report_descriptor.output_reports
            .get(0)
            .and_then(|report| report.report_id)
            .is_some_and(|report_id| u32::from(report_id) == 8) {
                return Some(device);
            }
    };

    None
}
