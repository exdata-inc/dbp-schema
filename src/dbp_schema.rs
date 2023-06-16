#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataset {
    #[prost(string, optional, tag = "1")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub structure_info: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub generated_from: ::prost::alloc::vec::Vec<RealWorldDataset>,
    #[prost(message, optional, boxed, tag = "6")]
    pub generated_using: ::core::option::Option<
        ::prost::alloc::boxed::Box<RealWorldDataBrewer>,
    >,
    #[prost(message, repeated, tag = "7")]
    pub generated_args: ::prost::alloc::vec::Vec<RealWorldDataBrewingArgument>,
    #[prost(message, optional, tag = "8")]
    pub collection_info: ::core::option::Option<RealWorldDataCollectionInfo>,
    #[prost(string, tag = "9")]
    pub distribution: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "10")]
    pub author: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub content_location: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "12")]
    pub date_created: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "13")]
    pub date_modified: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "14")]
    pub date_published: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, optional, tag = "15")]
    pub license: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub location_created: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewerInput {
    #[prost(string, optional, tag = "1")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "VariableCharacteristicEnumeration", tag = "7")]
    pub input_characteristic: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewerOutput {
    #[prost(string, optional, tag = "1")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "VariableCharacteristicEnumeration", tag = "7")]
    pub output_characteristic: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewingArgument {
    #[prost(string, optional, tag = "1")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "5")]
    pub argument_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewer {
    #[prost(string, optional, tag = "1")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub structure_info: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag = "5")]
    pub generated_from: ::core::option::Option<
        ::prost::alloc::boxed::Box<RealWorldDataset>,
    >,
    #[prost(message, optional, boxed, tag = "6")]
    pub generated_using: ::core::option::Option<
        ::prost::alloc::boxed::Box<RealWorldDataBrewer>,
    >,
    #[prost(message, repeated, tag = "7")]
    pub input_specs: ::prost::alloc::vec::Vec<RealWorldDataBrewerInput>,
    #[prost(message, repeated, tag = "8")]
    pub output_specs: ::prost::alloc::vec::Vec<RealWorldDataBrewerOutput>,
    #[prost(message, repeated, tag = "9")]
    pub arg_specs: ::prost::alloc::vec::Vec<RealWorldDataBrewingArgument>,
    #[prost(enumeration = "ConversionCharacteristicEnumeration", tag = "10")]
    pub conversion_characteristic: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataCollectionInfo {
    #[prost(string, optional, tag = "1")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub collection_style: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub collection_protocol: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "6")]
    pub listen_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub server_addres: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "8")]
    pub entry_point: ::core::option::Option<EntryPoint>,
}
/// <https://schema.org/EntryPoint>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntryPoint {
    #[prost(string, optional, tag = "1")]
    pub action_application: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub action_platform: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub content_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub encoding_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub http_method: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub url_template: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub additional_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub alternate_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub disambiguating_description: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "11")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub main_entity_of_page: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub potential_action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub same_as: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub subject_of: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
/// <https://schema.org/Thing>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Thing {
    #[prost(string, optional, tag = "1")]
    pub additional_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub alternate_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub disambiguating_description: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "5")]
    pub identifier: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub image: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub main_entity_of_page: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "9")]
    pub potential_action: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub same_as: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub subject_of: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewEvent {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConversionCharacteristicEnumeration {
    /// option allow_alias = true;
    SingleValueReplace = 0,
    ColMerge = 1,
    RowMerge = 2,
}
impl ConversionCharacteristicEnumeration {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConversionCharacteristicEnumeration::SingleValueReplace => {
                "SINGLE_VALUE_REPLACE"
            }
            ConversionCharacteristicEnumeration::ColMerge => "COL_MERGE",
            ConversionCharacteristicEnumeration::RowMerge => "ROW_MERGE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SINGLE_VALUE_REPLACE" => Some(Self::SingleValueReplace),
            "COL_MERGE" => Some(Self::ColMerge),
            "ROW_MERGE" => Some(Self::RowMerge),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VariableCharacteristicEnumeration {
    /// option allow_alias = true;
    Qualitative = 0,
    Quantitative = 1,
}
impl VariableCharacteristicEnumeration {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VariableCharacteristicEnumeration::Qualitative => "QUALITATIVE",
            VariableCharacteristicEnumeration::Quantitative => "QUANTITATIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "QUALITATIVE" => Some(Self::Qualitative),
            "QUANTITATIVE" => Some(Self::Quantitative),
            _ => None,
        }
    }
}
