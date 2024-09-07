use esp32_nimble::{enums::{AuthReq, AdvFlag, AdvType, ConnMode, DiscMode, SecurityIOCap}, utilities::BleUuid, BLEAddress, BLEAdvertisedDevice, BLEError, NimbleProperties};
use uuid::Uuid;
use crate::utils::timer_driver::TimerDriverError;

use super::{StandarCharacteristicId, StandarServiceId};
use std::hash::Hash;

const MAX_ADV_PAYLOAD_SIZE: usize = 31;
const PAYLOAD_FIELD_IDENTIFIER_SIZE: usize = 2;

/// Enums the different errors possible when working with BLE  
#[derive(Debug)]
pub enum BleError{
    ServiceDoesNotFit,
    ServiceTooBig,
    ServiceUnknown,
    StartingFailure,
    StoppingFailure,
    TimerDriverError(TimerDriverError),
    Code(u32, String),
    ServiceNotFound,
    CharacteristicNotFound,
    PropertiesError,
    AdvertisementError,
    StartingAdvertisementError,
    IncorrectHandle,
    ConnectionError,
    InvalidParameters,
    DeviceNotFound,
    AlreadyConnected
}

impl From<BLEError> for BleError {

    /// Creates a BleError from a BLEError
    /// 
    /// # Arguments
    /// 
    /// - `value`: The BLEError to transform
    /// 
    /// # Returns
    /// 
    /// The new BleError
    fn from(value: BLEError) -> Self {
        match value.code() {
            esp_idf_svc::sys::BLE_HS_EMSGSIZE => BleError::ServiceDoesNotFit,
            esp_idf_svc::sys::BLE_HS_EDONE => BleError::AlreadyConnected,
            esp_idf_svc::sys::BLE_HS_ENOTCONN  => BleError::DeviceNotFound,
            _ => BleError::Code(value.code(), value.to_string()),
        }
    }
}

impl BleError {
    
    /// Creates a BleError from an u32
    /// 
    /// # Arguments
    /// 
    /// - `code`: An u32 representing the error code
    /// 
    /// # Returns
    /// 
    /// An option containing the corresponding BleError, or a None if conversion fails
    fn from_code(code: u32) -> Option<BleError> {
        match BLEError::convert(code) {
            Ok(_) => None,
            Err(err) => Some(BleError::from(err)),
        }
    }
}

/// Enums the posible discoverable modes:
/// * `Non-Discoverable Mode`: The device does not advertise itself. Other devices will connect only if they know the specific address.
/// * `Limited Discoverable Mode`: The device does the advertisement during a limited amount of time.
/// * `General Discoverable Mode`: The advertisment is done continuously, so any other device can see it in any moment.
/// Both Limited and General Discoverable Mode have min_interval and max_interval:
/// * `min_interval`: The minimum advertising interval, time between advertisememts. This value 
/// must range between 20ms and 10240ms in 0.625ms units.
/// * `max_interval`: The maximum advertising intervaltime between advertisememts. TThis value 
/// must range between 20ms and 10240ms in 0.625ms units.
pub enum DiscoverableMode {
    NonDiscoverable,
    LimitedDiscoverable(u16, u16), // TODO: ADD support
    GeneralDiscoverable(u16, u16)
}

impl DiscoverableMode {

    /// Gets the DiscMode from a DiscoverableMode
    /// 
    /// # Returns
    /// 
    /// The corresponding DiscMode
    pub fn get_code(&self) -> DiscMode {
        match self {
            DiscoverableMode::NonDiscoverable => DiscMode::Non,
            DiscoverableMode::LimitedDiscoverable(_, _) => DiscMode::Ltd ,
            DiscoverableMode::GeneralDiscoverable(_, _) => DiscMode::Gen,
        }
    }
}

/// Enums the posible connection modes: 
/// * `NonConnectable`: The device does not allow connections.
/// * `DirectedConnectable`: The device only allows connections from a specific device.
/// * `UndirectedConnectable`: The divice allows connections from any device.
pub enum ConnectionMode {
    NonConnectable,
    DirectedConnectable, //TODO: ADD support
    UndirectedConnectable,
}

impl ConnectionMode {

    /// Gets the ConnMode from a ConnectionMode
    /// 
    /// # Returns
    /// 
    /// The corresponding ConnMode
    pub fn get_code(&self) -> ConnMode {
        match self {
            ConnectionMode::NonConnectable => ConnMode::Non,
            ConnectionMode::DirectedConnectable => ConnMode::Dir,
            ConnectionMode::UndirectedConnectable => ConnMode::Und,
        }
    }
}

