//! Legacy names and ownership helpers backed by the current SV2 binary codec.
//!
//! Byte strings own their bytes. Request IDs retain mutable borrows when decoded,
//! because routing updates those bytes in the original receive buffer.
//! Wire validation and encoding use binary_sv2.

pub use super::ffi::{
    _c_export_cvec, _c_export_cvec2, _c_export_u24, cvec2_push, cvec_from_buffer, free_vec,
    free_vec_2, init_cvec2, CVec, CVec2,
};

/// Legacy path retained for callers of the former codec crate's reexports.
pub mod binary_codec_sv2 {
    pub use super::*;
}
pub use ::binary_sv2::{
    decodable, encodable, from_bytes, to_bytes, Decodable, Deserialize, Encodable, EncodableField,
    Error, Fixed, GetMarker, GetSize, Serialize, SizeHint, Sv2DataType, U24,
};
use core::convert::{TryFrom, TryInto};
use core::marker::PhantomData;
use decodable::{DecodableField, FieldMarker};

/// Encodes into a caller-provided buffer with the legacy unit return value.
pub fn to_writer<T: Encodable>(src: T, dst: &mut [u8]) -> Result<(), Error> {
    ::binary_sv2::to_writer(src, dst).map(|_| ())
}

/// Preserves the legacy integer helper's byte layout.
pub fn u256_from_int<V: Into<u64>>(value: V) -> U256<'static> {
    let mut bytes = [0; 32];
    bytes[24..].copy_from_slice(&value.into().to_le_bytes());
    bytes.into()
}

/// Shared byte access used by the legacy sequence helpers.
pub trait LegacyBytes {
    fn legacy_bytes(&self) -> &[u8];
}

macro_rules! byte_wrapper {
    ($name:ident, $alias:ident, $borrowed:ident, $owned:ident, $fixed:literal) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name<'a>(::binary_sv2::$owned, PhantomData<&'a [u8]>);

        pub type $alias<'a> = $name<'a>;

        impl<'a> $name<'a> {
            pub fn to_vec(&self) -> Vec<u8> {
                self.0.to_owned_bytes()
            }
            pub fn inner_as_ref(&self) -> &[u8] {
                self.0.as_bytes()
            }
            pub fn inner_as_mut(&mut self) -> &mut [u8] {
                self.0.as_mut_bytes()
            }
            // Legacy fixed-size byte values count as a single element.
            pub fn len(&self) -> usize {
                if $fixed {
                    1
                } else {
                    self.0.len()
                }
            }
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
            pub fn into_static(self) -> $name<'static> {
                $name(self.0, PhantomData)
            }
            pub fn as_static(&self) -> $name<'static> {
                self.clone().into_static()
            }
            pub fn to_hex(&self) -> String {
                self.0.hex_string()
            }
            pub fn to_hex_reversed(&self) -> String {
                self.0.reversed_hex_string()
            }
        }

        impl IntoStatic for $name<'_> {
            type Static = $name<'static>;
            fn into_static(self) -> Self::Static {
                self.into_static()
            }
        }
        impl AsRef<[u8]> for $name<'_> {
            fn as_ref(&self) -> &[u8] {
                self.inner_as_ref()
            }
        }
        impl LegacyBytes for $name<'_> {
            fn legacy_bytes(&self) -> &[u8] {
                self.inner_as_ref()
            }
        }
        impl TryFrom<Vec<u8>> for $name<'_> {
            type Error = Error;
            fn try_from(value: Vec<u8>) -> Result<Self, Error> {
                ::binary_sv2::$owned::try_from(value).map(|value| Self(value, PhantomData))
            }
        }
        impl TryFrom<&[u8]> for $name<'_> {
            type Error = Error;
            fn try_from(value: &[u8]) -> Result<Self, Error> {
                ::binary_sv2::$owned::try_from(value).map(|value| Self(value, PhantomData))
            }
        }
        impl TryFrom<&mut [u8]> for $name<'_> {
            type Error = Error;
            fn try_from(value: &mut [u8]) -> Result<Self, Error> {
                Self::try_from(&*value)
            }
        }
        impl From<::binary_sv2::$owned> for $name<'_> {
            fn from(value: ::binary_sv2::$owned) -> Self {
                Self(value, PhantomData)
            }
        }
        impl From<::binary_sv2::$borrowed<'_>> for $name<'_> {
            fn from(value: ::binary_sv2::$borrowed<'_>) -> Self {
                Self(value.into_owned(), PhantomData)
            }
        }
        impl From<$name<'_>> for ::binary_sv2::$owned {
            fn from(value: $name<'_>) -> Self {
                value.0
            }
        }
        impl GetSize for $name<'_> {
            fn get_size(&self) -> usize {
                self.0.get_size()
            }
        }
        impl SizeHint for $name<'_> {
            fn size_hint(data: &[u8], offset: usize) -> Result<usize, Error> {
                <::binary_sv2::$owned as SizeHint>::size_hint(data, offset)
            }
            fn size_hint_(&self, data: &[u8], offset: usize) -> Result<usize, Error> {
                Self::size_hint(data, offset)
            }
        }
        impl GetMarker for $name<'_> {
            fn get_marker() -> FieldMarker {
                ::binary_sv2::$owned::get_marker()
            }
        }
        impl From<$name<'_>> for FieldMarker {
            fn from(_: $name<'_>) -> Self {
                $name::get_marker()
            }
        }
        impl<'a> Decodable<'a> for $name<'_> {
            fn get_structure(data: &[u8]) -> Result<Vec<FieldMarker>, Error> {
                ::binary_sv2::$owned::get_structure(data)
            }
            fn from_decoded_fields(data: Vec<DecodableField<'a>>) -> Result<Self, Error> {
                ::binary_sv2::$owned::from_decoded_fields(data)
                    .map(|value| Self(value, PhantomData))
            }
        }
        impl<'a> Sv2DataType<'a> for $name<'_> {
            fn from_bytes_(data: &'a mut [u8]) -> Result<Self, Error> {
                ::binary_sv2::$owned::from_bytes_(data).map(|value| Self(value, PhantomData))
            }
            fn to_slice(&'a self, dst: &mut [u8]) -> Result<usize, Error> {
                self.0.to_slice(dst)
            }
        }
        impl<'a> From<$name<'_>> for EncodableField<'a> {
            fn from(value: $name<'_>) -> Self {
                value.0.into()
            }
        }
    };
}

