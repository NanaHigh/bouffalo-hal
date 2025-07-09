#![no_std]
#![no_main]

use bouffalo_hal::{
    gpip::{Gpip, AdcChannels, AdcCommmand, AdcConfig, GpadcChannel},
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

    // let mut adc = Gpip::new(p.gpip, Some(AdcConfig::default()), None);
    // let chans = AdcChannels {
    //     pos_ch: GpadcChannel::ChannelTSENP,
    //     neg_ch: GpadcChannel::ChannelVGND,
    // };

    // adc.adc_channel_config(&[chans]);
    // adc.adc_tsen_init(false);
    writeln!(serial, "Init done").ok();

    // for _ in 0..5 {
    //     delay(100);
    //     writeln!(serial, "temp = {}.", adc.adc_get_tsen_temp() as u32);
    // }

    loop {}
}

pub fn delay(tim: u32) {
    unsafe {
        for _ in 0..tim * 100 {
            core::arch::asm!("nop");
        }
    }
}
