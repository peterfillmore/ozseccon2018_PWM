#![no_std]
#![no_main]

// pick a panicking behavior
extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_abort; // requires nightly
// extern crate panic_itm; // logs messages over ITM; requires ITM support
// extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger

//use cortex_m::asm;
use cortex_m_rt::entry;
//use cortex_m;
//use cortex_m::{interrupt::Mutex};

//extern crate stm32f0xx_hal;

//use stm32f0xx_hal as hal;
//use stm32f0xx_hal::stm32 as stm32;

use stm32f0xx_hal as hal;

//use cortex_m_semihosting::hprintln;

//use crate::hal::{prelude::*, stm32};

//use stm32f0xx_hal::stm32f0::u
use crate::hal::{
    prelude::*,
    rcc::*, 
    //spi::Spi,
    //gpio::*,
    time::*,
    timers::*,
    pwm::*,
    stm32,
    //device
};
use nb::block;

//use core::cell::RefCell;
use cortex_m::interrupt::{self, Mutex};

#[entry]
fn main() -> ! {
    let cp = stm32::CorePeripherals::take().unwrap();
    let mut dp = stm32::Peripherals::take().unwrap();
    //let mut rcc = dp.RCC.configure().sysclk(8.mhz()).freeze(&mut dp.FLASH);
    //let mut clocks = rcc.clocks;
    let rcc = dp.RCC.constrain();
    //let rcc = dp.RCC;
    let clocks = rcc.cfgr.sysclk(8.mhz()).freeze();
    //let mut rcc = dp.RCC.configure().sysclk(8.mhz()).freeze(&mut dp.FLASH);
    let mut timer  = Timer::tim3(dp.TIM3, Hertz(1), clocks); 
    //let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioa = dp.GPIOA.split();
    //let gpiob = dp.GPIOB.split(&mut rcc);
    let gpiob = dp.GPIOB.split();
    //let gpiob = dp.GPIOB.split();
    let mut led1_blue = gpioa.pa8.into_alternate_af2().internal_pull_up(true); 
    let mut led1_red = gpioa.pa9.into_alternate_af2().internal_pull_up(true); 
    let mut led1_green = gpioa.pa10.into_alternate_af2().internal_pull_up(true); 
    
    let mut led2_blue = gpioa.pa15.into_alternate_af2().internal_pull_up(true);   
    let mut led2_red = gpiob.pb3.into_alternate_af2().internal_pull_up(true);   
    let mut led2_green = gpiob.pb10.into_alternate_af2().internal_pull_up(true);   
   
    let mut tooth0 = gpiob.pb11.into_push_pull_output();
    let mut tooth1 = gpiob.pb12.into_push_pull_output();
    //let mut tooth1 = gpiob.pb12.into_push_pull_output();
    let mut tooth2 = gpiob.pb13.into_push_pull_output();
    let mut tooth3 = gpiob.pb14.into_push_pull_output();
    let mut tooth4 = gpiob.pb15.into_push_pull_output();
    //let mut ch4 = gpioa.pa11.into_alternate_af2().internal_pull_up(true); 
    //let mypins = Pwm::Pins {C1: true, C2: true, C3: false, C4: false}; 
    //let mut pwm = TIM1::pwm(dp.TIM1, (ch1, ch2, ch3, ch4) , Hertz(10000), clocks).0;
    let mut pwm = dp
        .TIM1
        .pwm(
            (led1_blue, led1_red, led1_green),
            //&mut afio.mapr,
            10.khz(),
            clocks,
            //&mut rcc.apb2,
        );
    let mut right_eye = dp
        .TIM2
        .pwm(
            (led2_blue, led2_red, led2_green),
            //&mut afio.mapr,
            10.khz(),
            clocks,
            //&mut rcc.apb2,
        );

    let mut pwm1 = pwm.0;
    let mut pwm2 = pwm.1;
    let mut pwm3 = pwm.2;
    
    let mut b_eye = right_eye.0;
    let mut r_eye = right_eye.1;
    let mut g_eye = right_eye.2;
    

    let max_duty = pwm1.get_max_duty();
    //hprintln!("Max Duty = {}", max_duty); 
    //pwm1.set_duty(max_duty); 
    //pwm1.enable();
    //pwm2.set_duty(max_duty); 
    //pwm2.enable(); 
    //let mut pwm1  = tim1(dp.TIM1, (ch1, ch2) , Hertz(10000), clocks);
    //cortex_m::interrupt::free(|cs| MY_TIM1.borrow(cs).replace(Some(dp.TIM1)));
 
    //ch1.internal_pull_up(true); 
    //let mut led1red = gpioa.pa9.into_push_pull_output();
     
    
    //let mut pwm2  = tim1(dp.TIM1, ch2, Hertz(2000), clocks);
    //let test: () = pwm1;   
    //let test: () = pwm1._channel;
    //pwm1.set_duty(400); 
    pwm1.enable();
    pwm2.enable();
    pwm3.enable();
    b_eye.enable();
    r_eye.enable();
    g_eye.enable();
 
    cortex_m::interrupt::free(move |cs| { 
        //MY_TIM1.borrow(cs).replace(Some(dp.TIM1));
        //let mytim1 = MY_TIM1.borrow(cs).borrow();
        //let _: () = mytim1; 
        //let mytim2 = MY_TIM1.borrow(cs).borrow();
        //let mut pwm2  = tim1(mytim2.unwrap(), ch2, Hertz(10000), clocks);
        
        
        //let test: () = Pwm::Pins;   
        loop {
                //pwm1.disable();
                pwm1.set_duty(max_duty/4); 
                //pwm1.enable();
                //led1red.toggle();
                block!(timer.wait()).ok();
                pwm1.set_duty(max_duty/100); 
                pwm2.set_duty(max_duty/4); 
                //pwm1.disable();
                block!(timer.wait()).ok();
                pwm2.set_duty(max_duty/100); 
                pwm3.set_duty(max_duty/4); 
                block!(timer.wait()).ok();
                pwm3.set_duty(max_duty/100); 
                b_eye.set_duty(max_duty/4);
                block!(timer.wait()).ok();
                b_eye.set_duty(max_duty/100);
                r_eye.set_duty(max_duty/4);
                block!(timer.wait()).ok();
                r_eye.set_duty(max_duty/100);
                g_eye.set_duty(max_duty/4);
                block!(timer.wait()).ok();
                g_eye.set_duty(max_duty/100);
                block!(timer.wait()).ok();
                tooth0.set_high(); 
                block!(timer.wait()).ok();
                tooth0.set_low(); 
                block!(timer.wait()).ok();
                tooth1.set_low(); 
                block!(timer.wait()).ok();
                tooth0.toggle(); 
                tooth1.toggle(); 
                block!(timer.wait()).ok();
                tooth1.toggle(); 
                tooth2.toggle(); 
                block!(timer.wait()).ok();
                tooth2.toggle(); 
                tooth3.toggle(); 
                block!(timer.wait()).ok();
                tooth3.toggle(); 
                tooth4.toggle(); 
                block!(timer.wait()).ok();
                tooth4.toggle(); 
                block!(timer.wait()).ok();
                }
        });
        loop {
            continue;
        }
}
//});