byte_wrapper!(LegacyU256, U256, U256, U256Owned, true);
byte_wrapper!(LegacyB032, B032, B032, B032Owned, false);
byte_wrapper!(LegacyB0255, B0255, B0255, B0255Owned, false);
byte_wrapper!(LegacyB064K, B064K, B064K, B064KOwned, false);
byte_wrapper!(LegacyB016M, B016M, B016M, B016MOwned, false);
byte_wrapper!(LegacySignature, Signature, Signature, SignatureOwned, true);

pub type Str0255<'a> = B0255<'a>;
pub type PubKey<'a> = U256<'a>;

impl From<[u8; 32]> for LegacyU256<'_> {
    fn from(value: [u8; 32]) -> Self {
        Self(value.into(), PhantomData)
    }
}

impl TryFrom<String> for LegacyB0255<'_> {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Error> {
        value.into_bytes().try_into()
    }
}

/// The former byte-backed u32 used in legacy job declaration messages.
/// Borrowed values deliberately retain write-through access for request-id routing.
#[derive(Debug)]
pub enum LegacyU32<'a> {
    Ref(&'a mut [u8]),
    Owned(Vec<u8>),
}
pub type U32AsRef<'a> = LegacyU32<'a>;

impl PartialEq for LegacyU32<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.inner_as_ref() == other.inner_as_ref()
    }
}
impl Eq for LegacyU32<'_> {}
impl Clone for LegacyU32<'_> {
    fn clone(&self) -> Self {
        Self::Owned(self.to_vec())
    }
}
impl<'a> LegacyU32<'a> {
    pub fn as_u32(&self) -> u32 {
        u32::from_le_bytes(
            self.inner_as_ref()
                .try_into()
                .expect("U32AsRef is four bytes"),
        )
    }
    pub fn to_vec(&self) -> Vec<u8> {
        self.inner_as_ref().to_vec()
    }
    pub fn inner_as_ref(&self) -> &[u8] {
        match self {
            Self::Ref(value) => value,
            Self::Owned(value) => value,
        }
    }
    pub fn inner_as_mut(&mut self) -> &mut [u8] {
        match self {
            Self::Ref(value) => value,
            Self::Owned(value) => value,
        }
    }
    pub fn len(&self) -> usize {
        1
    }
    pub fn is_empty(&self) -> bool {
        false
    }
    pub fn into_static(self) -> LegacyU32<'static> {
        LegacyU32::Owned(self.to_vec())
    }
    pub fn as_static(&self) -> LegacyU32<'static> {
        LegacyU32::Owned(self.to_vec())
    }
}
impl IntoStatic for LegacyU32<'_> {
    type Static = LegacyU32<'static>;
    fn into_static(self) -> Self::Static {
        self.into_static()
    }
}
impl From<u32> for LegacyU32<'_> {
    fn from(value: u32) -> Self {
        Self::Owned(value.to_le_bytes().to_vec())
    }
}
impl From<LegacyU32<'_>> for u32 {
    fn from(value: LegacyU32<'_>) -> Self {
        value.as_u32()
    }
}
impl From<&LegacyU32<'_>> for u32 {
    fn from(value: &LegacyU32<'_>) -> Self {
        value.as_u32()
    }
}
fn validate_u32(value: &[u8]) -> Result<(), Error> {
    if value.len() == 4 {
        Ok(())
    } else {
        Err(Error::ValueExceedsMaxSize(
            true,
            4,
            0,
            0,
            value.iter().take(32).copied().collect(),
            value.len(),
        ))
    }
}
impl TryFrom<&[u8]> for LegacyU32<'_> {
    type Error = Error;
    fn try_from(value: &[u8]) -> Result<Self, Error> {
        validate_u32(value)?;
        Ok(Self::Owned(value.to_vec()))
    }
}
impl<'a> TryFrom<&'a mut [u8]> for LegacyU32<'a> {
    type Error = Error;
    fn try_from(value: &'a mut [u8]) -> Result<Self, Error> {
        validate_u32(value)?;
        Ok(Self::Ref(value))
    }
}
impl TryFrom<Vec<u8>> for LegacyU32<'_> {
    type Error = Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Error> {
        validate_u32(&value)?;
        Ok(Self::Owned(value))
    }
}
impl AsRef<[u8]> for LegacyU32<'_> {
    fn as_ref(&self) -> &[u8] {
        self.inner_as_ref()
    }
}
impl LegacyBytes for LegacyU32<'_> {
    fn legacy_bytes(&self) -> &[u8] {
        self.inner_as_ref()
    }
}
impl GetSize for LegacyU32<'_> {
    fn get_size(&self) -> usize {
        4
    }
}
impl SizeHint for LegacyU32<'_> {
    fn size_hint(data: &[u8], offset: usize) -> Result<usize, Error> {
        u32::size_hint(data, offset)
    }
    fn size_hint_(&self, data: &[u8], offset: usize) -> Result<usize, Error> {
        Self::size_hint(data, offset)
    }
}
impl GetMarker for LegacyU32<'_> {
    fn get_marker() -> FieldMarker {
        u32::get_marker()
    }
}
impl From<LegacyU32<'_>> for FieldMarker {
    fn from(_: LegacyU32<'_>) -> Self {
        LegacyU32::get_marker()
    }
}
impl<'a> Decodable<'a> for LegacyU32<'a> {
    fn get_structure(data: &[u8]) -> Result<Vec<FieldMarker>, Error> {
        u32::get_structure(data)
    }
    fn from_decoded_fields(data: Vec<DecodableField<'a>>) -> Result<Self, Error> {
        u32::from_decoded_fields(data).map(Into::into)
    }
    fn from_bytes(data: &'a mut [u8]) -> Result<Self, Error> {
        Self::try_from(data.get_mut(..4).ok_or(Error::OutOfBound)?)
    }
}
impl<'a> Sv2DataType<'a> for LegacyU32<'a> {
    fn from_bytes_(data: &'a mut [u8]) -> Result<Self, Error> {
        Self::from_bytes(data)
    }
    fn to_slice(&'a self, dst: &mut [u8]) -> Result<usize, Error> {
        self.as_u32().to_bytes(dst)
    }
}
impl<'a> From<LegacyU32<'_>> for EncodableField<'a> {
    fn from(value: LegacyU32<'_>) -> Self {
        value.as_u32().into()
    }
}

