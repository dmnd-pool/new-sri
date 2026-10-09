//! Compatibility messages preserving the roles-logic-sv2 public API.
#![allow(clippy::useless_conversion)]
use super::common_messages_sv2::message_codec;
use crate::compat::binary_sv2::*;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct AllocateMiningJobToken<'decoder> {
    pub user_identifier: Str0255<'decoder>,
    pub request_id: u32,
}

impl<'decoder> From<AllocateMiningJobToken<'decoder>>
    for ::job_declaration_sv2::AllocateMiningJobTokenOwned
{
    fn from(value: AllocateMiningJobToken<'decoder>) -> Self {
        Self {
            user_identifier: value.user_identifier.into(),
            request_id: value.request_id.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::AllocateMiningJobTokenOwned>
    for AllocateMiningJobToken<'decoder>
{
    fn from(value: ::job_declaration_sv2::AllocateMiningJobTokenOwned) -> Self {
        Self {
            user_identifier: value.user_identifier.into(),
            request_id: value.request_id.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::AllocateMiningJobToken<'decoder>>
    for AllocateMiningJobToken<'decoder>
{
    fn from(value: ::job_declaration_sv2::AllocateMiningJobToken<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    AllocateMiningJobToken<'decoder>,
    ::job_declaration_sv2::AllocateMiningJobTokenOwned,
    ::job_declaration_sv2::AllocateMiningJobToken<'decoder>,
    size: [user_identifier, request_id]
);

impl<'decoder> AllocateMiningJobToken<'decoder> {
    pub fn into_static(self) -> AllocateMiningJobToken<'static> {
        AllocateMiningJobToken {
            user_identifier: self.user_identifier.into_static(),
            request_id: self.request_id,
        }
    }
    pub fn as_static(&self) -> AllocateMiningJobToken<'static> {
        self.clone().into_static()
    }
}

/// The released protocol only sends token and outputs. The legacy size is reconstructed from the outputs and async mining is enabled, as allocation no longer negotiates it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct AllocateMiningJobTokenSuccess<'decoder> {
    pub request_id: u32,
    pub mining_job_token: B0255<'decoder>,
    /// Compatibility field; absent from the released wire message.
    pub coinbase_output_max_additional_size: u32,
    pub coinbase_output: B064K<'decoder>,
    /// Compatibility field; absent from the released wire message.
    pub async_mining_allowed: bool,
}

impl<'decoder> From<AllocateMiningJobTokenSuccess<'decoder>>
    for ::job_declaration_sv2::AllocateMiningJobTokenSuccessOwned
{
    fn from(value: AllocateMiningJobTokenSuccess<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            mining_job_token: value.mining_job_token.into(),
            coinbase_outputs: value.coinbase_output.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::AllocateMiningJobTokenSuccessOwned>
    for AllocateMiningJobTokenSuccess<'decoder>
{
    fn from(value: ::job_declaration_sv2::AllocateMiningJobTokenSuccessOwned) -> Self {
        Self {
            coinbase_output_max_additional_size: value.coinbase_outputs.len() as u32,
            request_id: value.request_id.into(),
            mining_job_token: value.mining_job_token.into(),
            coinbase_output: value.coinbase_outputs.into(),
            async_mining_allowed: true,
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::AllocateMiningJobTokenSuccess<'decoder>>
    for AllocateMiningJobTokenSuccess<'decoder>
{
    fn from(value: ::job_declaration_sv2::AllocateMiningJobTokenSuccess<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    AllocateMiningJobTokenSuccess<'decoder>,
    ::job_declaration_sv2::AllocateMiningJobTokenSuccessOwned,
    ::job_declaration_sv2::AllocateMiningJobTokenSuccess<'decoder>,
    size: [request_id, mining_job_token, coinbase_output]
);

impl<'decoder> AllocateMiningJobTokenSuccess<'decoder> {
    pub fn into_static(self) -> AllocateMiningJobTokenSuccess<'static> {
        AllocateMiningJobTokenSuccess {
            request_id: self.request_id,
            mining_job_token: self.mining_job_token.into_static(),
            coinbase_output_max_additional_size: self.coinbase_output_max_additional_size,
            coinbase_output: self.coinbase_output.into_static(),
            async_mining_allowed: self.async_mining_allowed,
        }
    }
    pub fn as_static(&self) -> AllocateMiningJobTokenSuccess<'static> {
        self.clone().into_static()
    }
}

/// The legacy tx_list field carries witness transaction identifiers (wtxids), as required by the released protocol.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct DeclareMiningJob<'decoder> {
    pub request_id: u32,
    pub mining_job_token: B0255<'decoder>,
    pub version: u32,
    pub coinbase_prefix: B064K<'decoder>,
    pub coinbase_suffix: B064K<'decoder>,
    pub tx_list: Seq064K<'decoder, U256<'decoder>>,
    pub excess_data: B064K<'decoder>,
}

impl<'decoder> From<DeclareMiningJob<'decoder>> for ::job_declaration_sv2::DeclareMiningJobOwned {
    fn from(value: DeclareMiningJob<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            mining_job_token: value.mining_job_token.into(),
            version: value.version.into(),
            coinbase_tx_prefix: value.coinbase_prefix.into(),
            coinbase_tx_suffix: value.coinbase_suffix.into(),
            wtxid_list: value.tx_list.into(),
            excess_data: value.excess_data.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::DeclareMiningJobOwned> for DeclareMiningJob<'decoder> {
    fn from(value: ::job_declaration_sv2::DeclareMiningJobOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            mining_job_token: value.mining_job_token.into(),
            version: value.version.into(),
            coinbase_prefix: value.coinbase_tx_prefix.into(),
            coinbase_suffix: value.coinbase_tx_suffix.into(),
            tx_list: value.wtxid_list.into(),
            excess_data: value.excess_data.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::DeclareMiningJob<'decoder>>
    for DeclareMiningJob<'decoder>
{
    fn from(value: ::job_declaration_sv2::DeclareMiningJob<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    DeclareMiningJob<'decoder>,
    ::job_declaration_sv2::DeclareMiningJobOwned,
    ::job_declaration_sv2::DeclareMiningJob<'decoder>,
    size: [request_id, mining_job_token, version, coinbase_prefix, coinbase_suffix, tx_list => sequence, excess_data]
);

impl<'decoder> DeclareMiningJob<'decoder> {
    pub fn into_static(self) -> DeclareMiningJob<'static> {
        DeclareMiningJob {
            request_id: self.request_id,
            mining_job_token: self.mining_job_token.into_static(),
            version: self.version,
            coinbase_prefix: self.coinbase_prefix.into_static(),
            coinbase_suffix: self.coinbase_suffix.into_static(),
            tx_list: Seq064K::new(
                self.tx_list
                    .into_inner()
                    .into_iter()
                    .map(|item| item.into_static())
                    .collect(),
            )
            .expect("unchanged sequence length"),
            excess_data: self.excess_data.into_static(),
        }
    }
    pub fn as_static(&self) -> DeclareMiningJob<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct DeclareMiningJobSuccess<'decoder> {
    pub request_id: u32,
    pub new_mining_job_token: B0255<'decoder>,
}

impl<'decoder> From<DeclareMiningJobSuccess<'decoder>>
    for ::job_declaration_sv2::DeclareMiningJobSuccessOwned
{
    fn from(value: DeclareMiningJobSuccess<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            new_mining_job_token: value.new_mining_job_token.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::DeclareMiningJobSuccessOwned>
    for DeclareMiningJobSuccess<'decoder>
{
    fn from(value: ::job_declaration_sv2::DeclareMiningJobSuccessOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            new_mining_job_token: value.new_mining_job_token.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::DeclareMiningJobSuccess<'decoder>>
    for DeclareMiningJobSuccess<'decoder>
{
    fn from(value: ::job_declaration_sv2::DeclareMiningJobSuccess<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    DeclareMiningJobSuccess<'decoder>,
    ::job_declaration_sv2::DeclareMiningJobSuccessOwned,
    ::job_declaration_sv2::DeclareMiningJobSuccess<'decoder>,
    size: [request_id, new_mining_job_token]
);

impl<'decoder> DeclareMiningJobSuccess<'decoder> {
    pub fn into_static(self) -> DeclareMiningJobSuccess<'static> {
        DeclareMiningJobSuccess {
            request_id: self.request_id,
            new_mining_job_token: self.new_mining_job_token.into_static(),
        }
    }
    pub fn as_static(&self) -> DeclareMiningJobSuccess<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct DeclareMiningJobError<'decoder> {
    pub request_id: u32,
    pub error_code: Str0255<'decoder>,
    pub error_details: B064K<'decoder>,
}

impl<'decoder> From<DeclareMiningJobError<'decoder>>
    for ::job_declaration_sv2::DeclareMiningJobErrorOwned
{
    fn from(value: DeclareMiningJobError<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            error_code: value.error_code.into(),
            error_details: value.error_details.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::DeclareMiningJobErrorOwned>
    for DeclareMiningJobError<'decoder>
{
    fn from(value: ::job_declaration_sv2::DeclareMiningJobErrorOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            error_code: value.error_code.into(),
            error_details: value.error_details.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::DeclareMiningJobError<'decoder>>
    for DeclareMiningJobError<'decoder>
{
    fn from(value: ::job_declaration_sv2::DeclareMiningJobError<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    DeclareMiningJobError<'decoder>,
    ::job_declaration_sv2::DeclareMiningJobErrorOwned,
    ::job_declaration_sv2::DeclareMiningJobError<'decoder>,
    size: [request_id, error_code, error_details]
);

impl<'decoder> DeclareMiningJobError<'decoder> {
    pub fn into_static(self) -> DeclareMiningJobError<'static> {
        DeclareMiningJobError {
            request_id: self.request_id,
            error_code: self.error_code.into_static(),
            error_details: self.error_details.into_static(),
        }
    }
    pub fn as_static(&self) -> DeclareMiningJobError<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ProvideMissingTransactions<'decoder> {
    pub request_id: u32,
    pub unknown_tx_position_list: Seq064K<'decoder, u16>,
}

impl<'decoder> From<ProvideMissingTransactions<'decoder>>
    for ::job_declaration_sv2::ProvideMissingTransactionsOwned
{
    fn from(value: ProvideMissingTransactions<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            unknown_tx_position_list: value.unknown_tx_position_list.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::ProvideMissingTransactionsOwned>
    for ProvideMissingTransactions<'decoder>
{
    fn from(value: ::job_declaration_sv2::ProvideMissingTransactionsOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            unknown_tx_position_list: value.unknown_tx_position_list.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::ProvideMissingTransactions<'decoder>>
    for ProvideMissingTransactions<'decoder>
{
    fn from(value: ::job_declaration_sv2::ProvideMissingTransactions<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    ProvideMissingTransactions<'decoder>,
    ::job_declaration_sv2::ProvideMissingTransactionsOwned,
    ::job_declaration_sv2::ProvideMissingTransactions<'decoder>,
    size: [request_id, unknown_tx_position_list => sequence]
);

impl<'decoder> ProvideMissingTransactions<'decoder> {
    pub fn into_static(self) -> ProvideMissingTransactions<'static> {
        ProvideMissingTransactions {
            request_id: self.request_id,
            unknown_tx_position_list: self.unknown_tx_position_list.into_static(),
        }
    }
    pub fn as_static(&self) -> ProvideMissingTransactions<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ProvideMissingTransactionsSuccess<'decoder> {
    pub request_id: u32,
    pub transaction_list: Seq064K<'decoder, B016M<'decoder>>,
}

impl<'decoder> From<ProvideMissingTransactionsSuccess<'decoder>>
    for ::job_declaration_sv2::ProvideMissingTransactionsSuccessOwned
{
    fn from(value: ProvideMissingTransactionsSuccess<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            transaction_list: value.transaction_list.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::ProvideMissingTransactionsSuccessOwned>
    for ProvideMissingTransactionsSuccess<'decoder>
{
    fn from(value: ::job_declaration_sv2::ProvideMissingTransactionsSuccessOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            transaction_list: value.transaction_list.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::ProvideMissingTransactionsSuccess<'decoder>>
    for ProvideMissingTransactionsSuccess<'decoder>
{
    fn from(value: ::job_declaration_sv2::ProvideMissingTransactionsSuccess<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    ProvideMissingTransactionsSuccess<'decoder>,
    ::job_declaration_sv2::ProvideMissingTransactionsSuccessOwned,
    ::job_declaration_sv2::ProvideMissingTransactionsSuccess<'decoder>,
    size: [request_id, transaction_list => sequence]
);

impl<'decoder> ProvideMissingTransactionsSuccess<'decoder> {
    pub fn into_static(self) -> ProvideMissingTransactionsSuccess<'static> {
        ProvideMissingTransactionsSuccess {
            request_id: self.request_id,
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
    pub fn as_static(&self) -> ProvideMissingTransactionsSuccess<'static> {
        self.clone().into_static()
    }
}

/// Uses PushSolution on the wire; nonce and ntime are mapped by field name to preserve their new wire order.
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct SubmitSolutionJd<'decoder> {
    pub extranonce: B032<'decoder>,
    pub prev_hash: U256<'decoder>,
    pub ntime: u32,
    pub nonce: u32,
    pub nbits: u32,
    pub version: u32,
}

impl<'decoder> From<SubmitSolutionJd<'decoder>> for ::job_declaration_sv2::PushSolutionOwned {
    fn from(value: SubmitSolutionJd<'decoder>) -> Self {
        Self {
            extranonce: value.extranonce.into(),
            prev_hash: value.prev_hash.into(),
            nonce: value.nonce.into(),
            ntime: value.ntime.into(),
            nbits: value.nbits.into(),
            version: value.version.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::PushSolutionOwned> for SubmitSolutionJd<'decoder> {
    fn from(value: ::job_declaration_sv2::PushSolutionOwned) -> Self {
        Self {
            extranonce: value.extranonce.into(),
            prev_hash: value.prev_hash.into(),
            ntime: value.ntime.into(),
            nonce: value.nonce.into(),
            nbits: value.nbits.into(),
            version: value.version.into(),
        }
    }
}

impl<'decoder> From<::job_declaration_sv2::PushSolution<'decoder>> for SubmitSolutionJd<'decoder> {
    fn from(value: ::job_declaration_sv2::PushSolution<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SubmitSolutionJd<'decoder>,
    ::job_declaration_sv2::PushSolutionOwned,
    ::job_declaration_sv2::PushSolution<'decoder>,
    size: [extranonce, prev_hash, nonce, ntime, nbits, version]
);

impl<'decoder> SubmitSolutionJd<'decoder> {
    pub fn into_static(self) -> SubmitSolutionJd<'static> {
        SubmitSolutionJd {
            extranonce: self.extranonce.into_static(),
            prev_hash: self.prev_hash.into_static(),
            ntime: self.ntime,
            nonce: self.nonce,
            nbits: self.nbits,
            version: self.version,
        }
    }
    pub fn as_static(&self) -> SubmitSolutionJd<'static> {
        self.clone().into_static()
    }
}
