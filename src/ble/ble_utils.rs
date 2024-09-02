use esp32_nimble::{enums::{AuthReq, ConnMode, DiscMode, SecurityIOCap}, utilities::BleUuid, BLEError, NimbleProperties};
use uuid::Uuid;
use crate::utils::timer_driver::TimerDriverError;

use super::{StandarCharacteristicId, StandarServiceId};
use std::hash::Hash;

const MAX_ADV_PAYLOAD_SIZE: usize = 31;
const PAYLOAD_FIELD_IDENTIFIER_SIZE: usize = 2;


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
}

impl From<BLEError> for BleError {
    fn from(value: BLEError) -> Self {
        match value.code() {
            esp_idf_svc::sys::BLE_HS_EMSGSIZE => BleError::ServiceDoesNotFit,
            _ => BleError::Code(value.code(), value.to_string()),
        }
    }
}

impl BleError {
        
    fn from_code(code: u32) -> Option<BleError> {
        match BLEError::convert(code) {
            Ok(_) => None,
            Err(err) => Some(BleError::from(err)),
        }
    }
}

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
    pub fn get_code(&self) -> DiscMode {
        match self {
            DiscoverableMode::NonDiscoverable => DiscMode::Non,
            DiscoverableMode::LimitedDiscoverable(_, _) => DiscMode::Ltd ,
            DiscoverableMode::GeneralDiscoverable(_, _) => DiscMode::Gen,
        }
    }
}

/// * `NonConnectable`: The device does not allow connections.
/// * `DirectedConnectable`: The device only allows connections from a specific device.
/// * `UndirectedConnectable`: The divice allows connections from any device.
pub enum ConnectionMode {
    NonConnectable,
    DirectedConnectable, //TODO: ADD support
    UndirectedConnectable,
}

impl ConnectionMode {
    pub fn get_code(&self) -> ConnMode {
        match self {
            ConnectionMode::NonConnectable => ConnMode::Non,
            ConnectionMode::DirectedConnectable => ConnMode::Dir,
            ConnectionMode::UndirectedConnectable => ConnMode::Und,
        }
    }
}

#[derive(Clone)]
pub struct Service {
    pub id: BleId,
    pub data: Vec<u8>,
    pub characteristics: Vec<Characteristic>
}

impl Service {
    pub fn new(id: &BleId, data: Vec<u8>) -> Result<Service, BleError> {
        let header_bytes = if data.is_empty() {PAYLOAD_FIELD_IDENTIFIER_SIZE} else {PAYLOAD_FIELD_IDENTIFIER_SIZE * 2};
        if data.len() + header_bytes + id.byte_size() > MAX_ADV_PAYLOAD_SIZE {
            Err(BleError::ServiceTooBig)
        } else {
            Ok(Service{id: id.clone(), data, characteristics: vec![]})
        }
    }

    pub fn add_characteristic(&mut self, characteristic: Characteristic) -> &mut Self {
        self.characteristics.push(characteristic);
        self
    }
}

/// in case of repeated name service (using ByName), the first one will be overwritten
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

impl BleId {
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

    fn byte_size(&self) -> usize{
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


#[derive(Clone)]
pub struct Characteristic{
    pub id: BleId,
    pub properties: u16,
    pub data: Vec<u8>
}

impl Characteristic {
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

    pub fn writable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE)
    }

    pub fn readeable(&mut self, value: bool) -> &mut Self{
        self.toggle(value, NimbleProperties::READ)
    }
    
    pub fn notifiable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::NOTIFY)
    }

    pub fn readeable_enc(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_ENC)
    }

    pub fn readeable_authen(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_AUTHEN)
    }

    pub fn readeable_author(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_AUTHOR)
   
    }

    pub fn writeable_no_rsp(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_NO_RSP)
    }

    pub fn writeable_enc(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_ENC)
    }

    pub fn writeable_authen(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_AUTHEN)
    }

    pub fn writeable_author(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_AUTHOR)
    }

    pub fn broadcastable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::BROADCAST)
    }

    pub fn indicatable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::INDICATE)
    }
    
    pub fn update_data(&mut self, data: Vec<u8>) -> &mut Self{
        self.data = data;
        self
    }

}

/// This field specifies the device's input and output capabilities, 
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

pub struct Security {
    pub passkey: u32, // TODO: I think the passkey can only be 6 digits long. If so, add a step that checks this
    pub auth_mode: u8,
    pub io_capabilities: IOCapabilities,
}

impl Security {
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

    /// Sets the Allow Bonding authorization requirement. When the bonding is allowed, devices remember the 
    /// pairing information. This allows to make future conexions to be faster
    /// and more secure. Useful for devices that get connected with frequency.
    pub fn allow_bonding(&mut self, value: bool) -> &mut Self {
        self.toggle(value, AuthReq::Bond);
        self
    }

    /// Sets the Man in the Middle authorization requirement. Authentication requires a verification
    /// that makes it hard for a third party to intercept the communication.
    pub fn man_in_the_middle(&mut self, value: bool) -> &mut Self {
        self.toggle(value, AuthReq::Mitm);
        self
    }

    /// Sets the Secure Connection authorization requirement. 
    /// This is a more secure version of BLE pairing by using the 
    /// elliptic curve Diffie-Hellman algorithm. This is part of standard Bluetooth 4.2 and newer versions. 
    pub fn secure_connection(&mut self, value: bool) -> &mut Self {
        self.toggle(value, AuthReq::Sc);
        self
    }
}