/// Converts a legacy value into one independent of the receive buffer.
pub trait IntoStatic {
    type Static: 'static;
    fn into_static(self) -> Self::Static;
}
macro_rules! static_primitives {
    ($($ty:ty),*) => { $(
        impl IntoStatic for $ty {
            type Static = Self;
            fn into_static(self) -> Self { self }
        }
    )* };
}
static_primitives!(bool, u8, u16, u32, u64, f32, U24);

macro_rules! sequence_wrapper {
    ($name:ident, $alias:ident, $owned:ident, $header:expr, $max:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct $name<'a, T>(pub Vec<T>, PhantomData<&'a T>);
        pub type $alias<'a, T> = $name<'a, T>;

        impl<'a, T> $name<'a, T> {
            pub fn new(inner: Vec<T>) -> Result<Self, Error> {
                ::binary_sv2::$owned::new(inner).map(|value| Self(value.into_inner(), PhantomData))
            }
            pub fn into_inner(self) -> Vec<T> {
                self.0
            }
            pub fn into_static(self) -> $name<'static, T::Static>
            where
                T: IntoStatic,
            {
                $name(
                    self.0.into_iter().map(IntoStatic::into_static).collect(),
                    PhantomData,
                )
            }
            pub fn as_static(&self) -> $name<'static, T::Static>
            where
                T: Clone + IntoStatic,
            {
                self.clone().into_static()
            }
            pub fn len(&self) -> usize {
                self.0.len()
            }
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }
        impl<T: LegacyBytes> $name<'_, T> {
            pub fn to_vec(&self) -> Vec<Vec<u8>> {
                self.0
                    .iter()
                    .map(|value| value.legacy_bytes().to_vec())
                    .collect()
            }
            pub fn inner_as_ref(&self) -> Vec<&[u8]> {
                self.0.iter().map(LegacyBytes::legacy_bytes).collect()
            }
        }
        impl<T> TryFrom<$name<'_, T>> for Vec<T> {
            type Error = Error;
            fn try_from(value: $name<'_, T>) -> Result<Self, Error> {
                Ok(value.0)
            }
        }
        impl<T> From<Vec<T>> for $name<'_, T> {
            fn from(value: Vec<T>) -> Self {
                Self(value, PhantomData)
            }
        }
        impl<T, U: From<T>> From<$name<'_, T>> for ::binary_sv2::$owned<U> {
            fn from(value: $name<'_, T>) -> Self {
                Self::new(value.0.into_iter().map(Into::into).collect())
                    .expect("legacy sequence must fit its length prefix")
            }
        }
        impl<T, U: From<T>> From<::binary_sv2::$owned<T>> for $name<'_, U> {
            fn from(value: ::binary_sv2::$owned<T>) -> Self {
                Self(
                    value.into_inner().into_iter().map(Into::into).collect(),
                    PhantomData,
                )
            }
        }
        impl<T, U: From<T>> From<::binary_sv2::$alias<'_, T>> for $name<'_, U> {
            fn from(value: ::binary_sv2::$alias<'_, T>) -> Self {
                Self(
                    value.into_inner().into_iter().map(Into::into).collect(),
                    PhantomData,
                )
            }
        }
        impl<T: GetSize> GetSize for $name<'_, T> {
            fn get_size(&self) -> usize {
                $header + self.0.iter().map(GetSize::get_size).sum::<usize>()
            }
        }
        impl<'a, T> Decodable<'a> for $name<'_, T>
        where
            T: 'a + Sv2DataType<'a> + GetMarker + GetSize + Decodable<'a>,
        {
            fn get_structure(data: &[u8]) -> Result<Vec<FieldMarker>, Error> {
                ::binary_sv2::$owned::<T>::get_structure(data)
            }
            fn from_decoded_fields(data: Vec<DecodableField<'a>>) -> Result<Self, Error> {
                ::binary_sv2::$owned::<T>::from_decoded_fields(data)
                    .map(|value| Self(value.into_inner(), PhantomData))
            }
        }
        impl<'a, T: Into<EncodableField<'a>>> From<$name<'_, T>> for EncodableField<'a> {
            fn from(value: $name<'_, T>) -> Self {
                assert!(
                    value.0.len() <= $max,
                    "legacy sequence exceeds its length prefix"
                );
                let mut fields = Vec::with_capacity(value.0.len() + 1);
                if $header == 1 {
                    fields.push((value.0.len() as u8).into());
                } else {
                    fields.push((value.0.len() as u16).into());
                }
                fields.extend(value.0.into_iter().map(Into::into));
                EncodableField::Struct(fields)
            }
        }
    };
}

