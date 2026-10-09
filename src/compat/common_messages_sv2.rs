//! Compatibility messages preserving the roles-logic-sv2 public API.
#![allow(clippy::useless_conversion)]
use crate::compat::binary_sv2::*;
pub use ::common_messages_sv2::{
    has_requires_std_job, has_version_rolling, has_work_selection, Protocol,
};
use alloc::vec::Vec;

pub use crate::compat::ffi::{
    CSetupConnection, CSetupConnectionError, _c_export_channel_endpoint_changed,
    _c_export_setup_conn_succ,
};

// Decode with the released SRI message types. Encode the listed wire fields
// with the SRI binary codec, without an intermediate owned message.
macro_rules! message_codec {
    ($legacy:ident $(<$life:lifetime>)?, $modern:ty, $wire:ty, size: [$($field:ident $(=> $sequence:ident)?),* $(,)?] $(; $extra:expr)? $(, $decode:item)?) => {
        impl $(<$life>)? ::binary_sv2::GetSize for $legacy $(<$life>)? {
            fn get_size(&self) -> usize {
                0 $(+ ::binary_sv2::GetSize::get_size(&self.$field))* $(+ $extra)?
            }
        }
        impl<'decoder> ::binary_sv2::Decodable<'decoder> for $legacy $(<$life>)? {
            $($decode)?
            fn get_structure(data: &[u8]) -> Result<Vec<::binary_sv2::decodable::FieldMarker>, ::binary_sv2::Error> {
                <$wire as ::binary_sv2::Decodable<'decoder>>::get_structure(data)
            }
            fn from_decoded_fields(fields: Vec<::binary_sv2::decodable::DecodableField<'decoder>>) -> Result<Self, ::binary_sv2::Error> {
                <$wire as ::binary_sv2::Decodable<'decoder>>::from_decoded_fields(fields).map(Into::into)
            }
        }
        impl<'decoder> From<$legacy $(<$life>)?> for ::binary_sv2::encodable::EncodableField<'decoder> {
            fn from(value: $legacy $(<$life>)?) -> Self {
                message_codec!(@encode value, $modern, [$($field $(=> $sequence)?),*] $(; $extra)?)
            }
        }
    };
    (@encode $value:ident, $modern:ty, [$($field:ident $(=> $sequence:ident)?),*]) => {{
        let count = 0 $(+ message_codec!(@count $value.$field $(, $sequence)?))*;
        let mut fields = Vec::with_capacity(count);
        $(message_codec!(@append fields, $value.$field $(, $sequence)?);)*
        ::binary_sv2::encodable::EncodableField::Struct(fields)
    }};
    (@count $field:expr) => { 1 };
    (@count $field:expr, sequence) => { 1 + $field.len() };
    (@append $fields:ident, $field:expr) => { $fields.push($field.into()) };
    (@append $fields:ident, $field:expr, sequence) => { $field.append_fields(&mut $fields) };
    (@encode $value:ident, $modern:ty, [$($field:ident),*]; $extra:expr) => {{
        let message: $modern = $value.into();
        message.into()
    }};
}
pub(super) use message_codec;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(C)]
pub struct ChannelEndpointChanged {
    pub channel_id: u32,
}

impl From<ChannelEndpointChanged> for ::common_messages_sv2::ChannelEndpointChangedOwned {
    fn from(value: ChannelEndpointChanged) -> Self {
        Self {
            channel_id: value.channel_id.into(),
        }
    }
}

impl From<::common_messages_sv2::ChannelEndpointChangedOwned> for ChannelEndpointChanged {
    fn from(value: ::common_messages_sv2::ChannelEndpointChangedOwned) -> Self {
        Self {
            channel_id: value.channel_id.into(),
        }
    }
}

message_codec!(
    ChannelEndpointChanged,
    ::common_messages_sv2::ChannelEndpointChangedOwned,
    ::common_messages_sv2::ChannelEndpointChanged,
    size: [channel_id]
);

