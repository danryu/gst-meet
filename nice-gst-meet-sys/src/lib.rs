// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from ../../gir-files (@ 8e47c67)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(
  clippy::approx_constant,
  clippy::type_complexity,
  clippy::unreadable_literal,
  clippy::upper_case_acronyms
)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

#[allow(unused_imports)]
use glib::ffi::{gboolean, gconstpointer, gpointer, GType};
#[allow(unused_imports)]
use libc::{
  c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
  intptr_t, size_t, ssize_t, time_t, uintptr_t, FILE, 
};

#[cfg(unix)]
use libc::{sockaddr, sockaddr_in, sockaddr_in6};

#[cfg(windows)]
use windows_sys::Win32::Networking::WinSock::{
    SOCKADDR, SOCKADDR_IN, SOCKADDR_IN6,
};

// Optional: Define type aliases on Windows
#[cfg(windows)]
type sockaddr = SOCKADDR;

#[cfg(windows)]
type sockaddr_in = SOCKADDR_IN;

#[cfg(windows)]
type sockaddr_in6 = SOCKADDR_IN6;

// Enums
pub type NiceCandidateTransport = c_int;
pub const NICE_CANDIDATE_TRANSPORT_UDP: NiceCandidateTransport = 0;
pub const NICE_CANDIDATE_TRANSPORT_TCP_ACTIVE: NiceCandidateTransport = 1;
pub const NICE_CANDIDATE_TRANSPORT_TCP_PASSIVE: NiceCandidateTransport = 2;
pub const NICE_CANDIDATE_TRANSPORT_TCP_SO: NiceCandidateTransport = 3;

pub type NiceCandidateType = c_int;
pub const NICE_CANDIDATE_TYPE_HOST: NiceCandidateType = 0;
pub const NICE_CANDIDATE_TYPE_SERVER_REFLEXIVE: NiceCandidateType = 1;
pub const NICE_CANDIDATE_TYPE_PEER_REFLEXIVE: NiceCandidateType = 2;
pub const NICE_CANDIDATE_TYPE_RELAYED: NiceCandidateType = 3;

pub type NiceCompatibility = c_int;
pub const NICE_COMPATIBILITY_RFC5245: NiceCompatibility = 0;
pub const NICE_COMPATIBILITY_DRAFT19: NiceCompatibility = 0;
pub const NICE_COMPATIBILITY_GOOGLE: NiceCompatibility = 1;
pub const NICE_COMPATIBILITY_MSN: NiceCompatibility = 2;
pub const NICE_COMPATIBILITY_WLM2009: NiceCompatibility = 3;
pub const NICE_COMPATIBILITY_OC2007: NiceCompatibility = 4;
pub const NICE_COMPATIBILITY_OC2007R2: NiceCompatibility = 5;
pub const NICE_COMPATIBILITY_LAST: NiceCompatibility = 5;

pub type NiceComponentState = c_int;
pub const NICE_COMPONENT_STATE_DISCONNECTED: NiceComponentState = 0;
pub const NICE_COMPONENT_STATE_GATHERING: NiceComponentState = 1;
pub const NICE_COMPONENT_STATE_CONNECTING: NiceComponentState = 2;
pub const NICE_COMPONENT_STATE_CONNECTED: NiceComponentState = 3;
pub const NICE_COMPONENT_STATE_READY: NiceComponentState = 4;
pub const NICE_COMPONENT_STATE_FAILED: NiceComponentState = 5;
pub const NICE_COMPONENT_STATE_LAST: NiceComponentState = 6;

pub type NiceComponentType = c_int;
pub const NICE_COMPONENT_TYPE_RTP: NiceComponentType = 1;
pub const NICE_COMPONENT_TYPE_RTCP: NiceComponentType = 2;

pub type NiceNominationMode = c_int;
pub const NICE_NOMINATION_MODE_REGULAR: NiceNominationMode = 0;
pub const NICE_NOMINATION_MODE_AGGRESSIVE: NiceNominationMode = 1;

pub type NiceProxyType = c_int;
pub const NICE_PROXY_TYPE_NONE: NiceProxyType = 0;
pub const NICE_PROXY_TYPE_SOCKS5: NiceProxyType = 1;
pub const NICE_PROXY_TYPE_HTTP: NiceProxyType = 2;
pub const NICE_PROXY_TYPE_LAST: NiceProxyType = 2;

