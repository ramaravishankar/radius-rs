// Code generated by machine generator; DO NOT EDIT.

use std::net::Ipv4Addr;

use crate::attribute::Attribute;
use crate::attributes::AVPType;
use crate::packet::Packet;

pub type FramedCompression = u32;
pub const FRAMED_COMPRESSION_NONE: FramedCompression = 0;
pub const FRAMED_COMPRESSION_VAN_JACOBSON_TCP_IP: FramedCompression = 1;
pub const FRAMED_COMPRESSION_IPX_HEADER_COMPRESSION: FramedCompression = 2;
pub const FRAMED_COMPRESSION_STAC_LZS: FramedCompression = 3;

pub type FramedProtocol = u32;
pub const FRAMED_PROTOCOL_PPP: FramedProtocol = 1;
pub const FRAMED_PROTOCOL_SLIP: FramedProtocol = 2;
pub const FRAMED_PROTOCOL_ARAP: FramedProtocol = 3;
pub const FRAMED_PROTOCOL_GANDALF_SLML: FramedProtocol = 4;
pub const FRAMED_PROTOCOL_XYLOGICS_IPX_SLIP: FramedProtocol = 5;
pub const FRAMED_PROTOCOL_X_75_SYNCHRONOUS: FramedProtocol = 6;

pub type FramedRouting = u32;
pub const FRAMED_ROUTING_NONE: FramedRouting = 0;
pub const FRAMED_ROUTING_BROADCAST: FramedRouting = 1;
pub const FRAMED_ROUTING_LISTEN: FramedRouting = 2;
pub const FRAMED_ROUTING_BROADCAST_LISTEN: FramedRouting = 3;

pub type LoginService = u32;
pub const LOGIN_SERVICE_TELNET: LoginService = 0;
pub const LOGIN_SERVICE_RLOGIN: LoginService = 1;
pub const LOGIN_SERVICE_TCP_CLEAR: LoginService = 2;
pub const LOGIN_SERVICE_PORT_MASTER: LoginService = 3;
pub const LOGIN_SERVICE_LAT: LoginService = 4;
pub const LOGIN_SERVICE_X25_PAD: LoginService = 5;
pub const LOGIN_SERVICE_X25_T3POS: LoginService = 6;
pub const LOGIN_SERVICE_TCP_CLEAR_QUIET: LoginService = 8;

pub type LoginTCPPort = u32;
pub const LOGIN_TCP_PORT_TELNET: LoginTCPPort = 23;
pub const LOGIN_TCP_PORT_RLOGIN: LoginTCPPort = 513;
pub const LOGIN_TCP_PORT_RSH: LoginTCPPort = 514;

pub type NasPortType = u32;
pub const NAS_PORT_TYPE_ASYNC: NasPortType = 0;
pub const NAS_PORT_TYPE_SYNC: NasPortType = 1;
pub const NAS_PORT_TYPE_ISDN: NasPortType = 2;
pub const NAS_PORT_TYPE_ISDN_V120: NasPortType = 3;
pub const NAS_PORT_TYPE_ISDN_V110: NasPortType = 4;
pub const NAS_PORT_TYPE_VIRTUAL: NasPortType = 5;
pub const NAS_PORT_TYPE_PIAFS: NasPortType = 6;
pub const NAS_PORT_TYPE_HDLC_CLEAR_CHANNEL: NasPortType = 7;
pub const NAS_PORT_TYPE_X_25: NasPortType = 8;
pub const NAS_PORT_TYPE_X_75: NasPortType = 9;
pub const NAS_PORT_TYPE_G_3_FAX: NasPortType = 10;
pub const NAS_PORT_TYPE_SDSL: NasPortType = 11;
pub const NAS_PORT_TYPE_ADSL_CAP: NasPortType = 12;
pub const NAS_PORT_TYPE_ADSL_DMT: NasPortType = 13;
pub const NAS_PORT_TYPE_IDSL: NasPortType = 14;
pub const NAS_PORT_TYPE_ETHERNET: NasPortType = 15;
pub const NAS_PORT_TYPE_X_DSL: NasPortType = 16;
pub const NAS_PORT_TYPE_CABLE: NasPortType = 17;
pub const NAS_PORT_TYPE_WIRELESS_OTHER: NasPortType = 18;
pub const NAS_PORT_TYPE_WIRELESS_802_11: NasPortType = 19;

