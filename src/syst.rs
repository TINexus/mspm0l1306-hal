
use cortex_m; // I think this is what allows us to create main

use cortex_m_rt::exception; // I think this is what allows us to create main

use mspm0l130x as pac;

pub struct SYST;


pub enum SystClkSource {
    /// Core-provided clock
    Core,
    /// External reference clock
    External,
}
const SYST_CSR_CLKSOURCE: u32 = 1 << 2;


// This was all Taken from Cortex-M syst.rs so that it could all be accessed from the same place
impl SYST{

    pub fn clear_current(&mut self) {
        cortex_m::Peripherals::take().unwrap().SYST.clear_current();
    }

    pub fn disable_counter(&mut self) {
        cortex_m::Peripherals::take().unwrap().SYST.disable_counter();
    }

    pub fn disable_interrupt(&mut self) {
        cortex_m::Peripherals::take().unwrap().SYST.disable_interrupt();
    }

    pub fn enable_counter(&mut self) {
        cortex_m::Peripherals::take().unwrap().SYST.enable_counter();
    }

    pub fn enable_interrupt(&mut self) {
        cortex_m::Peripherals::take().unwrap().SYST.enable_interrupt();
    }


    pub fn get_clock_source(&mut self) -> SystClkSource {
        if cortex_m::Peripherals::take().unwrap().SYST.csr.read() & SYST_CSR_CLKSOURCE != 0 {
            SystClkSource::Core
        } else {
            SystClkSource::External
        }
    }

    pub fn has_wrapped(&mut self) -> bool {
        return cortex_m::Peripherals::take().unwrap().SYST.has_wrapped();
    }

    pub fn is_counter_enabled(&mut self) -> bool {
        return cortex_m::Peripherals::take().unwrap().SYST.is_counter_enabled();
    }

    pub fn is_interrupt_enabled(&mut self) -> bool {
        return cortex_m::Peripherals::take().unwrap().SYST.is_interrupt_enabled();
    }

    pub fn set_clock_source(&mut self, clk_source: SystClkSource) {
        match clk_source {
            SystClkSource::External => cortex_m::Peripherals::take().unwrap().SYST.set_clock_source(cortex_m::peripheral::syst::SystClkSource::External),
            SystClkSource::Core => cortex_m::Peripherals::take().unwrap().SYST.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core),
        }
    }

    pub fn set_reload(&mut self, value: u32){
        cortex_m::Peripherals::take().unwrap().SYST.set_reload(value);
    }


}




pub fn interruptsetupsysttick(){


    SYST.set_clock_source(SystClkSource::Core);

    SYST.set_reload(32_000_000);
    SYST.clear_current();
    SYST.enable_counter();
    SYST.enable_interrupt();


}



