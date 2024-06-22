use esp_idf_svc::{hal::{delay::FreeRtos, gpio::*}, /*handle::RawHandle,*/ sys::{/*esp_timer_create,*/ EspError, ESP_ERR_INVALID_ARG, ESP_ERR_INVALID_STATE}};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
pub use esp_idf_svc::hal::gpio::{InterruptType, Pull};
use crate::timer_driver::{TimerDriver,TimerDriverError};
use crate::error_text_parser::map_enable_disable_errors;
use crate::peripherals::Peripheral;

type AtomicInterruptUpdateCode = AtomicU8;

const DEFAULT_PULL: Pull = Pull::Up;

//sudo usermod -a -G tty palito
//sudo usermod -a -G dialout palito

pub struct DigitalIn<'a>{
    pub pin_driver: PinDriver<'a, AnyIOPin, Input>,
    timer_driver: TimerDriver<'a>,
    interrupt_type: InterruptType,
    pub interrupt_update_code: Arc<AtomicInterruptUpdateCode>,
    user_callback: fn()->(),
    debounce_ms: Option<u32>,
}

pub enum InterruptUpdate {
    ExecAndEnablePin,
    EnableTimerDriver,
    TimerReached,
    UnsubscribeTimerDriver,
    ExecAndUnsubscribePin,
    None
}

#[derive(Debug)]
pub enum DigitalInError {
    CannotSetPullForPin,
    CannotSetPinAsInput,
    StateAlreadySet,
    InvalidPin,
    InvalidPeripheral,
    TimerDriverError (TimerDriverError)
}

impl InterruptUpdate{
    fn get_code(self)-> u8{
        self as u8
    }

    fn get_atomic_code(self)-> AtomicInterruptUpdateCode{
        AtomicInterruptUpdateCode::new(self.get_code())
    }

    fn from_code(code:u8)-> Self {
        match code{
            0 => Self::ExecAndEnablePin,
            1 => Self::EnableTimerDriver,
            2 => Self::TimerReached,
            3 => Self::UnsubscribeTimerDriver,
            4 => Self::ExecAndUnsubscribePin,
            _ => Self::None,
        }
    }

    fn from_atomic_code(atomic_code: Arc<AtomicInterruptUpdateCode>) -> Self {
        InterruptUpdate::from_code(atomic_code.load(Ordering::Acquire))
    }
}

/// Create a new DigitalIn for a Pin, and define an iterruptType to watch for.
/// By default pull is set to Down
impl <'a>DigitalIn<'a> {
    pub fn new(timer_driver: TimerDriver<'a>, per: Peripheral, interrupt_type: InterruptType) -> Result<DigitalIn, DigitalInError> { //flank default: asc
        let gpio = per.into_any_io_pin().map_err(|_| DigitalInError::InvalidPeripheral)?;
        let mut pin_driver = PinDriver::input(gpio).map_err(|_| DigitalInError::CannotSetPinAsInput)?;
        pin_driver.set_interrupt_type(interrupt_type).map_err(|_| DigitalInError::InvalidPin)?;

        let mut digital_in = DigitalIn {
            pin_driver: pin_driver,
            timer_driver: timer_driver,
            interrupt_type: interrupt_type, 
            interrupt_update_code: Arc::from(InterruptUpdate::None.get_atomic_code()),
            debounce_ms: None,
            user_callback: || -> () {},
        };

        digital_in.set_pull(Pull::Down).unwrap();
        //dig_in.set_pull(Pull::Down)?;
        return Ok(digital_in)
    }

    pub fn set_pull(&mut self, pull_type: Pull)-> Result<(), DigitalInError>{
        self.pin_driver.set_pull(pull_type).map_err(|_| DigitalInError::CannotSetPullForPin)
    }
    
    /// time unit: ms
    fn trigger_if_mantains_after(&mut self, time_micro:u32)-> Result<impl FnMut() + Send + 'static, DigitalInError>{
        
        let interrupt_update_code_ref = self.interrupt_update_code.clone();
        let after_timer_cljr = move || {
            interrupt_update_code_ref.store(InterruptUpdate::TimerReached.get_code(), Ordering::SeqCst);
        };
        
        // unsafe{
        //     self.timer_driver.subscribe(after_timer_cljr).map_err(|_| DigitalInError::InvalidTimer)?;
        // }
        // self.timer_driver.set_alarm(((time_ms as u64) * self.timer_driver.tick_hz()/1000) as u64).map_err(|_| DigitalInError::CouldNotSetTimer)?;
        self.timer_driver.interrupt_after(time_micro, after_timer_cljr).map_err(|err| DigitalInError::TimerDriverError(err))?;
        
        let interrupt_update_code_ref = self.interrupt_update_code.clone();
        let start_timer_cljr = move || {
            interrupt_update_code_ref.store(InterruptUpdate::EnableTimerDriver.get_code(), Ordering::SeqCst);
        };
        
