use std::{cell::{Ref, RefCell, RefMut}, sync::Arc, rc::Rc};
use esp_idf_svc::{hal::{delay::BLOCK, task::queue::Queue}, sys::configTICK_RATE_HZ};

pub type SharableRef<T> = Rc<RefCell<T>>;

/// Error types related to ISR queue operations.
#[derive(Debug)]
pub enum ISRQueueError {
    Timeout,
    Empty,
}

/// Trait to handle sherable references.
pub trait SharableRefExt<T>{
    /// Creates a new SharableRef.
    ///
    /// # Arguments
    ///
    /// - `inner`: The value to wrap in a sharable reference.
    ///
    /// # Returns
    /// 
    /// A new `SharableRef<T>` wrapping the inner value.
    fn new_sharable(inner: T) -> SharableRef<T>;
    
    /// Returns a shared reference to the inner value.
    ///
    /// # Returns
    ///
    /// A `Ref<T>` sharing ownership of the inner value.
    fn deref(&self) -> Ref<T>;
        
    /// Returns a mutable shared reference to the inner value.
    ///
    /// # Returns
    ///
    /// A `RefMut<T>` sharing mutable ownership of the inner value.
    fn deref_mut(&mut self) -> RefMut<T>;
}


impl<T> SharableRefExt<T> for SharableRef<T>{
    fn new_sharable(inner: T) -> SharableRef<T>{
        Rc::new(RefCell::new(inner))
    }
    fn deref_mut(&mut self) -> RefMut<T> {
        self.borrow_mut()
    }
    fn deref(&self) -> Ref<T>{
        self.borrow()
    }
}

/// A queue that wraper for a Queue in an `Arc<Queue<T>>` for shared ownership, in order to share data.
#[derive(Clone)]
pub struct ISRQueue<T: Copy>{
    q: Arc<Queue<T>>
}

/// A wrapper of `ISRQueue` to share Vec<u8> easily.
#[derive(Clone)]
pub struct ISRByteArrayQueue{
    q: ISRQueue<Option<u8>>
}

impl<T: Copy> ISRQueue<T>{
    /// Creates a new empty `ISRQueue` with the given initial capacity.
    ///
    /// # Arguments
    ///
    /// * `size` - The initial capacity of the queue.
    ///
    /// # Returns
    ///
    /// A new `ISRQueue<T>` instance.
    pub fn new(size: usize)-> Self{
        Self{q: Arc::new(Queue::new(size))}
    }
}

impl <T:Copy> ISRQueueTrait<T> for ISRQueue<T>{
    fn send_timeout(&mut self,item: T, micro: u32)-> Result<(), ISRQueueError> {
        match self.q.send_back(item, micro_to_ticks(micro)) {
            Ok(_) => Ok(()), 
            Err(_) => Err(ISRQueueError::Timeout)
        }
    }
    
    fn receive_timeout(&mut self, micro: u32)-> Result<T, ISRQueueError> {
        match self.q.recv_front(micro_to_ticks(micro)){
            Some((item, _)) => Ok(item), 
            None => Err(ISRQueueError::Timeout)
        }
    }
}

impl ISRByteArrayQueue{
    /// Creates a new empty `ISRByteArrayQueue` with the given initial capacity, capable of holding 32 * size bytes.
    ///
    /// # Arguments
    ///
    /// * `size` - The initial capacity of the queue.
    ///
    /// # Returns
    ///
    /// A new `ISRByteArrayQueue` instance.
    pub fn new(size: usize)-> Self{
        Self { q: ISRQueue::new(size * 32) }
    }
}

impl ISRQueueTrait<Vec<u8>> for ISRByteArrayQueue{
    fn send_timeout(&mut self, item: Vec<u8>, micro: u32)-> Result<(), ISRQueueError> {
        for byte in item{
            self.q.send_timeout(Some(byte), micro)?
        }
        self.q.send_timeout(None, micro)
    }

    fn receive_timeout(&mut self, micro: u32)-> Result<Vec<u8>, ISRQueueError> {
        let mut byte_vec = vec![];
        while let Some(byte) = self.q.receive_timeout(micro)?{
            byte_vec.push(byte)
        }
        Ok(byte_vec)
    }
}

pub trait ISRQueueTrait<T>{
    
    /// Attempts to send an item to the queue blocking until space is available or a 
    /// timeout occurs.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to attempt sending to the queue.
    /// * `micro` - The maximum duration to wait in microseconds.
    ///
    /// # Returns
    ///
    /// A `Result` with Ok if the read operation completed successfully, or an 
    /// `ISRQueueError` if it fails.
    /// 
    ///  # Errors
    ///
    /// - `ISRQueueError::Timeout`: If the operation exceeded the specified timeout.
    fn send_timeout(&mut self, item: T, micro: u32)-> Result<(), ISRQueueError>;
  
    
    /// Receives an item from the front of the queue, blocking until an item is 
    /// available or a timeout occurs.
    ///
    /// # Arguments
    ///
    /// * `micro` - The maximum duration to wait in microseconds.
    /// 
    /// # Returns
    ///
    /// The item if it was successfully received or an `ISRQueueError` if it fails.
    /// 
    /// # Errors
    /// 
    /// - `ISRQueueError::Timeout`: if the timeout occurred before an item became available.
    fn receive_timeout(&mut self, micro: u32)-> Result<T, ISRQueueError>;

    /// Sends an item to the queue, blocking until space is available.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to send to the queue.
    fn send(&mut self, item: T){
        self.send_timeout(item, BLOCK).unwrap()
    }
    
    /// Attempts to send an item to the queue without blocking.
    ///
    /// # Arguments
    ///
    /// * `item` - The item to attempt sending to the queue.
    ///
    /// # Returns
    ///
    /// A `Result` with Ok if the read operation completed successfully, or an 
    /// `ISRQueueError` if it fails.
    /// 
    ///  # Errors
    ///
    /// - `ISRQueueError::Timeout`: If the operation fails.
    fn try_send(&mut self, item: T) -> Result<(), ISRQueueError>{
        self.send_timeout(item, 0).map_err(|_| ISRQueueError::Empty)
    }

    /// Receives an item from the front of the queue, blocking until an item is available.
    ///
    /// # Returns
    ///
    /// The received item from the queue.
    fn receive(&mut self) -> T {
        self.receive_timeout(BLOCK).unwrap()
    }
    
    /// Attempts to receive an item from the front of the queue without blocking.
    ///
    /// # Returns
    ///
    /// The item if it was successfully received or an `ISRQueueError` if it fails.
    /// 
    /// # Errors
    /// 
    /// - `ISRQueueError::Empty`: if there were no items available in the queue.
    fn try_recv(&mut self) -> Result<T, ISRQueueError> {
        self.receive_timeout(0).map_err(|_| ISRQueueError::Empty)
    }
}

/// Converts microseconds to system ticks based on the configured tick rate.
///
/// # Arguments
///
/// * `time_us` - The duration in microseconds.
///
/// # Returns
///
/// The converted duration in ticks using a u32 value.
pub fn micro_to_ticks(time_us: u32) -> u32 {
    ((configTICK_RATE_HZ as u64) * (time_us as u64) / 1_000_000_u64) as u32
}
