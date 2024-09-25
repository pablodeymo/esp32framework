use esp32_nimble::{DescriptorProperties, NimbleProperties};

use super::{BleError, BleId};

const MAX_ADV_PAYLOAD_SIZE: usize = 31;
const PAYLOAD_FIELD_IDENTIFIER_SIZE: usize = 2;

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

/// Abstracion of the BLE characteristic. Contains:
/// - `id`: The id lets clients identified each service characteristic.
/// - `properties`: Properties especify how the clients will be able to interact with the characteristic.
/// - `data`: The value that the clients will be able to see or write (depending on the properties).
#[derive(Clone)]
pub struct Characteristic{
    pub id: BleId,
    pub properties: u16,
    pub data: Vec<u8>,
    pub descriptors: Vec<Descriptor>,
}

impl Characteristic {

    /// Creates a Characteristic with its id and data.
    /// It has no properties, this needs to be set separately.
    /// 
    /// # Arguments
    /// 
    /// - `id`: The BleId to identify the characteristic
    /// - `data`: A vector of bytes representing the desired data
    /// 
    /// # Returns
    /// 
    /// The new Characteristic
    pub fn new(id: BleId, data: Vec<u8>) -> Self {
        Characteristic{id, properties: 0, data, descriptors: vec![]}
    }

    pub fn add_descriptor(&mut self, descriptor: Descriptor) -> &mut Self {
        self.descriptors.push(descriptor);
        self
    }

    /// Adds or removes a property to the characteristic
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed
    /// - `flag`: The NimbleProperty to add or remove
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
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
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn writable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE)
    }

    /// Adds or removes the readeable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be read by the client.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn readeable(&mut self, value: bool) -> &mut Self{
        self.toggle(value, NimbleProperties::READ)
    }
    
    /// Adds or removes the notifiable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be published to the client, without waiting for an acknowledgement.
    ///  
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn notifiable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::NOTIFY)
    }

    /// Adds or removes the readeable_enc characteristic to the properties.
    /// 
    /// It allows the characteristics data to be read by the client, only when the communication is encrypted.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn readeable_enc(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_ENC)
    }

    /// Adds or removes the readeable_authen characteristic to the properties.
    /// 
    /// It allows the characteristics data to be read by the client, only when the communication is authenticated.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn readeable_authen(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_AUTHEN)
    }

    /// Adds or removes the readeable_author characteristic to the properties.
    /// 
    /// It allows the characteristics data to be read by the client, only when authorized by the server.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn readeable_author(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::READ_AUTHOR)
   
    }

    /// Adds or removes the writeable_no_rsp characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client, without waiting for a response.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn writeable_no_rsp(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_NO_RSP)
    }

    /// Adds or removes the writeable_enc characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client, only when the communication is encrypted.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn writeable_enc(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_ENC)
    }

    /// Adds or removes the writeable_authen characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client, only when the communication is authenticated.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn writeable_authen(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_AUTHEN)
    }

    /// Adds or removes the writeable_author characteristic to the properties.
    /// 
    /// It allows the characteristics data to be written by the client, only when authorized by the server.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn writeable_author(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::WRITE_AUTHOR)
    }

    /// Adds or removes the broadcastable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be broadcasted by the server.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn broadcastable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::BROADCAST)
    }

    /// Adds or removes the indicatable characteristic to the properties.
    /// 
    /// It allows the characteristics data to be published to the client and waits for an acknowledgement.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn indicatable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, NimbleProperties::INDICATE)
    }
    
    /// Sets a new data to the characteristic.
    /// 
    /// When updating the data, the server needs to be notified about the characteristic data change. If not,
    /// server will never use the new values and clients will never get the last information.
    /// 
    /// # Arguments
    /// 
    /// - `data`: A vector of bytes representing the updated data
    /// 
    /// # Returns
    /// 
    /// The Characteristic itself
    pub fn update_data(&mut self, data: Vec<u8>) -> &mut Self{
        self.data = data;
        self
    }

}


