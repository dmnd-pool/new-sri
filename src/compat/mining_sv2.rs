//! Compatibility messages preserving the roles-logic-sv2 public API.
#![allow(clippy::useless_conversion)]
use super::common_messages_sv2::message_codec;
use crate::compat::binary_sv2::*;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct CloseChannel<'decoder> {
    pub channel_id: u32,
    pub reason_code: Str0255<'decoder>,
}

impl<'decoder> From<CloseChannel<'decoder>> for ::mining_sv2::CloseChannelOwned {
    fn from(value: CloseChannel<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            reason_code: value.reason_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::CloseChannelOwned> for CloseChannel<'decoder> {
    fn from(value: ::mining_sv2::CloseChannelOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            reason_code: value.reason_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::CloseChannel<'decoder>> for CloseChannel<'decoder> {
    fn from(value: ::mining_sv2::CloseChannel<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    CloseChannel<'decoder>,
    ::mining_sv2::CloseChannelOwned,
    ::mining_sv2::CloseChannel<'decoder>
);

impl<'decoder> CloseChannel<'decoder> {
    pub fn into_static(self) -> CloseChannel<'static> {
        CloseChannel {
            channel_id: self.channel_id,
            reason_code: self.reason_code.into_static(),
        }
    }
    pub fn as_static(&self) -> CloseChannel<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewMiningJob<'decoder> {
    pub channel_id: u32,
    pub job_id: u32,
    pub min_ntime: Sv2Option<'decoder, u32>,
    pub version: u32,
    pub merkle_root: U256<'decoder>,
}

impl<'decoder> From<NewMiningJob<'decoder>> for ::mining_sv2::NewMiningJobOwned {
    fn from(value: NewMiningJob<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            job_id: value.job_id.into(),
            min_ntime: value.min_ntime.into(),
            version: value.version.into(),
            merkle_root: value.merkle_root.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::NewMiningJobOwned> for NewMiningJob<'decoder> {
    fn from(value: ::mining_sv2::NewMiningJobOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            job_id: value.job_id.into(),
            min_ntime: value.min_ntime.into(),
            version: value.version.into(),
            merkle_root: value.merkle_root.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::NewMiningJob<'decoder>> for NewMiningJob<'decoder> {
    fn from(value: ::mining_sv2::NewMiningJob<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    NewMiningJob<'decoder>,
    ::mining_sv2::NewMiningJobOwned,
    ::mining_sv2::NewMiningJob<'decoder>
);

impl<'decoder> NewMiningJob<'decoder> {
    pub fn into_static(self) -> NewMiningJob<'static> {
        NewMiningJob {
            channel_id: self.channel_id,
            job_id: self.job_id,
            min_ntime: self.min_ntime.into_static(),
            version: self.version,
            merkle_root: self.merkle_root.into_static(),
        }
    }
    pub fn as_static(&self) -> NewMiningJob<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewExtendedMiningJob<'decoder> {
    pub channel_id: u32,
    pub job_id: u32,
    pub min_ntime: Sv2Option<'decoder, u32>,
    pub version: u32,
    pub version_rolling_allowed: bool,
    pub merkle_path: Seq0255<'decoder, U256<'decoder>>,
    pub coinbase_tx_prefix: B064K<'decoder>,
    pub coinbase_tx_suffix: B064K<'decoder>,
}

impl<'decoder> From<NewExtendedMiningJob<'decoder>> for ::mining_sv2::NewExtendedMiningJobOwned {
    fn from(value: NewExtendedMiningJob<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            job_id: value.job_id.into(),
            min_ntime: value.min_ntime.into(),
            version: value.version.into(),
            version_rolling_allowed: value.version_rolling_allowed.into(),
            merkle_path: value.merkle_path.into(),
            coinbase_tx_prefix: value.coinbase_tx_prefix.into(),
            coinbase_tx_suffix: value.coinbase_tx_suffix.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::NewExtendedMiningJobOwned> for NewExtendedMiningJob<'decoder> {
    fn from(value: ::mining_sv2::NewExtendedMiningJobOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            job_id: value.job_id.into(),
            min_ntime: value.min_ntime.into(),
            version: value.version.into(),
            version_rolling_allowed: value.version_rolling_allowed.into(),
            merkle_path: value.merkle_path.into(),
            coinbase_tx_prefix: value.coinbase_tx_prefix.into(),
            coinbase_tx_suffix: value.coinbase_tx_suffix.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::NewExtendedMiningJob<'decoder>>
    for NewExtendedMiningJob<'decoder>
{
    fn from(value: ::mining_sv2::NewExtendedMiningJob<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    NewExtendedMiningJob<'decoder>,
    ::mining_sv2::NewExtendedMiningJobOwned,
    ::mining_sv2::NewExtendedMiningJob<'decoder>
);

impl<'decoder> NewExtendedMiningJob<'decoder> {
    pub fn into_static(self) -> NewExtendedMiningJob<'static> {
        NewExtendedMiningJob {
            channel_id: self.channel_id,
            job_id: self.job_id,
            min_ntime: self.min_ntime.into_static(),
            version: self.version,
            version_rolling_allowed: self.version_rolling_allowed,
            merkle_path: Seq0255::new(
                self.merkle_path
                    .into_inner()
                    .into_iter()
                    .map(|item| item.into_static())
                    .collect(),
            )
            .expect("unchanged sequence length"),
            coinbase_tx_prefix: self.coinbase_tx_prefix.into_static(),
            coinbase_tx_suffix: self.coinbase_tx_suffix.into_static(),
        }
    }
    pub fn as_static(&self) -> NewExtendedMiningJob<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenStandardMiningChannel<'decoder> {
    pub request_id: U32AsRef<'decoder>,
    pub user_identity: Str0255<'decoder>,
    pub nominal_hash_rate: f32,
    pub max_target: U256<'decoder>,
}

impl<'decoder> From<OpenStandardMiningChannel<'decoder>>
    for ::mining_sv2::OpenStandardMiningChannelOwned
{
    fn from(value: OpenStandardMiningChannel<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            user_identity: value.user_identity.into(),
            nominal_hash_rate: value.nominal_hash_rate.into(),
            max_target: value.max_target.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenStandardMiningChannelOwned>
    for OpenStandardMiningChannel<'decoder>
{
    fn from(value: ::mining_sv2::OpenStandardMiningChannelOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            user_identity: value.user_identity.into(),
            nominal_hash_rate: value.nominal_hash_rate.into(),
            max_target: value.max_target.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenStandardMiningChannel<'decoder>>
    for OpenStandardMiningChannel<'decoder>
{
    fn from(value: ::mining_sv2::OpenStandardMiningChannel<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    OpenStandardMiningChannel<'decoder>,
    ::mining_sv2::OpenStandardMiningChannelOwned,
    ::mining_sv2::OpenStandardMiningChannel<'decoder>,
    fn from_bytes(data: &'decoder mut [u8]) -> Result<Self, ::binary_sv2::Error> {
        let message =
            <::mining_sv2::OpenStandardMiningChannel<'_> as ::binary_sv2::Decodable>::from_bytes(
                data,
            )?
            .into_owned();
        let mut message = Self::from(message);
        // Routing changes this identifier before forwarding the original frame.
        message.request_id = (&mut data[..4]).try_into()?;
        Ok(message)
    }
);

impl<'decoder> OpenStandardMiningChannel<'decoder> {
    pub fn into_static(self) -> OpenStandardMiningChannel<'static> {
        OpenStandardMiningChannel {
            request_id: self.request_id.into_static(),
            user_identity: self.user_identity.into_static(),
            nominal_hash_rate: self.nominal_hash_rate,
            max_target: self.max_target.into_static(),
        }
    }
    pub fn as_static(&self) -> OpenStandardMiningChannel<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenStandardMiningChannelSuccess<'decoder> {
    pub request_id: U32AsRef<'decoder>,
    pub channel_id: u32,
    pub target: U256<'decoder>,
    pub extranonce_prefix: B032<'decoder>,
    pub group_channel_id: u32,
}

impl<'decoder> From<OpenStandardMiningChannelSuccess<'decoder>>
    for ::mining_sv2::OpenStandardMiningChannelSuccessOwned
{
    fn from(value: OpenStandardMiningChannelSuccess<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            channel_id: value.channel_id.into(),
            target: value.target.into(),
            extranonce_prefix: value.extranonce_prefix.into(),
            group_channel_id: value.group_channel_id.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenStandardMiningChannelSuccessOwned>
    for OpenStandardMiningChannelSuccess<'decoder>
{
    fn from(value: ::mining_sv2::OpenStandardMiningChannelSuccessOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            channel_id: value.channel_id.into(),
            target: value.target.into(),
            extranonce_prefix: value.extranonce_prefix.into(),
            group_channel_id: value.group_channel_id.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenStandardMiningChannelSuccess<'decoder>>
    for OpenStandardMiningChannelSuccess<'decoder>
{
    fn from(value: ::mining_sv2::OpenStandardMiningChannelSuccess<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    OpenStandardMiningChannelSuccess<'decoder>,
    ::mining_sv2::OpenStandardMiningChannelSuccessOwned,
    ::mining_sv2::OpenStandardMiningChannelSuccess<'decoder>,
    fn from_bytes(data: &'decoder mut [u8]) -> Result<Self, ::binary_sv2::Error> {
        let message = <::mining_sv2::OpenStandardMiningChannelSuccess<'_> as ::binary_sv2::Decodable>::from_bytes(data)?.into_owned();
        let mut message = Self::from(message);
        // Routing changes this identifier before forwarding the original frame.
        message.request_id = (&mut data[..4]).try_into()?;
        Ok(message)
    }
);

impl<'decoder> OpenStandardMiningChannelSuccess<'decoder> {
    pub fn into_static(self) -> OpenStandardMiningChannelSuccess<'static> {
        OpenStandardMiningChannelSuccess {
            request_id: self.request_id.into_static(),
            channel_id: self.channel_id,
            target: self.target.into_static(),
            extranonce_prefix: self.extranonce_prefix.into_static(),
            group_channel_id: self.group_channel_id,
        }
    }
    pub fn as_static(&self) -> OpenStandardMiningChannelSuccess<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenExtendedMiningChannel<'decoder> {
    pub request_id: u32,
    pub user_identity: Str0255<'decoder>,
    pub nominal_hash_rate: f32,
    pub max_target: U256<'decoder>,
    pub min_extranonce_size: u16,
}

impl<'decoder> From<OpenExtendedMiningChannel<'decoder>>
    for ::mining_sv2::OpenExtendedMiningChannelOwned
{
    fn from(value: OpenExtendedMiningChannel<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            user_identity: value.user_identity.into(),
            nominal_hash_rate: value.nominal_hash_rate.into(),
            max_target: value.max_target.into(),
            min_extranonce_size: value.min_extranonce_size.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenExtendedMiningChannelOwned>
    for OpenExtendedMiningChannel<'decoder>
{
    fn from(value: ::mining_sv2::OpenExtendedMiningChannelOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            user_identity: value.user_identity.into(),
            nominal_hash_rate: value.nominal_hash_rate.into(),
            max_target: value.max_target.into(),
            min_extranonce_size: value.min_extranonce_size.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenExtendedMiningChannel<'decoder>>
    for OpenExtendedMiningChannel<'decoder>
{
    fn from(value: ::mining_sv2::OpenExtendedMiningChannel<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    OpenExtendedMiningChannel<'decoder>,
    ::mining_sv2::OpenExtendedMiningChannelOwned,
    ::mining_sv2::OpenExtendedMiningChannel<'decoder>
);

impl<'decoder> OpenExtendedMiningChannel<'decoder> {
    pub fn into_static(self) -> OpenExtendedMiningChannel<'static> {
        OpenExtendedMiningChannel {
            request_id: self.request_id,
            user_identity: self.user_identity.into_static(),
            nominal_hash_rate: self.nominal_hash_rate,
            max_target: self.max_target.into_static(),
            min_extranonce_size: self.min_extranonce_size,
        }
    }
    pub fn as_static(&self) -> OpenExtendedMiningChannel<'static> {
        self.clone().into_static()
    }
}

/// The new group channel identifier has no legacy field. Outgoing messages use group zero and incoming identifiers are discarded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenExtendedMiningChannelSuccess<'decoder> {
    pub request_id: u32,
    pub channel_id: u32,
    pub target: U256<'decoder>,
    pub extranonce_size: u16,
    pub extranonce_prefix: B032<'decoder>,
}

impl<'decoder> From<OpenExtendedMiningChannelSuccess<'decoder>>
    for ::mining_sv2::OpenExtendedMiningChannelSuccessOwned
{
    fn from(value: OpenExtendedMiningChannelSuccess<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            channel_id: value.channel_id.into(),
            target: value.target.into(),
            extranonce_size: value.extranonce_size.into(),
            extranonce_prefix: value.extranonce_prefix.into(),
            group_channel_id: 0,
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenExtendedMiningChannelSuccessOwned>
    for OpenExtendedMiningChannelSuccess<'decoder>
{
    fn from(value: ::mining_sv2::OpenExtendedMiningChannelSuccessOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            channel_id: value.channel_id.into(),
            target: value.target.into(),
            extranonce_size: value.extranonce_size.into(),
            extranonce_prefix: value.extranonce_prefix.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenExtendedMiningChannelSuccess<'decoder>>
    for OpenExtendedMiningChannelSuccess<'decoder>
{
    fn from(value: ::mining_sv2::OpenExtendedMiningChannelSuccess<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    OpenExtendedMiningChannelSuccess<'decoder>,
    ::mining_sv2::OpenExtendedMiningChannelSuccessOwned,
    ::mining_sv2::OpenExtendedMiningChannelSuccess<'decoder>
);

impl<'decoder> OpenExtendedMiningChannelSuccess<'decoder> {
    pub fn into_static(self) -> OpenExtendedMiningChannelSuccess<'static> {
        OpenExtendedMiningChannelSuccess {
            request_id: self.request_id,
            channel_id: self.channel_id,
            target: self.target.into_static(),
            extranonce_size: self.extranonce_size,
            extranonce_prefix: self.extranonce_prefix.into_static(),
        }
    }
    pub fn as_static(&self) -> OpenExtendedMiningChannelSuccess<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenMiningChannelError<'decoder> {
    pub request_id: u32,
    pub error_code: Str0255<'decoder>,
}

impl<'decoder> From<OpenMiningChannelError<'decoder>>
    for ::mining_sv2::OpenMiningChannelErrorOwned
{
    fn from(value: OpenMiningChannelError<'decoder>) -> Self {
        Self {
            request_id: value.request_id.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenMiningChannelErrorOwned>
    for OpenMiningChannelError<'decoder>
{
    fn from(value: ::mining_sv2::OpenMiningChannelErrorOwned) -> Self {
        Self {
            request_id: value.request_id.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::OpenMiningChannelError<'decoder>>
    for OpenMiningChannelError<'decoder>
{
    fn from(value: ::mining_sv2::OpenMiningChannelError<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    OpenMiningChannelError<'decoder>,
    ::mining_sv2::OpenMiningChannelErrorOwned,
    ::mining_sv2::OpenMiningChannelError<'decoder>
);

impl<'decoder> OpenMiningChannelError<'decoder> {
    pub fn into_static(self) -> OpenMiningChannelError<'static> {
        OpenMiningChannelError {
            request_id: self.request_id,
            error_code: self.error_code.into_static(),
        }
    }
    pub fn as_static(&self) -> OpenMiningChannelError<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Reconnect<'decoder> {
    pub new_host: Str0255<'decoder>,
    pub new_port: u16,
}

impl<'decoder> From<Reconnect<'decoder>> for ::common_messages_sv2::ReconnectOwned {
    fn from(value: Reconnect<'decoder>) -> Self {
        Self {
            new_host: value.new_host.into(),
            new_port: value.new_port.into(),
        }
    }
}

impl<'decoder> From<::common_messages_sv2::ReconnectOwned> for Reconnect<'decoder> {
    fn from(value: ::common_messages_sv2::ReconnectOwned) -> Self {
        Self {
            new_host: value.new_host.into(),
            new_port: value.new_port.into(),
        }
    }
}

impl<'decoder> From<::common_messages_sv2::Reconnect<'decoder>> for Reconnect<'decoder> {
    fn from(value: ::common_messages_sv2::Reconnect<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    Reconnect<'decoder>,
    ::common_messages_sv2::ReconnectOwned,
    ::common_messages_sv2::Reconnect<'decoder>
);

impl<'decoder> Reconnect<'decoder> {
    pub fn into_static(self) -> Reconnect<'static> {
        Reconnect {
            new_host: self.new_host.into_static(),
            new_port: self.new_port,
        }
    }
    pub fn as_static(&self) -> Reconnect<'static> {
        self.clone().into_static()
    }
}

/// The legacy value remaining and extranonce size fields are no longer sent on the wire; incoming messages initialize them to zero.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetCustomMiningJob<'decoder> {
    pub channel_id: u32,
    pub request_id: u32,
    pub token: B0255<'decoder>,
    pub version: u32,
    pub prev_hash: U256<'decoder>,
    pub min_ntime: u32,
    pub nbits: u32,
    pub coinbase_tx_version: u32,
    pub coinbase_prefix: B0255<'decoder>,
    pub coinbase_tx_input_n_sequence: u32,
    /// Compatibility field; absent from the released wire message.
    pub coinbase_tx_value_remaining: u64,
    pub coinbase_tx_outputs: B064K<'decoder>,
    pub coinbase_tx_locktime: u32,
    pub merkle_path: Seq0255<'decoder, U256<'decoder>>,
    /// Compatibility field; absent from the released wire message.
    pub extranonce_size: u16,
}

impl<'decoder> From<SetCustomMiningJob<'decoder>> for ::mining_sv2::SetCustomMiningJobOwned {
    fn from(value: SetCustomMiningJob<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            request_id: value.request_id.into(),
            token: value.token.into(),
            version: value.version.into(),
            prev_hash: value.prev_hash.into(),
            min_ntime: value.min_ntime.into(),
            nbits: value.nbits.into(),
            coinbase_tx_version: value.coinbase_tx_version.into(),
            coinbase_prefix: value.coinbase_prefix.into(),
            coinbase_tx_input_n_sequence: value.coinbase_tx_input_n_sequence.into(),
            coinbase_tx_outputs: value.coinbase_tx_outputs.into(),
            coinbase_tx_locktime: value.coinbase_tx_locktime.into(),
            merkle_path: value.merkle_path.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetCustomMiningJobOwned> for SetCustomMiningJob<'decoder> {
    fn from(value: ::mining_sv2::SetCustomMiningJobOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            request_id: value.request_id.into(),
            token: value.token.into(),
            version: value.version.into(),
            prev_hash: value.prev_hash.into(),
            min_ntime: value.min_ntime.into(),
            nbits: value.nbits.into(),
            coinbase_tx_version: value.coinbase_tx_version.into(),
            coinbase_prefix: value.coinbase_prefix.into(),
            coinbase_tx_input_n_sequence: value.coinbase_tx_input_n_sequence.into(),
            coinbase_tx_value_remaining: 0,
            coinbase_tx_outputs: value.coinbase_tx_outputs.into(),
            coinbase_tx_locktime: value.coinbase_tx_locktime.into(),
            merkle_path: value.merkle_path.into(),
            extranonce_size: 0,
        }
    }
}

impl<'decoder> From<::mining_sv2::SetCustomMiningJob<'decoder>> for SetCustomMiningJob<'decoder> {
    fn from(value: ::mining_sv2::SetCustomMiningJob<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetCustomMiningJob<'decoder>,
    ::mining_sv2::SetCustomMiningJobOwned,
    ::mining_sv2::SetCustomMiningJob<'decoder>
);

impl<'decoder> SetCustomMiningJob<'decoder> {
    pub fn into_static(self) -> SetCustomMiningJob<'static> {
        SetCustomMiningJob {
            channel_id: self.channel_id,
            request_id: self.request_id,
            token: self.token.into_static(),
            version: self.version,
            prev_hash: self.prev_hash.into_static(),
            min_ntime: self.min_ntime,
            nbits: self.nbits,
            coinbase_tx_version: self.coinbase_tx_version,
            coinbase_prefix: self.coinbase_prefix.into_static(),
            coinbase_tx_input_n_sequence: self.coinbase_tx_input_n_sequence,
            coinbase_tx_value_remaining: self.coinbase_tx_value_remaining,
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
            extranonce_size: self.extranonce_size,
        }
    }
    pub fn as_static(&self) -> SetCustomMiningJob<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCustomMiningJobSuccess {
    pub channel_id: u32,
    pub request_id: u32,
    pub job_id: u32,
}

impl From<SetCustomMiningJobSuccess> for ::mining_sv2::SetCustomMiningJobSuccessOwned {
    fn from(value: SetCustomMiningJobSuccess) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            request_id: value.request_id.into(),
            job_id: value.job_id.into(),
        }
    }
}

impl From<::mining_sv2::SetCustomMiningJobSuccessOwned> for SetCustomMiningJobSuccess {
    fn from(value: ::mining_sv2::SetCustomMiningJobSuccessOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            request_id: value.request_id.into(),
            job_id: value.job_id.into(),
        }
    }
}

message_codec!(
    SetCustomMiningJobSuccess,
    ::mining_sv2::SetCustomMiningJobSuccessOwned,
    ::mining_sv2::SetCustomMiningJobSuccess
);

impl SetCustomMiningJobSuccess {
    pub fn into_static(self) -> SetCustomMiningJobSuccess {
        SetCustomMiningJobSuccess {
            channel_id: self.channel_id,
            request_id: self.request_id,
            job_id: self.job_id,
        }
    }
    pub fn as_static(&self) -> SetCustomMiningJobSuccess {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCustomMiningJobError<'decoder> {
    pub channel_id: u32,
    pub request_id: u32,
    pub error_code: Str0255<'decoder>,
}

impl<'decoder> From<SetCustomMiningJobError<'decoder>>
    for ::mining_sv2::SetCustomMiningJobErrorOwned
{
    fn from(value: SetCustomMiningJobError<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            request_id: value.request_id.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetCustomMiningJobErrorOwned>
    for SetCustomMiningJobError<'decoder>
{
    fn from(value: ::mining_sv2::SetCustomMiningJobErrorOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            request_id: value.request_id.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetCustomMiningJobError<'decoder>>
    for SetCustomMiningJobError<'decoder>
{
    fn from(value: ::mining_sv2::SetCustomMiningJobError<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetCustomMiningJobError<'decoder>,
    ::mining_sv2::SetCustomMiningJobErrorOwned,
    ::mining_sv2::SetCustomMiningJobError<'decoder>
);

impl<'decoder> SetCustomMiningJobError<'decoder> {
    pub fn into_static(self) -> SetCustomMiningJobError<'static> {
        SetCustomMiningJobError {
            channel_id: self.channel_id,
            request_id: self.request_id,
            error_code: self.error_code.into_static(),
        }
    }
    pub fn as_static(&self) -> SetCustomMiningJobError<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetExtranoncePrefix<'decoder> {
    pub channel_id: u32,
    pub extranonce_prefix: B032<'decoder>,
}

impl<'decoder> From<SetExtranoncePrefix<'decoder>> for ::mining_sv2::SetExtranoncePrefixOwned {
    fn from(value: SetExtranoncePrefix<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            extranonce_prefix: value.extranonce_prefix.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetExtranoncePrefixOwned> for SetExtranoncePrefix<'decoder> {
    fn from(value: ::mining_sv2::SetExtranoncePrefixOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            extranonce_prefix: value.extranonce_prefix.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetExtranoncePrefix<'decoder>> for SetExtranoncePrefix<'decoder> {
    fn from(value: ::mining_sv2::SetExtranoncePrefix<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetExtranoncePrefix<'decoder>,
    ::mining_sv2::SetExtranoncePrefixOwned,
    ::mining_sv2::SetExtranoncePrefix<'decoder>
);

impl<'decoder> SetExtranoncePrefix<'decoder> {
    pub fn into_static(self) -> SetExtranoncePrefix<'static> {
        SetExtranoncePrefix {
            channel_id: self.channel_id,
            extranonce_prefix: self.extranonce_prefix.into_static(),
        }
    }
    pub fn as_static(&self) -> SetExtranoncePrefix<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetGroupChannel<'decoder> {
    pub group_channel_id: u32,
    pub channel_ids: Seq064K<'decoder, u32>,
}

impl<'decoder> From<SetGroupChannel<'decoder>> for ::mining_sv2::SetGroupChannelOwned {
    fn from(value: SetGroupChannel<'decoder>) -> Self {
        Self {
            group_channel_id: value.group_channel_id.into(),
            channel_ids: value.channel_ids.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetGroupChannelOwned> for SetGroupChannel<'decoder> {
    fn from(value: ::mining_sv2::SetGroupChannelOwned) -> Self {
        Self {
            group_channel_id: value.group_channel_id.into(),
            channel_ids: value.channel_ids.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetGroupChannel<'decoder>> for SetGroupChannel<'decoder> {
    fn from(value: ::mining_sv2::SetGroupChannel<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetGroupChannel<'decoder>,
    ::mining_sv2::SetGroupChannelOwned,
    ::mining_sv2::SetGroupChannel<'decoder>
);

impl<'decoder> SetGroupChannel<'decoder> {
    pub fn into_static(self) -> SetGroupChannel<'static> {
        SetGroupChannel {
            group_channel_id: self.group_channel_id,
            channel_ids: self.channel_ids.into_static(),
        }
    }
    pub fn as_static(&self) -> SetGroupChannel<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetNewPrevHash<'decoder> {
    pub channel_id: u32,
    pub job_id: u32,
    pub prev_hash: U256<'decoder>,
    pub min_ntime: u32,
    pub nbits: u32,
}

impl<'decoder> From<SetNewPrevHash<'decoder>> for ::mining_sv2::SetNewPrevHashOwned {
    fn from(value: SetNewPrevHash<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            job_id: value.job_id.into(),
            prev_hash: value.prev_hash.into(),
            min_ntime: value.min_ntime.into(),
            nbits: value.nbits.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetNewPrevHashOwned> for SetNewPrevHash<'decoder> {
    fn from(value: ::mining_sv2::SetNewPrevHashOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            job_id: value.job_id.into(),
            prev_hash: value.prev_hash.into(),
            min_ntime: value.min_ntime.into(),
            nbits: value.nbits.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetNewPrevHash<'decoder>> for SetNewPrevHash<'decoder> {
    fn from(value: ::mining_sv2::SetNewPrevHash<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetNewPrevHash<'decoder>,
    ::mining_sv2::SetNewPrevHashOwned,
    ::mining_sv2::SetNewPrevHash<'decoder>
);

impl<'decoder> SetNewPrevHash<'decoder> {
    pub fn into_static(self) -> SetNewPrevHash<'static> {
        SetNewPrevHash {
            channel_id: self.channel_id,
            job_id: self.job_id,
            prev_hash: self.prev_hash.into_static(),
            min_ntime: self.min_ntime,
            nbits: self.nbits,
        }
    }
    pub fn as_static(&self) -> SetNewPrevHash<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetTarget<'decoder> {
    pub channel_id: u32,
    pub maximum_target: U256<'decoder>,
}

impl<'decoder> From<SetTarget<'decoder>> for ::mining_sv2::SetTargetOwned {
    fn from(value: SetTarget<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            maximum_target: value.maximum_target.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetTargetOwned> for SetTarget<'decoder> {
    fn from(value: ::mining_sv2::SetTargetOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            maximum_target: value.maximum_target.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SetTarget<'decoder>> for SetTarget<'decoder> {
    fn from(value: ::mining_sv2::SetTarget<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetTarget<'decoder>,
    ::mining_sv2::SetTargetOwned,
    ::mining_sv2::SetTarget<'decoder>
);

impl<'decoder> SetTarget<'decoder> {
    pub fn into_static(self) -> SetTarget<'static> {
        SetTarget {
            channel_id: self.channel_id,
            maximum_target: self.maximum_target.into_static(),
        }
    }
    pub fn as_static(&self) -> SetTarget<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubmitSharesStandard {
    pub channel_id: u32,
    pub sequence_number: u32,
    pub job_id: u32,
    pub nonce: u32,
    pub ntime: u32,
    pub version: u32,
}

impl From<SubmitSharesStandard> for ::mining_sv2::SubmitSharesStandardOwned {
    fn from(value: SubmitSharesStandard) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            sequence_number: value.sequence_number.into(),
            job_id: value.job_id.into(),
            nonce: value.nonce.into(),
            ntime: value.ntime.into(),
            version: value.version.into(),
        }
    }
}

impl From<::mining_sv2::SubmitSharesStandardOwned> for SubmitSharesStandard {
    fn from(value: ::mining_sv2::SubmitSharesStandardOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            sequence_number: value.sequence_number.into(),
            job_id: value.job_id.into(),
            nonce: value.nonce.into(),
            ntime: value.ntime.into(),
            version: value.version.into(),
        }
    }
}

message_codec!(
    SubmitSharesStandard,
    ::mining_sv2::SubmitSharesStandardOwned,
    ::mining_sv2::SubmitSharesStandard
);

impl SubmitSharesStandard {
    pub fn into_static(self) -> SubmitSharesStandard {
        SubmitSharesStandard {
            channel_id: self.channel_id,
            sequence_number: self.sequence_number,
            job_id: self.job_id,
            nonce: self.nonce,
            ntime: self.ntime,
            version: self.version,
        }
    }
    pub fn as_static(&self) -> SubmitSharesStandard {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubmitSharesExtended<'decoder> {
    pub channel_id: u32,
    pub sequence_number: u32,
    pub job_id: u32,
    pub nonce: u32,
    pub ntime: u32,
    pub version: u32,
    pub extranonce: B032<'decoder>,
}

impl<'decoder> From<SubmitSharesExtended<'decoder>> for ::mining_sv2::SubmitSharesExtendedOwned {
    fn from(value: SubmitSharesExtended<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            sequence_number: value.sequence_number.into(),
            job_id: value.job_id.into(),
            nonce: value.nonce.into(),
            ntime: value.ntime.into(),
            version: value.version.into(),
            extranonce: value.extranonce.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SubmitSharesExtendedOwned> for SubmitSharesExtended<'decoder> {
    fn from(value: ::mining_sv2::SubmitSharesExtendedOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            sequence_number: value.sequence_number.into(),
            job_id: value.job_id.into(),
            nonce: value.nonce.into(),
            ntime: value.ntime.into(),
            version: value.version.into(),
            extranonce: value.extranonce.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SubmitSharesExtended<'decoder>>
    for SubmitSharesExtended<'decoder>
{
    fn from(value: ::mining_sv2::SubmitSharesExtended<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SubmitSharesExtended<'decoder>,
    ::mining_sv2::SubmitSharesExtendedOwned,
    ::mining_sv2::SubmitSharesExtended<'decoder>
);

impl<'decoder> SubmitSharesExtended<'decoder> {
    pub fn into_static(self) -> SubmitSharesExtended<'static> {
        SubmitSharesExtended {
            channel_id: self.channel_id,
            sequence_number: self.sequence_number,
            job_id: self.job_id,
            nonce: self.nonce,
            ntime: self.ntime,
            version: self.version,
            extranonce: self.extranonce.into_static(),
        }
    }
    pub fn as_static(&self) -> SubmitSharesExtended<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubmitSharesSuccess {
    pub channel_id: u32,
    pub last_sequence_number: u32,
    pub new_submits_accepted_count: u32,
    pub new_shares_sum: u64,
}

impl From<SubmitSharesSuccess> for ::mining_sv2::SubmitSharesSuccessOwned {
    fn from(value: SubmitSharesSuccess) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            last_sequence_number: value.last_sequence_number.into(),
            new_submits_accepted_count: value.new_submits_accepted_count.into(),
            new_shares_sum: value.new_shares_sum.into(),
        }
    }
}

impl From<::mining_sv2::SubmitSharesSuccessOwned> for SubmitSharesSuccess {
    fn from(value: ::mining_sv2::SubmitSharesSuccessOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            last_sequence_number: value.last_sequence_number.into(),
            new_submits_accepted_count: value.new_submits_accepted_count.into(),
            new_shares_sum: value.new_shares_sum.into(),
        }
    }
}

message_codec!(
    SubmitSharesSuccess,
    ::mining_sv2::SubmitSharesSuccessOwned,
    ::mining_sv2::SubmitSharesSuccess
);

impl SubmitSharesSuccess {
    pub fn into_static(self) -> SubmitSharesSuccess {
        SubmitSharesSuccess {
            channel_id: self.channel_id,
            last_sequence_number: self.last_sequence_number,
            new_submits_accepted_count: self.new_submits_accepted_count,
            new_shares_sum: self.new_shares_sum,
        }
    }
    pub fn as_static(&self) -> SubmitSharesSuccess {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubmitSharesError<'decoder> {
    pub channel_id: u32,
    pub sequence_number: u32,
    pub error_code: Str0255<'decoder>,
}

impl<'decoder> From<SubmitSharesError<'decoder>> for ::mining_sv2::SubmitSharesErrorOwned {
    fn from(value: SubmitSharesError<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            sequence_number: value.sequence_number.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SubmitSharesErrorOwned> for SubmitSharesError<'decoder> {
    fn from(value: ::mining_sv2::SubmitSharesErrorOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            sequence_number: value.sequence_number.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::SubmitSharesError<'decoder>> for SubmitSharesError<'decoder> {
    fn from(value: ::mining_sv2::SubmitSharesError<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SubmitSharesError<'decoder>,
    ::mining_sv2::SubmitSharesErrorOwned,
    ::mining_sv2::SubmitSharesError<'decoder>
);

impl<'decoder> SubmitSharesError<'decoder> {
    pub fn into_static(self) -> SubmitSharesError<'static> {
        SubmitSharesError {
            channel_id: self.channel_id,
            sequence_number: self.sequence_number,
            error_code: self.error_code.into_static(),
        }
    }
    pub fn as_static(&self) -> SubmitSharesError<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateChannel<'decoder> {
    pub channel_id: u32,
    pub nominal_hash_rate: f32,
    pub maximum_target: U256<'decoder>,
}

impl<'decoder> From<UpdateChannel<'decoder>> for ::mining_sv2::UpdateChannelOwned {
    fn from(value: UpdateChannel<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            nominal_hash_rate: value.nominal_hash_rate.into(),
            maximum_target: value.maximum_target.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::UpdateChannelOwned> for UpdateChannel<'decoder> {
    fn from(value: ::mining_sv2::UpdateChannelOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            nominal_hash_rate: value.nominal_hash_rate.into(),
            maximum_target: value.maximum_target.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::UpdateChannel<'decoder>> for UpdateChannel<'decoder> {
    fn from(value: ::mining_sv2::UpdateChannel<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    UpdateChannel<'decoder>,
    ::mining_sv2::UpdateChannelOwned,
    ::mining_sv2::UpdateChannel<'decoder>
);

impl<'decoder> UpdateChannel<'decoder> {
    pub fn into_static(self) -> UpdateChannel<'static> {
        UpdateChannel {
            channel_id: self.channel_id,
            nominal_hash_rate: self.nominal_hash_rate,
            maximum_target: self.maximum_target.into_static(),
        }
    }
    pub fn as_static(&self) -> UpdateChannel<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateChannelError<'decoder> {
    pub channel_id: u32,
    pub error_code: Str0255<'decoder>,
}

impl<'decoder> From<UpdateChannelError<'decoder>> for ::mining_sv2::UpdateChannelErrorOwned {
    fn from(value: UpdateChannelError<'decoder>) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::UpdateChannelErrorOwned> for UpdateChannelError<'decoder> {
    fn from(value: ::mining_sv2::UpdateChannelErrorOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::mining_sv2::UpdateChannelError<'decoder>> for UpdateChannelError<'decoder> {
    fn from(value: ::mining_sv2::UpdateChannelError<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    UpdateChannelError<'decoder>,
    ::mining_sv2::UpdateChannelErrorOwned,
    ::mining_sv2::UpdateChannelError<'decoder>
);

impl<'decoder> UpdateChannelError<'decoder> {
    pub fn into_static(self) -> UpdateChannelError<'static> {
        UpdateChannelError {
            channel_id: self.channel_id,
            error_code: self.error_code.into_static(),
        }
    }
    pub fn as_static(&self) -> UpdateChannelError<'static> {
        self.clone().into_static()
    }
}

impl<'d> NewMiningJob<'d> {
    pub fn is_future(&self) -> bool {
        self.min_ntime.clone().into_inner().is_none()
    }
    pub fn set_future(&mut self) {
        self.min_ntime = Sv2Option::new(None);
    }
    pub fn set_no_future(&mut self, min_ntime: u32) {
        self.min_ntime = Sv2Option::new(Some(min_ntime));
    }
}

impl<'d> NewExtendedMiningJob<'d> {
    pub fn is_future(&self) -> bool {
        self.min_ntime.clone().into_inner().is_none()
    }
    pub fn set_future(&mut self) {
        self.min_ntime = Sv2Option::new(None);
    }
    pub fn set_no_future(&mut self, min_ntime: u32) {
        self.min_ntime = Sv2Option::new(Some(min_ntime));
    }
}

impl<'decoder> OpenStandardMiningChannel<'decoder> {
    pub fn get_request_id_as_u32(&self) -> u32 {
        (&self.request_id).into()
    }

    pub fn update_id(&mut self, new_id: u32) {
        let bytes_new = new_id.to_le_bytes();
        let bytes_old = self.request_id.inner_as_mut();
        bytes_old[0] = bytes_new[0];
        bytes_old[1] = bytes_new[1];
        bytes_old[2] = bytes_new[2];
        bytes_old[3] = bytes_new[3];
    }
}

impl<'decoder> OpenStandardMiningChannelSuccess<'decoder> {
    pub fn get_request_id_as_u32(&self) -> u32 {
        (&self.request_id).into()
    }

    pub fn update_id(&mut self, new_id: u32) {
        let bytes_new = new_id.to_le_bytes();
        let bytes_old = self.request_id.inner_as_mut();
        bytes_old[0] = bytes_new[0];
        bytes_old[1] = bytes_new[1];
        bytes_old[2] = bytes_new[2];
        bytes_old[3] = bytes_new[3];
    }
}

impl<'decoder> OpenExtendedMiningChannel<'decoder> {
    pub fn get_request_id_as_u32(&self) -> u32 {
        self.request_id
    }
}

impl<'a> OpenMiningChannelError<'a> {
    pub fn new_max_target_out_of_range(request_id: u32) -> Self {
        Self {
            request_id,
            error_code: "max-target-out-of-range".to_string().try_into().unwrap(),
        }
    }
    pub fn unsupported_extranonce_size(request_id: u32) -> Self {
        Self {
            request_id,
            error_code: "unsupported-min-extranonce-size"
                .to_string()
                .try_into()
                .unwrap(),
        }
    }
    pub fn new_unknown_user(request_id: u32) -> Self {
        Self {
            request_id,
            error_code: "unknown-user".to_string().try_into().unwrap(),
        }
    }
}

impl<'a> SubmitSharesError<'a> {
    pub fn invalid_channel_error_code() -> &'static str {
        "invalid-channel-id"
    }
    pub fn stale_share_error_code() -> &'static str {
        "stale-share"
    }
    pub fn difficulty_too_low_error_code() -> &'static str {
        "difficulty-too-low"
    }
    pub fn invalid_job_id_error_code() -> &'static str {
        "invalid-job-id"
    }
}

use alloc::string::ToString;
use core::{convert::TryInto, ops::Range};

const MAX_EXTRANONCE_LEN: usize = 32;

/// Target is a 256-bit unsigned integer in little-endian
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    head: u128, // least significant bits
    tail: u128, // most significant bits
}

impl Target {
    pub fn new(head: u128, tail: u128) -> Self {
        Self { head, tail }
    }
}

impl From<[u8; 32]> for Target {
    fn from(v: [u8; 32]) -> Self {
        // below unwraps never panics
        let head = u128::from_le_bytes(v[0..16].try_into().unwrap());
        let tail = u128::from_le_bytes(v[16..32].try_into().unwrap());
        Self { head, tail }
    }
}

impl From<Extranonce> for alloc::vec::Vec<u8> {
    fn from(v: Extranonce) -> Self {
        v.extranonce
    }
}

impl<'a> From<U256<'a>> for Target {
    fn from(v: U256<'a>) -> Self {
        let inner = v.inner_as_ref();
        // below unwraps never panics
        let head = u128::from_le_bytes(inner[0..16].try_into().unwrap());
        let tail = u128::from_le_bytes(inner[16..32].try_into().unwrap());
        Self { head, tail }
    }
}

impl From<Target> for U256<'static> {
    fn from(v: Target) -> Self {
        let mut inner = v.head.to_le_bytes().to_vec();
        inner.extend_from_slice(&v.tail.to_le_bytes());
        // below unwraps never panics
        inner.try_into().unwrap()
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Target {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        if self.tail == other.tail && self.head == other.head {
            core::cmp::Ordering::Equal
        } else if self.tail != other.tail {
            self.tail.cmp(&other.tail)
        } else {
            self.head.cmp(&other.head)
        }
    }
}

// WARNING: do not derive Copy on this type. Some operations performed to a copy of an extranonce
// do not affect the original, and this may lead to different extranonce inconsistency
/// Extranonce bytes which need to be added to the coinbase to form a fully valid submission.
///
/// Representation is in big endian, so tail is for the digits relative to smaller powers
///
/// `full coinbase = coinbase_tx_prefix + extranonce + coinbase_tx_suffix`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extranonce {
    extranonce: alloc::vec::Vec<u8>,
}

// this function converts a U256 type in little endian to Extranonce type
impl<'a> From<U256<'a>> for Extranonce {
    fn from(v: U256<'a>) -> Self {
        let extranonce: alloc::vec::Vec<u8> = v.inner_as_ref().into();
        Self { extranonce }
    }
}

// This function converts an Extranonce type to U256n little endian
impl<'a> From<Extranonce> for U256<'a> {
    fn from(v: Extranonce) -> Self {
        let inner = v.extranonce;
        debug_assert!(inner.len() <= 32);
        // below unwraps never panics
        inner.try_into().unwrap()
    }
}

// this function converts an extranonce to the type B032
impl<'a> From<B032<'a>> for Extranonce {
    fn from(v: B032<'a>) -> Self {
        let extranonce: alloc::vec::Vec<u8> = v.inner_as_ref().into();
        Self { extranonce }
    }
}

// this function converts an Extranonce type in B032 in little endian
impl<'a> From<Extranonce> for B032<'a> {
    fn from(v: Extranonce) -> Self {
        let inner = v.extranonce.to_vec();
        // below unwraps never panics
        inner.try_into().unwrap()
    }
}

impl Default for Extranonce {
    fn default() -> Self {
        Self {
            extranonce: vec![0; 32],
        }
    }
}

impl core::convert::TryFrom<alloc::vec::Vec<u8>> for Extranonce {
    type Error = ();

    fn try_from(v: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        if v.len() > MAX_EXTRANONCE_LEN {
            Err(())
        } else {
            Ok(Extranonce { extranonce: v })
        }
    }
}

impl AsRef<[u8]> for Extranonce {
    fn as_ref(&self) -> &[u8] {
        self.extranonce.as_ref()
    }
}

impl Extranonce {
    pub fn new(len: usize) -> Option<Self> {
        if len > MAX_EXTRANONCE_LEN {
            None
        } else {
            let extranonce = vec![0; len];
            Some(Self { extranonce })
        }
    }

    /// this function converts a Extranonce type to b032 type
    pub fn from_vec_with_len(mut extranonce: alloc::vec::Vec<u8>, len: usize) -> Self {
        extranonce.resize(len, 0);
        Self { extranonce }
    }

    pub fn into_b032(self) -> B032<'static> {
        self.into()
    }
    // B032 type is more used, this is why the output signature is not ExtendedExtranoncee the B032
    // type is more used, this is why the output signature is not ExtendedExtranoncee
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<B032<'_>> {
        increment_bytes_be(&mut self.extranonce).ok()?;
        // below unwraps never panics
        Some(self.extranonce.clone().try_into().unwrap())
    }

    pub fn to_vec(self) -> alloc::vec::Vec<u8> {
        self.extranonce
    }

    /// Return only the prefix part of the extranonce
    /// If the required size is greater than the extranonce len it return None
    pub fn into_prefix(
        &self,
        prefix_len: usize,
        additional_coinbase_script_data: &[u8],
    ) -> Option<B032<'static>> {
        if prefix_len > self.extranonce.len() {
            None
        } else {
            let mut prefix = Vec::with_capacity(prefix_len + additional_coinbase_script_data.len());
            for b in additional_coinbase_script_data {
                prefix.push(*b);
            }
            for i in 0..prefix_len {
                prefix.push(self.extranonce[i]);
            }
            // unwrap is sage as prefix_len can not be greater than 32 cause is not possible to
            // contruct Extranonce with the inner vecto greater than 32.
            Some(prefix.try_into().unwrap())
        }
    }
}

impl From<&mut ExtendedExtranonce> for Extranonce {
    fn from(v: &mut ExtendedExtranonce) -> Self {
        let mut extranonce = v.inner.to_vec();
        extranonce.truncate(v.range_2.end);
        Self { extranonce }
    }
}

#[derive(Debug, Clone)]
/// Downstram and upstream are relative to an actor of the protocol P. In simple terms, upstream is
/// the part of the protocol that a user P sees when he looks above and downstream when he looks
/// beneath.
///
/// An ExtendedExtranonce is defined by 3 ranges:
///
///  - range_0: is the range that represents the extended extranonce part that reserved by upstream
/// relative to P (for most upstreams nodes, e.g. a pool, this is [0..0]) and it is fixed for P.
///  - range_1: is the range that represents the extended extranonce part reserved to P. P assigns
/// to every relative downstream an extranonce with different value in the range 1 in the
/// following way: if D_i is the (i+1)-th downstream that connected to P, then D_i gets from P and
/// extranonce with range_1=i (note that the concatenation of range_1 and range_1 is the range_0
/// relative to D_i and range_2 of P is the range_1 of D_i).
///  - range_2: is the range that P reserve for the downstreams.
///
///
/// In the following examples, we examine the extended extranonce in some cases.
///
/// The user P is the pool.
///  - range_0 -> 0..0, there is no upstream relative to the pool P, so no space reserved by the
/// upstream
///  - range_1 -> 0..16 the pool P increments the first 16 bytes to be sure the each pool's
/// downstream get a different extranonce or a different extended extranoce search space (more
/// on that below*)
///  - range_2 -> 16..32 this bytes are not changed by the pool but are changed by the pool's
/// downstream
///
/// The user P is the translator.
///  - range_0 -> 0..16 these bytes are set by the pool and P shouldn't change them
///  - range_1 -> 16..24 these bytes are modified by P each time that a sv1 mining device connect,
///  so we can be sure that each connected sv1 mining device get a different extended extranonce
///  search space
///  - range_2 -> 24..32 these bytes are left free for the sv1 miniing device
///
/// The user P is a sv1 mining device.
///  - range_0 -> 0..24 these byteshadd set by the device's upstreams
/// range_1 -> 24..32 these bytes are changed by P (if capable) in order to increment the
/// search space
/// range_2 -> 32..32 no more downstream
///
///
/// About how the pool work having both extended and standard downstreams:
///
/// the pool reserve the first 16 bytes for herself and let downstreams change the lase 16, so
/// each one of the possible 2^16 number represent an extended channel
/// pool space                               | downstream space
/// 0000 0000 0000 0000      0000 0000 0000 0000    extended channel number 0
/// 0000 0000 0000 0001      0000 0000 0000 0000    extended channel number 1
/// 0000 0000 0000 0002      0000 0000 0000 0000    extended channel number 2
/// 0000 0000 0000 0003      0000 0000 0000 0000    extended channel number 3
/// ....
/// In order to assign extranonces to standard channels the pool reserve to herself the first
/// extended channel, so the extended extranonce received by the first open extended channel wont
/// be the channel 0 but the channel 1
/// 0000 0000 0000 0001      0000 0000 0000 0000
/// and all the extranonces for standard channel are generate from the extended channel number 0
/// so the first standard cahnnel will be
/// 0000 0000 0000 0000      0000 0000 0000 0000
/// where the second will be
/// 0000 0000 0000 0000      0000 0000 0000 0001 ecc ecc
///
///
/// ExtendedExtranonce type is meant to be used in cases where the extranonce length is not
/// 32bytes. So, the inner part is an array of 32bytes, but only the firsts corresponding to the
/// range_2.end are used by the pool
///
/// # Examples
///
/// ```
/// use roles_logic_sv2::mining_sv2::*;
/// use core::convert::TryInto;
/// // Cretae an extended extranonce of len 32 reserve the first 7 bytes for the pool.
/// let mut pool_extended_extranonce = ExtendedExtranonce::new(0..0, 0..7, 7..32);
///
/// // On open extended channel the pool do
/// let new_extended_channel_extranonce = pool_extended_extranonce.next_extended(3).unwrap();
/// let new_extended_channel_extranonce_inner = vec![0,0,0,0,0,0,1];
/// assert_eq!(new_extended_channel_extranonce.clone().to_vec(), new_extended_channel_extranonce_inner);
///
/// // Then the pool receive a request to open a standard channel
/// let new_standard_channel_extranonce = pool_extended_extranonce.next_standard().unwrap();
/// let new_standard_channel_extranonce_inner = vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1];
/// assert_eq!(new_standard_channel_extranonce.to_vec(), new_standard_channel_extranonce_inner);
///
/// // Now the proxy receive the ExtdendedExtranonce previously created
/// // The proxy know the extranonce space reserved to the pool is 7 bytes and that the total
/// // extranonce len is 32 bytes and decide to reserve 4 bytes for him and leave the remaining 21 do
/// // futher downstreams.
/// let range_0 = 0..7;
/// let range_1 = 7..11;
/// let range_2 = 11..32;
/// let mut proxy_extended_extranonce = ExtendedExtranonce::from_upstream_extranonce(new_extended_channel_extranonce,range_0, range_1, range_2).unwrap();
///
/// // The proxy generate an extended extranonce for downstream
/// let new_extended_channel_extranonce = proxy_extended_extranonce.next_extended(3).unwrap();
/// let new_extended_channel_extranonce_inner = vec![0,0,0,0,0,0,1,0,0,0,1];
/// assert_eq!(new_extended_channel_extranonce.clone().to_vec(), new_extended_channel_extranonce_inner);
///
/// // When the proxy receive a share from downstream and want to recreate the all extranonce eg
/// // cause it want to check the share's work
/// let received_extranonce: Extranonce = vec![0_u8,0,0,0,0,0,0,8,0,50,0,0,0,0,0,0,0,0,0,0,0].try_into().unwrap();
/// let share_complete_extranonce = proxy_extended_extranonce.extranonce_from_downstream_extranonce(received_extranonce.clone()).unwrap();
/// let share_complete_extranonce_inner = vec![0,0,0,0,0,0,1,0,0,0,1,0,0,0,0,0,0,0,8,0,50,0,0,0,0,0,0,0,0,0,0,0];
/// assert_eq!(share_complete_extranonce.to_vec(), share_complete_extranonce_inner);
///
///
/// // Now the proxy want to send the extranonce received from downstream and the part of extranonce
/// // owned by him to the pool
/// let extranonce_to_send = proxy_extended_extranonce.without_upstream_part(Some(received_extranonce)).unwrap();
/// let extranonce_to_send_inner = vec![0,0,0,1,0,0,0,0,0,0,0,8,0,50,0,0,0,0,0,0,0,0,0,0,0];
/// assert_eq!(extranonce_to_send.to_vec(),extranonce_to_send_inner);
/// ```
pub struct ExtendedExtranonce {
    inner: [u8; MAX_EXTRANONCE_LEN],
    range_0: core::ops::Range<usize>,
    range_1: core::ops::Range<usize>,
    range_2: core::ops::Range<usize>,
}
/// the trait PartialEq is implemented in such a way that only the relevant bytes are compared.
/// If range_2.end is set to 20, then the following ExtendedExtranonces are equal
/// ExtendedExtranonce {
///     inner: [0000 0000 0000 0000      0000 0000 0000 0000],
///     range_0: [0..5],
///     range_1: [5..10],
///     range_2: [10, 20],
/// }
/// ExtendedExtranonce {
///     inner: [0000 0000 0000 0000      0000 1111 1111 1111],
///     range_0: [0..5],
///     range_1: [5..10],
///     range_2: [10, 20],
/// }
impl PartialEq for ExtendedExtranonce {
    fn eq(&self, other: &Self) -> bool {
        let len = self.range_2.end;
        self.inner[0..len] == other.inner[0..len]
            && self.range_0 == other.range_0
            && self.range_1 == other.range_1
            && self.range_2 == other.range_2
    }
}

impl ExtendedExtranonce {
    /// every extranonce start from zero.
    pub fn new(range_0: Range<usize>, range_1: Range<usize>, range_2: Range<usize>) -> Self {
        debug_assert!(range_0.start == 0);
        debug_assert!(range_0.end == range_1.start);
        debug_assert!(range_1.end == range_2.start);
        Self {
            inner: [0; MAX_EXTRANONCE_LEN],
            range_0,
            range_1,
            range_2,
        }
    }

    pub fn new_with_inner_only_test(
        range_0: Range<usize>,
        range_1: Range<usize>,
        range_2: Range<usize>,
        mut inner: alloc::vec::Vec<u8>,
    ) -> Self {
        inner.resize(MAX_EXTRANONCE_LEN, 0);
        let inner = inner.try_into().unwrap();
        Self {
            inner,
            range_0,
            range_1,
            range_2,
        }
    }

    pub fn get_len(&self) -> usize {
        self.range_2.end
    }

    pub fn get_range2_len(&self) -> usize {
        self.range_2.end - self.range_2.start
    }

    pub fn get_range0_len(&self) -> usize {
        self.range_0.end - self.range_0.start
    }

    pub fn get_prefix_len(&self) -> usize {
        self.range_1.end - self.range_0.start
    }

    /// Suppose that P receives from the upstream an extranonce that needs to be converted into any
    /// ExtendedExtranonce, eg when an extended channel is opened. Then range_0 (that should
    /// be provided along the Extranonce) is reserved for the upstream and can't be modiefied by
    /// P. If the bytes recerved to P (range_1 and range_2) are not set to zero, returns None,
    /// otherwise returns Some(ExtendedExtranonce). If the range_2.end field is greater than 32,
    /// returns None.
    pub fn from_upstream_extranonce(
        v: Extranonce,
        range_0: Range<usize>,
        range_1: Range<usize>,
        range_2: Range<usize>,
    ) -> Option<Self> {
        debug_assert!(range_0.start <= range_0.end);
        debug_assert!(range_0.end <= range_1.start);
        debug_assert!(range_1.start <= range_1.end);
        debug_assert!(range_1.end <= range_2.start);
        debug_assert!(range_2.start <= range_2.end);
        if range_2.end > MAX_EXTRANONCE_LEN {
            return None;
        }
        let mut inner = v.extranonce;
        inner.resize(range_2.end, 0);
        let rest = vec![0; MAX_EXTRANONCE_LEN - inner.len()];
        // below unwraps never panics
        let inner: [u8; MAX_EXTRANONCE_LEN] = [inner, rest].concat().try_into().unwrap();
        Some(Self {
            inner,
            range_0,
            range_1,
            range_2,
        })
    }

    /// Specular of [Self::from_upstream_extranonce]
    pub fn extranonce_from_downstream_extranonce(
        &self,
        dowstream_extranonce: Extranonce,
    ) -> Option<Extranonce> {
        if dowstream_extranonce.extranonce.len() != self.range_2.end - self.range_2.start {
            return None;
        }
        let mut res = self.inner[self.range_0.start..self.range_1.end].to_vec();
        for b in dowstream_extranonce.extranonce {
            res.push(b)
        }
        res.try_into().ok()
    }

    /// This function takes in input an ExtendedExtranonce for the extended channel. The number
    /// represented by the bytes in range_2 is incremented by 1 and the ExtendedExtranonce is
    /// converted in an Extranonce. If range_2 is at maximum value, the output is None.
    pub fn next_standard(&mut self) -> Option<Extranonce> {
        let reserved_extranonce_bytes = &mut self.inner[self.range_1.start..self.range_1.end];
        for b in reserved_extranonce_bytes {
            *b = 0
        }
        let non_reserved_extranonces_bytes = &mut self.inner[self.range_2.start..self.range_2.end];
        match increment_bytes_be(non_reserved_extranonces_bytes) {
            Ok(_) => Some(self.into()),
            Err(_) => None,
        }
    }

    /// This function calculates the next extranonce, but the output is ExtendedExtranonce. The
    /// required_len variable represents the range requested by the downstream to use. The part
    /// incremented is range_1, as every downstream must have different jobs.
    pub fn next_extended(&mut self, required_len: usize) -> Option<Extranonce> {
        if required_len > self.range_2.end - self.range_2.start {
            return None;
        };
        let extended_part = &mut self.inner[self.range_1.start..self.range_1.end];
        match increment_bytes_be(extended_part) {
            Ok(_) => {
                let result = self.inner[..self.range_1.end].to_vec();
                // Safe unwrap result will be always less the MAX_EXTRANONCE_LEN
                Some(result.try_into().unwrap())
            }
            Err(_) => None,
        }
    }

    /// Return a vec with the extranonce bytes that belong to self and downstream removing the
    /// ones owned by upstream (using Sv1 terms the extranonce1 is removed)
    /// If dowstream_extranonce is Some(v) it replace the downstream extranonce part with v
    pub fn without_upstream_part(
        &self,
        downstream_extranonce: Option<Extranonce>,
    ) -> Option<Extranonce> {
        match downstream_extranonce {
            Some(downstream_extranonce) => {
                if downstream_extranonce.extranonce.len() != self.range_2.end - self.range_2.start {
                    return None;
                }
                let mut res = self.inner[self.range_1.start..self.range_1.end].to_vec();
                for b in downstream_extranonce.extranonce {
                    res.push(b)
                }
                Some(res.try_into().ok()?)
            }
            None => self.inner[self.range_1.start..self.range_2.end]
                .to_vec()
                .try_into()
                .ok(),
        }
    }

    pub fn upstream_part(&self) -> Extranonce {
        self.inner[self.range_0.start..self.range_1.end]
            .to_vec()
            .try_into()
            .unwrap()
    }
}
/// This function is used to increment extranonces, and it is used in next_standard and in
/// next_extended methods. If the input consists of an array of 255 as u8 (the maximum value) then
/// the input cannot be incremented. In this case, the input is not changed and the function returns
/// Err(()). In every other case, the function increments the input and returns Ok(())
fn increment_bytes_be(bs: &mut [u8]) -> Result<(), ()> {
    for b in bs.iter_mut().rev() {
        if *b != u8::MAX {
            *b += 1;
            return Ok(());
        } else {
            *b = 0;
        }
    }
    for b in bs.iter_mut() {
        *b = u8::MAX
    }
    Err(())
}