pub type ServiceType = u32;
pub const SERVICE_TYPE_LOGIN_USER: ServiceType = 1;
pub const SERVICE_TYPE_FRAMED_USER: ServiceType = 2;
pub const SERVICE_TYPE_CALLBACK_LOGIN_USER: ServiceType = 3;
pub const SERVICE_TYPE_CALLBACK_FRAMED_USER: ServiceType = 4;
pub const SERVICE_TYPE_OUTBOUND_USER: ServiceType = 5;
pub const SERVICE_TYPE_ADMINISTRATIVE_USER: ServiceType = 6;
pub const SERVICE_TYPE_NAS_PROMPT_USER: ServiceType = 7;
pub const SERVICE_TYPE_AUTHENTICATE_ONLY: ServiceType = 8;
pub const SERVICE_TYPE_CALLBACK_NAS_PROMPT: ServiceType = 9;
pub const SERVICE_TYPE_CALL_CHECK: ServiceType = 10;
pub const SERVICE_TYPE_CALLBACK_ADMINISTRATIVE: ServiceType = 11;

pub type TerminationAction = u32;
pub const TERMINATION_ACTION_DEFAULT: TerminationAction = 0;
pub const TERMINATION_ACTION_RADIUS_REQUEST: TerminationAction = 1;

pub const USER_NAME_TYPE: AVPType = 1;
pub fn delete_user_name(packet: &mut Packet) {
    packet.delete(USER_NAME_TYPE);
}
pub fn lookup_user_name(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(USER_NAME_TYPE)
}
pub fn lookup_all_user_name(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(USER_NAME_TYPE)
}
pub fn add_user_name(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(USER_NAME_TYPE, &attr);
}

pub const USER_PASSWORD_TYPE: AVPType = 2;
pub fn delete_user_password(packet: &mut Packet) {
    packet.delete(USER_PASSWORD_TYPE);
}
pub fn lookup_user_password(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(USER_PASSWORD_TYPE)
}
pub fn lookup_all_user_password(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(USER_PASSWORD_TYPE)
}
pub fn add_user_password(packet: &mut Packet, value: &[u8]) -> Result<(), String> {
    let attr =
        Attribute::from_user_password(value, packet.get_secret(), packet.get_authenticator())?;
    packet.add(USER_PASSWORD_TYPE, &attr);
    Ok(())
}

pub const CHAP_PASSWORD_TYPE: AVPType = 3;
pub fn delete_chap_password(packet: &mut Packet) {
    packet.delete(CHAP_PASSWORD_TYPE);
}
pub fn lookup_chap_password(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(CHAP_PASSWORD_TYPE)
}
pub fn lookup_all_chap_password(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(CHAP_PASSWORD_TYPE)
}
pub fn add_chap_password(packet: &mut Packet, value: &[u8]) {
    let attr = Attribute::from_bytes(value);
    packet.add(CHAP_PASSWORD_TYPE, &attr);
}

pub const NAS_IP_ADDRESS_TYPE: AVPType = 4;
pub fn delete_nas_ip_address(packet: &mut Packet) {
    packet.delete(NAS_IP_ADDRESS_TYPE);
}
pub fn lookup_nas_ip_address(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(NAS_IP_ADDRESS_TYPE)
}
pub fn lookup_all_nas_ip_address(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(NAS_IP_ADDRESS_TYPE)
}
pub fn add_nas_ip_address(packet: &mut Packet, value: &Ipv4Addr) {
    let attr = Attribute::from_ipv4(value);
    packet.add(NAS_IP_ADDRESS_TYPE, &attr);
}

pub const NAS_PORT_TYPE: AVPType = 5;
pub fn delete_nas_port(packet: &mut Packet) {
    packet.delete(NAS_PORT_TYPE);
}
pub fn lookup_nas_port(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(NAS_PORT_TYPE)
}
pub fn lookup_all_nas_port(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(NAS_PORT_TYPE)
}
pub fn add_nas_port(packet: &mut Packet, value: u32) {
    let attr = Attribute::from_u32(value);
    packet.add(NAS_PORT_TYPE, &attr);
}