/// A struct representing a Bluetooth Low Energy (BLE) service.
/// A BLE service is a container that holds related characteristics. This struct includes:
///
/// - `id`: The unique identifier (`BleId`) of the service, typically a UUID corresponding to a standard
///   or custom BLE service.
/// - `data`: A vector of bytes (`Vec<u8>`) that may store additional service-specific data.
/// - `characteristics`: A vector of `Characteristic` objects that define the various features
///   offered by the service. Each characteristic may have its own unique properties and data.
///
/// This struct is used to define and manage the services offered by a BLE device.
#[derive(Clone)]
pub struct Service {
    pub id: BleId,
    pub data: Vec<u8>,
    pub characteristics: Vec<Characteristic>
}

impl Service {

    /// Creates a new Service
    /// 
    /// # Arguments
    /// 
    /// - `id`: The BleId to identify the service
    /// - `data`: A vector of bytes that represent the data of the service
    /// 
    /// # Returns
    /// 
    /// A `Result` containing the new `Service` instance, or a `BleError` if the
    /// initialization fails.
    /// 
    /// # Errors
    /// 
    /// - `BleError::ServiceTooBig`: If the len of data and the len of the id exceed the maximum size 
    pub fn new(id: &BleId, data: Vec<u8>) -> Result<Service, BleError> {
        let header_bytes = if data.is_empty() {PAYLOAD_FIELD_IDENTIFIER_SIZE} else {PAYLOAD_FIELD_IDENTIFIER_SIZE * 2};
        if data.len() + header_bytes + id.byte_size() > MAX_ADV_PAYLOAD_SIZE {
            Err(BleError::ServiceTooBig)
        } else {
            Ok(Service{id: id.clone(), data, characteristics: vec![]})
        }
    }

    /// Adds a new characteristic to thee service
    /// 
    /// # Arguments
    /// 
    /// - `characteristic`: The Characterisitc struct representing the BLE charactersitic to add
    /// 
    /// # Returns
    /// 
    /// The Service itself
    pub fn add_characteristic(&mut self, characteristic: Characteristic) -> &mut Self {
        self.characteristics.push(characteristic);
        self
    }
}

/// Enums the possible types of Ids:
/// - `StandardService`: The UUIDs of standard Bluetooth Low Energy (BLE) services.
/// - `StandarCharacteristic`: The UUIDs of standard Bluetooth Low Energy (BLE) characteristics.
/// - `ByName`: A string that can be made into a BLE id.
/// - `FromUuid16`: A way to get a BLE id from an u16.
/// - `FromUuid32`: A way to get a BLE id from an u32.
/// - `FromUuid128`: A way to get a BLE id from an [u8;16].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BleId {
    StandardService(StandarServiceId),
    StandarCharacteristic(StandarCharacteristicId),
    ByName(String),
    FromUuid16(u16),
    FromUuid32(u32),
    FromUuid128([u8; 16]),
}


impl Hash for BleId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_uuid().to_string().hash(state)
    }
}

impl From<BleUuid> for BleId{
    fn from(value: BleUuid) -> Self {
        match value{
            BleUuid::Uuid16(id) => BleId::FromUuid16(id),
            BleUuid::Uuid32(id) => BleId::FromUuid32(id),
            BleUuid::Uuid128(id) => BleId::FromUuid128(id),
        }
    }
}

impl From<&BleUuid> for BleId{
    fn from(value: &BleUuid) -> Self {
        Self::from(*value)
    }
}


impl BleId {

    /// Creates a BleUuid from a BleId
    /// 
    /// # Returns
    /// 
    /// The corresponfing BleUuid
    pub fn to_uuid(&self) -> BleUuid {
        match self {
            BleId::StandardService(service) => {BleUuid::from_uuid16(*service as u16)},
            BleId::StandarCharacteristic(characteristic) => {BleUuid::from_uuid16(*characteristic as u16)},
            BleId::ByName(name) => {
                let arr: [u8;4] = Uuid::new_v3(&Uuid::NAMESPACE_OID, name.as_bytes()).into_bytes()[0..4].try_into().unwrap();
                BleUuid::from_uuid32(u32::from_be_bytes(arr))
            },
            BleId::FromUuid16(uuid) => BleUuid::from_uuid16(*uuid),
            BleId::FromUuid32(uuid) => BleUuid::from_uuid32(*uuid),
            BleId::FromUuid128(uuid) => BleUuid::from_uuid128(*uuid),
        }
        
    }

