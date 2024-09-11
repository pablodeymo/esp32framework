use std::future::join;
use std::future::Future;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;
use esp32_nimble::enums::AuthReq;
use esp_idf_svc::hal::adc::ADC1;
use esp_idf_svc::hal::adc::*;
// use esp_idf_svc::hal::adc::config::{Config, Resolution};
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::delay::TICK_RATE_HZ;
//use esp_idf_svc::hal::task::notification::Notification;
use esp_idf_svc::hal::i2c;
use esp32_nimble::BLEDevice;
use esp_idf_svc::hal::task::block_on;
use oneshot::AdcDriver;
use std::cell::RefCell;
pub type SharableAdcDriver<'a> = Rc<AdcDriver<'a, ADC1>>;
pub type SharableI2CDriver<'a> = Rc<RefCell<Option<i2c::I2C0>>>;

use crate::ble::ble_client::BleClient;
use crate::ble::{BleBeacon,BleServer,Service,Security};
use crate::gpio::AnalogIn;
use crate::gpio::{AnalogInPwm,
    DigitalIn,
    DigitalOut, 
    AnalogOut};
use crate::serial::Parity;
use crate::serial::StopBit;
use crate::serial::UART;
use crate::serial::{I2CMaster, I2CSlave};
use crate::utils::auxiliary::SharableRef;
use crate::utils::auxiliary::SharableRefExt;
use crate::utils::notification::Notification;
use crate::utils::notification::Notifier;
use crate::utils::timer_driver::TimerDriver;
    
use crate::microcontroller_src::peripherals::*;
use crate::microcontroller_src::interrupt_driver::InterruptDriver;

const TICKS_PER_MILLI: f32 = TICK_RATE_HZ as f32 / 1000_f32;

pub struct Microcontroller<'a> {
    peripherals: Peripherals,
    timer_drivers: Vec<TimerDriver<'a>>,
    interrupt_drivers: Vec<Box<dyn InterruptDriver + 'a>>,
    adc_driver: Option<SharableAdcDriver<'a>>,
    notification: Notification,
    i2c_driver: SharableI2CDriver<'a>
}

impl <'a>Microcontroller<'a>{
    pub fn new() -> Self {
        esp_idf_svc::sys::link_patches();
        let peripherals = Peripherals::new();
        
        Microcontroller{
            peripherals,
            timer_drivers: vec![],
            interrupt_drivers: Vec::new(),
            adc_driver: None,
            notification: Notification::new(),
            i2c_driver: Rc::new(RefCell::new(None)),
        }
    }

    pub fn get_timer_driver(&mut self)-> TimerDriver<'a>{
        let mut timer_driver = if self.timer_drivers.len() < 2{
            let timer = self.peripherals.get_timer(self.timer_drivers.len());
            TimerDriver::new(timer, self.notification.notifier()).unwrap()
        }else{
            self.timer_drivers.swap_remove(0)
        };

