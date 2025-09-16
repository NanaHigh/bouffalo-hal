#![no_std]
#![no_main]

use bouffalo_hal::{
    efuse::Efuse,
    gpip::{AdcChannels, AdcCommand, AdcConfig, AdcIntStatus, AdcResult, Gpip},
    hbn::{GpadcChannel, GpadcVref},
    prelude::*,
    uart::Config,
};
use bouffalo_rt::{
    Clocks, Peripherals, entry, interrupt,
    soc::bl808::{M0Machine, McuLpInterrupt},
};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use embedded_time::rate::*;
use panic_halt as _;

static ADC_INTERRUPT_COUNT: AtomicU32 = AtomicU32::new(0);
static ADC_CONVERSION_DONE: AtomicBool = AtomicBool::new(false);

#[interrupt]
fn gpadc_dma() {
    ADC_INTERRUPT_COUNT.fetch_add(1, Ordering::SeqCst);

    // Simply set a flag that an interrupt occurred
    // The main logic will handle reading data and clearing interrupts
    ADC_CONVERSION_DONE.store(true, Ordering::SeqCst);
}

#[entry]
fn main(p: Peripherals, c: Clocks) -> ! {
    let tx = p.uart_muxes.sig2.into_transmit(p.gpio.io14);
    let rx = p.uart_muxes.sig3.into_receive(p.gpio.io15);
    let config = Config::default().set_baudrate(2000000.Bd());
    let mut serial = p.uart0.freerun(config, (tx, rx), &c).unwrap();

    writeln!(serial, "Welcome to ADC interrupt demo!").ok();

    let mut gpip = Gpip::new(
        p.gpip,
        Some(AdcConfig::default().set_vref(GpadcVref::V3p2)),
        None,
        &p.glb,
        &p.hbn,
    );

    let efuse = Efuse::new(p.efuse);
    gpip.adc_calibrate(&efuse, &p.hbn, None);

    // Clear ADC FIFO to ensure clean start
    gpip.adc_feature_control(AdcCommand::ClearFifo, false, &p.hbn);

    gpip.adc_rxint_mask(true); // mask interrupt initially
    p.plic.set_priority(McuLpInterrupt::GpadcDma, 1);
    p.plic.enable(McuLpInterrupt::GpadcDma, M0Machine);

    // Enable global interrupts using direct assembly (more reliable)
    unsafe {
        // Enable external interrupts in mie register (bit 11)
        core::arch::asm!("csrs mie, {}", in(reg) 1 << 11);
        // Enable global interrupts in mstatus register (bit 3)
        core::arch::asm!("csrs mstatus, {}", in(reg) 1 << 3);
    }

    // Read and display registers again after enabling
    unsafe {
        let mut mstatus: usize;
        let mut mie: usize;
        core::arch::asm!("csrr {}, mstatus", out(reg) mstatus);
        core::arch::asm!("csrr {}, mie", out(reg) mie);
        writeln!(
            serial,
            "After enable - mstatus: 0x{:08x}, mie: 0x{:08x}",
            mstatus, mie
        )
        .ok();
    }

    // Re-check interrupt configuration after enabling global interrupts
    writeln!(serial, "After global interrupt enable:").ok();
    writeln!(
        serial,
        "PLIC priority: {}",
        p.plic.get_priority(McuLpInterrupt::GpadcDma)
    )
    .ok();
    writeln!(
        serial,
        "PLIC enabled: {}",
        p.plic.is_enabled(McuLpInterrupt::GpadcDma, M0Machine)
    )
    .ok();

    let chans = [
        AdcChannels {
            pos_ch: GpadcChannel::Channel0,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel1,
            neg_ch: GpadcChannel::ChannelVGND,
        },
        AdcChannels {
            pos_ch: GpadcChannel::Channel2,
            neg_ch: GpadcChannel::ChannelVGND,
        },
    ];

    let mut raw_data = [0u32; 26];

    for chan in chans {
        ADC_CONVERSION_DONE.store(false, Ordering::SeqCst);
        raw_data.fill(0);

        gpip.adc_feature_control(AdcCommand::ClearFifo, false, &p.hbn);
        gpip.adc_channel_config(&[chan], &p.hbn);

        gpip.adc_rxint_mask(false); // unmask interrupt
        writeln!(serial, "ADC interrupt unmasked").ok();

        // Check interrupt mask status directly from register
        unsafe {
            let gpip_config = core::ptr::read_volatile(0x20002000 as *const u32);
            writeln!(serial, "GPIP config reg: 0x{:08x}", gpip_config).ok();
            let rdy_mask_bit = (gpip_config >> 12) & 1; // GPADC_RDY_MASK at bit 12
            writeln!(
                serial,
                "RDY_MASK bit: {} (0=enabled, 1=disabled)",
                rdy_mask_bit
            )
            .ok();
        }

        // Check interrupt status
        let int_status_before = gpip.adc_get_intstatus(&p.hbn);
        writeln!(
            serial,
            "Interrupt status before start: adc_ready={}",
            int_status_before.adc_ready
        )
        .ok();

        gpip.adc_start_conversion(&p.hbn);

        writeln!(
            serial,
            "Channel {:?} - ADC started, waiting for data...",
            chan.pos_ch
        )
        .ok();

        // Check initial ADC status right after start
        let initial_fifo_count = gpip.adc_get_complete_num();
        let initial_int_status = gpip.adc_get_intstatus(&p.hbn);
        writeln!(
            serial,
            "Initial - FIFO count: {}, adc_ready: {}",
            initial_fifo_count, initial_int_status.adc_ready
        )
        .ok();

        let mut collected_count = 0usize;
        let mut timeout_counter = 0u32;

        // According to documentation: CPU sets gpadc_rdy_mask to 0, ADC will generate
        // interrupt when FIFO has data. In interrupt function, read data based on
        // gpadc_fifo_data_count and then set gpadc_rdy_clr to clear interrupt.
        while collected_count < 26 {
            // Wait for interrupt signal with timeout
            let mut wait_counter = 0u32;
            while !ADC_CONVERSION_DONE.load(Ordering::SeqCst) {
                core::hint::spin_loop();
                wait_counter += 1;

                // Add timeout to prevent infinite loop
                if wait_counter > 100000 {
                    // Reduced timeout for faster feedback
                    timeout_counter += 1;
                    writeln!(
                        serial,
                        "Timeout #{}, checking ADC status...",
                        timeout_counter
                    )
                    .ok();

                    // Check if there's data available without interrupt
                    let fifo_count = gpip.adc_get_complete_num();
                    let int_status = gpip.adc_get_intstatus(&p.hbn);
                    let interrupt_count = ADC_INTERRUPT_COUNT.load(Ordering::SeqCst);
                    writeln!(
                        serial,
                        "FIFO count: {}, adc_ready: {}, total interrupts: {}",
                        fifo_count, int_status.adc_ready, interrupt_count
                    )
                    .ok();

                    if fifo_count > 0 || int_status.adc_ready {
                        // Manually trigger data processing
                        writeln!(serial, "Found data without interrupt, processing manually").ok();
                        ADC_CONVERSION_DONE.store(true, Ordering::SeqCst);
                        break;
                    }

                    if timeout_counter > 5 {
                        writeln!(serial, "Too many timeouts, stopping this channel").ok();
                        break;
                    }
                    wait_counter = 0;
                }
            }

            if timeout_counter > 5 {
                break;
            }

            ADC_CONVERSION_DONE.store(false, Ordering::SeqCst);

            // Check interrupt status
            let int_status = gpip.adc_get_intstatus(&p.hbn);
            let fifo_count = gpip.adc_get_complete_num();

            writeln!(
                serial,
                "Processing: adc_ready={}, fifo_count={}, collected={}",
                int_status.adc_ready, fifo_count, collected_count
            )
            .ok();

            if int_status.adc_ready || fifo_count > 0 {
                // Read data from FIFO according to fifo_count
                let data_to_read = core::cmp::min(fifo_count as usize, 26 - collected_count);
                writeln!(serial, "Reading {} data points", data_to_read).ok();

                for i in 0..data_to_read {
                    if collected_count < 26 {
                        raw_data[collected_count] = gpip.adc_get_raw_data();
                        writeln!(
                            serial,
                            "Data[{}]: 0x{:08x}",
                            collected_count, raw_data[collected_count]
                        )
                        .ok();
                        collected_count += 1;
                    }
                }

                // Clear interrupt as specified in documentation
                let clear_flags = AdcIntStatus {
                    adc_ready: true,
                    fifo_underrun: false,
                    fifo_overrun: false,
                    neg_saturation: false,
                    pos_saturation: false,
                };
                gpip.adc_int_clear(clear_flags, &p.hbn);

                // Stop when we have enough data
                if collected_count >= 26 {
                    writeln!(
                        serial,
                        "Collected all {} samples, stopping conversion",
                        collected_count
                    )
                    .ok();
                    gpip.adc_stop_conversion(&p.hbn);
                    gpip.adc_rxint_mask(true); // mask interrupt
                    break;
                }
            } else {
                writeln!(serial, "No data ready, continuing...").ok();
            }
        }

        // Process results (equivalent to C processing)
        let result = &mut [AdcResult {
            pos_chan: Some(chan.pos_ch),
            neg_chan: Some(chan.neg_ch),
            value: 0,
            millivolt: 0,
        }; 26];

        gpip.adc_parse_result(&raw_data, result, &p.hbn);

        // Print interrupt count and results
        let interrupt_count = ADC_INTERRUPT_COUNT.load(Ordering::SeqCst);
        writeln!(serial, "ADC interrupts received: {}", interrupt_count).ok();

        // Print results (skip first 10 samples like C version)
        for j in 10..26 {
            writeln!(serial, "raw data: {:08x}", raw_data[j]).ok();
            writeln!(
                serial,
                "pos chan {:?}, {} mv",
                result[j].pos_chan.unwrap(),
                result[j].millivolt
            )
            .ok();
        }
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
