use std::cell::RefCell;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::Arc;
use esp32_nimble::{BLEAddress, BLEService};
use esp32_nimble::{utilities::mutex::Mutex, BLEAdvertisementData, BLEAdvertising, BLEConnDesc, BLEDevice, BLEError, BLEServer, NimbleProperties};
use esp_idf_svc::hal::task;


use crate::utils::auxiliary::ISRQueue;
use crate::utils::auxiliary::SharableRef;
use crate::utils::auxiliary::SharableRefExt;
use crate::utils::esp32_framework_error::Esp32FrameworkError;
use crate::utils::notification::Notifier;
use crate::InterruptDriver;
use sharable_reference_macro::sharable_reference_wrapper;

use super::{BleError, BleId, Characteristic, ConnectionMode, DiscoverableMode, Service};

/// Abstraction to create a BLE server, the side that has the information to be used in a connection
/// oriented relationship. Contains:
/// * `advertising_name`: Clients scanning will see the advertising name before connection.
/// * `services`: The servere will hace information for the clients to see. All this information will be encapsulated on different services.
/// * `user_on_connection`: Callback that will be executed for each client connected.
/// * `user_on_disconnection`: Callback that will be executed for each client disconnected.
pub struct _BleServer<'a> {
    advertising_name: String,
    ble_server: &'a mut BLEServer,
    services: Vec<Service>,               
    advertisement: &'a Mutex<BLEAdvertising>,
    user_on_connection: Option<ConnectionCallback<'a>>,
    user_on_disconnection: Option<ConnectionCallback<'a>>
}

#[derive(Clone)]
pub struct BleServer<'a>{
    inner: SharableRef<_BleServer<'a>>

}

struct ConnectionCallback<'a>{
    callback: Box<dyn FnMut(&mut BleServer<'a>, &ConnectionInformation) + 'a>,
    info_queue: ISRQueue<ConnectionInformation>,
    notifier: Notifier
}

impl<'a> ConnectionCallback<'a>{
    fn new(notifier: Notifier) -> Self{
        Self { callback: Box::new(|_,_| {}), info_queue: ISRQueue::new(1000), notifier}
    }

    fn set_callback<C: FnMut(&mut BleServer<'a>, &ConnectionInformation) + 'a>(&mut self, callback: C){
        self.callback = Box::new(callback);
    }

    fn handle_connection_changes(&mut self, server: &mut BleServer<'a>){
        while let Ok(item)  = self.info_queue.try_recv() {
            (self.callback)(server, &item);
        }
    }
}

/// Contains information about the new client connected that can be user on
/// connection or disconnection callbacks
#[derive(Debug, Clone, Copy)]
pub struct ConnectionInformation{
    pub address: BLEAddress,
    pub id_address: BLEAddress,
    pub conn_handle: u16,
    pub interval: u16,
    pub timeout: u16,
    pub latency: u16,
    pub mtu: u16,
    pub bonded: bool,
    pub encrypted: bool,
    pub authenticated: bool,
    pub sec_key_size: u32,
    pub rssi: Result<i8, u32>,
    pub disconnection_result: Option<u32>,
}

impl ConnectionInformation{
    fn from_BLEConnDesc(desc: &BLEConnDesc, is_connected: bool, desc_res: Result<(), BLEError>) -> Self{
        let mut res = None;
        if !is_connected{
            res = match desc_res {
                Ok(_) => None,
                Err(err) => Some(err.code()),
            };
        } 

        let rssi = match desc.get_rssi() {
            Ok(rssi) => Ok(rssi),
            Err(err) => Err(err.code()),
        };
        
        ConnectionInformation{
            address: desc.address(),
            id_address:desc.id_address(),
            conn_handle: desc.conn_handle(),
            interval: desc.interval(),
            timeout: desc.timeout(),
            latency: desc.latency(),
            mtu: desc.mtu(),
            bonded: desc.bonded(),
            encrypted: desc.encrypted(),
            authenticated: desc.authenticated(),
            sec_key_size: desc.sec_key_size(),
            rssi,
            disconnection_result: res,
        }
    }

}

#[sharable_reference_wrapper]
impl <'a>_BleServer<'a> {
    pub fn new(name: String, ble_device: &mut BLEDevice, services: &Vec<Service>, connection_notifier: Notifier, disconnection_notifier: Notifier) -> Self {
        let mut server = _BleServer{
            advertising_name: name,
            ble_server: ble_device.get_server(),
            services: services.clone(),
            advertisement: ble_device.get_advertising(),
            user_on_connection: Some(ConnectionCallback::new(connection_notifier)),
            user_on_disconnection: Some(ConnectionCallback::new(disconnection_notifier)),
        };
            
        for service in services {
            server.set_service(service);
        }

        server
    }

