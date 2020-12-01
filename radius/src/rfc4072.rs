// Code generated by machine generator; DO NOT EDIT.

use crate::avp::{AVPType, AVP};
use crate::packet::Packet;

pub const EAP_KEY_NAME_TYPE: AVPType = 102;
pub fn delete_eap_key_name(packet: &mut Packet) {
    packet.delete(EAP_KEY_NAME_TYPE);
}
pub fn add_eap_key_name(packet: &mut Packet, value: &[u8]) {
    packet.add(AVP::from_bytes(EAP_KEY_NAME_TYPE, value));
}
pub fn lookup_eap_key_name(packet: &Packet) -> Option<Vec<u8>> {
    packet.lookup(EAP_KEY_NAME_TYPE).map(|v| v.encode_bytes())
}
pub fn lookup_all_eap_key_name(packet: &Packet) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(EAP_KEY_NAME_TYPE) {
        vec.push(avp.encode_bytes())
    }
    vec
}