        let timer_driver_copy = timer_driver.create_child_copy().unwrap();
        self.timer_drivers.push(timer_driver);
        timer_driver_copy
    }

    /// Creates a DigitalIn on the ESP pin with number 'pin_num' to read digital inputs.
    pub fn set_pin_as_digital_in(&mut self, pin_num: usize)-> DigitalIn<'a> {
        let pin_peripheral = self.peripherals.get_digital_pin(pin_num);
        let dgin = DigitalIn::new(self.get_timer_driver(), pin_peripheral, Some(self.notification.notifier())).unwrap();
        self.interrupt_drivers.push(Box::new(dgin.clone()));
        dgin
    }
    
    /// Creates a DigitalOut on the ESP pin with number 'pin_num' to writee digital outputs.
    pub fn set_pin_as_digital_out(&mut self, pin_num: usize) -> DigitalOut<'a> {
        let pin_peripheral = self.peripherals.get_digital_pin(pin_num);
        let dgout = DigitalOut::new(self.get_timer_driver(), pin_peripheral).unwrap();
        self.interrupt_drivers.push(Box::new(dgout.clone()));
        dgout
    }
    
    /// Starts an adc driver if no other was started before. Bitwidth is always set to 12, since 
    /// the ESP32-C6 only allows that width
    fn start_adc_driver(&mut self) {
        if self.adc_driver.is_none() {
            self.peripherals.get_adc();
            let driver = AdcDriver::new(unsafe{ADC1::new()}).unwrap();
            self.adc_driver.replace(Rc::new(driver));
        }
    }

    // TODO: simplificar instanciacion analog in
    /// Sets pin as analog input with attenuation set to 2.5dB
    pub fn set_pin_as_analog_in_low_atten(&mut self, pin_num: usize) -> AnalogIn<'a> {
        self.start_adc_driver();
        let pin_peripheral = self.peripherals.get_analog_pin(pin_num);
        AnalogIn::new(pin_peripheral, self.adc_driver.clone().unwrap(), attenuation::DB_2_5).unwrap()
    }
    
    /// Sets pin as analog input with attenuation set to 6dB  
    pub fn set_pin_as_analog_in_medium_atten(&mut self, pin_num: usize) -> AnalogIn<'a> {
        self.start_adc_driver();
        let pin_peripheral = self.peripherals.get_analog_pin(pin_num);
        AnalogIn::new(pin_peripheral, self.adc_driver.clone().unwrap(), attenuation::DB_6).unwrap()
    }
    
    /// Sets pin as analog input with attenuation set to 11dB  
    pub fn set_pin_as_analog_in_high_atten(&mut self, pin_num: usize) -> AnalogIn<'a> {
        self.start_adc_driver();
        let pin_peripheral = self.peripherals.get_analog_pin(pin_num);
        AnalogIn::new(pin_peripheral, self.adc_driver.clone().unwrap(), attenuation::DB_11).unwrap()
    }

    /// Sets pin as analog input with attenuation set to 0dB  
    pub fn set_pin_as_analog_in_no_atten(&mut self, pin_num: usize) -> AnalogIn<'a> {
        self.start_adc_driver();
        let pin_peripheral = self.peripherals.get_analog_pin(pin_num);
        AnalogIn::new(pin_peripheral, self.adc_driver.clone().unwrap(), attenuation::NONE).unwrap()
    }

    pub fn set_pin_as_analog_out(&mut self, pin_num: usize, freq_hz: u32, resolution: u32) -> AnalogOut<'a> {
        let (pwm_channel, pwm_timer) = self.peripherals.get_next_pwm();
        let pin_peripheral = self.peripherals.get_pwm_pin(pin_num);
        let anlg_out = AnalogOut::new(pwm_channel, pwm_timer, pin_peripheral, self.get_timer_driver(), freq_hz, resolution).unwrap();
        self.interrupt_drivers.push(Box::new(anlg_out.clone()));
        anlg_out
    } 

    pub fn set_pin_as_default_analog_out(&mut self, pin_num: usize) -> AnalogOut<'a> {
        let (pwm_channel, pwm_timer) = self.peripherals.get_next_pwm();
        let pin_peripheral = self.peripherals.get_pwm_pin(pin_num);
        let anlg_out = AnalogOut::default(pwm_channel, pwm_timer, pin_peripheral, self.get_timer_driver()).unwrap();
        self.interrupt_drivers.push(Box::new(anlg_out.clone()));
        anlg_out
    }

    pub fn set_pin_as_analog_in_pwm(&mut self, pin_num: usize, freq_hz: u32) -> AnalogInPwm<'a> {
        
        let pin_peripheral = self.peripherals.get_digital_pin(pin_num);
        let timer_driver = self.get_timer_driver();            // TODO: Add a better error. If there is no timers a text should sayy this
        AnalogInPwm::new(timer_driver, pin_peripheral, freq_hz).unwrap()
    }
    
    pub fn set_pin_as_default_analog_in_pwm(&mut self, pin_num: usize) -> AnalogInPwm<'a> {
        let pin_peripheral = self.peripherals.get_digital_pin(pin_num);
        let timer_driver = self.get_timer_driver();
        AnalogInPwm::default(timer_driver, pin_peripheral).unwrap()
    }
    
    pub fn set_pins_for_i2c_master(&mut self, sda_pin: usize, scl_pin: usize) -> I2CMaster<'a> {
        let sda_peripheral = self.peripherals.get_digital_pin(sda_pin);
        let scl_peripheral = self.peripherals.get_digital_pin(scl_pin);

        match self.peripherals.get_i2c(){
            Peripheral::I2C => {
                I2CMaster::new(sda_peripheral, scl_peripheral, unsafe{i2c::I2C0::new()}).unwrap()
            }
            _ => {
                panic!("I2C Driver already taken!");
            },
        }
    }

    pub fn set_pins_for_i2c_slave(&mut self, sda_pin: usize, scl_pin: usize, slave_addr: u8) -> I2CSlave<'a> {
        let sda_peripheral = self.peripherals.get_digital_pin(sda_pin);
        let scl_peripheral = self.peripherals.get_digital_pin(scl_pin);

        match self.peripherals.get_i2c(){
            Peripheral::I2C => {
                I2CSlave::new(sda_peripheral, scl_peripheral, unsafe{i2c::I2C0::new()}, slave_addr).unwrap()
            }
            _ => {
                panic!("I2C Driver already taken!");
            },
        }
    }

    pub fn set_pins_for_default_uart(&mut self, tx_pin: usize, rx_pin: usize, uart_num: usize) -> UART<'a> {
        let tx_peripheral = self.peripherals.get_digital_pin(tx_pin);
        let rx_peripheral = self.peripherals.get_digital_pin(rx_pin);
        let uart_peripheral = self.peripherals.get_uart(uart_num);

        UART::default(tx_peripheral, rx_peripheral, uart_peripheral).unwrap()
    }

    pub fn set_pins_for_uart(&mut self, tx_pin: usize, rx_pin: usize, uart_num: usize, baudrate: u32, parity: Parity, stopbit: StopBit) -> UART<'a> {
        let tx_peripheral = self.peripherals.get_digital_pin(tx_pin);
        let rx_peripheral = self.peripherals.get_digital_pin(rx_pin);
        let uart_peripheral = self.peripherals.get_uart(uart_num);

        UART::new(tx_peripheral, rx_peripheral, uart_peripheral, baudrate, parity, stopbit).unwrap()
    }

    pub fn ble_beacon(&mut self, advertising_name: String, services: &Vec<Service>)-> BleBeacon<'a>{
        self.peripherals.get_ble_device(); // TODO ver safety
        let ble_device = BLEDevice::take();
        BleBeacon::new(ble_device, self.get_timer_driver(), advertising_name, services).unwrap()
    }
    
    // TODO &VEc<Services>
    pub fn ble_server(&mut self, advertising_name: String, services: &Vec<Service>)-> BleServer<'a>{
        self.peripherals.get_ble_device();
        let ble_device = BLEDevice::take();
        BleServer::new(advertising_name, ble_device, services, self.notification.notifier(),self.notification.notifier() )
    }

    fn config_bluetooth_security(&mut self, ble_device: &mut BLEDevice, security_config: Security){
        ble_device.security()
        .set_auth(AuthReq::from_bits(security_config.auth_mode.to_le()).unwrap())
        .set_passkey(security_config.passkey)
        .set_io_cap(security_config.io_capabilities.get_code())
        .resolve_rpa();
    }

    // TODO &VEc<Services>
    pub fn ble_secure_server(&mut self, advertising_name: String, services: &Vec<Service>, security_config: Security)-> BleServer<'a>{
        self.peripherals.get_ble_device();
        let ble_device = BLEDevice::take();
        self.config_bluetooth_security(ble_device,security_config);
        let ble_server = BleServer::new(advertising_name, ble_device, services, self.notification.notifier(),self.notification.notifier() );
        self.interrupt_drivers.push(Box::new(ble_server.clone()));
        ble_server
    }

    pub fn ble_client(&mut self)-> BleClient{
        self.peripherals.get_ble_device();
        let ble_device = BLEDevice::take();
        BleClient::new(ble_device)
    }

    pub fn update(&mut self) {
        //timer_drivers must be updated before other drivers since this may efect the other drivers updates
        for timer_driver in &mut self.timer_drivers{
            timer_driver.update_interrupt().unwrap();
        }
        for driver in &mut self.interrupt_drivers{
            driver.update_interrupt().unwrap();
        }
    }
    
    fn wait_for_updates_indefinitly(&mut self){
        loop{
            self.notification.blocking_wait();
            self.update();
        }
    }

    /*
    fn _wait_for_updates_until(&mut self, miliseconds:u32){
        let starting_time = Instant::now();
        let mut elapsed = Duration::from_millis(0);
        let sleep_time = Duration::from_millis(miliseconds as u64);
        
        while elapsed < sleep_time{
            let timeout = ((sleep_time - elapsed).as_millis() as f32 * TICKS_PER_MILLI) as u32;
            if self.notification.wait(timeout).is_some(){
                self.update();
            }
            elapsed = starting_time.elapsed();
        }
    }
    */

    fn wait_for_updates_until(&mut self, miliseconds:u32){
        let timer_driver = match self.timer_drivers.first_mut(){
            Some(timer_driver) => timer_driver,
            None => &mut self.get_timer_driver(),
        };
        
        let timed_out = SharableRef::new_sharable(false);
        let mut timed_out_ref = timed_out.clone();

        timer_driver.interrupt_after(miliseconds as u64 * 1000, move || {
            *timed_out_ref.deref_mut() = true
        });

        timer_driver.enable().unwrap();

        while !*timed_out.deref(){
            self.notification.blocking_wait();
            self.update();
            println!("Updating");
        }
    }

    pub fn wait_for_updates(&mut self, miliseconds:Option<u32>){
        match miliseconds{
            Some(milis) => self.wait_for_updates_until(milis),
            None => self.wait_for_updates_indefinitly(),
        }
    }

    pub fn sleep(&self, miliseconds:u32){
        FreeRtos::delay_ms(miliseconds)
    }

    async fn wait_for_updates_until_finished(&mut self, finished: SharableRef<bool>){
        while !*finished.deref(){
            self.notification.wait().await;
            self.update();
        }
    }

    pub fn block_on<F: Future>(&mut self, fut: F)-> F::Output{
        let finished = SharableRef::new_sharable(false);
        let fut = wrap_user_future(self.notification.notifier(), finished.clone(), fut);
        block_on(join!(fut, self.wait_for_updates_until_finished(finished))).0
    }
}

impl<'a> Default for Microcontroller<'a> {
    fn default() -> Self {
    Self::new()
    }
}

async fn wrap_user_future<F: Future>(notifier: Notifier, mut finished: SharableRef<bool>, fut: F)-> F::Output{
    let res = fut.await;
    *finished.deref_mut() = true;
    notifier.notify().unwrap();
    res
}