pub type NiceRelayType = c_int;
pub const NICE_RELAY_TYPE_TURN_UDP: NiceRelayType = 0;
pub const NICE_RELAY_TYPE_TURN_TCP: NiceRelayType = 1;
pub const NICE_RELAY_TYPE_TURN_TLS: NiceRelayType = 2;

pub type PseudoTcpDebugLevel = c_int;
pub const PSEUDO_TCP_DEBUG_NONE: PseudoTcpDebugLevel = 0;
pub const PSEUDO_TCP_DEBUG_NORMAL: PseudoTcpDebugLevel = 1;
pub const PSEUDO_TCP_DEBUG_VERBOSE: PseudoTcpDebugLevel = 2;

pub type PseudoTcpShutdown = c_int;
pub const PSEUDO_TCP_SHUTDOWN_RD: PseudoTcpShutdown = 0;
pub const PSEUDO_TCP_SHUTDOWN_WR: PseudoTcpShutdown = 1;
pub const PSEUDO_TCP_SHUTDOWN_RDWR: PseudoTcpShutdown = 2;

pub type PseudoTcpState = c_int;
pub const PSEUDO_TCP_LISTEN: PseudoTcpState = 0;
pub const PSEUDO_TCP_SYN_SENT: PseudoTcpState = 1;
pub const PSEUDO_TCP_SYN_RECEIVED: PseudoTcpState = 2;
pub const PSEUDO_TCP_ESTABLISHED: PseudoTcpState = 3;
pub const PSEUDO_TCP_CLOSED: PseudoTcpState = 4;
pub const PSEUDO_TCP_FIN_WAIT_1: PseudoTcpState = 5;
pub const PSEUDO_TCP_FIN_WAIT_2: PseudoTcpState = 6;
pub const PSEUDO_TCP_CLOSING: PseudoTcpState = 7;
pub const PSEUDO_TCP_TIME_WAIT: PseudoTcpState = 8;
pub const PSEUDO_TCP_CLOSE_WAIT: PseudoTcpState = 9;
pub const PSEUDO_TCP_LAST_ACK: PseudoTcpState = 10;

pub type PseudoTcpWriteResult = c_int;
pub const WR_SUCCESS: PseudoTcpWriteResult = 0;
pub const WR_TOO_LARGE: PseudoTcpWriteResult = 1;
pub const WR_FAIL: PseudoTcpWriteResult = 2;

// Constants
pub const NICE_AGENT_MAX_REMOTE_CANDIDATES: c_int = 25;
pub const NICE_CANDIDATE_MAX_FOUNDATION: c_int = 33;
pub const NICE_CANDIDATE_MAX_LOCAL_ADDRESSES: c_int = 64;
pub const NICE_CANDIDATE_MAX_TURN_SERVERS: c_int = 8;

// Flags
pub type NiceAgentOption = c_uint;
pub const NICE_AGENT_OPTION_REGULAR_NOMINATION: NiceAgentOption = 1;
pub const NICE_AGENT_OPTION_RELIABLE: NiceAgentOption = 2;
pub const NICE_AGENT_OPTION_LITE_MODE: NiceAgentOption = 4;
pub const NICE_AGENT_OPTION_ICE_TRICKLE: NiceAgentOption = 8;
pub const NICE_AGENT_OPTION_SUPPORT_RENOMINATION: NiceAgentOption = 16;
pub const NICE_AGENT_OPTION_CONSENT_FRESHNESS: NiceAgentOption = 32;

// Unions
#[repr(C)]
#[derive(Copy, Clone)]
pub union NiceAddress_s {
  pub addr: sockaddr,
  pub ip4: sockaddr_in,
  pub ip6: sockaddr_in6,
}

impl ::std::fmt::Debug for NiceAddress_s {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("NiceAddress_s @ {:p}", self))
      .finish()
  }
}

// Callbacks
pub type NiceAgentRecvFunc =
  Option<unsafe extern "C" fn(*mut NiceAgent, c_uint, c_uint, c_uint, *mut c_char, gpointer)>;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NiceAddress {
  pub s: NiceAddress_s,
}

impl ::std::fmt::Debug for NiceAddress {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("NiceAddress @ {:p}", self))
      .field("s", &self.s)
      .finish()
  }
}

#[repr(C)]
#[derive(Clone)]
pub struct NiceAgentClass {
  pub parent_class: gobject_sys::GObjectClass,
}

