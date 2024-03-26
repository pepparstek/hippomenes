#![no_std]
#![no_main]
use core::panic::PanicInfo;
use hippomenes_rt as _;

#[rtic::app(device = hippomenes_core)]
mod app {
    use hippomenes_core::{Pin, Pin0};
    use hippomenes_hal::UART;
    #[shared]
    struct Shared {
        dummy: bool,
    }

    #[local]
    struct Local {
        uart: UART<Pin0>,
        buf: [u8; 8],
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let peripherals = cx.device;
        let pin = peripherals.gpio.pins().pin0;
        let timer = peripherals.timer;
        let mut uart = UART::new(pin, timer, 119_200);
        let buf = [0x13, 0x37, 0x13, 0x37, 0x13, 0x37, 0x13, 0x37];
        uart.send(buf);
        (Shared { dummy: true }, Local { uart, buf })
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }

    #[task(binds = Interrupt2, priority=7, shared=[dummy], local=[uart, buf])]
    fn uart_done(cx: uart_done::Context) {
        cx.local.uart.send(*cx.local.buf);
    }
}

#[panic_handler]
fn p(_: &PanicInfo) -> ! {
    loop {}
}