pub const SERVICE_TYPE_TYPE: AVPType = 6;
pub fn delete_service_type(packet: &mut Packet) {
    packet.delete(SERVICE_TYPE_TYPE);
}
pub fn lookup_service_type(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(SERVICE_TYPE_TYPE)
}
pub fn lookup_all_service_type(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(SERVICE_TYPE_TYPE)
}
pub fn add_service_type(packet: &mut Packet, value: ServiceType) {
    let attr = Attribute::from_u32(value as u32);
    packet.add(SERVICE_TYPE_TYPE, &attr);
}

pub const FRAMED_PROTOCOL_TYPE: AVPType = 7;
pub fn delete_framed_protocol(packet: &mut Packet) {
    packet.delete(FRAMED_PROTOCOL_TYPE);
}
pub fn lookup_framed_protocol(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_PROTOCOL_TYPE)
}
pub fn lookup_all_framed_protocol(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_PROTOCOL_TYPE)
}
pub fn add_framed_protocol(packet: &mut Packet, value: FramedProtocol) {
    let attr = Attribute::from_u32(value as u32);
    packet.add(FRAMED_PROTOCOL_TYPE, &attr);
}

pub const FRAMED_IP_ADDRESS_TYPE: AVPType = 8;
pub fn delete_framed_ip_address(packet: &mut Packet) {
    packet.delete(FRAMED_IP_ADDRESS_TYPE);
}
pub fn lookup_framed_ip_address(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_IP_ADDRESS_TYPE)
}
pub fn lookup_all_framed_ip_address(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_IP_ADDRESS_TYPE)
}
pub fn add_framed_ip_address(packet: &mut Packet, value: &Ipv4Addr) {
    let attr = Attribute::from_ipv4(value);
    packet.add(FRAMED_IP_ADDRESS_TYPE, &attr);
}

pub const FRAMED_IP_NETMASK_TYPE: AVPType = 9;
pub fn delete_framed_ip_netmask(packet: &mut Packet) {
    packet.delete(FRAMED_IP_NETMASK_TYPE);
}
pub fn lookup_framed_ip_netmask(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_IP_NETMASK_TYPE)
}
pub fn lookup_all_framed_ip_netmask(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_IP_NETMASK_TYPE)
}
pub fn add_framed_ip_netmask(packet: &mut Packet, value: &Ipv4Addr) {
    let attr = Attribute::from_ipv4(value);
    packet.add(FRAMED_IP_NETMASK_TYPE, &attr);
}

pub const FRAMED_ROUTING_TYPE: AVPType = 10;
pub fn delete_framed_routing(packet: &mut Packet) {
    packet.delete(FRAMED_ROUTING_TYPE);
}
pub fn lookup_framed_routing(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_ROUTING_TYPE)
}
pub fn lookup_all_framed_routing(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_ROUTING_TYPE)
}
pub fn add_framed_routing(packet: &mut Packet, value: FramedRouting) {
    let attr = Attribute::from_u32(value as u32);
    packet.add(FRAMED_ROUTING_TYPE, &attr);
}

pub const FILTER_ID_TYPE: AVPType = 11;
pub fn delete_filter_id(packet: &mut Packet) {
    packet.delete(FILTER_ID_TYPE);
}
pub fn lookup_filter_id(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FILTER_ID_TYPE)
}
pub fn lookup_all_filter_id(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FILTER_ID_TYPE)
}
pub fn add_filter_id(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(FILTER_ID_TYPE, &attr);
}

pub const FRAMED_MTU_TYPE: AVPType = 12;
pub fn delete_framed_mtu(packet: &mut Packet) {
    packet.delete(FRAMED_MTU_TYPE);
}
pub fn lookup_framed_mtu(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_MTU_TYPE)
}
pub fn lookup_all_framed_mtu(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_MTU_TYPE)
}
pub fn add_framed_mtu(packet: &mut Packet, value: u32) {
    let attr = Attribute::from_u32(value);
    packet.add(FRAMED_MTU_TYPE, &attr);
}

