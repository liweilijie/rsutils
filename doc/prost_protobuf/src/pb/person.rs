/// define a person
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Person {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Unique ID number for this person.
    #[prost(int32, tag = "2")]
    pub id: i32,
    #[prost(string, tag = "3")]
    pub email: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(btree_map = "string, int32", tag = "5")]
    pub scores: ::prost::alloc::collections::BTreeMap<
        ::prost::alloc::string::String,
        i32,
    >,
    #[prost(message, repeated, tag = "6")]
    pub phones: ::prost::alloc::vec::Vec<person::PhoneNumber>,
}
/// Nested message and enum types in `Person`.
pub mod person {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PhoneNumber {
        #[prost(string, tag = "1")]
        pub number: ::prost::alloc::string::String,
        #[prost(enumeration = "PhoneType", tag = "2")]
        pub phone_type: i32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum PhoneType {
        Mobile = 0,
        Home = 1,
        Work = 2,
    }
    impl PhoneType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PhoneType::Mobile => "MOBILE",
                PhoneType::Home => "HOME",
                PhoneType::Work => "WORK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MOBILE" => Some(Self::Mobile),
                "HOME" => Some(Self::Home),
                "WORK" => Some(Self::Work),
                _ => None,
            }
        }
    }
}
/// Our address book file is just one of these.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBook {
    #[prost(message, repeated, tag = "1")]
    pub people: ::prost::alloc::vec::Vec<Person>,
}
