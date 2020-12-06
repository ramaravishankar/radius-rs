// Code generated by machine generator; DO NOT EDIT.

use crate::avp::{AVPError, AVPType, AVP};
use crate::packet::Packet;

pub const GSS_ACCEPTOR_SERVICE_NAME_TYPE: AVPType = 164;
pub fn delete_gss_acceptor_service_name(packet: &mut Packet) {
    packet.delete(GSS_ACCEPTOR_SERVICE_NAME_TYPE);
}
pub fn add_gss_acceptor_service_name(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(GSS_ACCEPTOR_SERVICE_NAME_TYPE, value));
}
pub fn lookup_gss_acceptor_service_name(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(GSS_ACCEPTOR_SERVICE_NAME_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_gss_acceptor_service_name(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(GSS_ACCEPTOR_SERVICE_NAME_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const GSS_ACCEPTOR_HOST_NAME_TYPE: AVPType = 165;
pub fn delete_gss_acceptor_host_name(packet: &mut Packet) {
    packet.delete(GSS_ACCEPTOR_HOST_NAME_TYPE);
}
pub fn add_gss_acceptor_host_name(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(GSS_ACCEPTOR_HOST_NAME_TYPE, value));
}
pub fn lookup_gss_acceptor_host_name(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(GSS_ACCEPTOR_HOST_NAME_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_gss_acceptor_host_name(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(GSS_ACCEPTOR_HOST_NAME_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const GSS_ACCEPTOR_SERVICE_SPECIFICS_TYPE: AVPType = 166;
pub fn delete_gss_acceptor_service_specifics(packet: &mut Packet) {
    packet.delete(GSS_ACCEPTOR_SERVICE_SPECIFICS_TYPE);
}
pub fn add_gss_acceptor_service_specifics(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(GSS_ACCEPTOR_SERVICE_SPECIFICS_TYPE, value));
}
pub fn lookup_gss_acceptor_service_specifics(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(GSS_ACCEPTOR_SERVICE_SPECIFICS_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_gss_acceptor_service_specifics(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(GSS_ACCEPTOR_SERVICE_SPECIFICS_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}

pub const GSS_ACCEPTOR_REALM_NAME_TYPE: AVPType = 167;
pub fn delete_gss_acceptor_realm_name(packet: &mut Packet) {
    packet.delete(GSS_ACCEPTOR_REALM_NAME_TYPE);
}
pub fn add_gss_acceptor_realm_name(packet: &mut Packet, value: &str) {
    packet.add(AVP::from_string(GSS_ACCEPTOR_REALM_NAME_TYPE, value));
}
pub fn lookup_gss_acceptor_realm_name(packet: &Packet) -> Option<Result<String, AVPError>> {
    packet
        .lookup(GSS_ACCEPTOR_REALM_NAME_TYPE)
        .map(|v| v.encode_string())
}
pub fn lookup_all_gss_acceptor_realm_name(packet: &Packet) -> Result<Vec<String>, AVPError> {
    let mut vec = Vec::new();
    for avp in packet.lookup_all(GSS_ACCEPTOR_REALM_NAME_TYPE) {
        vec.push(avp.encode_string()?)
    }
    Ok(vec)
}