    pub fn connection_handler<C: FnMut(&mut BleServer, &ConnectionInformation) + 'a>(&mut self, handler: C) -> &mut Self
    {
        let user_on_connection = self.user_on_connection.as_mut().unwrap();
        let notifier_ref = user_on_connection.notifier.clone();
        let mut con_info_ref = user_on_connection.info_queue.clone();
        user_on_connection.set_callback(handler);
        
        self.ble_server.on_connect(move |_, info| {
            notifier_ref.notify().unwrap();
            _ = con_info_ref.send_timeout(ConnectionInformation::from_BLEConnDesc(info, true, Ok(())), 1_000_000); //
        });
        self
    }


    pub fn disconnect_handler<C: FnMut(&mut BleServer, &ConnectionInformation) + 'a>(&mut self, handler: C) -> &mut Self
    {
        let user_on_disconnection = self.user_on_disconnection.as_mut().unwrap();
        let notifier_ref = user_on_disconnection.notifier.clone();
        let mut con_info_ref = user_on_disconnection.info_queue.clone();
        user_on_disconnection.set_callback(handler);
        
        self.ble_server.on_disconnect(move |info, res| {
            notifier_ref.notify().unwrap();
            _ = con_info_ref.send_timeout(ConnectionInformation::from_BLEConnDesc(info, false,res), 1_000_000);
        });
        self
    }

    /// The conn_handle is obtained with the ConnectionInformation inside the closure of 
    /// connection_handler
    /// * `min_interval`: The minimum connection interval, time between BLE events. This value 
    /// must range between 7.5ms and 4000ms in 1.25ms units, this interval will be used while transferring data
    /// in max speed.
    /// * `max_interval`: The maximum connection interval, time between BLE events. This value 
    /// must range between 7.5ms and 4000ms in 1.25ms units, this interval will be used to save energy.
    /// * `latency`: The number of packets that can be skipped (packets will be skipped only if there is no data to answer).
    /// * `timeout`: The maximum time to wait after the last packet arrived to consider connection lost. 
    pub fn set_connection_settings(&mut self, info: &ConnectionInformation, min_interval: u16, max_interval: u16, latency: u16, timeout: u16) -> Result<(), BleError>{
        self.ble_server.update_conn_params(info.conn_handle, min_interval, max_interval, latency, timeout).map_err(|e| BleError::from(e))
    }

    /// Set the advertising time parameters:
    /// * `min_interval`: The minimum advertising interval, time between advertisememts. This value 
    /// must range between 20ms and 10240ms in 0.625ms units.
    /// * `max_interval`: The maximum advertising intervaltime between advertisememts. TThis value 
    /// must range between 20ms and 10240ms in 0.625ms units.
    fn set_advertising_interval(&mut self, min_interval: u16, max_interval: u16) -> &mut Self {
        self.advertisement.lock().min_interval(min_interval).max_interval(max_interval);
        self
    }

    /// Sets a high duty cycle has intervals between advertising packets are 
    /// typically in the range of 20 ms to 100 ms.
    /// Valid only if advertisement_type is directed-connectable.
    pub fn set_high_advertising_duty_cycle(&mut self) -> &mut Self{
        self.advertisement.lock().high_duty_cycle(true);
        self
    }

    /// Sets a low duty cycle has ntervals between advertising packets are 
    /// typically in the range of 1,000 ms to 10,240 ms.
    /// Valid only if advertisement_type is directed-connectable.
    pub fn set_low_advertising_duty_cycle(&mut self) -> &mut Self {
        self.advertisement.lock().high_duty_cycle(false);
        self
    }

    /// Sets the discover mode. The possible modes are:
    pub fn set_discoverable_mode(&mut self, disc_mode: DiscoverableMode) -> &mut Self {
        match disc_mode {
            DiscoverableMode::NonDiscoverable => self.advertisement.lock().disc_mode(disc_mode.get_code()),
            DiscoverableMode::LimitedDiscoverable(min_interval, max_interval) => self.advertisement.lock().disc_mode(disc_mode.get_code())
                .min_interval(min_interval)
                .max_interval(max_interval),
            DiscoverableMode::GeneralDiscoverable(min_interval, max_interval) => self.advertisement.lock().disc_mode(disc_mode.get_code()).min_interval(min_interval).max_interval(max_interval),
        };
        self.advertisement.lock().disc_mode(disc_mode.get_code());
        self
    }

    ///Sets the connection mode of the advertisment.
    pub fn set_connection_mode(&mut self, conn_mode: ConnectionMode) -> &mut Self {
        self.advertisement.lock().advertisement_type(conn_mode.get_code());
        self
    }
    
    /// Set or overwrite a service to the server
    pub fn set_service(&mut self, service: &Service) -> Result<(),BleError> {
        self.ble_server.create_service(service.id.to_uuid());

        for characteristic in &service.characteristics{
            self.set_characteristic(service.id.clone(), characteristic)?;
        }
        Ok(())
    }