#[derive(Debug, Clone)]
pub struct Descriptor {
    pub id: BleId,
    pub properties: u8,
    pub data: Vec<u8>,
}

impl Descriptor {

    /// Creates a Descriptor with its id and data.
    /// It has no properties, this needs to be set separately.
    /// 
    /// # Arguments
    /// 
    /// - `id`: The BleId to identify the Descriptor
    /// - `data`: A vector of bytes representing the desired data
    /// 
    /// # Returns
    /// 
    /// The new Descriptor
    pub fn new(id: BleId, data: Vec<u8>) -> Self {
        Descriptor{id, properties: 0, data}
    }

    /// Get the properties of a Descriptor.
    /// 
    /// # Returns
    /// 
    /// A `Result` with the properties if the operation completed successfully, or an `BleError` if it fails.
    /// 
    /// # Errors
    /// 
    /// - `BleError::PropertiesError`: If a Descriptor has an invalid property or no properties at all.
    pub fn get_properties(&self) -> Result<DescriptorProperties, BleError> {
        match DescriptorProperties::from_bits(self.properties.to_le()) {
            Some(properties) => Ok(properties),
            None => Err(BleError::PropertiesError),
        }
    }

    /// Adds or removes a property to the descriptor
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed
    /// - `flag`: The DescriptorProperties to add or remove
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    fn toggle(&mut self, value: bool, flag: DescriptorProperties) -> &mut Self {
        if value {
            self.properties |= flag.bits();
        }else {
            self.properties &= !flag.bits();
        }
        self
    }
    
    /// Adds or removes the writable flag to the properties.
    /// 
    /// It allows the descriptors data to be written by the client.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    pub fn writable(&mut self, value: bool) -> &mut Self {
        self.toggle(value, DescriptorProperties::WRITE)
    }

    /// Adds or removes the readeable flag to the properties.
    /// 
    /// It allows the descriptors data to be read by the client.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    pub fn readeable(&mut self, value: bool) -> &mut Self{
        self.toggle(value, DescriptorProperties::READ)
    }

    /// Adds or removes the readeable_enc flag to the properties.
    /// 
    /// It allows the descriptor data to be read by the client, only when the communication is encrypted.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    pub fn readeable_enc(&mut self, value: bool) -> &mut Self {
        self.toggle(value, DescriptorProperties::READ_ENC)
    }

    /// Adds or removes the readeable_authen flag to the properties.
    /// 
    /// It allows the descriptor data to be read by the client, only when the communication is authenticated.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    pub fn readeable_authen(&mut self, value: bool) -> &mut Self {
        self.toggle(value, DescriptorProperties::READ_AUTHEN)
    }

    /// Adds or removes the readeable_author flag to the properties.
    /// 
    /// It allows the descriptor data to be read by the client, only when authorized by the server.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    pub fn readeable_author(&mut self, value: bool) -> &mut Self {
        self.toggle(value, DescriptorProperties::READ_AUTHOR)
   
    }

    /// Adds or removes the writeable_enc flag to the properties.
    /// 
    /// It allows the descriptor data to be written by the client, only when the communication is encrypted.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    pub fn writeable_enc(&mut self, value: bool) -> &mut Self {
        self.toggle(value, DescriptorProperties::WRITE_ENC)
    }

    /// Adds or removes the writeable_authen flag to the properties.
    /// 
    /// It allows the descriptor data to be written by the client, only when the communication is authenticated.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    pub fn writeable_authen(&mut self, value: bool) -> &mut Self {
        self.toggle(value, DescriptorProperties::WRITE_AUTHEN)
    }

    /// Adds or removes the writeable_author flag to the properties.
    /// 
    /// It allows the descriptor data to be written by the client, only when authorized by the server.
    /// 
    /// # Arguments
    /// 
    /// - `value`: A bool. When True the property is added. When False the property is removed.
    /// 
    /// # Returns
    /// 
    /// The Descriptor itself
    pub fn writeable_author(&mut self, value: bool) -> &mut Self {
        self.toggle(value, DescriptorProperties::WRITE_AUTHOR)
    }
}