    /// Gets the byte size
    /// 
    /// # Returns
    /// 
    /// The usize representing the byte size
    fn byte_size(&self) -> usize {
        match self {
            BleId::StandardService(service) => service.byte_size(),
            BleId::StandarCharacteristic(characteristic) => characteristic.byte_size(),
            BleId::ByName(_) => 16,
            BleId::FromUuid16(_) => 2,
            BleId::FromUuid32(_) => 4,
            BleId::FromUuid128(_) => 16,
        }
    }
}

pub struct BleAdvertisedDevice {
    device: BLEAdvertisedDevice    
}

impl BleAdvertisedDevice{

    /// Gets the name of the device
    /// 
    /// # Returns
    /// 
    /// A String representing the name
    pub fn name(&self)-> String{
        self.device.name().to_string()
    }

    /// Get the address of the advertising device.
    /// 
    /// # Returns
    /// 
    /// A BLEAddress
    pub fn addr(&self)-> &BLEAddress{
        self.device.addr()
    }

    /// Get the advertisement type.
    /// 
    /// # Returns
    /// 
    /// A AdvType
    pub fn adv_type(&self) -> AdvType {
        self.device.adv_type()
    }

    /// Get the advertisement flags.
    /// 
    /// # Returns
    /// 
    /// An `Option` with the AdvFlag if there is one, otherwise `None`
    pub fn adv_flags(&self) -> Option<AdvFlag> {
        self.device.adv_flags()
    }
    
    pub fn rssi(&self) -> i32 {
        self.device.rssi()
    }

    pub fn get_service_uuids(&self) -> Vec<BleId> {
        self.device.get_service_uuids().map(|id| BleId::from(id)).collect()
    }

    pub fn is_advertising_service(&self, id: &BleId) -> bool {
        self.get_service_uuids().contains(id)
    }

    pub fn get_service_data_list(&self) -> Vec<(BleId, &[u8])> {
        self.device.get_service_data_list()
        .map(|s| (BleId::from(s.uuid()), s.data()))
        .collect()
    }

    pub fn get_service_data(&self, id: BleId) -> Option<(BleId, &[u8])> {
        self.get_service_data_list().into_iter().find(|s| s.0 == id)
    }

    pub fn get_manufacture_data(&self) -> Option<&[u8]> {
        self.device.get_manufacture_data()
    }
}

impl From<&BLEAdvertisedDevice> for BleAdvertisedDevice{
    fn from(value: &BLEAdvertisedDevice) -> Self {
        BleAdvertisedDevice { device: value.clone() }
    }
}


/// Abstracion of the BLE characteristic. Contains:
/// * `id`: The id lets clients identified each service characteristic.
/// * `properties`: Properties especify how the clients will be able to interact with the characteristic.
/// * `data`: The value that the clients will be able to see or write (depending on the properties).
#[derive(Clone)]
pub struct Characteristic{
    pub id: BleId,
    pub properties: u16,
    pub data: Vec<u8>
}

impl Characteristic {

    /// Creates a Characteristic with its id and data.
    /// 
    /// It has no properties, this needs to be set separately.
    pub fn new(id: BleId, data: Vec<u8>) -> Self {
        Characteristic{id, properties: 0, data}
    }

    fn toggle(&mut self, value: bool, flag: NimbleProperties) -> &mut Self {
        if value {
            self.properties |= flag.bits();
        }else {
            self.properties &= !flag.bits();
        }
        self
    }