sequence_wrapper!(LegacySeq0255, Seq0255, Seq0255Owned, 1, 255);
sequence_wrapper!(LegacySeq064K, Seq064K, Seq064KOwned, 2, 65535);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LegacySv2Option<'a, T>(pub Vec<T>, PhantomData<&'a T>);
pub type Sv2Option<'a, T> = LegacySv2Option<'a, T>;

impl<T> LegacySv2Option<'_, T> {
    pub fn new(inner: Option<T>) -> Self {
        Self(inner.into_iter().collect(), PhantomData)
    }
    pub fn into_inner(mut self) -> Option<T> {
        assert!(
            self.0.len() <= 1,
            "Sv2Option can contain at most one element"
        );
        self.0.pop()
    }
    pub fn into_static(self) -> LegacySv2Option<'static, T::Static>
    where
        T: IntoStatic,
    {
        LegacySv2Option::new(self.into_inner().map(IntoStatic::into_static))
    }
    pub fn as_static(&self) -> LegacySv2Option<'static, T::Static>
    where
        T: Clone + IntoStatic,
    {
        self.clone().into_static()
    }
}
impl<T: LegacyBytes> LegacySv2Option<'_, T> {
    pub fn to_option(&self) -> Option<Vec<u8>> {
        self.0.first().map(|value| value.legacy_bytes().to_vec())
    }
    pub fn inner_as_ref(&self) -> Option<&[u8]> {
        self.0.first().map(LegacyBytes::legacy_bytes)
    }
}
impl<T, U: From<T>> From<LegacySv2Option<'_, T>> for ::binary_sv2::Sv2OptionOwned<U> {
    fn from(value: LegacySv2Option<'_, T>) -> Self {
        Self::new(value.into_inner().map(Into::into))
    }
}
impl<T, U: From<T>> From<::binary_sv2::Sv2OptionOwned<T>> for LegacySv2Option<'_, U> {
    fn from(value: ::binary_sv2::Sv2OptionOwned<T>) -> Self {
        Self::new(value.into_inner().map(Into::into))
    }
}
impl<T, U: From<T>> From<::binary_sv2::Sv2Option<'_, T>> for LegacySv2Option<'_, U> {
    fn from(value: ::binary_sv2::Sv2Option<'_, T>) -> Self {
        Self::new(value.into_inner().map(Into::into))
    }
}
impl<T: GetSize> GetSize for LegacySv2Option<'_, T> {
    fn get_size(&self) -> usize {
        1 + self.0.iter().map(GetSize::get_size).sum::<usize>()
    }
}
impl<'a, T> Decodable<'a> for LegacySv2Option<'_, T>
where
    T: 'a + Sv2DataType<'a> + GetMarker + GetSize + Decodable<'a>,
{
    fn get_structure(data: &[u8]) -> Result<Vec<FieldMarker>, Error> {
        ::binary_sv2::Sv2OptionOwned::<T>::get_structure(data)
    }
    fn from_decoded_fields(data: Vec<DecodableField<'a>>) -> Result<Self, Error> {
        ::binary_sv2::Sv2OptionOwned::<T>::from_decoded_fields(data)
            .map(|value| Self::new(value.into_inner()))
    }
}
impl<'a, T: Into<EncodableField<'a>>> From<LegacySv2Option<'_, T>> for EncodableField<'a> {
    fn from(value: LegacySv2Option<'_, T>) -> Self {
        let mut fields = Vec::with_capacity(2);
        let inner = value.into_inner();
        fields.push(u8::from(inner.is_some()).into());
        fields.extend(inner.into_iter().map(Into::into));
        EncodableField::Struct(fields)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_byte_sequences_match_current_wire_format() {
        let legacy = Seq0255::new(vec![U256::from([3; 32]), U256::from([9; 32])]).unwrap();
        let current: ::binary_sv2::Seq0255Owned<::binary_sv2::U256Owned> = legacy.clone().into();
        let expected = to_bytes(current).unwrap();
        assert_eq!(to_bytes(legacy.clone()).unwrap(), expected);

        let mut wire = expected;
        let decoded: Seq0255<'_, U256<'_>> = from_bytes(&mut wire).unwrap();
        assert_eq!(decoded.into_static(), legacy.into_static());
    }

    #[test]
    fn nested_static_conversion_outlives_the_buffer() {
        let mut wire = vec![1, 1, 0, 0, 0];
        let decoded: Seq0255<'_, U32AsRef<'_>> = from_bytes(&mut wire).unwrap();
        let owned: Seq0255<'static, U32AsRef<'static>> = decoded.into_static();
        drop(wire);
        assert_eq!(owned.into_inner()[0].as_u32(), 1);
    }

    #[test]
    fn request_id_mutation_updates_the_receive_buffer() {
        let mut wire = 7_u32.to_le_bytes();
        {
            let mut request_id: U32AsRef<'_> = from_bytes(&mut wire).unwrap();
            assert_eq!(request_id, U32AsRef::from(7));
            request_id
                .inner_as_mut()
                .copy_from_slice(&29_u32.to_le_bytes());
        }
        assert_eq!(u32::from_le_bytes(wire), 29);
    }

    #[test]
    fn bounded_values_reject_invalid_wire_lengths() {
        assert!(B032::try_from(vec![0; 33]).is_err());
        assert!(Seq0255::new(vec![0_u8; 256]).is_err());
        assert!(from_bytes::<Sv2Option<'_, u32>>(&mut [2, 0, 0, 0, 0, 0, 0, 0, 0]).is_err());
        assert!(from_bytes::<Seq064K<'_, U256<'_>>>(&mut [1, 0, 7]).is_err());
    }
}
