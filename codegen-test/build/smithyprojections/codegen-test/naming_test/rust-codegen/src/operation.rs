// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use crate::input::ErrCollisionsInput;
use crate::input::ReservedWordsAsMembersInput;
use crate::input::StructureNamePunningInput;
pub struct ErrCollisions {
    input: ErrCollisionsInput,
}
impl ErrCollisions {
    /// Creates a new builder-style object to manufacture [`ErrCollisionsInput`](crate::input::ErrCollisionsInput)
    pub fn builder() -> crate::input::err_collisions_input::Builder {
        crate::input::err_collisions_input::Builder::default()
    }
    pub fn build_http_request(&self) -> ::http::request::Request<Vec<u8>> {
        ErrCollisionsInput::assemble(self.input.request_builder_base(), self.input.build_body())
    }
    pub fn new(input: ErrCollisionsInput) -> Self {
        Self { input }
    }
}

pub struct ReservedWordsAsMembers {
    input: ReservedWordsAsMembersInput,
}
impl ReservedWordsAsMembers {
    /// Creates a new builder-style object to manufacture [`ReservedWordsAsMembersInput`](crate::input::ReservedWordsAsMembersInput)
    pub fn builder() -> crate::input::reserved_words_as_members_input::Builder {
        crate::input::reserved_words_as_members_input::Builder::default()
    }
    pub fn build_http_request(&self) -> ::http::request::Request<Vec<u8>> {
        ReservedWordsAsMembersInput::assemble(
            self.input.request_builder_base(),
            self.input.build_body(),
        )
    }
    pub fn new(input: ReservedWordsAsMembersInput) -> Self {
        Self { input }
    }
}
#[cfg(test)]
#[allow(unreachable_code, unused_variables)]
mod reserved_words_as_members_request_test {

    use crate::input::ReservedWordsAsMembersInput;
    /// Test ID: reserved_words
    #[test]
    fn test_reserved_words_request() {
        let input = ReservedWordsAsMembersInput::builder()
            .r#as(5)
            .r#async(true)
            .build();
        let http_request = input.build_http_request();

        assert_eq!(http_request.method(), "POST");
        assert_eq!(http_request.uri().path(), "/");

        ::protocol_test_helpers::assert_ok(::protocol_test_helpers::validate_body(
            &http_request.body(),
            "{\"as\": 5, \"async\": true}",
            ::protocol_test_helpers::MediaType::from("application/json"),
        ));
    }
}

pub struct StructureNamePunning {
    input: StructureNamePunningInput,
}
impl StructureNamePunning {
    /// Creates a new builder-style object to manufacture [`StructureNamePunningInput`](crate::input::StructureNamePunningInput)
    pub fn builder() -> crate::input::structure_name_punning_input::Builder {
        crate::input::structure_name_punning_input::Builder::default()
    }
    pub fn build_http_request(&self) -> ::http::request::Request<Vec<u8>> {
        StructureNamePunningInput::assemble(
            self.input.request_builder_base(),
            self.input.build_body(),
        )
    }
    pub fn new(input: StructureNamePunningInput) -> Self {
        Self { input }
    }
}
#[cfg(test)]
#[allow(unreachable_code, unused_variables)]
mod structure_name_punning_request_test {

    use crate::input::StructureNamePunningInput;
    use crate::model::String;
    /// Test ID: structure_punning
    #[test]
    fn test_structure_punning_request() {
        let input = StructureNamePunningInput::builder()
            .regular_string("hello!".to_string())
            .punned_string(String::builder().ps_member(true).build())
            .build();
        let http_request = input.build_http_request();

        assert_eq!(http_request.method(), "POST");
        assert_eq!(http_request.uri().path(), "/");

        ::protocol_test_helpers::assert_ok(::protocol_test_helpers::validate_body(
            &http_request.body(),
            "{\"regular_string\": \"hello!\", \"punned_string\": { \"ps_member\": true }}",
            ::protocol_test_helpers::MediaType::from("application/json"),
        ));
    }
}