impl ::std::fmt::Debug for NiceAgentClass {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("NiceAgentClass @ {:p}", self))
      .field("parent_class", &self.parent_class)
      .finish()
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NiceCandidate {
  pub type_: NiceCandidateType,
  pub transport: NiceCandidateTransport,
  pub addr: NiceAddress,
  pub base_addr: NiceAddress,
  pub priority: u32,
  pub stream_id: c_uint,
  pub component_id: c_uint,
  pub foundation: [c_char; 33],
  pub username: *mut c_char,
  pub password: *mut c_char,
}

impl ::std::fmt::Debug for NiceCandidate {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("NiceCandidate @ {:p}", self))
      .field("type_", &self.type_)
      .field("transport", &self.transport)
      .field("addr", &self.addr)
      .field("base_addr", &self.base_addr)
      .field("priority", &self.priority)
      .field("stream_id", &self.stream_id)
      .field("component_id", &self.component_id)
      .field("username", &self.username)
      .field("password", &self.password)
      .finish()
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NiceInputMessage {
  pub buffers: *mut gio::ffi::GInputVector,
  pub n_buffers: c_int,
  pub from: *mut NiceAddress,
  pub length: size_t,
}

impl ::std::fmt::Debug for NiceInputMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("NiceInputMessage @ {:p}", self))
      .field("buffers", &self.buffers)
      .field("n_buffers", &self.n_buffers)
      .field("from", &self.from)
      .field("length", &self.length)
      .finish()
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NiceOutputMessage {
  pub buffers: *mut gio::ffi::GOutputVector,
  pub n_buffers: c_int,
}

impl ::std::fmt::Debug for NiceOutputMessage {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("NiceOutputMessage @ {:p}", self))
      .field("buffers", &self.buffers)
      .field("n_buffers", &self.n_buffers)
      .finish()
  }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PseudoTcpCallbacks {
  pub user_data: gpointer,
  pub PseudoTcpOpened: Option<unsafe extern "C" fn(*mut PseudoTcpSocket, gpointer)>,
  pub PseudoTcpReadable: Option<unsafe extern "C" fn(*mut PseudoTcpSocket, gpointer)>,
  pub PseudoTcpWritable: Option<unsafe extern "C" fn(*mut PseudoTcpSocket, gpointer)>,
  pub PseudoTcpClosed: Option<unsafe extern "C" fn(*mut PseudoTcpSocket, u32, gpointer)>,
  pub WritePacket: Option<
    unsafe extern "C" fn(
      *mut PseudoTcpSocket,
      *const c_char,
      u32,
      gpointer,
    ) -> PseudoTcpWriteResult,
  >,
}

impl ::std::fmt::Debug for PseudoTcpCallbacks {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("PseudoTcpCallbacks @ {:p}", self))
      .field("user_data", &self.user_data)
      .field("PseudoTcpOpened", &self.PseudoTcpOpened)
      .field("PseudoTcpReadable", &self.PseudoTcpReadable)
      .field("PseudoTcpWritable", &self.PseudoTcpWritable)
      .field("PseudoTcpClosed", &self.PseudoTcpClosed)
      .field("WritePacket", &self.WritePacket)
      .finish()
  }
}

#[repr(C)]
pub struct _PseudoTcpSocketClass(c_void);

pub type PseudoTcpSocketClass = *mut _PseudoTcpSocketClass;

// Classes
#[repr(C)]
pub struct NiceAgent(c_void);

impl ::std::fmt::Debug for NiceAgent {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("NiceAgent @ {:p}", self)).finish()
  }
}

#[repr(C)]
pub struct PseudoTcpSocket(c_void);

impl ::std::fmt::Debug for PseudoTcpSocket {
  fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
    f.debug_struct(&format!("PseudoTcpSocket @ {:p}", self))
      .finish()
  }
}

