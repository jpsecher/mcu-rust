#![no_std]
#![no_main]

use bsp::entry;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{clocks::init_clocks_and_plls, pac, usb::UsbBus, watchdog::Watchdog};
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::device::{UsbDeviceBuilder, UsbVidPid};
use usbd_serial::{SerialPort, UsbError, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();
    let usb_bus = UsbBusAllocator::new(UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        // .product("Serial port")
        .device_class(USB_CLASS_CDC)
        .build();
    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
        let mut buf = [0u8; 64];
        match serial.read(&mut buf[..]) {
            Ok(count) => {
                // count bytes were read to &buf[..count]
            }
            Err(UsbError::WouldBlock) => {}
            Err(err) => {}
        };
        match serial.write(&[0x3a, 0x29]) {
            Ok(count) => {
                // count bytes were written
            }
            Err(UsbError::WouldBlock) => {}
            Err(err) => {}
        };
    }
}