pub const FRAMED_COMPRESSION_TYPE: AVPType = 13;
pub fn delete_framed_compression(packet: &mut Packet) {
    packet.delete(FRAMED_COMPRESSION_TYPE);
}
pub fn lookup_framed_compression(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_COMPRESSION_TYPE)
}
pub fn lookup_all_framed_compression(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_COMPRESSION_TYPE)
}
pub fn add_framed_compression(packet: &mut Packet, value: FramedCompression) {
    let attr = Attribute::from_u32(value as u32);
    packet.add(FRAMED_COMPRESSION_TYPE, &attr);
}

pub const LOGIN_IP_HOST_TYPE: AVPType = 14;
pub fn delete_login_ip_host(packet: &mut Packet) {
    packet.delete(LOGIN_IP_HOST_TYPE);
}
pub fn lookup_login_ip_host(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(LOGIN_IP_HOST_TYPE)
}
pub fn lookup_all_login_ip_host(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(LOGIN_IP_HOST_TYPE)
}
pub fn add_login_ip_host(packet: &mut Packet, value: &Ipv4Addr) {
    let attr = Attribute::from_ipv4(value);
    packet.add(LOGIN_IP_HOST_TYPE, &attr);
}

pub const LOGIN_SERVICE_TYPE: AVPType = 15;
pub fn delete_login_service(packet: &mut Packet) {
    packet.delete(LOGIN_SERVICE_TYPE);
}
pub fn lookup_login_service(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(LOGIN_SERVICE_TYPE)
}
pub fn lookup_all_login_service(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(LOGIN_SERVICE_TYPE)
}
pub fn add_login_service(packet: &mut Packet, value: LoginService) {
    let attr = Attribute::from_u32(value as u32);
    packet.add(LOGIN_SERVICE_TYPE, &attr);
}

pub const LOGIN_TCP_PORT_TYPE: AVPType = 16;
pub fn delete_login_tcp_port(packet: &mut Packet) {
    packet.delete(LOGIN_TCP_PORT_TYPE);
}
pub fn lookup_login_tcp_port(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(LOGIN_TCP_PORT_TYPE)
}
pub fn lookup_all_login_tcp_port(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(LOGIN_TCP_PORT_TYPE)
}
pub fn add_login_tcp_port(packet: &mut Packet, value: LoginTCPPort) {
    let attr = Attribute::from_u32(value as u32);
    packet.add(LOGIN_TCP_PORT_TYPE, &attr);
}

pub const REPLY_MESSAGE_TYPE: AVPType = 18;
pub fn delete_reply_message(packet: &mut Packet) {
    packet.delete(REPLY_MESSAGE_TYPE);
}
pub fn lookup_reply_message(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(REPLY_MESSAGE_TYPE)
}
pub fn lookup_all_reply_message(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(REPLY_MESSAGE_TYPE)
}
pub fn add_reply_message(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(REPLY_MESSAGE_TYPE, &attr);
}

pub const CALLBACK_NUMBER_TYPE: AVPType = 19;
pub fn delete_callback_number(packet: &mut Packet) {
    packet.delete(CALLBACK_NUMBER_TYPE);
}
pub fn lookup_callback_number(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(CALLBACK_NUMBER_TYPE)
}
pub fn lookup_all_callback_number(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(CALLBACK_NUMBER_TYPE)
}
pub fn add_callback_number(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(CALLBACK_NUMBER_TYPE, &attr);
}

pub const CALLBACK_ID_TYPE: AVPType = 20;
pub fn delete_callback_id(packet: &mut Packet) {
    packet.delete(CALLBACK_ID_TYPE);
}
pub fn lookup_callback_id(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(CALLBACK_ID_TYPE)
}
pub fn lookup_all_callback_id(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(CALLBACK_ID_TYPE)
}
pub fn add_callback_id(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(CALLBACK_ID_TYPE, &attr);
}

