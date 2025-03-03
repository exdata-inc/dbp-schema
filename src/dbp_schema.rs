/// JSON-LD representing a real-world dataset (subclass of: "schema:Dataset")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataset {
    /// URL of itself (this JSON-LD)
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// URL (unused)
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Structural information
    #[prost(message, optional, tag = "4")]
    pub structure_info: ::core::option::Option<RealWorldDataFieldProfile>,
    /// Dataset from which this dataset was generated (input dataset during brewing)
    #[prost(message, repeated, tag = "5")]
    pub generated_from: ::prost::alloc::vec::Vec<RealWorldDataset>,
    /// Brewing program used to create this dataset
    #[prost(message, optional, tag = "6")]
    pub generated_using: ::core::option::Option<RealWorldDataBrewerInfo>,
    /// Parameters used when creating this dataset (input parameters during brewing)
    #[prost(message, repeated, tag = "7")]
    pub generated_args: ::prost::alloc::vec::Vec<RealWorldDataBrewingArgument>,
    /// Collection information
    #[prost(message, optional, tag = "8")]
    pub collection_info: ::core::option::Option<RealWorldDataCollectionInfo>,
    /// Data storage locations (repeated because data may be stored in multiple locations depending on retention period)
    #[prost(message, repeated, tag = "9")]
    pub distribution: ::prost::alloc::vec::Vec<RealWorldDataStoringInfo>,
    /// Entity collecting the data
    #[prost(string, optional, tag = "10")]
    pub author: ::core::option::Option<::prost::alloc::string::String>,
    /// Location where the data is collected
    #[prost(string, optional, tag = "11")]
    pub content_location: ::core::option::Option<::prost::alloc::string::String>,
    /// Timestamp when data collection started
    #[prost(message, optional, tag = "12")]
    pub date_created: ::core::option::Option<::prost_types::Timestamp>,
    /// Timestamp when the most recent data collection began
    #[prost(message, optional, tag = "13")]
    pub date_modified: ::core::option::Option<::prost_types::Timestamp>,
    /// Timestamp when the data was published
    #[prost(message, optional, tag = "14")]
    pub date_published: ::core::option::Option<::prost_types::Timestamp>,
    /// License
    #[prost(string, optional, tag = "15")]
    pub license: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub location_created: ::core::option::Option<::prost::alloc::string::String>,
    /// Description
    #[prost(string, optional, tag = "17")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "18")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// JSON-LD representing the specification of input data for real-world data brewing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewerInput {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Key (name) of the brewing input argument
    #[prost(string, optional, tag = "4")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Type of the brewing input data
    #[prost(string, optional, tag = "5")]
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Value of the brewing input (used when sending Demand)
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// A characteristic of an input data entry in a real world dataset.
    #[prost(enumeration = "VariableCharacteristicEnumeration", optional, tag = "7")]
    pub input_characteristic: ::core::option::Option<i32>,
    /// Target dataset
    #[prost(message, optional, tag = "8")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    #[prost(message, repeated, tag = "9")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// JSON-LD representing the specification of output data for real-world data brewing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewerOutput {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// A type of an output data entry in a real world dataset.
    #[prost(string, optional, tag = "5")]
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// A characteristic of an output data entry in a real world dataset.
    #[prost(enumeration = "VariableCharacteristicEnumeration", optional, tag = "7")]
    pub output_characteristic: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// An argument that was passed to a brewer to generate a dataset from another.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewingArgument {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// A type of an argument passed to a brewer.
    #[prost(string, optional, tag = "5")]
    pub argument_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// JSON-LD representing the specification of the brewer (microprogram) used in real-world data brewing
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewerInfo {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub input_specs: ::prost::alloc::vec::Vec<RealWorldDataBrewerInput>,
    #[prost(message, repeated, tag = "5")]
    pub output_specs: ::prost::alloc::vec::Vec<RealWorldDataBrewerOutput>,
    #[prost(message, repeated, tag = "6")]
    pub arg_specs: ::prost::alloc::vec::Vec<RealWorldDataBrewingArgument>,
    /// A characteristic of a conversion done by the brewer.
    #[prost(enumeration = "ConversionCharacteristicEnumeration", optional, tag = "7")]
    pub conversion_characteristic: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// Information for collecting real-world data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataCollectionInfo {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// A style of data collection (Server, Client, Pub/Sub).
    #[prost(string, optional, tag = "4")]
    pub collection_style: ::core::option::Option<::prost::alloc::string::String>,
    /// A protocol of data collection (HTTP, HTTPS, FTP, etc.).
    #[prost(string, optional, tag = "5")]
    pub collection_protocol: ::core::option::Option<::prost::alloc::string::String>,
    /// A listening address for data collection (URL).
    #[prost(string, optional, tag = "6")]
    pub listen_address: ::core::option::Option<::prost::alloc::string::String>,
    /// A server address for data collection (URL).
    #[prost(string, optional, tag = "7")]
    pub server_address: ::core::option::Option<::prost::alloc::string::String>,
    /// An entry point for data collection (action_application, URL).
    #[prost(message, optional, tag = "8")]
    pub entry_point: ::core::option::Option<EntryPoint>,
    #[prost(message, repeated, tag = "9")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
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
    pub id: ::core::option::Option<::prost::alloc::string::String>,
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
    pub id: ::core::option::Option<::prost::alloc::string::String>,
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
/// JSON-LD representing the field profile of a real-world data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataFieldProfile {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub encoding_format: ::core::option::Option<::prost::alloc::string::String>,
    /// JSON-LD schema (including @graph)
    #[prost(message, optional, tag = "5")]
    pub structure: ::core::option::Option<RealWorldDataStructureGraph>,
    #[prost(string, optional, tag = "6")]
    pub graphql_schema: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// JSON-LD representing the structure of real-world data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataStructureGraph {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// JSON-LD schema (@graph)
    #[prost(message, optional, tag = "4")]
    pub at_graph: ::core::option::Option<::prost_types::Struct>,
    #[prost(message, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// JSON-LD representing the storage information for real-world data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataStoringInfo {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// URL of the file system or database
    #[prost(string, optional, tag = "6")]
    pub base_url: ::core::option::Option<::prost::alloc::string::String>,
    /// If file system, the pattern following the URL; if database, the pattern for specifying date/time in queries
    #[prost(string, optional, tag = "7")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
    /// Transfer speed of the storage
    #[prost(int32, optional, tag = "8")]
    pub transmission_speed: ::core::option::Option<i32>,
    /// Type of storage (e.g., SSD, HDD, optical disc, magnetic tape, etc.)
    #[prost(string, optional, tag = "9")]
    pub storage_type: ::core::option::Option<::prost::alloc::string::String>,
    /// HTTP entry point (see schema:EntryPoint)
    #[prost(message, optional, tag = "10")]
    pub entry_point: ::core::option::Option<EntryPoint>,
    /// Server address
    #[prost(string, optional, tag = "11")]
    pub server_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Extended path pattern formats
    #[prost(message, repeated, tag = "12")]
    pub extended_path_pattern_formats: ::prost::alloc::vec::Vec<
        RealWorldDataPathPatternFormat,
    >,
    #[prost(message, repeated, tag = "13")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataRegisterDemand {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    #[prost(message, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataRegisterSupply {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    #[prost(string, optional, tag = "5")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataCollectionDemand {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub collection_info: ::core::option::Option<RealWorldDataCollectionInfo>,
    #[prost(message, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataCollectionSupply {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub collection_info: ::core::option::Option<RealWorldDataCollectionInfo>,
    #[prost(string, optional, tag = "5")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataCollectionStatus {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub collection_info: ::core::option::Option<RealWorldDataCollectionInfo>,
    #[prost(string, optional, tag = "5")]
    pub content_reference_time: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub active_connections: ::prost::alloc::vec::Vec<::prost_types::Struct>,
    #[prost(message, repeated, tag = "7")]
    pub traffic_statistics: ::prost::alloc::vec::Vec<::prost_types::Struct>,
    #[prost(message, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewingDemand {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub brewer_info: ::core::option::Option<RealWorldDataBrewerInfo>,
    #[prost(message, repeated, tag = "5")]
    pub brewer_input: ::prost::alloc::vec::Vec<RealWorldDataBrewerInput>,
    #[prost(message, repeated, tag = "6")]
    pub brewing_argument: ::prost::alloc::vec::Vec<RealWorldDataBrewingArgument>,
    #[prost(message, repeated, tag = "7")]
    pub brewer_output_store: ::prost::alloc::vec::Vec<RealWorldDataStoringInfo>,
    /// Start time of the data to be read
    #[prost(string, optional, tag = "8")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// End time of the data to be read
    #[prost(string, optional, tag = "9")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "10")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataBrewingSupply {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub brewer_info: ::core::option::Option<RealWorldDataBrewerInfo>,
    #[prost(message, repeated, tag = "5")]
    pub brewer_output: ::prost::alloc::vec::Vec<RealWorldDataBrewerOutput>,
    #[prost(message, repeated, tag = "6")]
    pub brewing_argument: ::prost::alloc::vec::Vec<RealWorldDataBrewingArgument>,
    /// Start time of the data to be read
    #[prost(string, optional, tag = "7")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// End time of the data to be read
    #[prost(string, optional, tag = "8")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "9")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// Configuration for the RWDB system to periodically issue RealWorldDataBrewingDemand
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataPeriodicBrewingConfig {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Demand to be issued
    #[prost(message, optional, tag = "4")]
    pub brewing_config: ::core::option::Option<RealWorldDataBrewingDemand>,
    /// Time interval (cron format)
    #[prost(string, optional, tag = "5")]
    pub cron_config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub enabled: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataReadDemand {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Real-world dataset to be read
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// Start time of the data to be read
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// End time of the data to be read
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    /// Structural query (SPARQL)
    #[prost(string, optional, tag = "7")]
    pub sparql_query: ::core::option::Option<::prost::alloc::string::String>,
    /// Structural query (GraphQL)
    #[prost(string, optional, tag = "8")]
    pub graphql_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "9")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataReadSupply {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Real-world dataset to be read
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// Start time of the data to be read
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// End time of the data to be read
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    /// Structural query (SPARQL)
    #[prost(string, optional, tag = "7")]
    pub sparql_query: ::core::option::Option<::prost::alloc::string::String>,
    /// Structural query (GraphQL)
    #[prost(string, optional, tag = "8")]
    pub graphql_query: ::core::option::Option<::prost::alloc::string::String>,
    /// Actually sent via MBUS?
    #[prost(bytes = "vec", optional, tag = "9")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "10")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataWriteDemand {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Real-world dataset to be written
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// Start time of the data to be written
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// End time of the data to be written
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    /// Actually sent via MBUS?
    #[prost(bytes = "vec", optional, tag = "7")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataWriteSupply {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// Start time of the written data
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// End time of the written data
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataMoveDemand {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// RealWorldDataset to be moved
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// How far back in time the start of the data to be moved should be (e.g., 8d → 8 days ago)
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// How far back in time the end of the data to be moved should be (e.g., 7d → 7 days ago)
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    /// Source location of the data
    #[prost(message, optional, tag = "7")]
    pub move_from: ::core::option::Option<RealWorldDataStoringInfo>,
    /// Destination for the data
    #[prost(message, optional, tag = "8")]
    pub move_to: ::core::option::Option<RealWorldDataStoringInfo>,
    #[prost(message, repeated, tag = "9")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataMoveSupply {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Moved RealWorldDataset
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// How far back in time the start of the moved data should be (e.g., 8d → 8 days ago)
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// How far back in time the end of the moved data should be (e.g., 7d → 7 days ago)
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    /// Source location of the data
    #[prost(message, optional, tag = "7")]
    pub move_from: ::core::option::Option<RealWorldDataStoringInfo>,
    /// Destination for the data
    #[prost(message, optional, tag = "8")]
    pub move_to: ::core::option::Option<RealWorldDataStoringInfo>,
    /// RealWorldDataset at the destination
    #[prost(message, optional, tag = "9")]
    pub moved_dataset: ::core::option::Option<RealWorldDataset>,
    #[prost(message, repeated, tag = "10")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// Configuration for the RWDB system to periodically move RealWorldDataset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataPeriodicMoveConfig {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Move configuration
    #[prost(message, optional, tag = "4")]
    pub move_config: ::core::option::Option<RealWorldDataMoveDemand>,
    /// Time interval (cron format)
    #[prost(string, optional, tag = "5")]
    pub cron_config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub enabled: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataRemoveDemand {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// RealWorldDataStoringInfo of the RealWorldDataset to be removed
    #[prost(message, optional, tag = "4")]
    pub dataset_store: ::core::option::Option<RealWorldDataStoringInfo>,
    /// How far back in time the start of the data to be removed should be (e.g., 8d → 8 days ago)
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// How far back in time the end of the data to be removed should be (e.g., 7d → 7 days ago)
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataRemoveSupply {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// RealWorldDataStoringInfo of the removed RealWorldDataset
    #[prost(message, optional, tag = "4")]
    pub dataset_store: ::core::option::Option<RealWorldDataStoringInfo>,
    /// How far back in time the start of the removed data was (e.g., 8d → 8 days ago)
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// How far back in time the end of the removed data was (e.g., 7d → 7 days ago)
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// Configuration for the RWDB system to periodically remove RealWorldDataset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataPeriodicRemoveConfig {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Remove configuration
    #[prost(message, optional, tag = "4")]
    pub remove_config: ::core::option::Option<RealWorldDataRemoveDemand>,
    /// Time interval (cron format)
    #[prost(string, optional, tag = "5")]
    pub cron_config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "6")]
    pub enabled: ::core::option::Option<bool>,
    #[prost(message, repeated, tag = "7")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataPathPatternFormat {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Path pattern before replacement
    #[prost(string, optional, tag = "4")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
    /// Key name in dbp:tags (within RealWorldDataset) to be replaced
    #[prost(string, optional, tag = "5")]
    pub dataset_property: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// Represents extended properties for RealWorldData*
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataTags {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Tag key
    #[prost(string, optional, tag = "4")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// Tag value
    #[prost(string, optional, tag = "5")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Enumerates conversion characteristics, such as single-value-replace, col-merge, or row-merge.
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
/// Enumerates value characteristics, such as qualitative or quantitative.
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
