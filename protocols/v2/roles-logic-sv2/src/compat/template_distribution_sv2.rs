//! Compatibility messages preserving the roles-logic-sv2 public API.
#![allow(clippy::useless_conversion)]
use super::common_messages_sv2::message_codec;
use crate::compat::binary_sv2::*;
use alloc::vec::Vec;

pub use crate::compat::ffi::{
    CNewTemplate, CRequestTransactionDataError, CRequestTransactionDataSuccess, CSetNewPrevHash,
    CSubmitSolution, _c_export_coinbase_out, _c_export_req_tx_data,
};

/// Current wire message for callers that can provide an exact additional sigop budget.
pub use ::template_distribution_sv2::CoinbaseOutputConstraints;

/// Uses CoinbaseOutputConstraints on the wire. The legacy API cannot specify sigops; reserve 20 per additional byte, capped at the block limit of 20,000 legacy sigops. This conservatively covers arbitrary legacy output scripts.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(C)]
pub struct CoinbaseOutputDataSize {
    pub coinbase_output_max_additional_size: u32,
}

impl From<CoinbaseOutputDataSize> for ::template_distribution_sv2::CoinbaseOutputConstraintsOwned {
    fn from(value: CoinbaseOutputDataSize) -> Self {
        Self {
            coinbase_output_max_additional_size: value.coinbase_output_max_additional_size.into(),
            coinbase_output_max_additional_sigops: value
                .coinbase_output_max_additional_size
                .saturating_mul(20)
                .min(20_000) as u16,
        }
    }
}

impl From<::template_distribution_sv2::CoinbaseOutputConstraintsOwned> for CoinbaseOutputDataSize {
    fn from(value: ::template_distribution_sv2::CoinbaseOutputConstraintsOwned) -> Self {
        Self {
            coinbase_output_max_additional_size: value.coinbase_output_max_additional_size.into(),
        }
    }
}

message_codec!(
    CoinbaseOutputDataSize,
    ::template_distribution_sv2::CoinbaseOutputConstraintsOwned,
    ::template_distribution_sv2::CoinbaseOutputConstraints
);