pub const FRAMED_ROUTE_TYPE: AVPType = 22;
pub fn delete_framed_route(packet: &mut Packet) {
    packet.delete(FRAMED_ROUTE_TYPE);
}
pub fn lookup_framed_route(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_ROUTE_TYPE)
}
pub fn lookup_all_framed_route(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_ROUTE_TYPE)
}
pub fn add_framed_route(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(FRAMED_ROUTE_TYPE, &attr);
}

pub const FRAMED_IPX_NETWORK_TYPE: AVPType = 23;
pub fn delete_framed_ipx_network(packet: &mut Packet) {
    packet.delete(FRAMED_IPX_NETWORK_TYPE);
}
pub fn lookup_framed_ipx_network(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_IPX_NETWORK_TYPE)
}
pub fn lookup_all_framed_ipx_network(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_IPX_NETWORK_TYPE)
}
pub fn add_framed_ipx_network(packet: &mut Packet, value: &Ipv4Addr) {
    let attr = Attribute::from_ipv4(value);
    packet.add(FRAMED_IPX_NETWORK_TYPE, &attr);
}

pub const STATE_TYPE: AVPType = 24;
pub fn delete_state(packet: &mut Packet) {
    packet.delete(STATE_TYPE);
}
pub fn lookup_state(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(STATE_TYPE)
}
pub fn lookup_all_state(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(STATE_TYPE)
}
pub fn add_state(packet: &mut Packet, value: &[u8]) {
    let attr = Attribute::from_bytes(value);
    packet.add(STATE_TYPE, &attr);
}

pub const CLASS_TYPE: AVPType = 25;
pub fn delete_class(packet: &mut Packet) {
    packet.delete(CLASS_TYPE);
}
pub fn lookup_class(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(CLASS_TYPE)
}
pub fn lookup_all_class(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(CLASS_TYPE)
}
pub fn add_class(packet: &mut Packet, value: &[u8]) {
    let attr = Attribute::from_bytes(value);
    packet.add(CLASS_TYPE, &attr);
}

pub const VENDOR_SPECIFIC_TYPE: AVPType = 26;
pub fn delete_vendor_specific(packet: &mut Packet) {
    packet.delete(VENDOR_SPECIFIC_TYPE);
}
pub fn lookup_vendor_specific(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(VENDOR_SPECIFIC_TYPE)
}
pub fn lookup_all_vendor_specific(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(VENDOR_SPECIFIC_TYPE)
}

pub const SESSION_TIMEOUT_TYPE: AVPType = 27;
pub fn delete_session_timeout(packet: &mut Packet) {
    packet.delete(SESSION_TIMEOUT_TYPE);
}
pub fn lookup_session_timeout(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(SESSION_TIMEOUT_TYPE)
}
pub fn lookup_all_session_timeout(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(SESSION_TIMEOUT_TYPE)
}
pub fn add_session_timeout(packet: &mut Packet, value: u32) {
    let attr = Attribute::from_u32(value);
    packet.add(SESSION_TIMEOUT_TYPE, &attr);
}

pub const IDLE_TIMEOUT_TYPE: AVPType = 28;
pub fn delete_idle_timeout(packet: &mut Packet) {
    packet.delete(IDLE_TIMEOUT_TYPE);
}
pub fn lookup_idle_timeout(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(IDLE_TIMEOUT_TYPE)
}
pub fn lookup_all_idle_timeout(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(IDLE_TIMEOUT_TYPE)
}
pub fn add_idle_timeout(packet: &mut Packet, value: u32) {
    let attr = Attribute::from_u32(value);
    packet.add(IDLE_TIMEOUT_TYPE, &attr);
}

pub const TERMINATION_ACTION_TYPE: AVPType = 29;
pub fn delete_termination_action(packet: &mut Packet) {
    packet.delete(TERMINATION_ACTION_TYPE);
}
pub fn lookup_termination_action(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(TERMINATION_ACTION_TYPE)
}
pub fn lookup_all_termination_action(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(TERMINATION_ACTION_TYPE)
}
pub fn add_termination_action(packet: &mut Packet, value: TerminationAction) {
    let attr = Attribute::from_u32(value as u32);
    packet.add(TERMINATION_ACTION_TYPE, &attr);
}

