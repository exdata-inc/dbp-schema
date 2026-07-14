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
/// JSON-LD representing a real-world data field profile
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataFieldProfile {
    /// JSON-LD basic
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// structure as JSON-LD schema
    ///
    /// JSON-LD schema (including @graph)
    #[prost(message, optional, tag = "10")]
    pub structure: ::core::option::Option<RealWorldDataStructureGraph>,
    /// Original data format related
    ///
    /// schema.org's schema:encodingFormat (file MIME type)
    #[prost(string, optional, tag = "20")]
    pub encoding_format: ::core::option::Option<::prost::alloc::string::String>,
    /// Newline character in the file
    #[prost(string, optional, tag = "21")]
    pub new_line_character: ::core::option::Option<::prost::alloc::string::String>,
    /// Character encoding in the file
    #[prost(string, optional, tag = "22")]
    pub text_encoding: ::core::option::Option<::prost::alloc::string::String>,
    /// Compression related
    ///
    /// SemantiPack version used for metadata generation
    #[prost(string, optional, tag = "30")]
    pub rwd_profile_generator_version: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// SemantiPack version used for compression
    #[prost(string, optional, tag = "31")]
    pub semanti_pack_version: ::core::option::Option<::prost::alloc::string::String>,
    /// JSON related
    ///
    /// Whether JSON file has indentation
    #[prost(bool, optional, tag = "40")]
    pub json_has_indent: ::core::option::Option<bool>,
    /// Indent character string for JSON file with indentation
    #[prost(string, optional, tag = "41")]
    pub json_indent_character: ::core::option::Option<::prost::alloc::string::String>,
    /// CSV related
    ///
    /// Whether CSV file has header
    #[prost(bool, optional, tag = "50")]
    pub csv_has_header: ::core::option::Option<bool>,
    /// CSV quoting format (usage of ")
    #[prost(string, optional, tag = "51")]
    pub csv_quoting: ::core::option::Option<::prost::alloc::string::String>,
    /// CSV separator character
    #[prost(string, optional, tag = "52")]
    pub csv_separator: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether trailing columns can be omitted per row in CSV (when trailing column data is empty)
    #[prost(bool, optional, tag = "53")]
    pub csv_variable_columns_strip_trailing_commas: ::core::option::Option<bool>,
    /// Others
    ///
    /// GraphQL schema
    #[prost(string, optional, tag = "60")]
    pub graphql_schema: ::core::option::Option<::prost::alloc::string::String>,
    /// Date and time when structural information was created
    #[prost(string, optional, tag = "70")]
    pub date_created: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "80")]
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
    #[prost(message, repeated, tag = "4")]
    pub at_graph: ::prost::alloc::vec::Vec<GraphNode>,
    #[prost(message, repeated, tag = "5")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// JSON-LD schema (contents of @graph)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraphNode {
    #[prost(oneof = "graph_node::Node", tags = "1, 2, 3")]
    pub node: ::core::option::Option<graph_node::Node>,
}
/// Nested message and enum types in `GraphNode`.
pub mod graph_node {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Node {
        #[prost(message, tag = "1")]
        ClassNode(super::DbpClass),
        #[prost(message, tag = "2")]
        ListNode(super::DbpList),
        #[prost(message, tag = "3")]
        PropertyNode(super::RealWorldDataStructureProperty),
    }
}
/// Node referencing schema.org's @id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdRef {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Extension of rdfs:Class (subclass of: "rdfs:Class")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbpClass {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// rdfs:label
    #[prost(string, optional, tag = "2")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// Row structure ID when compressing with SemantiPack (matches list ID)
    #[prost(int32, optional, tag = "3")]
    pub dbpa_compress_row_id: ::core::option::Option<i32>,
    /// schema:domainIncludes
    #[prost(message, repeated, tag = "4")]
    pub domain_includes: ::prost::alloc::vec::Vec<IdRef>,
    /// schema:rangeIncludes
    #[prost(message, repeated, tag = "5")]
    pub range_includes: ::prost::alloc::vec::Vec<IdRef>,
}
/// Extension of rdf:List (subclass of: "rdf:List")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DbpList {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// rdfs:label
    #[prost(string, optional, tag = "2")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether to compress the list structure with SemantiPack
    #[prost(bool, optional, tag = "3")]
    pub dbpa_compress: ::core::option::Option<bool>,
    /// List structure ID when compressing with SemantiPack (matches row ID)
    #[prost(int32, optional, tag = "4")]
    pub dbpa_compress_list_id: ::core::option::Option<i32>,
    /// List of child list IDs when child lists exist during SemantiPack compression
    #[prost(int32, repeated, tag = "5")]
    pub dbpa_children_lists: ::prost::alloc::vec::Vec<i32>,
    /// schema:domainIncludes
    #[prost(message, repeated, tag = "6")]
    pub domain_includes: ::prost::alloc::vec::Vec<IdRef>,
    /// schema:rangeIncludes
    #[prost(message, repeated, tag = "7")]
    pub range_includes: ::prost::alloc::vec::Vec<IdRef>,
}
/// Class for describing data metadata in JSON-LD
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataStructureProperty {
    /// JSON-LD basic
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// rdfs:label
    #[prost(string, optional, tag = "2")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
    /// rdfs:comment Comment describing the contents of data field
    #[prost(string, optional, tag = "3")]
    pub comment: ::core::option::Option<::prost::alloc::string::String>,
    /// rdfs:subPropertyOf Corresponding item on schema.org
    #[prost(string, optional, tag = "4")]
    pub rdfs_sub_property_of: ::core::option::Option<::prost::alloc::string::String>,
    /// Relations (domain / range)
    ///
    /// schema:domainIncludes
    #[prost(message, repeated, tag = "10")]
    pub domain_includes: ::prost::alloc::vec::Vec<IdRef>,
    /// schema:rangeIncludes
    #[prost(message, repeated, tag = "11")]
    pub range_includes: ::prost::alloc::vec::Vec<IdRef>,
    /// Compression metadata
    ///
    /// List structure ID assigned to columns during SemantiPack compression (matches row ID)
    #[prost(int32, optional, tag = "20")]
    pub dbpa_compress_parent_list_id: ::core::option::Option<i32>,
    /// Column structure ID during SemantiPack compression
    #[prost(int32, optional, tag = "21")]
    pub dbpa_compress_column_id: ::core::option::Option<i32>,
    /// Number of bytes used to represent differential values during differential compression
    #[prost(int32, optional, tag = "22")]
    pub dbpa_diff_num_bytes: ::core::option::Option<i32>,
    /// Number of bytes used to represent the first value during differential compression
    #[prost(int32, optional, tag = "23")]
    pub dbpa_first_num_bytes: ::core::option::Option<i32>,
    /// Whether to strip trailing zeros in decimals
    #[prost(bool, optional, tag = "24")]
    pub strip_trailing_zeros: ::core::option::Option<bool>,
    /// Whether run-length compression should be used
    #[prost(bool, optional, tag = "25")]
    pub use_run_length: ::core::option::Option<bool>,
    /// Compression layer to forcibly use during SemantiPack compression
    #[prost(string, optional, tag = "26")]
    pub dbpa_compression_force_layer: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    /// Loss ratio between original data and SemantiPack lossy compression
    #[prost(float, optional, tag = "27")]
    pub lossy_compression_rate: ::core::option::Option<f32>,
    /// Whether FFT compression should be used during SemantiPack compression
    #[prost(bool, optional, tag = "28")]
    pub use_fft_compression: ::core::option::Option<bool>,
    /// Type related
    ///
    /// Semantic type of data
    #[prost(enumeration = "ItemType", optional, tag = "30")]
    pub item_type: ::core::option::Option<i32>,
    /// Semantic type satisfied by 95% or more of the data
    #[prost(enumeration = "ItemType", optional, tag = "31")]
    pub item_type95p: ::core::option::Option<i32>,
    /// Unit
    ///
    /// Value representing the unit of the variable (unece.org Rec 20 value)
    #[prost(message, optional, tag = "40")]
    pub unit_text: ::core::option::Option<::prost::alloc::string::String>,
    /// Variable characteristics
    ///
    /// Enum value representing the scale of the variable
    #[prost(enumeration = "VariableScaleTypeEnumeration", optional, tag = "50")]
    pub variable_scale_type: ::core::option::Option<i32>,
    /// Boolean metadata
    ///
    /// Whether the data field is mostly constant
    #[prost(bool, optional, tag = "60")]
    pub is_mostly_constant: ::core::option::Option<bool>,
    /// Whether the data field is an enum value
    #[prost(bool, optional, tag = "61")]
    pub is_enum_value: ::core::option::Option<bool>,
    /// Whether the data field is mostly monotonically increasing
    #[prost(bool, optional, tag = "62")]
    pub is_mostly_incremental: ::core::option::Option<bool>,
    /// Whether empty strings or null values may exist for numeric or datetime types
    #[prost(bool, optional, tag = "63")]
    pub is_nullable: ::core::option::Option<bool>,
    /// Indicates that the data field may not exist
    #[prost(bool, optional, tag = "64")]
    pub is_optional: ::core::option::Option<bool>,
    /// Datetime metadata
    ///
    /// Format string when the data field is of datetime type
    #[prost(string, optional, tag = "70")]
    pub dbpa_datetime_format: ::core::option::Option<::prost::alloc::string::String>,
    /// Minimum unit when the data field is of datetime type (string equivalent to @schema:unitText)
    #[prost(string, optional, tag = "71")]
    pub dbpa_datetime_precision: ::core::option::Option<::prost::alloc::string::String>,
    /// Number of bytes used to represent differences when the data field is of datetime type
    #[prost(int32, optional, tag = "72")]
    pub dbpa_datetime_diff_bytes: ::core::option::Option<i32>,
    /// Timezone offset value for date/time used during compression
    #[prost(int32, optional, tag = "73")]
    pub dbpa_date_time_format_offset: ::core::option::Option<i32>,
    /// Whether to represent UTC±0 timezone as UTC in RFC3339-compliant date/time format
    #[prost(bool, optional, tag = "74")]
    pub dbpa_date_time_format_utc_is_ut_cin_rfc3339: ::core::option::Option<bool>,
    /// Whether to represent UTC±0 timezone as Z in RFC3339-compliant date/time format
    #[prost(bool, optional, tag = "75")]
    pub dbpa_date_time_format_utc_is_zin_rfc3339: ::core::option::Option<bool>,
    /// List of column IDs when data field is of datetime type
    #[prost(int32, repeated, tag = "76")]
    pub dbpa_timestamp_representing_columns: ::prost::alloc::vec::Vec<i32>,
    /// Value representing the time unit of timestamp when data field is of datetime type
    #[prost(string, optional, tag = "77")]
    pub dbpa_timestamp_unit_text: ::core::option::Option<::prost::alloc::string::String>,
    /// Numeric range etc
    ///
    /// Minimum value of numeric range
    #[prost(string, optional, tag = "80")]
    pub range_min: ::core::option::Option<::prost::alloc::string::String>,
    /// Maximum value of numeric range
    #[prost(string, optional, tag = "81")]
    pub range_max: ::core::option::Option<::prost::alloc::string::String>,
    /// Number of bytes used for numeric representation during compression
    #[prost(int32, optional, tag = "82")]
    pub precision_bytes: ::core::option::Option<i32>,
    /// How many decimal places to retain when compressing float values
    #[prost(int32, optional, tag = "83")]
    pub decimal_places: ::core::option::Option<i32>,
    /// Most frequent value of differences when data field is monotonically increasing
    #[prost(string, optional, tag = "84")]
    pub base_increment: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether the most frequent value of differences should be used for compression when data field is monotonically increasing
    #[prost(bool, optional, tag = "85")]
    pub use_base_increment: ::core::option::Option<bool>,
    /// Value samples,  enumerated values, and structure path
    ///
    /// Used to pass value samples to LLM API during compression (non-enum)
    #[prost(string, repeated, tag = "90")]
    pub value_samples: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Used to pass list of enum values to LLM API during compression
    #[prost(string, repeated, tag = "91")]
    pub enum_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Data field catalogue
    ///
    /// Reference to the data field type in the data field catalogue
    #[prost(message, optional, tag = "100")]
    pub field_type: ::core::option::Option<DataFieldType>,
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
/// Kind of a data field in the data field catalogue, independent of any specific dataset (subclass of: "schema:Intangible")
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataFieldType {
    /// JSON-LD basic
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Identification
    ///
    /// Alternative notations of the field name (used as search keys)
    #[prost(string, repeated, tag = "10")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Lifecycle
    ///
    /// Lifecycle status of the catalogue entry (draft / stable / deprecated)
    #[prost(string, optional, tag = "20")]
    pub status: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "21")]
    pub version: ::core::option::Option<i32>,
    #[prost(message, optional, tag = "22")]
    pub date_modified: ::core::option::Option<::prost_types::Timestamp>,
    /// Catalogue entry that supersedes this entry (redirect target when duplicated entries are merged)
    #[prost(message, optional, boxed, tag = "23")]
    pub superseded_by: ::core::option::Option<::prost::alloc::boxed::Box<DataFieldType>>,
    /// Seed origin of the catalogue entry (human / extracted-from-profiles / llm-rec20 / open-data-survey)
    #[prost(string, optional, tag = "24")]
    pub provenance: ::core::option::Option<::prost::alloc::string::String>,
    /// Type and value
    #[prost(enumeration = "ItemType", optional, tag = "30")]
    pub item_type: ::core::option::Option<i32>,
    #[prost(enumeration = "VariableScaleTypeEnumeration", optional, tag = "31")]
    pub variable_scale_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "32")]
    pub range_min: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "33")]
    pub range_max: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "34")]
    pub decimal_places: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "35")]
    pub is_enum_value: ::core::option::Option<bool>,
    #[prost(string, repeated, tag = "36")]
    pub enum_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "37")]
    pub value_samples: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Unit
    #[prost(message, optional, tag = "40")]
    pub unit_text: ::core::option::Option<::prost::alloc::string::String>,
    /// Unit code of the variable (UNECE Recommendation 20 code)
    #[prost(string, optional, tag = "41")]
    pub unit_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Datetime
    #[prost(string, optional, tag = "50")]
    pub dbpa_datetime_format: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "51")]
    pub dbpa_timestamp_unit_text: ::core::option::Option<::prost::alloc::string::String>,
    /// Tendency (prior knowledge on how fields of this kind generally behave)
    #[prost(bool, optional, tag = "60")]
    pub is_mostly_constant: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "61")]
    pub is_mostly_incremental: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "62")]
    pub is_optional: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "63")]
    pub is_nullable: ::core::option::Option<bool>,
    /// Compression
    ///
    /// Recommended compression settings per use case
    #[prost(message, repeated, tag = "70")]
    pub compression_hints: ::prost::alloc::vec::Vec<CompressionHint>,
    #[prost(message, repeated, tag = "80")]
    pub tags: ::prost::alloc::vec::Vec<RealWorldDataTags>,
}
/// Recommended compression settings of a data field type for a specific use case
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompressionHint {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// Identifier of the use case for this compression hint (kebab-case, e.g. realtime-monitoring)
    #[prost(string, optional, tag = "4")]
    pub use_case: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Explanation of the error tolerable in this use case
    #[prost(string, optional, tag = "6")]
    pub accuracy_note: ::core::option::Option<::prost::alloc::string::String>,
    /// Recommended values (compression-related properties of RealWorldDataStructureProperty reused as recommendations)
    #[prost(int32, optional, tag = "10")]
    pub decimal_places: ::core::option::Option<i32>,
    #[prost(float, optional, tag = "11")]
    pub lossy_compression_rate: ::core::option::Option<f32>,
    #[prost(int32, optional, tag = "12")]
    pub precision_bytes: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "13")]
    pub use_fft_compression: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "14")]
    pub use_run_length: ::core::option::Option<bool>,
    #[prost(bool, optional, tag = "15")]
    pub use_base_increment: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "16")]
    pub dbpa_datetime_precision: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "17")]
    pub is_mostly_incremental: ::core::option::Option<bool>,
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
/// Enum representing semantic type of data (maximum string length for STRING is not represented)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ItemType {
    Integer = 0,
    Float = 1,
    String = 2,
    Boolean = 3,
    Date = 4,
    Time = 5,
    Datetime = 6,
    Null = 7,
}
impl ItemType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ItemType::Integer => "INTEGER",
            ItemType::Float => "FLOAT",
            ItemType::String => "STRING",
            ItemType::Boolean => "BOOLEAN",
            ItemType::Date => "DATE",
            ItemType::Time => "TIME",
            ItemType::Datetime => "DATETIME",
            ItemType::Null => "NULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INTEGER" => Some(Self::Integer),
            "FLOAT" => Some(Self::Float),
            "STRING" => Some(Self::String),
            "BOOLEAN" => Some(Self::Boolean),
            "DATE" => Some(Self::Date),
            "TIME" => Some(Self::Time),
            "DATETIME" => Some(Self::Datetime),
            "NULL" => Some(Self::Null),
            _ => None,
        }
    }
}
/// Enum representing variable scale
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VariableScaleTypeEnumeration {
    Nominal = 0,
    Ordinal = 1,
    Interval = 2,
    Proportional = 3,
}
impl VariableScaleTypeEnumeration {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VariableScaleTypeEnumeration::Nominal => "NOMINAL",
            VariableScaleTypeEnumeration::Ordinal => "ORDINAL",
            VariableScaleTypeEnumeration::Interval => "INTERVAL",
            VariableScaleTypeEnumeration::Proportional => "PROPORTIONAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NOMINAL" => Some(Self::Nominal),
            "ORDINAL" => Some(Self::Ordinal),
            "INTERVAL" => Some(Self::Interval),
            "PROPORTIONAL" => Some(Self::Proportional),
            _ => None,
        }
    }
}
