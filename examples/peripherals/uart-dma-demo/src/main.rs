#![no_std]
#![no_main]

use bouffalo_hal::{dma::*, prelude::*, uart::Config};
use bouffalo_rt::{entry, Clocks, Peripherals};
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
    let serial = p.uart0.freerun(config, pads, &c).unwrap().enabledma();
    let dma = Dma::new(p.dma0, 0);
    let hello = b"Hello, world!\r\n";
    let hello_addr = hello as *const u8;
    dma.test(hello_addr as u32);

    loop {}
}
