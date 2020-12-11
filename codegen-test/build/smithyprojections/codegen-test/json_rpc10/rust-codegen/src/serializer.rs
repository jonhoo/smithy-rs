// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use crate::model::MyUnion;
/// A shared structure that contains a single union member.
#[non_exhaustive]
#[derive(::serde::Serialize, ::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JsonUnionsInputBody<'a> {
    /// A union with a representative set of types for members.
    #[serde(rename = "contents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contents: &'a ::std::option::Option<MyUnion>,
}

#[non_exhaustive]
#[derive(::serde::Deserialize, ::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GreetingWithErrorsOutputBody {
    #[serde(rename = "greeting")]
    pub greeting: ::std::option::Option<::std::string::String>,
}

/// A shared structure that contains a single union member.
#[non_exhaustive]
#[derive(::serde::Deserialize, ::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JsonUnionsOutputBody {
    /// A union with a representative set of types for members.
    #[serde(rename = "contents")]
    pub contents: ::std::option::Option<MyUnion>,
}
