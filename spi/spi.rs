#![no_std]
#![no_main]

// Needs to be included.
#[cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use panic_halt as _;

use rp2040_hal as hal;
use hal::pac;  // Peripheral Access Crate
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

// extern crate panic_halt;
// extern crate embedded_hal;
// extern crate rp2040_hal;
// extern crate cortex_m;

use hal::clocks::Clock;
// use embedded_hal::digital::OutputPin;
// use cortex_m::prelude::*;
// use hal::fugit::RateExtU32;

// Bootloader block
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

const XTAL_FREQ_HZ: u32 = 12_000_000u32;

#[rp2040_hal::entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    // .ok()
    .unwrap();
    let sio = hal::Sio::new(pac.SIO);  // Single-cycle I/O block
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    // // Dedicated pins are implicitly used by the spi driver if they are in the correct mode
    // let spi_mosi = pins.gpio7.into_function::<hal::gpio::FunctionSpi>();
    // let spi_miso = pins.gpio4.into_function::<hal::gpio::FunctionSpi>();
    // let spi_sclk = pins.gpio6.into_function::<hal::gpio::FunctionSpi>();
    // let spi = hal::spi::Spi::<_, _, _, 8>::new(pac.SPI0, (spi_mosi, spi_miso, spi_sclk));
    // // Exchange the uninitialised SPI driver for an initialised one
    // let mut spi = spi.init(
    //     &mut pac.RESETS,
    //     clocks.peripheral_clock.freq(),
    //     16.MHz(),
    //     embedded_hal::spi::MODE_0,
    // );

    // Keep sending commands to the DAC, blink if ok
    let mut led_pin = pins.gpio25.into_push_pull_output();
    // let core = pac::CorePeripherals::take().unwrap();
    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    loop {
        // if spi.write(&[0b0001_0111, 0b11111111]).is_ok() {
        led_pin.set_high().unwrap();
        // }
        timer.delay_ms(250);
        led_pin.set_low().unwrap();
        timer.delay_ms(750);
    }
}