pub const CALLED_STATION_ID_TYPE: AVPType = 30;
pub fn delete_called_station_id(packet: &mut Packet) {
    packet.delete(CALLED_STATION_ID_TYPE);
}
pub fn lookup_called_station_id(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(CALLED_STATION_ID_TYPE)
}
pub fn lookup_all_called_station_id(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(CALLED_STATION_ID_TYPE)
}
pub fn add_called_station_id(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(CALLED_STATION_ID_TYPE, &attr);
}

pub const CALLING_STATION_ID_TYPE: AVPType = 31;
pub fn delete_calling_station_id(packet: &mut Packet) {
    packet.delete(CALLING_STATION_ID_TYPE);
}
pub fn lookup_calling_station_id(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(CALLING_STATION_ID_TYPE)
}
pub fn lookup_all_calling_station_id(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(CALLING_STATION_ID_TYPE)
}
pub fn add_calling_station_id(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(CALLING_STATION_ID_TYPE, &attr);
}

pub const NAS_IDENTIFIER_TYPE: AVPType = 32;
pub fn delete_nas_identifier(packet: &mut Packet) {
    packet.delete(NAS_IDENTIFIER_TYPE);
}
pub fn lookup_nas_identifier(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(NAS_IDENTIFIER_TYPE)
}
pub fn lookup_all_nas_identifier(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(NAS_IDENTIFIER_TYPE)
}
pub fn add_nas_identifier(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(NAS_IDENTIFIER_TYPE, &attr);
}

pub const PROXY_STATE_TYPE: AVPType = 33;
pub fn delete_proxy_state(packet: &mut Packet) {
    packet.delete(PROXY_STATE_TYPE);
}
pub fn lookup_proxy_state(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(PROXY_STATE_TYPE)
}
pub fn lookup_all_proxy_state(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(PROXY_STATE_TYPE)
}
pub fn add_proxy_state(packet: &mut Packet, value: &[u8]) {
    let attr = Attribute::from_bytes(value);
    packet.add(PROXY_STATE_TYPE, &attr);
}

pub const LOGIN_LAT_SERVICE_TYPE: AVPType = 34;
pub fn delete_login_lat_service(packet: &mut Packet) {
    packet.delete(LOGIN_LAT_SERVICE_TYPE);
}
pub fn lookup_login_lat_service(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(LOGIN_LAT_SERVICE_TYPE)
}
pub fn lookup_all_login_lat_service(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(LOGIN_LAT_SERVICE_TYPE)
}
pub fn add_login_lat_service(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(LOGIN_LAT_SERVICE_TYPE, &attr);
}

pub const LOGIN_LAT_NODE_TYPE: AVPType = 35;
pub fn delete_login_lat_node(packet: &mut Packet) {
    packet.delete(LOGIN_LAT_NODE_TYPE);
}
pub fn lookup_login_lat_node(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(LOGIN_LAT_NODE_TYPE)
}
pub fn lookup_all_login_lat_node(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(LOGIN_LAT_NODE_TYPE)
}
pub fn add_login_lat_node(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(LOGIN_LAT_NODE_TYPE, &attr);
}

pub const LOGIN_LAT_GROUP_TYPE: AVPType = 36;
pub fn delete_login_lat_group(packet: &mut Packet) {
    packet.delete(LOGIN_LAT_GROUP_TYPE);
}
pub fn lookup_login_lat_group(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(LOGIN_LAT_GROUP_TYPE)
}
pub fn lookup_all_login_lat_group(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(LOGIN_LAT_GROUP_TYPE)
}
pub fn add_login_lat_group(packet: &mut Packet, value: &[u8]) {
    let attr = Attribute::from_bytes(value);
    packet.add(LOGIN_LAT_GROUP_TYPE, &attr);
}

pub const FRAMED_APPLE_TALK_LINK_TYPE: AVPType = 37;
pub fn delete_framed_apple_talk_link(packet: &mut Packet) {
    packet.delete(FRAMED_APPLE_TALK_LINK_TYPE);
}
pub fn lookup_framed_apple_talk_link(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_APPLE_TALK_LINK_TYPE)
}
pub fn lookup_all_framed_apple_talk_link(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_APPLE_TALK_LINK_TYPE)
}
pub fn add_framed_apple_talk_link(packet: &mut Packet, value: u32) {
    let attr = Attribute::from_u32(value);
    packet.add(FRAMED_APPLE_TALK_LINK_TYPE, &attr);
}

