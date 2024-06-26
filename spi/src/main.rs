#![no_std]
#![no_main]

use bsp::entry;
use defmt_rtt as _;
use embedded_hal::{digital::v2::OutputPin, prelude::_embedded_hal_blocking_spi_Write};
use fugit::RateExtU32;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    gpio::FunctionSpi,
    pac,
    sio::Sio,
    spi::Spi,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);
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
    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    let spi_mosi = pins.gpio7.into_function::<FunctionSpi>();
    let spi_sclk = pins.gpio6.into_function::<FunctionSpi>();
    let spi_device = pac.SPI0;
    let spi_pin_layout = (spi_mosi, spi_sclk);
    let mut myspi = Spi::<_, _, _, 8>::new(spi_device, spi_pin_layout).init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        bsp::XOSC_CRYSTAL_FREQ.Hz(),
        embedded_hal::spi::MODE_0,
    );
    // Keep sending commands to the DAC, blink if ok.
    let mut led_pin = pins.led.into_push_pull_output();
    let mut spi_cs = pins.gpio5.into_push_pull_output();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut dac_high_bits = 0;
    loop {
        // Set the high bits only.
        let mut buffer = [0b0011_0000 + dac_high_bits, 0b11111111];
        spi_cs.set_low().unwrap();
        if myspi.write(&mut buffer).is_ok() {
            led_pin.set_high().unwrap();
        }
        spi_cs.set_high().unwrap();
        delay.delay_ms(50);
        led_pin.set_low().unwrap();
        delay.delay_ms(50);
        dac_high_bits += 1;
        if dac_high_bits > 0b1111 {
            dac_high_bits = 0;
        }
    }
}