    pub fn set_services(&mut self, services: &Vec<Service>) -> Result<(),BleError> {
        for service in services {
            self.set_service(&service)?;
        }
        Ok(())
    }

    /// Set a new characteristic or update the value in an existent characteristic to the server. 
    /// Returns an error if the service does not exist or the properties are invalid.
    pub fn set_characteristic(&mut self, service_id: BleId, characteristic: &Characteristic) -> Result<(), BleError> {
        let server_service = task::block_on(async {
            self.ble_server.get_service(service_id.to_uuid()).await
        });

        // Check if there is a service with 'service_id' as its id.
        if let Some(service) = server_service {

            match self.try_to_update_characteristic(service, characteristic, false) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    // Create a new characteristic
                    match NimbleProperties::from_bits(characteristic.properties.to_le()) {
                        Some(properties) => {

                            let charac = service.lock().create_characteristic(
                                characteristic.id.to_uuid(),
                                properties,
                            );
                            charac.lock().set_value(&characteristic.data);
                            return Ok(());
                        },
                        None => {return Err(BleError::PropertiesError)},
                    }
                }
            }
        }
        Err(BleError::ServiceNotFound)
    }

    /// Checks if there is a BLECharacteristic on the BLEService with the corresponding id. If it exists, it updates its value. Apart from that,
    /// depending on thee notify boolean parameter, it may notify the connected device of the changed value.
    fn try_to_update_characteristic(&self, service: &Arc<Mutex<BLEService>>, characteristic: &Characteristic, notify: bool) -> Result<(), BleError> {
        // Check if there is a characteristic with characteristic.id in this service
        let locked_service = service.lock();
        let server_characteristic = task::block_on(async {
            locked_service.get_characteristic(characteristic.id.to_uuid()).await
        });

        if let Some(server_characteristic) = server_characteristic { 
            let mut res_characteristic = server_characteristic.lock();
            res_characteristic.set_value(&characteristic.data);
            if notify {
                res_characteristic.notify();
            }
            return Ok(());
        }
        
        Err(BleError::CharacteristicNotFound)
    }
    
    pub fn notify_value(&mut self, service_id: BleId, characteristic: &Characteristic) -> Result<(), BleError> {
        let server_service = task::block_on(async {
            self.ble_server.get_service(service_id.to_uuid()).await
        });

        if let Some(service) = server_service {
            self.try_to_update_characteristic(service, characteristic, true)?;
            return Ok(());
        }
        Err(BleError::ServiceNotFound)
        
    }

    pub fn start(&mut self) -> Result<(), BleError>{
        self.create_advertisement_data()?;
        self.advertisement.lock().start().map_err(|_| BleError::StartingAdvertisementError)
    }

    fn create_advertisement_data(&mut self) -> Result<(), BleError>{
        let mut adv_data = BLEAdvertisementData::new();
        adv_data.name(&self.advertising_name);
        for service in &self.services {
            adv_data.add_service_uuid(service.id.to_uuid());
        }
        self.advertisement.lock().set_data(&mut adv_data).map_err(|_| BleError::AdvertisementError)
    }
}

// TODO: refactor this!
impl<'a> InterruptDriver for BleServer<'a>{
    fn update_interrupt(&mut self)-> Result<(), Esp32FrameworkError> {
        let (mut user_on_connection, mut user_on_disconnection) = self.take_connection_callbacks();
        user_on_connection.handle_connection_changes(self);
        user_on_disconnection.handle_connection_changes(self);
        self.set_connection_callbacks(user_on_connection, user_on_disconnection);
        Ok(())
    }
}

impl<'a> BleServer<'a>{
    pub fn new(name: String, ble_device: &mut BLEDevice, services: &Vec<Service>, connection_notifier: Notifier, disconnection_notifier: Notifier) -> Self {
        Self { inner: SharableRef::new_sharable(
            _BleServer::new(name, ble_device, services, connection_notifier, disconnection_notifier)
        ) }
    }

    fn take_connection_callbacks(&mut self)->(ConnectionCallback<'a>, ConnectionCallback<'a>){
        let user_on_connection = self.inner.deref_mut().user_on_connection.take().unwrap();
        let user_on_disconnection = self.inner.deref_mut().user_on_disconnection.take().unwrap();
        (user_on_connection, user_on_disconnection)
    }

    fn set_connection_callbacks(&mut self, user_on_connection: ConnectionCallback<'a>, user_on_disconnection: ConnectionCallback<'a>){
        self.inner.deref_mut().user_on_connection = Some(user_on_connection);
        self.inner.deref_mut().user_on_disconnection = Some(user_on_disconnection);
    }
}