pub const FRAMED_APPLE_TALK_NETWORK_TYPE: AVPType = 38;
pub fn delete_framed_apple_talk_network(packet: &mut Packet) {
    packet.delete(FRAMED_APPLE_TALK_NETWORK_TYPE);
}
pub fn lookup_framed_apple_talk_network(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_APPLE_TALK_NETWORK_TYPE)
}
pub fn lookup_all_framed_apple_talk_network(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_APPLE_TALK_NETWORK_TYPE)
}
pub fn add_framed_apple_talk_network(packet: &mut Packet, value: u32) {
    let attr = Attribute::from_u32(value);
    packet.add(FRAMED_APPLE_TALK_NETWORK_TYPE, &attr);
}

pub const FRAMED_APPLE_TALK_ZONE_TYPE: AVPType = 39;
pub fn delete_framed_apple_talk_zone(packet: &mut Packet) {
    packet.delete(FRAMED_APPLE_TALK_ZONE_TYPE);
}
pub fn lookup_framed_apple_talk_zone(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(FRAMED_APPLE_TALK_ZONE_TYPE)
}
pub fn lookup_all_framed_apple_talk_zone(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(FRAMED_APPLE_TALK_ZONE_TYPE)
}
pub fn add_framed_apple_talk_zone(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(FRAMED_APPLE_TALK_ZONE_TYPE, &attr);
}

pub const CHAP_CHALLENGE_TYPE: AVPType = 60;
pub fn delete_chap_challenge(packet: &mut Packet) {
    packet.delete(CHAP_CHALLENGE_TYPE);
}
pub fn lookup_chap_challenge(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(CHAP_CHALLENGE_TYPE)
}
pub fn lookup_all_chap_challenge(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(CHAP_CHALLENGE_TYPE)
}
pub fn add_chap_challenge(packet: &mut Packet, value: &[u8]) {
    let attr = Attribute::from_bytes(value);
    packet.add(CHAP_CHALLENGE_TYPE, &attr);
}

pub const NAS_PORT_TYPE_TYPE: AVPType = 61;
pub fn delete_nas_port_type(packet: &mut Packet) {
    packet.delete(NAS_PORT_TYPE_TYPE);
}
pub fn lookup_nas_port_type(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(NAS_PORT_TYPE_TYPE)
}
pub fn lookup_all_nas_port_type(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(NAS_PORT_TYPE_TYPE)
}
pub fn add_nas_port_type(packet: &mut Packet, value: NasPortType) {
    let attr = Attribute::from_u32(value as u32);
    packet.add(NAS_PORT_TYPE_TYPE, &attr);
}

pub const PORT_LIMIT_TYPE: AVPType = 62;
pub fn delete_port_limit(packet: &mut Packet) {
    packet.delete(PORT_LIMIT_TYPE);
}
pub fn lookup_port_limit(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(PORT_LIMIT_TYPE)
}
pub fn lookup_all_port_limit(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(PORT_LIMIT_TYPE)
}
pub fn add_port_limit(packet: &mut Packet, value: u32) {
    let attr = Attribute::from_u32(value);
    packet.add(PORT_LIMIT_TYPE, &attr);
}

pub const LOGIN_LAT_PORT_TYPE: AVPType = 63;
pub fn delete_login_lat_port(packet: &mut Packet) {
    packet.delete(LOGIN_LAT_PORT_TYPE);
}
pub fn lookup_login_lat_port(packet: &Packet) -> Option<&Attribute> {
    packet.lookup(LOGIN_LAT_PORT_TYPE)
}
pub fn lookup_all_login_lat_port(packet: &Packet) -> Vec<&Attribute> {
    packet.lookup_all(LOGIN_LAT_PORT_TYPE)
}
pub fn add_login_lat_port(packet: &mut Packet, value: &str) {
    let attr = Attribute::from_string(value);
    packet.add(LOGIN_LAT_PORT_TYPE, &attr);
}
