#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatiMsg {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="bati_msg::BatiMsgType", tag="2")]
    pub r#type: i32,
    #[prost(bytes="vec", optional, tag="3")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag="4")]
    pub cid: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub uid: ::prost::alloc::string::String,
    #[prost(string, optional, tag="6")]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, tag="7")]
    pub ts: u64,
}
/// Nested message and enum types in `BatiMsg`.
pub mod bati_msg {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BatiMsgType {
        Unused = 0,
        Biz = 1,
        ConnQuit = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceMsg {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub service: ::prost::alloc::string::String,
    #[prost(enumeration="service_msg::ServiceMsgType", tag="3")]
    pub r#type: i32,
    #[prost(message, optional, tag="4")]
    pub biz_data: ::core::option::Option<BizData>,
    #[prost(message, optional, tag="5")]
    pub join_data: ::core::option::Option<JoinData>,
    #[prost(message, optional, tag="6")]
    pub quit_data: ::core::option::Option<QuitData>,
    #[prost(uint64, tag="7")]
    pub ts: u64,
}
/// Nested message and enum types in `ServiceMsg`.
pub mod service_msg {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ServiceMsgType {
        Unused = 0,
        ConnJoin = 1,
        ConnQuit = 2,
        Biz = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JoinData {
    #[prost(string, optional, tag="1")]
    pub cid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3")]
    pub join_service: ::core::option::Option<bool>,
    #[prost(string, repeated, tag="4")]
    pub rooms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuitData {
    #[prost(string, optional, tag="1")]
    pub cid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="2")]
    pub uid: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag="3")]
    pub quit_service: ::core::option::Option<bool>,
    #[prost(string, repeated, tag="4")]
    pub rooms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BizData {
    #[prost(enumeration="biz_data::BizMsgType", tag="1")]
    pub r#type: i32,
    #[prost(string, repeated, tag="2")]
    pub cids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="3")]
    pub uids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub room: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag="5")]
    pub broadcast_ratio: ::core::option::Option<u32>,
    #[prost(string, repeated, tag="6")]
    pub black_uids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="7")]
    pub white_uids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bytes="vec", optional, tag="8")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `BizData`.
pub mod biz_data {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BizMsgType {
        Unused = 0,
        Users = 1,
        Room = 2,
        Service = 3,
        All = 4,
    }
}