impl ChannelEndpointChanged {
    pub fn into_static(self) -> ChannelEndpointChanged {
        ChannelEndpointChanged {
            channel_id: self.channel_id,
        }
    }
    pub fn as_static(&self) -> ChannelEndpointChanged {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupConnection<'decoder> {
    pub protocol: Protocol,
    pub min_version: u16,
    pub max_version: u16,
    pub flags: u32,
    pub endpoint_host: Str0255<'decoder>,
    pub endpoint_port: u16,
    pub vendor: Str0255<'decoder>,
    pub hardware_version: Str0255<'decoder>,
    pub firmware: Str0255<'decoder>,
    pub device_id: Str0255<'decoder>,
}

impl<'decoder> From<SetupConnection<'decoder>> for ::common_messages_sv2::SetupConnectionOwned {
    fn from(value: SetupConnection<'decoder>) -> Self {
        Self {
            protocol: value.protocol.into(),
            min_version: value.min_version.into(),
            max_version: value.max_version.into(),
            flags: value.flags.into(),
            endpoint_host: value.endpoint_host.into(),
            endpoint_port: value.endpoint_port.into(),
            vendor: value.vendor.into(),
            hardware_version: value.hardware_version.into(),
            firmware: value.firmware.into(),
            device_id: value.device_id.into(),
        }
    }
}

impl<'decoder> From<::common_messages_sv2::SetupConnectionOwned> for SetupConnection<'decoder> {
    fn from(value: ::common_messages_sv2::SetupConnectionOwned) -> Self {
        Self {
            protocol: value.protocol.into(),
            min_version: value.min_version.into(),
            max_version: value.max_version.into(),
            flags: value.flags.into(),
            endpoint_host: value.endpoint_host.into(),
            endpoint_port: value.endpoint_port.into(),
            vendor: value.vendor.into(),
            hardware_version: value.hardware_version.into(),
            firmware: value.firmware.into(),
            device_id: value.device_id.into(),
        }
    }
}

impl<'decoder> From<::common_messages_sv2::SetupConnection<'decoder>>
    for SetupConnection<'decoder>
{
    fn from(value: ::common_messages_sv2::SetupConnection<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetupConnection<'decoder>,
    ::common_messages_sv2::SetupConnectionOwned,
    ::common_messages_sv2::SetupConnection<'decoder>,
    size: [protocol, min_version, max_version, flags, endpoint_host, endpoint_port, vendor, hardware_version, firmware, device_id]
);

impl<'decoder> SetupConnection<'decoder> {
    pub fn into_static(self) -> SetupConnection<'static> {
        SetupConnection {
            protocol: self.protocol,
            min_version: self.min_version,
            max_version: self.max_version,
            flags: self.flags,
            endpoint_host: self.endpoint_host.into_static(),
            endpoint_port: self.endpoint_port,
            vendor: self.vendor.into_static(),
            hardware_version: self.hardware_version.into_static(),
            firmware: self.firmware.into_static(),
            device_id: self.device_id.into_static(),
        }
    }
    pub fn as_static(&self) -> SetupConnection<'static> {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[repr(C)]
pub struct SetupConnectionSuccess {
    pub used_version: u16,
    pub flags: u32,
}

impl From<SetupConnectionSuccess> for ::common_messages_sv2::SetupConnectionSuccessOwned {
    fn from(value: SetupConnectionSuccess) -> Self {
        Self {
            used_version: value.used_version.into(),
            flags: value.flags.into(),
        }
    }
}

impl From<::common_messages_sv2::SetupConnectionSuccessOwned> for SetupConnectionSuccess {
    fn from(value: ::common_messages_sv2::SetupConnectionSuccessOwned) -> Self {
        Self {
            used_version: value.used_version.into(),
            flags: value.flags.into(),
        }
    }
}

message_codec!(
    SetupConnectionSuccess,
    ::common_messages_sv2::SetupConnectionSuccessOwned,
    ::common_messages_sv2::SetupConnectionSuccess,
    size: [used_version, flags]
);

impl SetupConnectionSuccess {
    pub fn into_static(self) -> SetupConnectionSuccess {
        SetupConnectionSuccess {
            used_version: self.used_version,
            flags: self.flags,
        }
    }
    pub fn as_static(&self) -> SetupConnectionSuccess {
        self.clone().into_static()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupConnectionError<'decoder> {
    pub flags: u32,
    pub error_code: Str0255<'decoder>,
}

impl<'decoder> From<SetupConnectionError<'decoder>>
    for ::common_messages_sv2::SetupConnectionErrorOwned
{
    fn from(value: SetupConnectionError<'decoder>) -> Self {
        Self {
            flags: value.flags.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::common_messages_sv2::SetupConnectionErrorOwned>
    for SetupConnectionError<'decoder>
{
    fn from(value: ::common_messages_sv2::SetupConnectionErrorOwned) -> Self {
        Self {
            flags: value.flags.into(),
            error_code: value.error_code.into(),
        }
    }
}

impl<'decoder> From<::common_messages_sv2::SetupConnectionError<'decoder>>
    for SetupConnectionError<'decoder>
{
    fn from(value: ::common_messages_sv2::SetupConnectionError<'decoder>) -> Self {
        value.into_owned().into()
    }
}

message_codec!(
    SetupConnectionError<'decoder>,
    ::common_messages_sv2::SetupConnectionErrorOwned,
    ::common_messages_sv2::SetupConnectionError<'decoder>,
    size: [flags, error_code]
);

impl<'decoder> SetupConnectionError<'decoder> {
    pub fn into_static(self) -> SetupConnectionError<'static> {
        SetupConnectionError {
            flags: self.flags,
            error_code: self.error_code.into_static(),
        }
    }
    pub fn as_static(&self) -> SetupConnectionError<'static> {
        self.clone().into_static()
    }
}

impl<'decoder> SetupConnection<'decoder> {
    /// Set the flag to indicate that the downstream requires a standard job
    pub fn set_requires_standard_job(&mut self) {
        self.flags |= 0b_0000_0000_0000_0000_0000_0000_0000_0001;
    }

    /// Legacy spelling for enabling the current JD full-template mode flag.
    ///
    /// The released protocol no longer negotiates asynchronous mining with this bit.
    pub fn set_async_job_nogotiation(&mut self) {
        self.allow_full_template_mode();
    }

    /// Advertise support for the current JD full-template mode.
    pub fn allow_full_template_mode(&mut self) {
        self.flags |= 1;
    }

    /// Check flags using the current protocol semantics, including JD full-template mode.
    pub fn check_flags(protocol: Protocol, available_flags: u32, required_flags: u32) -> bool {
        ::common_messages_sv2::SetupConnection::check_flags(
            protocol,
            available_flags,
            required_flags,
        )
    }

    /// Check whether received versions are supported.
    ///
    /// If the versions are not supported, return `None` otherwise return the biggest version
    /// available
    pub fn get_version(&self, min_version: u16, max_version: u16) -> Option<u16> {
        if self.min_version > max_version || min_version > self.max_version {
            None
        } else {
            Some(self.max_version.min(max_version))
        }
    }

    /// Checks whether passed flags indicate that the downstream requires standard job.
    pub fn requires_standard_job(&self) -> bool {
        has_requires_std_job(self.flags)
    }
}

#[cfg(test)]
mod compatibility_tests {
    use super::super::{
        job_declaration_sv2 as legacy_jd, mining_sv2 as legacy_mining,
        template_distribution_sv2 as legacy_td,
    };
    use super::*;
    use core::convert::TryInto;

    #[test]
    fn job_declaration_flags_follow_current_full_template_negotiation() {
        assert!(!SetupConnection::check_flags(
            Protocol::JobDeclarationProtocol,
            1,
            0
        ));
        assert!(SetupConnection::check_flags(
            Protocol::JobDeclarationProtocol,
            0,
            1
        ));
        assert!(SetupConnection::check_flags(
            Protocol::JobDeclarationProtocol,
            1,
            1
        ));
    }

    #[test]
    fn standard_channel_request_id_updates_the_received_frame() {
        let message = legacy_mining::OpenStandardMiningChannel {
            request_id: 7.into(),
            user_identity: "worker".to_string().try_into().unwrap(),
            nominal_hash_rate: 100.0,
            max_target: [255; 32].into(),
        };
        let mut bytes = ::binary_sv2::to_bytes(message).unwrap();
        let mut decoded: legacy_mining::OpenStandardMiningChannel =
            ::binary_sv2::from_bytes(&mut bytes).unwrap();
        decoded.update_id(99);
        assert_eq!(decoded.get_request_id_as_u32(), 99);
        let detached = decoded.into_static();
        assert_eq!(&bytes[..4], &99u32.to_le_bytes());
        assert_eq!(detached.get_request_id_as_u32(), 99);
    }

    #[test]
    fn standard_channel_success_id_updates_the_received_frame() {
        let message = legacy_mining::OpenStandardMiningChannelSuccess {
            request_id: 7.into(),
            channel_id: 1,
            target: [255; 32].into(),
            extranonce_prefix: vec![1, 2].try_into().unwrap(),
            group_channel_id: 3,
        };
        let mut bytes = ::binary_sv2::to_bytes(message).unwrap();
        let mut decoded: legacy_mining::OpenStandardMiningChannelSuccess =
            ::binary_sv2::from_bytes(&mut bytes).unwrap();
        decoded.update_id(99);
        assert_eq!(decoded.get_request_id_as_u32(), 99);
        drop(decoded);
        assert_eq!(&bytes[..4], &99u32.to_le_bytes());
    }

    #[test]
    fn extended_channel_success_uses_the_released_wire_layout() {
        let message = legacy_mining::OpenExtendedMiningChannelSuccess {
            request_id: 7,
            channel_id: 1,
            target: [255; 32].into(),
            extranonce_size: 8,
            extranonce_prefix: vec![1, 2].try_into().unwrap(),
        };
        let mut bytes = ::binary_sv2::to_bytes(message.clone()).unwrap();
        let current: ::mining_sv2::OpenExtendedMiningChannelSuccess =
            ::binary_sv2::from_bytes(&mut bytes).unwrap();
        assert_eq!(current.group_channel_id, 0);
        assert_eq!(current.extranonce_size, 8);
        assert_eq!(
            legacy_mining::OpenExtendedMiningChannelSuccess::from(current),
            message
        );
    }

    #[test]
    fn custom_job_drops_removed_wire_fields_but_static_conversion_keeps_them() {
        let message = legacy_mining::SetCustomMiningJob {
            channel_id: 1,
            request_id: 2,
            token: vec![3].try_into().unwrap(),
            version: 4,
            prev_hash: [5; 32].into(),
            min_ntime: 6,
            nbits: 7,
            coinbase_tx_version: 8,
            coinbase_prefix: vec![9].try_into().unwrap(),
            coinbase_tx_input_n_sequence: 10,
            coinbase_tx_value_remaining: 11,
            coinbase_tx_outputs: vec![12].try_into().unwrap(),
            coinbase_tx_locktime: 13,
            merkle_path: Seq0255::new(vec![[14; 32].into()]).unwrap(),
            extranonce_size: 15,
        };
        assert_eq!(message.as_static(), message);
        let mut bytes = ::binary_sv2::to_bytes(message.clone()).unwrap();
        let current: ::mining_sv2::SetCustomMiningJob =
            ::binary_sv2::from_bytes(&mut bytes).unwrap();
        assert_eq!(current.coinbase_tx_locktime, 13);
        let decoded = legacy_mining::SetCustomMiningJob::from(current);
        assert_eq!(decoded.coinbase_tx_value_remaining, 0);
        assert_eq!(decoded.extranonce_size, 0);
        assert_eq!(decoded.merkle_path, message.merkle_path);
        assert_eq!(decoded.coinbase_tx_outputs, message.coinbase_tx_outputs);
    }

    #[test]
    fn legacy_solution_uses_push_solution_nonce_before_ntime() {
        let message = legacy_jd::SubmitSolutionJd {
            extranonce: vec![1, 2].try_into().unwrap(),
            prev_hash: [3; 32].into(),
            ntime: 0x11223344,
            nonce: 0x55667788,
            nbits: 7,
            version: 8,
        };
        let mut bytes = ::binary_sv2::to_bytes(message.clone()).unwrap();
        // B032 header + two extranonce bytes + previous block hash.
        assert_eq!(&bytes[35..39], &message.nonce.to_le_bytes());
        assert_eq!(&bytes[39..43], &message.ntime.to_le_bytes());
        let current: ::job_declaration_sv2::PushSolution =
            ::binary_sv2::from_bytes(&mut bytes).unwrap();
        assert_eq!(legacy_jd::SubmitSolutionJd::from(current), message);
    }

    #[test]
    fn allocation_success_reconstructs_removed_legacy_fields() {
        let current = ::job_declaration_sv2::AllocateMiningJobTokenSuccessOwned {
            request_id: 1,
            mining_job_token: vec![2].try_into().unwrap(),
            coinbase_outputs: vec![3, 4, 5].try_into().unwrap(),
        };
        let mut bytes = ::binary_sv2::to_bytes(current).unwrap();
        let legacy: legacy_jd::AllocateMiningJobTokenSuccess =
            ::binary_sv2::from_bytes(&mut bytes).unwrap();
        assert_eq!(legacy.coinbase_output_max_additional_size, 3);
        assert!(legacy.async_mining_allowed);
        assert_eq!(legacy.coinbase_output.inner_as_ref(), &[3, 4, 5]);
        assert_eq!(::binary_sv2::to_bytes(legacy).unwrap(), bytes);
    }

    #[test]
    fn output_constraints_reserve_a_conservative_sigop_budget() {
        for (size, sigops) in [(0, 0), (42, 840), (1000, 20_000), (u32::MAX, 20_000)] {
            let legacy = legacy_td::CoinbaseOutputDataSize {
                coinbase_output_max_additional_size: size,
            };
            let mut bytes = ::binary_sv2::to_bytes(legacy).unwrap();
            assert_eq!(bytes.len(), 6);
            let current: ::template_distribution_sv2::CoinbaseOutputConstraints =
                ::binary_sv2::from_bytes(&mut bytes).unwrap();
            assert_eq!(current.coinbase_output_max_additional_sigops, sigops);
            assert_eq!(current.coinbase_output_max_additional_size, size);
        }
    }

    #[test]
    fn declared_witness_ids_and_coinbase_parts_keep_their_legacy_field_names() {
        let legacy = legacy_jd::DeclareMiningJob {
            request_id: 1,
            mining_job_token: vec![2].try_into().unwrap(),
            version: 3,
            coinbase_prefix: vec![4].try_into().unwrap(),
            coinbase_suffix: vec![5].try_into().unwrap(),
            tx_list: Seq064K::new(vec![[6; 32].into()]).unwrap(),
            excess_data: vec![7].try_into().unwrap(),
        };
        let mut bytes = ::binary_sv2::to_bytes(legacy.clone()).unwrap();
        let current: ::job_declaration_sv2::DeclareMiningJob =
            ::binary_sv2::from_bytes(&mut bytes).unwrap();
        assert_eq!(current.coinbase_tx_prefix.as_bytes(), &[4]);
        assert_eq!(current.coinbase_tx_suffix.as_bytes(), &[5]);
        assert_eq!(legacy_jd::DeclareMiningJob::from(current), legacy);
    }

    // Raw fixtures use the field order and widths in SRI v1.12.0's message definitions.
    // Checking both codecs against the same bytes catches omitted/reordered fields and size errors.
    #[test]
    fn every_legacy_message_matches_the_released_wire_schema() {
        macro_rules! check_wire {
            ($legacy:ty, $current:ty, $bytes:expr) => {{
                let expected: &[u8] = &$bytes;
                let mut current_bytes = expected.to_vec();
                let current: $current = ::binary_sv2::from_bytes(&mut current_bytes).unwrap();
                assert_eq!(
                    ::binary_sv2::to_bytes(current.into_owned()).unwrap(),
                    expected,
                    "released {} fixture",
                    stringify!($current)
                );
                let mut legacy_bytes = expected.to_vec();
                let legacy: $legacy = ::binary_sv2::from_bytes(&mut legacy_bytes).unwrap();
                assert_eq!(
                    ::binary_sv2::GetSize::get_size(&legacy),
                    expected.len(),
                    "{} size",
                    stringify!($legacy)
                );
                assert_eq!(
                    ::binary_sv2::to_bytes(legacy).unwrap(),
                    expected,
                    "{} wire round trip",
                    stringify!($legacy)
                );
            }};
        }
        check_wire!(
            crate::compat::common_messages_sv2::ChannelEndpointChanged,
            ::common_messages_sv2::ChannelEndpointChanged,
            [2, 0, 0, 0]
        );
        check_wire!(
            crate::compat::mining_sv2::Reconnect<'_>,
            ::common_messages_sv2::Reconnect<'_>,
            [2, 3, 4, 4, 0]
        );
        check_wire!(
            crate::compat::common_messages_sv2::SetupConnection<'_>,
            ::common_messages_sv2::SetupConnection<'_>,
            [
                0, 6, 0, 7, 0, 8, 0, 0, 0, 2, 9, 10, 10, 0, 2, 11, 12, 2, 12, 13, 2, 13, 14, 2, 14,
                15
            ]
        );
        check_wire!(
            crate::compat::common_messages_sv2::SetupConnectionSuccess,
            ::common_messages_sv2::SetupConnectionSuccess,
            [15, 0, 16, 0, 0, 0]
        );
        check_wire!(
            crate::compat::common_messages_sv2::SetupConnectionError<'_>,
            ::common_messages_sv2::SetupConnectionError<'_>,
            [17, 0, 0, 0, 2, 18, 19]
        );
        check_wire!(
            crate::compat::mining_sv2::CloseChannel<'_>,
            ::mining_sv2::CloseChannel<'_>,
            [19, 0, 0, 0, 2, 20, 21]
        );
        check_wire!(
            crate::compat::mining_sv2::NewMiningJob<'_>,
            ::mining_sv2::NewMiningJob<'_>,
            [
                21, 0, 0, 0, 22, 0, 0, 0, 1, 24, 0, 0, 0, 25, 0, 0, 0, 26, 26, 26, 26, 26, 26, 26,
                26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
                26, 26, 26, 26
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::NewExtendedMiningJob<'_>,
            ::mining_sv2::NewExtendedMiningJob<'_>,
            [
                27, 0, 0, 0, 28, 0, 0, 0, 1, 30, 0, 0, 0, 31, 0, 0, 0, 1, 2, 34, 34, 34, 34, 34,
                34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34, 34,
                34, 34, 34, 34, 34, 34, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35,
                35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 2, 0, 36, 37,
                2, 0, 37, 38
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::OpenStandardMiningChannel<'_>,
            ::mining_sv2::OpenStandardMiningChannel<'_>,
            [
                38, 0, 0, 0, 2, 39, 40, 0, 0, 247, 66, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41,
                41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41, 41
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::OpenStandardMiningChannelSuccess<'_>,
            ::mining_sv2::OpenStandardMiningChannelSuccess<'_>,
            [
                42, 0, 0, 0, 43, 0, 0, 0, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44,
                44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 44, 2, 45, 46,
                46, 0, 0, 0
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::OpenExtendedMiningChannel<'_>,
            ::mining_sv2::OpenExtendedMiningChannel<'_>,
            [
                47, 0, 0, 0, 2, 48, 49, 0, 0, 247, 66, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
                50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50, 50,
                51, 0
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::OpenExtendedMiningChannelSuccess<'_>,
            ::mining_sv2::OpenExtendedMiningChannelSuccess<'_>,
            [
                52, 0, 0, 0, 53, 0, 0, 0, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54,
                54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 54, 55, 0, 2,
                56, 57, 0, 0, 0, 0
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::OpenMiningChannelError<'_>,
            ::mining_sv2::OpenMiningChannelError<'_>,
            [57, 0, 0, 0, 2, 58, 59]
        );
        check_wire!(
            crate::compat::mining_sv2::SetCustomMiningJob<'_>,
            ::mining_sv2::SetCustomMiningJob<'_>,
            [
                59, 0, 0, 0, 60, 0, 0, 0, 2, 61, 62, 62, 0, 0, 0, 63, 63, 63, 63, 63, 63, 63, 63,
                63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63,
                63, 63, 63, 64, 0, 0, 0, 65, 0, 0, 0, 66, 0, 0, 0, 2, 67, 68, 68, 0, 0, 0, 2, 0,
                69, 70, 70, 0, 0, 0, 2, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72,
                72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 72, 73, 73, 73, 73,
                73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73, 73,
                73, 73, 73, 73, 73, 73, 73
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::SetCustomMiningJobSuccess,
            ::mining_sv2::SetCustomMiningJobSuccess,
            [74, 0, 0, 0, 75, 0, 0, 0, 76, 0, 0, 0]
        );
        check_wire!(
            crate::compat::mining_sv2::SetCustomMiningJobError<'_>,
            ::mining_sv2::SetCustomMiningJobError<'_>,
            [77, 0, 0, 0, 78, 0, 0, 0, 2, 79, 80]
        );
        check_wire!(
            crate::compat::mining_sv2::SetExtranoncePrefix<'_>,
            ::mining_sv2::SetExtranoncePrefix<'_>,
            [80, 0, 0, 0, 2, 81, 82]
        );
        check_wire!(
            crate::compat::mining_sv2::SetGroupChannel<'_>,
            ::mining_sv2::SetGroupChannel<'_>,
            [82, 0, 0, 0, 2, 0, 84, 0, 0, 0, 85, 0, 0, 0]
        );
        check_wire!(
            crate::compat::mining_sv2::SetNewPrevHash<'_>,
            ::mining_sv2::SetNewPrevHash<'_>,
            [
                86, 0, 0, 0, 87, 0, 0, 0, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88,
                88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 88, 89, 0, 0,
                0, 90, 0, 0, 0
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::SetTarget<'_>,
            ::mining_sv2::SetTarget<'_>,
            [
                91, 0, 0, 0, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92,
                92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92, 92
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::SubmitSharesStandard,
            ::mining_sv2::SubmitSharesStandard,
            [93, 0, 0, 0, 94, 0, 0, 0, 95, 0, 0, 0, 96, 0, 0, 0, 97, 0, 0, 0, 98, 0, 0, 0]
        );
        check_wire!(
            crate::compat::mining_sv2::SubmitSharesExtended<'_>,
            ::mining_sv2::SubmitSharesExtended<'_>,
            [
                99, 0, 0, 0, 100, 0, 0, 0, 101, 0, 0, 0, 102, 0, 0, 0, 103, 0, 0, 0, 104, 0, 0, 0,
                2, 105, 106
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::SubmitSharesSuccess,
            ::mining_sv2::SubmitSharesSuccess,
            [106, 0, 0, 0, 107, 0, 0, 0, 108, 0, 0, 0, 109, 0, 0, 0, 0, 0, 0, 0]
        );
        check_wire!(
            crate::compat::mining_sv2::SubmitSharesError<'_>,
            ::mining_sv2::SubmitSharesError<'_>,
            [110, 0, 0, 0, 111, 0, 0, 0, 2, 112, 113]
        );
        check_wire!(
            crate::compat::mining_sv2::UpdateChannel<'_>,
            ::mining_sv2::UpdateChannel<'_>,
            [
                113, 0, 0, 0, 0, 0, 247, 66, 115, 115, 115, 115, 115, 115, 115, 115, 115, 115, 115,
                115, 115, 115, 115, 115, 115, 115, 115, 115, 115, 115, 115, 115, 115, 115, 115,
                115, 115, 115, 115, 115
            ]
        );
        check_wire!(
            crate::compat::mining_sv2::UpdateChannelError<'_>,
            ::mining_sv2::UpdateChannelError<'_>,
            [116, 0, 0, 0, 2, 117, 118]
        );
        check_wire!(
            crate::compat::job_declaration_sv2::AllocateMiningJobToken<'_>,
            ::job_declaration_sv2::AllocateMiningJobToken<'_>,
            [2, 118, 119, 119, 0, 0, 0]
        );
        check_wire!(
            crate::compat::job_declaration_sv2::AllocateMiningJobTokenSuccess<'_>,
            ::job_declaration_sv2::AllocateMiningJobTokenSuccess<'_>,
            [120, 0, 0, 0, 2, 121, 122, 2, 0, 122, 123]
        );
        check_wire!(
            crate::compat::job_declaration_sv2::DeclareMiningJob<'_>,
            ::job_declaration_sv2::DeclareMiningJob<'_>,
            [
                123, 0, 0, 0, 2, 124, 125, 125, 0, 0, 0, 2, 0, 126, 127, 2, 0, 127, 128, 2, 0, 129,
                129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129,
                129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 129, 130,
                130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130,
                130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 130, 2, 0,
                131, 132
            ]
        );
        check_wire!(
            crate::compat::job_declaration_sv2::DeclareMiningJobSuccess<'_>,
            ::job_declaration_sv2::DeclareMiningJobSuccess<'_>,
            [132, 0, 0, 0, 2, 133, 134]
        );
        check_wire!(
            crate::compat::job_declaration_sv2::DeclareMiningJobError<'_>,
            ::job_declaration_sv2::DeclareMiningJobError<'_>,
            [134, 0, 0, 0, 2, 135, 136, 2, 0, 136, 137]
        );
        check_wire!(
            crate::compat::job_declaration_sv2::ProvideMissingTransactions<'_>,
            ::job_declaration_sv2::ProvideMissingTransactions<'_>,
            [137, 0, 0, 0, 2, 0, 139, 0, 140, 0]
        );
        check_wire!(
            crate::compat::job_declaration_sv2::ProvideMissingTransactionsSuccess<'_>,
            ::job_declaration_sv2::ProvideMissingTransactionsSuccess<'_>,
            [141, 0, 0, 0, 2, 0, 2, 0, 0, 143, 144, 2, 0, 0, 144, 145]
        );
        check_wire!(
            crate::compat::job_declaration_sv2::SubmitSolutionJd<'_>,
            ::job_declaration_sv2::PushSolution<'_>,
            [
                2, 145, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146,
                146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146, 146,
                146, 146, 147, 0, 0, 0, 148, 0, 0, 0, 149, 0, 0, 0, 150, 0, 0, 0
            ]
        );
        check_wire!(
            crate::compat::template_distribution_sv2::CoinbaseOutputDataSize,
            ::template_distribution_sv2::CoinbaseOutputConstraints,
            [151, 0, 0, 0, 204, 11]
        );
        check_wire!(
            crate::compat::template_distribution_sv2::NewTemplate<'_>,
            ::template_distribution_sv2::NewTemplate<'_>,
            [
                152, 0, 0, 0, 0, 0, 0, 0, 1, 154, 0, 0, 0, 155, 0, 0, 0, 2, 156, 157, 157, 0, 0, 0,
                158, 0, 0, 0, 0, 0, 0, 0, 159, 0, 0, 0, 2, 0, 160, 161, 161, 0, 0, 0, 2, 163, 163,
                163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163,
                163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 164, 164,
                164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164,
                164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164, 164
            ]
        );
        check_wire!(
            crate::compat::template_distribution_sv2::RequestTransactionData,
            ::template_distribution_sv2::RequestTransactionData,
            [165, 0, 0, 0, 0, 0, 0, 0]
        );
        check_wire!(
            crate::compat::template_distribution_sv2::RequestTransactionDataSuccess<'_>,
            ::template_distribution_sv2::RequestTransactionDataSuccess<'_>,
            [
                166, 0, 0, 0, 0, 0, 0, 0, 2, 0, 167, 168, 2, 0, 2, 0, 0, 169, 170, 2, 0, 0, 170,
                171
            ]
        );
        check_wire!(
            crate::compat::template_distribution_sv2::RequestTransactionDataError<'_>,
            ::template_distribution_sv2::RequestTransactionDataError<'_>,
            [171, 0, 0, 0, 0, 0, 0, 0, 2, 172, 173]
        );
        check_wire!(
            crate::compat::template_distribution_sv2::SetNewPrevHash<'_>,
            ::template_distribution_sv2::SetNewPrevHash<'_>,
            [
                173, 0, 0, 0, 0, 0, 0, 0, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174,
                174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174, 174,
                174, 174, 174, 174, 174, 175, 0, 0, 0, 176, 0, 0, 0, 177, 177, 177, 177, 177, 177,
                177, 177, 177, 177, 177, 177, 177, 177, 177, 177, 177, 177, 177, 177, 177, 177,
                177, 177, 177, 177, 177, 177, 177, 177, 177, 177
            ]
        );
        check_wire!(
            crate::compat::template_distribution_sv2::SubmitSolution<'_>,
            ::template_distribution_sv2::SubmitSolution<'_>,
            [178, 0, 0, 0, 0, 0, 0, 0, 179, 0, 0, 0, 180, 0, 0, 0, 181, 0, 0, 0, 2, 0, 182, 183]
        );
    }
}
