pub mod neuron;
pub mod connection;
pub mod network;

pub trait Serialization<T> {
    fn deserialize(s: &str) -> T;
    fn serialize(&self) -> String;
}