        Ok(start_timer_cljr)
    }
    fn subscribe_trigger<F: FnMut() + Send + 'static>(&mut self, mut func: F) -> Result<(), DigitalInError>{
        unsafe {
            self.pin_driver.subscribe(func).map_err(|err| map_enable_disable_errors(err))?;
        }
        self.pin_driver.enable_interrupt().map_err(|err| map_enable_disable_errors(err))
    }
    
    pub fn _trigger_on_flank<F: FnMut() + Send + 'static>(&mut self , user_callback: fn()->(), callback: F) -> Result<(), DigitalInError>{
        self.user_callback = user_callback;
        match self.debounce_ms{
            Some(debounce_ms) => {
                let wrapper = self.trigger_if_mantains_after(debounce_ms)?;
                self.subscribe_trigger(wrapper)
            },
            None => self.subscribe_trigger(callback),
        }
    }
    
    pub fn trigger_on_flank(&mut self , user_callback: fn()->())->Result<(), DigitalInError>{
        let interrupt_update_code_ref= self.interrupt_update_code.clone();
        let callback = move ||{
            interrupt_update_code_ref.store(InterruptUpdate::ExecAndEnablePin.get_code(), Ordering::SeqCst);
        };
        self._trigger_on_flank(user_callback, callback)
    }
    
    pub fn trigger_on_flank_first_n_times(&mut self, mut amount_of_times: usize , user_callback: fn()->()) -> Result<(), DigitalInError> {
        if amount_of_times == 0 {
            return Ok(())
        }
        
        let interrupt_update_code_ref = self.interrupt_update_code.clone();
        let callback = move || {
            amount_of_times -= 1;
            if amount_of_times == 0 {
                interrupt_update_code_ref.store(InterruptUpdate::ExecAndUnsubscribePin.get_code(), Ordering::SeqCst);
            }else{
                interrupt_update_code_ref.store(InterruptUpdate::ExecAndEnablePin.get_code(), Ordering::SeqCst);
                
            }
        };
        self._trigger_on_flank(user_callback, callback)
    }
    
    // fn enable_timer_driver(&mut self, enable: bool) -> Result<(),DigitalInError>{
    //     if enable{
    //         self.timer_driver.set_counter(0).map_err(|_| DigitalInError::CannotSetTimerCounter)?; 
    //         self.timer_driver.enable_interrupt().map_err(|_| DigitalInError::CouldNotSetTimer)?;
    //     }else{
    //         self.timer_driver.disable_interrupt().map_err(|_| DigitalInError::CouldNotSetTimer)?;
    //     }
    //     self.timer_driver.enable_alarm(enable).map_err(|_| DigitalInError::CouldNotSetTimer)?;
    //     self.timer_driver.enable(enable).map_err(|_| DigitalInError::CouldNotSetTimer)?;
    //     Ok(())   
    // }

    fn timer_reached(&mut self) -> Result<(), DigitalInError>{
        let level = match self.interrupt_type {
            InterruptType::PosEdge => Level::High,
            InterruptType::NegEdge => Level::Low,
            InterruptType::AnyEdge => todo!(),
            InterruptType::LowLevel => Level::Low,
            InterruptType::HighLevel => Level::High,
        };
        
        if self.pin_driver.get_level() == level{
            (self.user_callback)();
        }
        
        self.timer_driver.disable().map_err(|err| DigitalInError::TimerDriverError(err))?;
        self.pin_driver.enable_interrupt().map_err(|err| map_enable_disable_errors(err))
    }

    pub fn update_interrupt(&mut self)-> Result<(), DigitalInError>{
        let interrupt_update = InterruptUpdate::from_atomic_code(self.interrupt_update_code.clone());
        self.interrupt_update_code.store(InterruptUpdate::None.get_code(), Ordering::SeqCst);

        match interrupt_update {
            InterruptUpdate::ExecAndEnablePin => {
                (self.user_callback)();
                self.pin_driver.enable_interrupt().map_err(|err| map_enable_disable_errors(err))
            },
            InterruptUpdate::EnableTimerDriver => self.timer_driver.enable().map_err(|err| DigitalInError::TimerDriverError(err)),
            InterruptUpdate::TimerReached => self.timer_reached(),
            InterruptUpdate::ExecAndUnsubscribePin => {
                (self.user_callback)();
                self.pin_driver.unsubscribe().map_err(|err| map_enable_disable_errors(err))
                },
            InterruptUpdate::UnsubscribeTimerDriver => self.timer_driver.unsubscribe().map_err(|err| DigitalInError::TimerDriverError(err)),
            InterruptUpdate::None => Ok(()),
        }
    }
    
    pub fn get_level(&self) -> Level{
        self.pin_driver.get_level()
    }    
    
    pub fn is_high(&self) -> bool{
        self.pin_driver.get_level() == Level::High
    }
    
    pub fn is_low(&self) -> bool{
        self.pin_driver.get_level() == Level::Low
    }
    
    pub fn set_debounce(&mut self, time_micro: u32){
        self.debounce_ms = Some(time_micro);
    }
    
    fn set_read_intervals(self, read_interval: u32){
        
    }
}