impl CoinbaseOutputDataSize {
    pub fn into_static(self) -> CoinbaseOutputDataSize {
        CoinbaseOutputDataSize {
            coinbase_output_max_additional_size: self.coinbase_output_max_additional_size,
        }
    }
    pub fn as_static(&self) -> CoinbaseOutputDataSize {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewTemplate<'decoder> {
    pub template_id: u64,
    pub future_template: bool,
    pub version: u32,
    pub coinbase_tx_version: u32,
    pub coinbase_prefix: B0255<'decoder>,
    pub coinbase_tx_input_sequence: u32,
    pub coinbase_tx_value_remaining: u64,
    pub coinbase_tx_outputs_count: u32,
    pub coinbase_tx_outputs: B064K<'decoder>,
    pub coinbase_tx_locktime: u32,
    pub merkle_path: Seq0255<'decoder, U256<'decoder>>,
}

impl<'decoder> From<NewTemplate<'decoder>> for ::template_distribution_sv2::NewTemplateOwned {
    fn from(value: NewTemplate<'decoder>) -> Self {
        Self {
            template_id: value.template_id.into(),
            future_template: value.future_template.into(),
            version: value.version.into(),
            coinbase_tx_version: value.coinbase_tx_version.into(),
            coinbase_prefix: value.coinbase_prefix.into(),
            coinbase_tx_input_sequence: value.coinbase_tx_input_sequence.into(),
            coinbase_tx_value_remaining: value.coinbase_tx_value_remaining.into(),
            coinbase_tx_outputs_count: value.coinbase_tx_outputs_count.into(),
            coinbase_tx_outputs: value.coinbase_tx_outputs.into(),
            coinbase_tx_locktime: value.coinbase_tx_locktime.into(),
            merkle_path: value.merkle_path.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::NewTemplateOwned> for NewTemplate<'decoder> {
    fn from(value: ::template_distribution_sv2::NewTemplateOwned) -> Self {
        Self {
            template_id: value.template_id.into(),
            future_template: value.future_template.into(),
            version: value.version.into(),
            coinbase_tx_version: value.coinbase_tx_version.into(),
            coinbase_prefix: value.coinbase_prefix.into(),
            coinbase_tx_input_sequence: value.coinbase_tx_input_sequence.into(),
            coinbase_tx_value_remaining: value.coinbase_tx_value_remaining.into(),
            coinbase_tx_outputs_count: value.coinbase_tx_outputs_count.into(),
            coinbase_tx_outputs: value.coinbase_tx_outputs.into(),
            coinbase_tx_locktime: value.coinbase_tx_locktime.into(),
            merkle_path: value.merkle_path.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::NewTemplate<'decoder>> for NewTemplate<'decoder> {
    fn from(value: ::template_distribution_sv2::NewTemplate<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    NewTemplate<'decoder>,
    ::template_distribution_sv2::NewTemplateOwned,
    ::template_distribution_sv2::NewTemplate<'decoder>
);

impl<'decoder> NewTemplate<'decoder> {
    pub fn into_static(self) -> NewTemplate<'static> {
        NewTemplate {
            template_id: self.template_id,
            future_template: self.future_template,
            version: self.version,
            coinbase_tx_version: self.coinbase_tx_version,
            coinbase_prefix: self.coinbase_prefix.into_static(),
            coinbase_tx_input_sequence: self.coinbase_tx_input_sequence,
            coinbase_tx_value_remaining: self.coinbase_tx_value_remaining,
            coinbase_tx_outputs_count: self.coinbase_tx_outputs_count,
            coinbase_tx_outputs: self.coinbase_tx_outputs.into_static(),
            coinbase_tx_locktime: self.coinbase_tx_locktime,
            merkle_path: Seq0255::new(
                self.merkle_path
                    .into_inner()
                    .into_iter()
                    .map(|item| item.into_static())
                    .collect(),
            )
            .expect("unchanged sequence length"),
        }
    }
    pub fn as_static(&self) -> NewTemplate<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(C)]
pub struct RequestTransactionData {
    pub template_id: u64,
}

impl From<RequestTransactionData> for ::template_distribution_sv2::RequestTransactionDataOwned {
    fn from(value: RequestTransactionData) -> Self {
        Self {
            template_id: value.template_id.into(),
        }
    }
}

impl From<::template_distribution_sv2::RequestTransactionDataOwned> for RequestTransactionData {
    fn from(value: ::template_distribution_sv2::RequestTransactionDataOwned) -> Self {
        Self {
            template_id: value.template_id.into(),
        }
    }
}

message_codec!(
    RequestTransactionData,
    ::template_distribution_sv2::RequestTransactionDataOwned,
    ::template_distribution_sv2::RequestTransactionData
);

impl RequestTransactionData {
    pub fn into_static(self) -> RequestTransactionData {
        RequestTransactionData {
            template_id: self.template_id,
        }
    }
    pub fn as_static(&self) -> RequestTransactionData {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestTransactionDataSuccess<'decoder> {
    pub template_id: u64,
    pub excess_data: B064K<'decoder>,
    pub transaction_list: Seq064K<'decoder, B016M<'decoder>>,
}

impl<'decoder> From<RequestTransactionDataSuccess<'decoder>>
    for ::template_distribution_sv2::RequestTransactionDataSuccessOwned
{
    fn from(value: RequestTransactionDataSuccess<'decoder>) -> Self {
        Self {
            template_id: value.template_id.into(),
            excess_data: value.excess_data.into(),
            transaction_list: value.transaction_list.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::RequestTransactionDataSuccessOwned>
    for RequestTransactionDataSuccess<'decoder>
{
    fn from(value: ::template_distribution_sv2::RequestTransactionDataSuccessOwned) -> Self {
        Self {
            template_id: value.template_id.into(),
            excess_data: value.excess_data.into(),
            transaction_list: value.transaction_list.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::RequestTransactionDataSuccess<'decoder>>
    for RequestTransactionDataSuccess<'decoder>
{
    fn from(value: ::template_distribution_sv2::RequestTransactionDataSuccess<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    RequestTransactionDataSuccess<'decoder>,
    ::template_distribution_sv2::RequestTransactionDataSuccessOwned,
    ::template_distribution_sv2::RequestTransactionDataSuccess<'decoder>
);

impl<'decoder> RequestTransactionDataSuccess<'decoder> {
    pub fn into_static(self) -> RequestTransactionDataSuccess<'static> {
        RequestTransactionDataSuccess {
            template_id: self.template_id,
            excess_data: self.excess_data.into_static(),
            transaction_list: Seq064K::new(
                self.transaction_list
                    .into_inner()
                    .into_iter()
                    .map(|item| item.into_static())
                    .collect(),
            )
            .expect("unchanged sequence length"),
        }
    }
    pub fn as_static(&self) -> RequestTransactionDataSuccess<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestTransactionDataError<'decoder> {
    pub template_id: u64,
    pub error_code: Str0255<'decoder>,
}

impl<'decoder> From<RequestTransactionDataError<'decoder>>
    for ::template_distribution_sv2::RequestTransactionDataErrorOwned
{
    fn from(value: RequestTransactionDataError<'decoder>) -> Self {
        Self {
            template_id: value.template_id.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::RequestTransactionDataErrorOwned>
    for RequestTransactionDataError<'decoder>
{
    fn from(value: ::template_distribution_sv2::RequestTransactionDataErrorOwned) -> Self {
        Self {
            template_id: value.template_id.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::RequestTransactionDataError<'decoder>>
    for RequestTransactionDataError<'decoder>
{
    fn from(value: ::template_distribution_sv2::RequestTransactionDataError<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    RequestTransactionDataError<'decoder>,
    ::template_distribution_sv2::RequestTransactionDataErrorOwned,
    ::template_distribution_sv2::RequestTransactionDataError<'decoder>
);

impl<'decoder> RequestTransactionDataError<'decoder> {
    pub fn into_static(self) -> RequestTransactionDataError<'static> {
        RequestTransactionDataError {
            template_id: self.template_id,
            error_code: self.error_code.into_static(),
        }
    }
    pub fn as_static(&self) -> RequestTransactionDataError<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetNewPrevHash<'decoder> {
    pub template_id: u64,
    pub prev_hash: U256<'decoder>,
    pub header_timestamp: u32,
    pub n_bits: u32,
    pub target: U256<'decoder>,
}

impl<'decoder> From<SetNewPrevHash<'decoder>> for ::template_distribution_sv2::SetNewPrevHashOwned {
    fn from(value: SetNewPrevHash<'decoder>) -> Self {
        Self {
            template_id: value.template_id.into(),
            prev_hash: value.prev_hash.into(),
            header_timestamp: value.header_timestamp.into(),
            n_bits: value.n_bits.into(),
            target: value.target.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::SetNewPrevHashOwned> for SetNewPrevHash<'decoder> {
    fn from(value: ::template_distribution_sv2::SetNewPrevHashOwned) -> Self {
        Self {
            template_id: value.template_id.into(),
            prev_hash: value.prev_hash.into(),
            header_timestamp: value.header_timestamp.into(),
            n_bits: value.n_bits.into(),
            target: value.target.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::SetNewPrevHash<'decoder>>
    for SetNewPrevHash<'decoder>
{
    fn from(value: ::template_distribution_sv2::SetNewPrevHash<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetNewPrevHash<'decoder>,
    ::template_distribution_sv2::SetNewPrevHashOwned,
    ::template_distribution_sv2::SetNewPrevHash<'decoder>
);

impl<'decoder> SetNewPrevHash<'decoder> {
    pub fn into_static(self) -> SetNewPrevHash<'static> {
        SetNewPrevHash {
            template_id: self.template_id,
            prev_hash: self.prev_hash.into_static(),
            header_timestamp: self.header_timestamp,
            n_bits: self.n_bits,
            target: self.target.into_static(),
        }
    }
    pub fn as_static(&self) -> SetNewPrevHash<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmitSolution<'decoder> {
    pub template_id: u64,
    pub version: u32,
    pub header_timestamp: u32,
    pub header_nonce: u32,
    pub coinbase_tx: B064K<'decoder>,
}

impl<'decoder> From<SubmitSolution<'decoder>> for ::template_distribution_sv2::SubmitSolutionOwned {
    fn from(value: SubmitSolution<'decoder>) -> Self {
        Self {
            template_id: value.template_id.into(),
            version: value.version.into(),
            header_timestamp: value.header_timestamp.into(),
            header_nonce: value.header_nonce.into(),
            coinbase_tx: value.coinbase_tx.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::SubmitSolutionOwned> for SubmitSolution<'decoder> {
    fn from(value: ::template_distribution_sv2::SubmitSolutionOwned) -> Self {
        Self {
            template_id: value.template_id.into(),
            version: value.version.into(),
            header_timestamp: value.header_timestamp.into(),
            header_nonce: value.header_nonce.into(),
            coinbase_tx: value.coinbase_tx.into(),
        }
    }
}

impl<'decoder> From<::template_distribution_sv2::SubmitSolution<'decoder>>
    for SubmitSolution<'decoder>
{
    fn from(value: ::template_distribution_sv2::SubmitSolution<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SubmitSolution<'decoder>,
    ::template_distribution_sv2::SubmitSolutionOwned,
    ::template_distribution_sv2::SubmitSolution<'decoder>
);

impl<'decoder> SubmitSolution<'decoder> {
    pub fn into_static(self) -> SubmitSolution<'static> {
        SubmitSolution {
            template_id: self.template_id,
            version: self.version,
            header_timestamp: self.header_timestamp,
            header_nonce: self.header_nonce,
            coinbase_tx: self.coinbase_tx.into_static(),
        }
    }
    pub fn as_static(&self) -> SubmitSolution<'static> {
        self.clone().into_static()
    }
}

#[cfg(feature = "prop_test")]
mod property_generation {
    use super::*;
    use core::convert::TryInto;
    use quickcheck::{Arbitrary, Gen};

    fn random_hash(g: &mut Gen) -> U256<'static> {
        let mut hash = [0u8; 32];
        for byte in &mut hash {
            *byte = u8::arbitrary(g);
        }
        hash.into()
    }

    fn random_bytes(g: &mut Gen, limit: usize) -> Vec<u8> {
        let mut bytes = Vec::<u8>::arbitrary(g);
        bytes.truncate(limit);
        bytes
    }

    impl Arbitrary for NewTemplate<'static> {
        fn arbitrary(g: &mut Gen) -> Self {
            // A minimally encoded BIP34 height is required by the current job creator.
            // Heights above 16 use a script-number push rather than OP_1 through OP_16.
            let height = u32::arbitrary(g).max(17);
            let coinbase_prefix = bitcoin::script::Builder::new()
                .push_int(i64::from(height))
                .into_script()
                .into_bytes()
                .try_into()
                .unwrap();
            let merkle_path = (0..u8::arbitrary(g))
                .map(|_| random_hash(g))
                .collect::<Vec<_>>()
                .into();
            Self {
                template_id: u64::arbitrary(g),
                future_template: bool::arbitrary(g),
                version: u32::arbitrary(g),
                coinbase_tx_version: 2,
                coinbase_prefix,
                coinbase_tx_input_sequence: u32::arbitrary(g),
                coinbase_tx_value_remaining: u64::arbitrary(g),
                coinbase_tx_outputs_count: 0,
                coinbase_tx_outputs: Vec::new().try_into().unwrap(),
                coinbase_tx_locktime: u32::arbitrary(g),
                merkle_path,
            }
        }
    }

    impl NewTemplate<'static> {
        pub fn from_gen(g: &mut Gen) -> Self {
            Self::arbitrary(g)
        }
    }

    impl CoinbaseOutputDataSize {
        pub fn from_gen(g: &mut Gen) -> Self {
            Self {
                coinbase_output_max_additional_size: u32::arbitrary(g),
            }
        }
    }

    impl RequestTransactionData {
        pub fn from_gen(g: &mut Gen) -> Self {
            Self {
                template_id: u64::arbitrary(g),
            }
        }
    }

    impl RequestTransactionDataError<'static> {
        pub fn from_gen(g: &mut Gen) -> Self {
            Self {
                template_id: u64::arbitrary(g),
                error_code: random_bytes(g, u8::MAX as usize).try_into().unwrap(),
            }
        }
    }

    impl RequestTransactionDataSuccess<'static> {
        pub fn from_gen(g: &mut Gen) -> Self {
            let transaction: B016M<'static> = random_bytes(g, 0x00ff_ffff).try_into().unwrap();
            Self {
                template_id: u64::arbitrary(g),
                excess_data: random_bytes(g, u16::MAX as usize).try_into().unwrap(),
                transaction_list: vec![transaction].into(),
            }
        }
    }

    impl SetNewPrevHash<'static> {
        pub fn from_gen(g: &mut Gen) -> Self {
            Self {
                template_id: u64::arbitrary(g),
                prev_hash: random_hash(g),
                header_timestamp: u32::arbitrary(g),
                n_bits: u32::arbitrary(g),
                target: random_hash(g),
            }
        }
    }

    impl SubmitSolution<'static> {
        pub fn from_gen(g: &mut Gen) -> Self {
            Self {
                template_id: u64::arbitrary(g),
                version: u32::arbitrary(g),
                header_timestamp: u32::arbitrary(g),
                header_nonce: u32::arbitrary(g),
                coinbase_tx: random_bytes(g, u16::MAX as usize).try_into().unwrap(),
            }
        }
    }
}
