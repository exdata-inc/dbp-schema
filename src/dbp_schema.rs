#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataset {
    /// 自身(このJSON-LD)のURL
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    /// (人間が見てわかりやすい)データセットの名前
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// (未使用)
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// 構造情報
    #[prost(message, optional, tag = "4")]
    pub structure_info: ::core::option::Option<RealWorldDataStructureInfo>,
    /// このデータセットの基になったデータセット(醸造時の入力データセット)
    #[prost(message, repeated, tag = "5")]
    pub generated_from: ::prost::alloc::vec::Vec<RealWorldDataset>,
    /// このデータセットを作った醸造プログラム
    #[prost(message, optional, tag = "6")]
    pub generated_using: ::core::option::Option<RealWorldDataBrewerInfo>,
    /// このデータセットを作った際のパラメータ(醸造時の入力パラメータ)
    #[prost(message, repeated, tag = "7")]
    pub generated_args: ::prost::alloc::vec::Vec<RealWorldDataBrewingArgument>,
    /// 収集情報
    #[prost(message, optional, tag = "8")]
    pub collection_info: ::core::option::Option<RealWorldDataCollectionInfo>,
    /// データの保存先（データの記録期間に応じて、複数の場所に置く場合もあるため repeated）
    #[prost(message, repeated, tag = "9")]
    pub distribution: ::prost::alloc::vec::Vec<RealWorldDataStoringInfo>,
    /// データを集めている主体
    #[prost(string, optional, tag = "10")]
    pub author: ::core::option::Option<::prost::alloc::string::String>,
    /// データを集めている場所
    #[prost(string, optional, tag = "11")]
    pub content_location: ::core::option::Option<::prost::alloc::string::String>,
    /// データを集め始めた日時
    #[prost(message, optional, tag = "12")]
    pub date_created: ::core::option::Option<::prost_types::Timestamp>,
    /// 最新のデータを集め始めた日時
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
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub input_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "VariableCharacteristicEnumeration", optional, tag = "7")]
    pub input_characteristic: ::core::option::Option<i32>,
}
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
    #[prost(string, optional, tag = "5")]
    pub output_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(enumeration = "VariableCharacteristicEnumeration", optional, tag = "7")]
    pub output_characteristic: ::core::option::Option<i32>,
}
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
    #[prost(string, optional, tag = "5")]
    pub argument_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "6")]
    pub value: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
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
    #[prost(enumeration = "ConversionCharacteristicEnumeration", optional, tag = "7")]
    pub conversion_characteristic: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataCollectionInfo {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub collection_style: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub collection_protocol: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub listen_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub server_address: ::core::option::Option<::prost::alloc::string::String>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataStructureInfo {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub encoding_format: ::core::option::Option<::prost::alloc::string::String>,
    /// JSON-LD schema ( including @graph )
    #[prost(message, optional, tag = "5")]
    pub structure: ::core::option::Option<::prost_types::Struct>,
    #[prost(string, optional, tag = "6")]
    pub graphql_schema: ::core::option::Option<::prost::alloc::string::String>,
}
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
    #[prost(string, optional, tag = "6")]
    pub base_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub pattern: ::core::option::Option<::prost::alloc::string::String>,
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
}
/// RWDB システムが定期的に RealWorldDataBrewingDemand を投げる設定
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataPeriodicBrewingConfig {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// 投げる Demand
    #[prost(message, optional, tag = "4")]
    pub brewing_config: ::core::option::Option<RealWorldDataBrewingDemand>,
    /// 時間間隔 (cron format)
    #[prost(string, optional, tag = "5")]
    pub cron_config: ::core::option::Option<::prost::alloc::string::String>,
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
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    #[prost(string, optional, tag = "5")]
    pub graphql_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub timerange_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub sparql_query: ::core::option::Option<::prost::alloc::string::String>,
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
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    #[prost(string, optional, tag = "5")]
    pub graphql_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub timerange_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub sparql_query: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "8")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
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
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// 書き込むデータがいつからのデータか
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// 書き込むデータがいつまでのデータか
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "7")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
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
    /// 書き込んだデータがいつからのデータか
    #[prost(string, optional, tag = "5")]
    pub time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// 書き込んだデータがいつまでのデータか
    #[prost(string, optional, tag = "6")]
    pub time_period_end: ::core::option::Option<::prost::alloc::string::String>,
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
    /// 移動する RealWorldDataset
    #[prost(message, optional, tag = "4")]
    pub target_dataset: ::core::option::Option<RealWorldDataset>,
    /// 移動する RealWorldDataset のうち対象となるデータの開始時刻はどれだけ前か (ex. 8d → 8日前)
    #[prost(string, optional, tag = "5")]
    pub target_time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// 移動する RealWorldDataset のうち対象となるデータの終了時刻はどれだけ前か (ex. 7d → 7日前)
    #[prost(string, optional, tag = "6")]
    pub target_time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    /// データの移動元
    #[prost(message, optional, tag = "7")]
    pub move_from: ::core::option::Option<RealWorldDataStoringInfo>,
    /// データの移動先
    #[prost(message, optional, tag = "8")]
    pub move_to: ::core::option::Option<RealWorldDataStoringInfo>,
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
    /// 移動した RealWorldDataset
    #[prost(message, optional, tag = "4")]
    pub target_dataset: ::core::option::Option<RealWorldDataset>,
    /// 移動した RealWorldDataset のうち対象となるデータの開始時刻はどれだけ前か (ex. 8d → 8日前)
    #[prost(string, optional, tag = "5")]
    pub target_time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// 移動した RealWorldDataset のうち対象となるデータの終了時刻はどれだけ前か (ex. 7d → 7日前)
    #[prost(string, optional, tag = "6")]
    pub target_time_period_end: ::core::option::Option<::prost::alloc::string::String>,
    /// データの移動元
    #[prost(message, optional, tag = "7")]
    pub move_from: ::core::option::Option<RealWorldDataStoringInfo>,
    /// データの移動先
    #[prost(message, optional, tag = "8")]
    pub move_to: ::core::option::Option<RealWorldDataStoringInfo>,
    /// 移動先の RealWorldDataset
    #[prost(message, optional, tag = "9")]
    pub moved_dataset: ::core::option::Option<RealWorldDataset>,
}
/// RWDB システムが定期的に RealWorldDataset を移動する設定
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataPeriodicMoveConfig {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// 移動設定
    #[prost(message, optional, tag = "4")]
    pub move_config: ::core::option::Option<RealWorldDataMoveDemand>,
    /// 時間間隔 (cron format)
    #[prost(string, optional, tag = "5")]
    pub cron_config: ::core::option::Option<::prost::alloc::string::String>,
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
    /// 削除する RealWorldDataset
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// 削除する RealWorldDataset のうち対象となるデータの開始時刻はどれだけ前か (ex. 8d → 8日前)
    #[prost(string, optional, tag = "5")]
    pub target_time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// 削除する RealWorldDataset のうち対象となるデータの終了時刻はどれだけ前か (ex. 7d → 7日前)
    #[prost(string, optional, tag = "6")]
    pub target_time_period_end: ::core::option::Option<::prost::alloc::string::String>,
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
    /// 削除した RealWorldDataset
    #[prost(message, optional, tag = "4")]
    pub dataset: ::core::option::Option<RealWorldDataset>,
    /// 削除した RealWorldDataset のうち対象となるデータの開始時刻はどれだけ前か (ex. 8d → 8日前)
    #[prost(string, optional, tag = "5")]
    pub target_time_period_start: ::core::option::Option<::prost::alloc::string::String>,
    /// 削除した RealWorldDataset のうち対象となるデータの終了時刻はどれだけ前か (ex. 7d → 7日前)
    #[prost(string, optional, tag = "6")]
    pub target_time_period_end: ::core::option::Option<::prost::alloc::string::String>,
}
/// RWDB システムが定期的に RealWorldDataset を削除する設定
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealWorldDataPeriodicRemoveConfig {
    #[prost(string, optional, tag = "1")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
    /// 削除設定
    #[prost(message, optional, tag = "4")]
    pub remove_config: ::core::option::Option<RealWorldDataRemoveDemand>,
    /// 時間間隔 (cron format)
    #[prost(string, optional, tag = "5")]
    pub cron_config: ::core::option::Option<::prost::alloc::string::String>,
}
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