#[link(name = "nice")]
extern "C" {

  //=========================================================================
  // NiceAddress
  //=========================================================================
  pub fn nice_address_copy_to_sockaddr(addr: *const NiceAddress, sin: *mut sockaddr);
  pub fn nice_address_dup(addr: *const NiceAddress) -> *mut NiceAddress;
  pub fn nice_address_equal(a: *const NiceAddress, b: *const NiceAddress) -> gboolean;
  #[cfg(any(feature = "v0_1_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_8")))]
  pub fn nice_address_equal_no_port(a: *const NiceAddress, b: *const NiceAddress) -> gboolean;
  pub fn nice_address_free(addr: *mut NiceAddress);
  pub fn nice_address_get_port(addr: *const NiceAddress) -> c_uint;
  pub fn nice_address_init(addr: *mut NiceAddress);
  pub fn nice_address_ip_version(addr: *const NiceAddress) -> c_int;
  pub fn nice_address_is_private(addr: *const NiceAddress) -> gboolean;
  pub fn nice_address_is_valid(addr: *const NiceAddress) -> gboolean;
  pub fn nice_address_set_from_sockaddr(addr: *mut NiceAddress, sin: *const sockaddr);
  pub fn nice_address_set_from_string(addr: *mut NiceAddress, str: *const c_char) -> gboolean;
  pub fn nice_address_set_ipv4(addr: *mut NiceAddress, addr_ipv4: u32);
  pub fn nice_address_set_ipv6(addr: *mut NiceAddress, addr_ipv6: *const u8);
  pub fn nice_address_set_port(addr: *mut NiceAddress, port: c_uint);
  pub fn nice_address_to_string(addr: *const NiceAddress, dst: *mut c_char);
  pub fn nice_address_new() -> *mut NiceAddress;

  //=========================================================================
  // NiceCandidate
  //=========================================================================
  pub fn nice_candidate_get_type() -> GType;
  pub fn nice_candidate_new(type_: NiceCandidateType) -> *mut NiceCandidate;
  pub fn nice_candidate_copy(candidate: *const NiceCandidate) -> *mut NiceCandidate;
  #[cfg(any(feature = "v0_1_15", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
  pub fn nice_candidate_equal_target(
    candidate1: *const NiceCandidate,
    candidate2: *const NiceCandidate,
  ) -> gboolean;
  pub fn nice_candidate_free(candidate: *mut NiceCandidate);
  #[cfg(any(feature = "v0_1_18", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
  pub fn nice_candidate_transport_to_string(transport: NiceCandidateTransport) -> *const c_char;
  #[cfg(any(feature = "v0_1_18", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_18")))]
  pub fn nice_candidate_type_to_string(type_: NiceCandidateType) -> *const c_char;

  //=========================================================================
  // NiceAgent
  //=========================================================================
  pub fn nice_agent_get_type() -> GType;
  pub fn nice_agent_new(
    ctx: *mut glib::ffi::GMainContext,
    compat: NiceCompatibility,
  ) -> *mut NiceAgent;
  #[cfg(any(feature = "v0_1_15", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_15")))]
  pub fn nice_agent_new_full(
    ctx: *mut glib::ffi::GMainContext,
    compat: NiceCompatibility,
    flags: NiceAgentOption,
  ) -> *mut NiceAgent;
  pub fn nice_agent_new_reliable(
    ctx: *mut glib::ffi::GMainContext,
    compat: NiceCompatibility,
  ) -> *mut NiceAgent;
  pub fn nice_agent_add_local_address(agent: *mut NiceAgent, addr: *mut NiceAddress) -> gboolean;
  pub fn nice_agent_add_stream(agent: *mut NiceAgent, n_components: c_uint) -> c_uint;
  pub fn nice_agent_attach_recv(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    ctx: *mut glib::ffi::GMainContext,
    func: NiceAgentRecvFunc,
    data: gpointer,
  ) -> gboolean;
  #[cfg(any(feature = "v0_1_16", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_16")))]
  pub fn nice_agent_close_async(
    agent: *mut NiceAgent,
    callback: gio::ffi::GAsyncReadyCallback,
    callback_data: gpointer,
  );
  #[cfg(any(feature = "v0_1_20", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_20")))]
  pub fn nice_agent_consent_lost(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> gboolean;
  #[cfg(any(feature = "v0_1_6", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_6")))]
  pub fn nice_agent_forget_relays(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> gboolean;
  pub fn nice_agent_gather_candidates(agent: *mut NiceAgent, stream_id: c_uint) -> gboolean;
  #[cfg(any(feature = "v0_1_4", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_4")))]
  pub fn nice_agent_generate_local_candidate_sdp(
    agent: *mut NiceAgent,
    candidate: *mut NiceCandidate,
  ) -> *mut c_char;
  #[cfg(any(feature = "v0_1_4", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_4")))]
  pub fn nice_agent_generate_local_sdp(agent: *mut NiceAgent) -> *mut c_char;
  #[cfg(any(feature = "v0_1_4", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_4")))]
  pub fn nice_agent_generate_local_stream_sdp(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    include_non_ice: gboolean,
  ) -> *mut c_char;
  #[cfg(any(feature = "v0_1_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_8")))]
  pub fn nice_agent_get_component_state(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> NiceComponentState;
  pub fn nice_agent_get_default_local_candidate(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> *mut NiceCandidate;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn nice_agent_get_io_stream(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> *mut gio::ffi::GIOStream;
  pub fn nice_agent_get_local_candidates(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> *mut glib::ffi::GSList;
  pub fn nice_agent_get_local_credentials(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    ufrag: *mut *mut c_char,
    pwd: *mut *mut c_char,
  ) -> gboolean;
  pub fn nice_agent_get_remote_candidates(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> *mut glib::ffi::GSList;
  pub fn nice_agent_get_selected_pair(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    local: *mut *mut NiceCandidate,
    remote: *mut *mut NiceCandidate,
  ) -> gboolean;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn nice_agent_get_selected_socket(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> *mut gio::ffi::GSocket;
  #[cfg(any(feature = "v0_1_17", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_17")))]
  pub fn nice_agent_get_sockets(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
  ) -> *mut glib::ffi::GPtrArray;
  #[cfg(any(feature = "v0_1_4", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_4")))]
  pub fn nice_agent_get_stream_name(agent: *mut NiceAgent, stream_id: c_uint) -> *const c_char;
  #[cfg(any(feature = "v0_1_4", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_4")))]
  pub fn nice_agent_parse_remote_candidate_sdp(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    sdp: *const c_char,
  ) -> *mut NiceCandidate;
  #[cfg(any(feature = "v0_1_4", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_4")))]
  pub fn nice_agent_parse_remote_sdp(agent: *mut NiceAgent, sdp: *const c_char) -> c_int;
  #[cfg(any(feature = "v0_1_4", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_4")))]
  pub fn nice_agent_parse_remote_stream_sdp(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    sdp: *const c_char,
    ufrag: *mut *mut c_char,
    pwd: *mut *mut c_char,
  ) -> *mut glib::ffi::GSList;
  #[cfg(any(feature = "v0_1_16", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_16")))]
  pub fn nice_agent_peer_candidate_gathering_done(
    agent: *mut NiceAgent,
    stream_id: c_uint,
  ) -> gboolean;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn nice_agent_recv(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    buf: *mut u8,
    buf_len: size_t,
    cancellable: *mut gio::ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
  ) -> ssize_t;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn nice_agent_recv_messages(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    messages: *mut NiceInputMessage,
    n_messages: c_uint,
    cancellable: *mut gio::ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
  ) -> c_int;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn nice_agent_recv_messages_nonblocking(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    messages: *mut NiceInputMessage,
    n_messages: c_uint,
    cancellable: *mut gio::ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
  ) -> c_int;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn nice_agent_recv_nonblocking(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    buf: *mut u8,
    buf_len: size_t,
    cancellable: *mut gio::ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
  ) -> ssize_t;
  pub fn nice_agent_remove_stream(agent: *mut NiceAgent, stream_id: c_uint);
  pub fn nice_agent_restart(agent: *mut NiceAgent) -> gboolean;
  #[cfg(any(feature = "v0_1_6", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_6")))]
  pub fn nice_agent_restart_stream(agent: *mut NiceAgent, stream_id: c_uint) -> gboolean;
  pub fn nice_agent_send(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    len: c_uint,
    buf: *const c_char,
  ) -> c_int;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn nice_agent_send_messages_nonblocking(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    messages: *const NiceOutputMessage,
    n_messages: c_uint,
    cancellable: *mut gio::ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
  ) -> c_int;
  pub fn nice_agent_set_local_credentials(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    ufrag: *const c_char,
    pwd: *const c_char,
  ) -> gboolean;
  pub fn nice_agent_set_port_range(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    min_port: c_uint,
    max_port: c_uint,
  );
  pub fn nice_agent_set_relay_info(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    server_ip: *const c_char,
    server_port: c_uint,
    username: *const c_char,
    password: *const c_char,
    type_: NiceRelayType,
  ) -> gboolean;
  pub fn nice_agent_set_remote_candidates(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    candidates: *const glib::ffi::GSList,
  ) -> c_int;
  pub fn nice_agent_set_remote_credentials(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    ufrag: *const c_char,
    pwd: *const c_char,
  ) -> gboolean;
  pub fn nice_agent_set_selected_pair(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    lfoundation: *const c_char,
    rfoundation: *const c_char,
  ) -> gboolean;
  pub fn nice_agent_set_selected_remote_candidate(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    component_id: c_uint,
    candidate: *mut NiceCandidate,
  ) -> gboolean;
  pub fn nice_agent_set_software(agent: *mut NiceAgent, software: *const c_char);
  #[cfg(any(feature = "v0_1_4", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_4")))]
  pub fn nice_agent_set_stream_name(
    agent: *mut NiceAgent,
    stream_id: c_uint,
    name: *const c_char,
  ) -> gboolean;
  pub fn nice_agent_set_stream_tos(agent: *mut NiceAgent, stream_id: c_uint, tos: c_int);

  //=========================================================================
  // PseudoTcpSocket
  //=========================================================================
  pub fn pseudo_tcp_socket_get_type() -> GType;
  pub fn pseudo_tcp_socket_new(
    conversation: u32,
    callbacks: *mut PseudoTcpCallbacks,
  ) -> *mut PseudoTcpSocket;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn pseudo_tcp_socket_can_send(self_: *mut PseudoTcpSocket) -> gboolean;
  pub fn pseudo_tcp_socket_close(self_: *mut PseudoTcpSocket, force: gboolean);
  pub fn pseudo_tcp_socket_connect(self_: *mut PseudoTcpSocket) -> gboolean;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn pseudo_tcp_socket_get_available_bytes(self_: *mut PseudoTcpSocket) -> c_int;
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn pseudo_tcp_socket_get_available_send_space(self_: *mut PseudoTcpSocket) -> size_t;
  pub fn pseudo_tcp_socket_get_error(self_: *mut PseudoTcpSocket) -> c_int;
  pub fn pseudo_tcp_socket_get_next_clock(
    self_: *mut PseudoTcpSocket,
    timeout: *mut u64,
  ) -> gboolean;
  #[cfg(any(feature = "v0_1_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_8")))]
  pub fn pseudo_tcp_socket_is_closed(self_: *mut PseudoTcpSocket) -> gboolean;
  #[cfg(any(feature = "v0_1_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_8")))]
  pub fn pseudo_tcp_socket_is_closed_remotely(self_: *mut PseudoTcpSocket) -> gboolean;
  pub fn pseudo_tcp_socket_notify_clock(self_: *mut PseudoTcpSocket);
  #[cfg(any(feature = "v0_1_5", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
  pub fn pseudo_tcp_socket_notify_message(
    self_: *mut PseudoTcpSocket,
    message: *mut NiceInputMessage,
  ) -> gboolean;
  pub fn pseudo_tcp_socket_notify_mtu(self_: *mut PseudoTcpSocket, mtu: u16);
  pub fn pseudo_tcp_socket_notify_packet(
    self_: *mut PseudoTcpSocket,
    buffer: *const c_char,
    len: u32,
  ) -> gboolean;
  pub fn pseudo_tcp_socket_recv(
    self_: *mut PseudoTcpSocket,
    buffer: *mut c_char,
    len: size_t,
  ) -> c_int;
  pub fn pseudo_tcp_socket_send(
    self_: *mut PseudoTcpSocket,
    buffer: *const c_char,
    len: u32,
  ) -> c_int;
  #[cfg(any(feature = "v0_1_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_8")))]
  pub fn pseudo_tcp_socket_set_time(self_: *mut PseudoTcpSocket, current_time: u32);
  #[cfg(any(feature = "v0_1_8", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_8")))]
  pub fn pseudo_tcp_socket_shutdown(self_: *mut PseudoTcpSocket, how: PseudoTcpShutdown);

  //=========================================================================
  // Other functions
  //=========================================================================
  #[cfg(any(feature = "v0_1_6", feature = "dox"))]
  #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_6")))]
  pub fn nice_component_state_to_string(state: NiceComponentState) -> *const c_char;
  pub fn nice_debug_disable(with_stun: gboolean);
  pub fn nice_debug_enable(with_stun: gboolean);
  pub fn nice_interfaces_get_ip_for_interface(interface_name: *mut c_char) -> *mut c_char;
  pub fn nice_interfaces_get_local_interfaces() -> *mut glib::ffi::GList;
  pub fn nice_interfaces_get_local_ips(include_loopback: gboolean) -> *mut glib::ffi::GList;
  pub fn pseudo_tcp_set_debug_level(level: PseudoTcpDebugLevel);

}