    /// Adds or removes the writable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client.
    pub fn writable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE)
    }

    /// Adds or removes the readeable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be read by the client.
    pub fn readeable(&mut self, value: bool) -> &mut Self{
        self.toggle(value, NimbleProperties::READ)
    }
    
    /// Adds or removes the notifiable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be published to the client, without waiting for an acknowledgement.
    pub fn notifiable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::NOTIFY)
    }

    /// Adds or removes the readeable_enc characteristic to the properties.
    /// 
    /// It allows the characteristics data to be read by the client, only when the communication is encrypted.
    pub fn readeable_enc(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_ENC)
    }

    /// Adds or removes the readeable_authen characteristic to the properties.
    /// 
    /// It allows the characteristics data to be read by the client, only when the communication is authenticated.
    pub fn readeable_authen(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_AUTHEN)
    }

    /// Adds or removes the readeable_author characteristic to the properties.
    /// 
    /// It allows the characteristics data to be read by the client, only when authorized by the server.
    pub fn readeable_author(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_AUTHOR)
   
    }

    /// Adds or removes the writeable_no_rsp characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client, without waiting for a response.
    pub fn writeable_no_rsp(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_NO_RSP)
    }

    /// Adds or removes the writeable_enc characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client, only when the communication is encrypted.
    pub fn writeable_enc(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_ENC)
    }

    /// Adds or removes the writeable_authen characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client, only when the communication is authenticated.
    pub fn writeable_authen(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_AUTHEN)
    }

    /// Adds or removes the writeable_author characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client, only when authorized by the server.
    pub fn writeable_author(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_AUTHOR)
    }

    /// Adds or removes the broadcastable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be broadcasted by the server.
    pub fn broadcastable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::BROADCAST)
    }

    /// Adds or removes the indicatable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be published to the client and waits for an acknowledgement.
    pub fn indicatable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::INDICATE)
    }
    
    /// Sets a new data to the characteristic.
    /// 
    /// When updating the data, the server needs to be notified about the characteristic data change. If not,
    /// server will never use the new values and clients will never get the last information. 
    pub fn update_data(&mut self, data: Vec<u8>) -> &mut Self{
        self.data = data;
        self
    }

}

/// Enums the device's input and output capabilities, 
/// which help determine the level of security and the key
/// generation method for pairing:
/// * `DisplayOnly`: It is capable of displaying information on a 
/// screen but cannot receive inputs.
/// * `DisplayYesNo`: It can display information and/or yes/no questions, 
/// allowing for limited interaction.
/// * `KeyboardOnly`: It can receive input through a keyboard 
/// (e.g., entering a PIN during pairing).
/// * `NoInputNoOutput`: It has no means to display information or 
/// receive input from, for example, keyboards or buttons.
/// * `KeyboardDisplay`: It can receive input through a keyboard and it 
/// is capable of displaying information.
pub enum IOCapabilities {
    DisplayOnly,
    DisplayYesNo,
    KeyboardOnly,
    NoInputNoOutput,
    KeyboardDisplay,
}

impl IOCapabilities {
    pub fn get_code(&self) -> SecurityIOCap {
        match self {
            IOCapabilities::DisplayOnly => SecurityIOCap::DisplayOnly,
            IOCapabilities::DisplayYesNo => SecurityIOCap::DisplayYesNo,
            IOCapabilities::KeyboardOnly => SecurityIOCap::KeyboardOnly,
            IOCapabilities::NoInputNoOutput => SecurityIOCap::NoInputNoOutput,
            IOCapabilities::KeyboardDisplay => SecurityIOCap::KeyboardDisplay,
        }
    }
}
/// Contains the necessary to have a secure BLE server.
/// This includes a passkey, the I/O capabilities and the
/// authorization requirements.
pub struct Security {
    pub passkey: u32, // TODO: I think the passkey can only be 6 digits long. If so, add a step that checks this
    pub auth_mode: u8,
    pub io_capabilities: IOCapabilities,
}

impl Security {

    /// Creates a Security with its passkey and I/O capabilities. 
    /// 
    /// It has no authentication requirements, this need to be set separately
    pub fn new(passkey: u32, io_capabilities: IOCapabilities) -> Self {
        Security { passkey, auth_mode: 0, io_capabilities }
    }

    fn toggle(&mut self, value: bool, flag: AuthReq) -> &mut Self {
        if value {
            self.auth_mode |= flag.bits();
        }else {
            self.auth_mode &= !flag.bits();
        }
        self
    }

    /// Sets the Allow Bonding authorization requirement.
    /// 
    /// When the bonding is allowed, devices remember the 
    /// pairing information. This allows to make future conexions to be faster
    /// and more secure. Useful for devices that get connected with frequency.
    pub fn allow_bonding(&mut self, value: bool) -> &mut Self {
        self.toggle(value, AuthReq::Bond);
        self
    }

    /// Sets the Man in the Middle authorization requirement.
    /// 
    /// Authentication requires a verification
    /// that makes it hard for a third party to intercept the communication.
    pub fn man_in_the_middle(&mut self, value: bool) -> &mut Self {
        self.toggle(value, AuthReq::Mitm);
        self
    }

    /// Sets the Secure Connection authorization requirement. 
    /// 
    /// This is a more secure version of BLE pairing by using the 
    /// elliptic curve Diffie-Hellman algorithm. This is part of standard Bluetooth 4.2 and newer versions. 
    pub fn secure_connection(&mut self, value: bool) -> &mut Self {
        self.toggle(value, AuthReq::Sc);
        self
    }
}