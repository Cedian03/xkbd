use usb_device as usbd; 
use usbd::class_prelude::*;

use crate::key_report::{KeyReport, REPORT_SIZE};

const INTERFACE_CLASS_HID: u8 = 0x03;
const INTERFACE_SUB_CLASS_NONE: u8 = 0x00;
const INTERFACE_PROTOCOL_KEYBOARD: u8 = 0x01;

const HID_SPEC_VERSION_1_11: u16 = 0x01_11;
const COUNTRY_CODE_NONE: u8 = 0x00;

const DESCRIPTOR_TYPE_HID: u8 = 0x21;
const DESCRIPTOR_HID: &[u8] = &[
    lsb(HID_SPEC_VERSION_1_11),
    msb(HID_SPEC_VERSION_1_11),
    COUNTRY_CODE_NONE,
    1, // Number of descriptors
    DESCRIPTOR_TYPE_REPORT,
    lsb(DESCRIPTOR_REPORT.len() as u16),
    msb(DESCRIPTOR_REPORT.len() as u16),
];
   
const DESCRIPTOR_TYPE_REPORT: u8 = 0x22;
const DESCRIPTOR_REPORT: &[u8] = &[
    0x05, 0x01,  // Usage Page (Generic Desktop)
    0x09, 0x06,  // Usage (Keyboard)
    0xA1, 0x01,  // Collection (Application)
    0x05, 0x07,  //   Usage Page (Key Codes)
    0x19, 0xE0,  //   Usage Minimum (224)
    0x29, 0xE7,  //   Usage Maximum (231)
    0x15, 0x00,  //   Logical Minimum (0)
    0x25, 0x01,  //   Logical Minimum (1)
    0x75, 0x01,  //   Report Size (1)
    0x95, 0x08,  //   Report Count (8)
    0x81, 0x02,  //   Input (Data, Variable, Absolute)
    0x95, 0x01,  //   Report Count (1)
    0x75, 0x08,  //   Report Size (8)
    0x81, 0x01,  //   Input (Constant)
    0x95, 0x05,  //   Report Count (5)
    0x75, 0x01,  //   Report Size (1)
    0x05, 0x08,  //   Usage Page (Page# for LEDs)
    0x19, 0x01,  //   Usage Minimum (1)
    0x29, 0x05,  //   Usage Maximum (5)
    0x91, 0x02,  //   Output (Data, Variable, Absolute)
    0x95, 0x01,  //   Report Count (1)
    0x75, 0x03,  //   Report Size (3)
    0x91, 0x01,  //   Output (Constant)
    0x95, 0x06,  //   Report Count (6)
    0x75, 0x08,  //   Report Size (8)
    0x15, 0x00,  //   Logical Minimum (0)
    0x25, 0x65,  //   Logical Maximum(101)
    0x05, 0x07,  //   Usage Page (Key Codes)
    0x19, 0x00,  //   Usage Minimum (0)
    0x29, 0x65,  //   Usage Maximum (101)
    0x81, 0x00,  //   Input (Data, Array)
    0xC0         // End Collection
]; 

pub struct KeyClass<'a, B: UsbBus> {
    number: InterfaceNumber,
    endpoint: EndpointIn<'a, B>,
}

impl<'a, B: UsbBus> KeyClass<'a, B> {
    pub fn new(alloc: &'a UsbBusAllocator<B>, interval: u8) -> Self {
        Self {
            number: alloc.interface(),
            endpoint: alloc.interrupt(REPORT_SIZE as u16, interval),
        }
    }

    pub fn push(&mut self, report: &KeyReport) -> bool {
        self.endpoint.write(&report.pack()).is_ok_and(|x| x == REPORT_SIZE)
    }
}

impl<B: UsbBus> UsbClass<B> for KeyClass<'_, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> usbd::Result<()> {
        writer.interface(
            self.number, 
            INTERFACE_CLASS_HID,
            INTERFACE_SUB_CLASS_NONE,
            INTERFACE_PROTOCOL_KEYBOARD,
        )?;

        writer.write(
            DESCRIPTOR_TYPE_HID, 
            DESCRIPTOR_HID
        )?; 

        writer.endpoint(&self.endpoint)?;

        Ok(())
    }

    fn control_in(&mut self, xfer: ControlIn<B>) {
        let req = *xfer.request();

        // TODO: Filter requests?

        match (req.request_type, req.request) {
            (control::RequestType::Standard, control::Request::GET_DESCRIPTOR) => {
                match msb(req.value) {
                    DESCRIPTOR_TYPE_HID => {
                        let desc: [u8; DESCRIPTOR_HID.len() + 2] = [
                            DESCRIPTOR_HID.len() as u8 + 2, // Length of the entire descriptor
                            DESCRIPTOR_TYPE_HID,
                            DESCRIPTOR_HID[0],
                            DESCRIPTOR_HID[1],
                            DESCRIPTOR_HID[2],
                            DESCRIPTOR_HID[3],
                            DESCRIPTOR_HID[4],
                            DESCRIPTOR_HID[5],
                            DESCRIPTOR_HID[6],
                        ];

                        xfer.accept_with(&desc).ok();
                    }
                    DESCRIPTOR_TYPE_REPORT => {
                        xfer.accept_with_static(DESCRIPTOR_REPORT).ok(); // TODO: Error handling.
                    }
                    _ => {}  // TODO: Reject?
                }
            } 
            _ => {} // TODO: Reject?
        }
    }
}

/// Least significant byte
const fn lsb(x: u16) -> u8 {
    (x & 0xFF) as u8
}

/// Most significant byte
const fn msb(x: u16) -> u8 {
    (x >> 8) as u8
}