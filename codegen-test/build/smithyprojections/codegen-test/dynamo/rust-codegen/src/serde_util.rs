// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use smithy_types::Blob;
use smithy_types::Instant;
pub fn optioninstant_epoch_seconds_ser<S>(
    _inp: &Option<Instant>,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    match _inp {
        Some(ts) => _serializer.serialize_some(&ts.epoch_seconds()),
        None => _serializer.serialize_none(),
    }
}

pub fn optioninstant_epoch_seconds_deser<'de, D>(_deser: D) -> Result<Option<Instant>, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    todo!()
}

pub fn blob_ser<S>(
    _inp: &Blob,
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    _serializer.serialize_str(&::smithy_http::base64::encode(_inp.as_ref()))
}

pub fn blob_deser<'de, D>(_deser: D) -> Result<Blob, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    todo!()
}

pub fn vecblob_ser<S>(
    _inp: &[Blob],
    _serializer: S,
) -> Result<<S as ::serde::Serializer>::Ok, <S as ::serde::Serializer>::Error>
where
    S: ::serde::Serializer,
{
    todo!()
}

pub fn vecblob_deser<'de, D>(_deser: D) -> Result<Vec<Blob>, D::Error>
where
    D: ::serde::Deserializer<'de>,
{
    todo!()
}
