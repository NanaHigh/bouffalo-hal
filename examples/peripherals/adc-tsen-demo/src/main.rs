#![no_std]
#![no_main]

use bouffalo_hal::{
    gpip::{AdcChannels, AdcConfig, GpadcChannel, Gpip},
    prelude::*,
    uart::Config,
};
use bouffalo_rt::{Clocks, Peripherals, entry};
use embedded_time::rate::*;
use panic_halt as _;

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.gpio.io14.into_uart();
    let rx = p.gpio.io15.into_uart();
    let sig2 = p.uart_muxes.sig2.into_transmit::<0>();
    let sig3 = p.uart_muxes.sig3.into_receive::<0>();
    let pads = ((tx, sig2), (rx, sig3));

    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, pads, &c).unwrap();

    let mut adc = Gpip::new(p.gpip, Some(AdcConfig::default()), None, &p.glb);

    const BASE_ADDR: *const u32 = 0x20002000 as *const u32;
    const MAX_OFFSET: u32 = 0x938;

    writeln!(
        serial,
        "Rust code printing register values from base address 0x{:08X}",
        BASE_ADDR as u32
    )
    .ok();

    let mut offset = 0u32;
    while offset <= MAX_OFFSET {
        unsafe {
            let addr = BASE_ADDR.add((offset / 4) as usize);
            let val = core::ptr::read_volatile(addr);
            writeln!(serial, "val[0x{:08X}]: 0x{:08X}", offset, val).ok();
        }
        offset += 4;
    }

    let chans = AdcChannels {
        pos_ch: GpadcChannel::ChannelTSENP,
        neg_ch: GpadcChannel::ChannelVGND,
    };

    adc.adc_channel_config(&[chans]);
    adc.adc_tsen_init(false);
    // adc.adc_update_trim(Some(AdcConfig::default()));

    writeln!(serial, "Init done").ok();

    for _ in 0..5 {
        delay(100);
        writeln!(serial, "adc read: {}", adc.adc_get_raw_data()).ok();
        writeln!(serial, "adc complete num: {}", adc.adc_get_complete_num()).ok();
        let temp = adc.adc_get_tsen_temp(&mut serial) as u32;
        writeln!(serial, "temp = {}.", temp).ok();
    }

    loop {}
}

pub fn delay(tim: u32) {
    unsafe {
        for _ in 0..tim * 100 {
            core::arch::asm!("nop");
        }
    }
}
