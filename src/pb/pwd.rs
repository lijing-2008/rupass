#[derive(Eq, Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwdKey {
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user: ::prost::alloc::string::String,
}
#[derive(Eq, Hash)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwdInfo {
    #[prost(string, tag="1")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub notes: ::prost::alloc::string::String,
}
