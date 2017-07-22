#![no_std]

extern crate cortex_m;
use cortex_m::interrupt::free;

extern crate lpc81x;

fn main() {
    free(|cs| {
        let syscon = lpc81x::SYSCON.borrow(cs);
        gpio_init(syscon);
        
        let swm = lpc81x::SWM.borrow(cs);
        swm.pinassign0.write(|w| unsafe { w.u0_txd_o().bits(4)
            .u0_rxd_i().bits(0) });
        swm.pinenable0.write(|w| w.reset_en().clear_bit()
            .swclk_en().set_bit().swdio_en().set_bit());
            
        let gpio = lpc81x::GPIO_PORT.borrow(cs);
        
        
        //gpio.gpio_port.b2.write(|w| unsafe { w.bits.write(0x01) } );
        gpio.dir0.write(|w| unsafe { w.bits(4) } );
        gpio.clr0.write(|w| unsafe { w.bits(4) } );
        //gpio.
    
    });

}

fn gpio_init(syscon : &lpc81x::SYSCON) {
    syscon.sysahbclkctrl.write(|w| w.gpio().set_bit());
    syscon.presetctrl.write(|w| w.gpio_rst_n().clear_bit());
    syscon.presetctrl.write(|w| w.gpio_rst_n().set_bit());
}
