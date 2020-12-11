// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct Vec {
    #[serde(rename = "pv_member")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pv_member: ::std::option::Option<bool>,
}
/// See [`Vec`](crate::model::Vec)
pub mod vec {

    use crate::model::Vec;
    /// A builder for [`Vec`](crate::model::Vec)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        pv_member: ::std::option::Option<bool>,
    }
    impl Builder {
        pub fn pv_member(mut self, inp: bool) -> Self {
            self.pv_member = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`Vec`](crate::model::Vec)
        pub fn build(self) -> Vec {
            Vec {
                pv_member: self.pv_member,
            }
        }
    }
}
impl Vec {
    /// Creates a new builder-style object to manufacture [`Vec`](crate::model::Vec)
    pub fn builder() -> crate::model::vec::Builder {
        crate::model::vec::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    ::serde::Deserialize,
    ::serde::Serialize,
    ::std::clone::Clone,
    ::std::cmp::PartialEq,
    ::std::fmt::Debug,
)]
pub struct String {
    #[serde(rename = "ps_member")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ps_member: ::std::option::Option<bool>,
}
/// See [`String`](crate::model::String)
pub mod string {

    use crate::model::String;
    /// A builder for [`String`](crate::model::String)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        ps_member: ::std::option::Option<bool>,
    }
    impl Builder {
        pub fn ps_member(mut self, inp: bool) -> Self {
            self.ps_member = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`String`](crate::model::String)
        pub fn build(self) -> String {
            String {
                ps_member: self.ps_member,
            }
        }
    }
}
impl String {
    /// Creates a new builder-style object to manufacture [`String`](crate::model::String)
    pub fn builder() -> crate::model::string::Builder {
        crate::model::string::Builder::default()
    }
}
