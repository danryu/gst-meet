// Generated by gir (https://github.com/gtk-rs/gir @ 5433e2582f83)
// from C:\Users\dan\Documents\code\libnice\build\nice (@ ac4bb22ebbfc)
// from C:\vcpkg\installed\x64-windows\share\gir-1.0 (@ 65be7019941e)
// DO NOT EDIT

mod agent;
pub use self::agent::Agent;

mod address;
pub use self::address::Address;

mod candidate;
pub use self::candidate::Candidate;

mod enums;
pub use self::enums::CandidateTransport;
pub use self::enums::CandidateType;
pub use self::enums::Compatibility;
pub use self::enums::ComponentState;
pub use self::enums::RelayType;

mod flags;
#[cfg(feature = "v0_1_15")]
#[cfg_attr(docsrs, doc(cfg(feature = "v0_1_15")))]
pub use self::flags::AgentOption;

