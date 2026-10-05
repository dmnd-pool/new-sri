//! Legacy C layouts and ownership helpers. Protocol encoding stays in the SV2 crates.
//!
//! A C vector owns its allocation unless created with `as_shared_buffer`. Raw
//! handles may be copied for FFI, but an owned allocation must only be freed once.

use super::binary_sv2::{self as binary, Error, LegacyBytes, Seq0255, Seq064K, U24};
use super::common_messages_sv2::{
    ChannelEndpointChanged, Protocol, SetupConnection, SetupConnectionError, SetupConnectionSuccess,
};
use super::template_distribution_sv2::{
    CoinbaseOutputDataSize, NewTemplate, RequestTransactionData, RequestTransactionDataError,
    RequestTransactionDataSuccess, SetNewPrevHash, SubmitSolution,
};
use core::convert::TryInto;

/// Raw byte-vector handle with the original C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CVec {
    data: *mut u8,
    len: usize,
    capacity: usize,
}

impl CVec {
    /// The handle must refer to a live allocation with exclusive mutable access.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        if self.len == 0 {
            return &mut [];
        }
        // SAFETY: The FFI owner guarantees valid, exclusive access to the handle.
        unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
    }

    /// Borrows a caller-owned buffer, which must outlive every use of this handle.
    /// A zero capacity marks this allocation as borrowed for `free_vec`.
    pub fn as_shared_buffer(value: &mut [u8]) -> Self {
        Self {
            data: value.as_mut_ptr(),
            len: value.len(),
            capacity: 0,
        }
    }

    fn clone_owned(&self) -> Self {
        if self.len == 0 {
            return Vec::new().into();
        }
        // SAFETY: Cloning a live handle only reads its initialized bytes.
        Self::from(unsafe { core::slice::from_raw_parts(self.data, self.len) })
    }
}

impl From<Vec<u8>> for CVec {
    fn from(mut value: Vec<u8>) -> Self {
        let result = Self {
            data: value.as_mut_ptr(),
            len: value.len(),
            capacity: value.capacity(),
        };
        core::mem::forget(value);
        result
    }
}

impl From<&[u8]> for CVec {
    fn from(value: &[u8]) -> Self {
        value.to_vec().into()
    }
}

macro_rules! byte_handles {
    ($($ty:ident),+ $(,)?) => { $(
        impl From<binary::$ty<'_>> for CVec {
            fn from(value: binary::$ty<'_>) -> Self { value.legacy_bytes().into() }
        }
    )+ };
}
byte_handles!(U256, B032, B0255, B064K, B016M, Signature, U32AsRef);

/// Copies a foreign buffer into a Rust-owned C vector.
///
/// # Safety
/// For nonzero `len`, `data` must point to `len` initialized readable bytes.
#[no_mangle]
pub unsafe extern "C" fn cvec_from_buffer(data: *const u8, len: usize) -> CVec {
    if len == 0 {
        return Vec::new().into();
    }
    // SAFETY: The caller provides a readable foreign buffer.
    CVec::from(unsafe { core::slice::from_raw_parts(data, len) })
}

/// Raw vector-of-vectors handle with the original C layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CVec2 {
    data: *mut CVec,
    len: usize,
    capacity: usize,
}

impl CVec2 {
    /// The handle must refer to live elements with exclusive mutable access.
    pub fn as_mut_slice(&mut self) -> &mut [CVec] {
        if self.len == 0 {
            return &mut [];
        }
        // SAFETY: The FFI owner guarantees valid, exclusive access to the handle.
        unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
    }
}

impl From<Vec<CVec>> for CVec2 {
    fn from(mut value: Vec<CVec>) -> Self {
        let result = Self {
            data: value.as_mut_ptr(),
            len: value.len(),
            capacity: value.capacity(),
        };
        core::mem::forget(value);
        result
    }
}

impl From<CVec2> for Vec<CVec> {
    fn from(value: CVec2) -> Self {
        if value.capacity == 0 {
            return Vec::new();
        }
        // SAFETY: Consuming the owned handle transfers its allocation back to Rust.
        unsafe { Vec::from_raw_parts(value.data, value.len, value.capacity) }
    }
}

impl<T: Into<CVec>> From<Seq0255<'_, T>> for CVec2 {
    fn from(value: Seq0255<'_, T>) -> Self {
        value
            .into_inner()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>()
            .into()
    }
}
impl<T: Into<CVec>> From<Seq064K<'_, T>> for CVec2 {
    fn from(value: Seq064K<'_, T>) -> Self {
        value
            .into_inner()
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>()
            .into()
    }
}

/// Releases a live owned handle and clears it. Shared buffers remain caller-owned.
pub fn free_vec(value: &mut CVec) {
    let old = core::mem::replace(value, Vec::new().into());
    if old.capacity != 0 {
        // SAFETY: Owned handles originate from a Vec with this exact capacity.
        unsafe {
            drop(Vec::from_raw_parts(old.data, old.len, old.capacity));
        }
    }
}

/// Releases a live vector handle and each of its uniquely owned elements.
pub fn free_vec_2(value: &mut CVec2) {
    let old = core::mem::replace(value, Vec::<CVec>::new().into());
    let elements: Vec<CVec> = old.into();
    for mut element in elements {
        free_vec(&mut element);
    }
}

/// Initializes an empty vector handle.
///
/// # Safety
/// The caller must eventually free the resulting handle and avoid duplicate owners.
#[no_mangle]
pub unsafe extern "C" fn init_cvec2() -> CVec2 {
    Vec::<CVec>::new().into()
}

/// Moves a byte-vector handle into a vector handle.
///
/// # Safety
/// Both handles must be valid and uniquely owned; `element` ownership is transferred.
#[no_mangle]
pub unsafe extern "C" fn cvec2_push(value: &mut CVec2, element: CVec) {
    let previous = core::mem::replace(value, Vec::<CVec>::new().into());
    let mut elements: Vec<CVec> = previous.into();
    elements.push(element);
    *value = elements.into();
}

macro_rules! read_field {
    (copy, $field:expr) => {
        $field
    };
    (bytes, $field:expr) => {
        $field.as_mut_slice().try_into()?
    };
    (seq0255, $field:expr) => {
        Seq0255::new(
            $field
                .as_mut_slice()
                .iter_mut()
                .map(|value| value.as_mut_slice().try_into())
                .collect::<Result<Vec<_>, Error>>()?,
        )?
    };
    (seq064k, $field:expr) => {
        Seq064K::new(
            $field
                .as_mut_slice()
                .iter_mut()
                .map(|value| value.as_mut_slice().try_into())
                .collect::<Result<Vec<_>, Error>>()?,
        )?
    };
}
macro_rules! drop_field {
    (copy, $field:expr) => {};
    (bytes, $field:expr) => {
        free_vec(&mut $field);
    };
    (seq0255, $field:expr) => {
        free_vec_2(&mut $field);
    };
    (seq064k, $field:expr) => {
        free_vec_2(&mut $field);
    };
}

// Field order and visibility exactly match the former public C representations.
macro_rules! c_message {
    ($(#[$attr:meta])* $name:ident, $rust:ident, $free:ident {
        $($vis:vis $field:ident: $ty:ty => $kind:ident),+ $(,)?
    }) => {
        #[repr(C)]
        $(#[$attr])*
        pub struct $name { $($vis $field: $ty),+ }
        impl<'a> From<$rust<'a>> for $name {
            fn from(value: $rust<'a>) -> Self {
                Self { $($field: value.$field.into()),+ }
            }
        }
        impl $name {
            #[allow(clippy::wrong_self_convention)]
            pub fn to_rust_rep_mut(&mut self) -> Result<$rust<'_>, Error> {
                Ok($rust { $($field: read_field!($kind, self.$field)),+ })
            }
        }
        impl Drop for $name {
            fn drop(&mut self) { $(drop_field!($kind, self.$field);)+ }
        }
        #[no_mangle]
        pub extern "C" fn $free(value: $name) { drop(value); }
    };
}

c_message!(#[derive(Debug)] CSetupConnection, SetupConnection, free_setup_connection {
    pub protocol: Protocol => copy,
    pub min_version: u16 => copy,
    pub max_version: u16 => copy,
    pub flags: u32 => copy,
    pub endpoint_host: CVec => bytes,
    pub endpoint_port: u16 => copy,
    pub vendor: CVec => bytes,
    pub hardware_version: CVec => bytes,
    pub firmware: CVec => bytes,
    pub device_id: CVec => bytes,
});

impl Clone for CSetupConnection {
    fn clone(&self) -> Self {
        Self {
            protocol: self.protocol,
            min_version: self.min_version,
            max_version: self.max_version,
            flags: self.flags,
            endpoint_host: self.endpoint_host.clone_owned(),
            endpoint_port: self.endpoint_port,
            vendor: self.vendor.clone_owned(),
            hardware_version: self.hardware_version.clone_owned(),
            firmware: self.firmware.clone_owned(),
            device_id: self.device_id.clone_owned(),
        }
    }
}

c_message!(#[derive(Debug)] CSetupConnectionError, SetupConnectionError, free_setup_connection_error {
    flags: u32 => copy,
    error_code: CVec => bytes,
});

impl Clone for CSetupConnectionError {
    fn clone(&self) -> Self {
        Self {
            flags: self.flags,
            error_code: self.error_code.clone_owned(),
        }
    }
}

c_message!(CNewTemplate, NewTemplate, free_new_template {
    template_id: u64 => copy,
    future_template: bool => copy,
    version: u32 => copy,
    coinbase_tx_version: u32 => copy,
    coinbase_prefix: CVec => bytes,
    coinbase_tx_input_sequence: u32 => copy,
    coinbase_tx_value_remaining: u64 => copy,
    coinbase_tx_outputs_count: u32 => copy,
    coinbase_tx_outputs: CVec => bytes,
    coinbase_tx_locktime: u32 => copy,
    merkle_path: CVec2 => seq0255,
});

c_message!(CRequestTransactionDataSuccess, RequestTransactionDataSuccess, free_request_tx_data_success {
    template_id: u64 => copy,
    excess_data: CVec => bytes,
    transaction_list: CVec2 => seq064k,
});

c_message!(CRequestTransactionDataError, RequestTransactionDataError, free_request_tx_data_error {
    template_id: u64 => copy,
    error_code: CVec => bytes,
});

c_message!(CSetNewPrevHash, SetNewPrevHash, free_set_new_prev_hash {
    template_id: u64 => copy,
    prev_hash: CVec => bytes,
    header_timestamp: u32 => copy,
    n_bits: u32 => copy,
    target: CVec => bytes,
});

c_message!(CSubmitSolution, SubmitSolution, free_submit_solution {
    template_id: u64 => copy,
    version: u32 => copy,
    header_timestamp: u32 => copy,
    header_nonce: u32 => copy,
    coinbase_tx: CVec => bytes,
});

#[no_mangle]
// Legacy cbindgen marker; the argument is deliberately never inspected.
#[allow(improper_ctypes_definitions)]
pub extern "C" fn _c_export_u24(_: U24) {}
#[no_mangle]
pub extern "C" fn _c_export_cvec(_: CVec) {}
#[no_mangle]
pub extern "C" fn _c_export_cvec2(_: CVec2) {}
#[no_mangle]
pub extern "C" fn _c_export_channel_endpoint_changed(_: ChannelEndpointChanged) {}
#[no_mangle]
pub extern "C" fn _c_export_setup_conn_succ(_: SetupConnectionSuccess) {}
#[no_mangle]
pub extern "C" fn _c_export_coinbase_out(_: CoinbaseOutputDataSize) {}
#[no_mangle]
pub extern "C" fn _c_export_req_tx_data(_: RequestTransactionData) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_c_messages_round_trip_and_clone_owns_its_buffers() {
        let message = SetupConnection {
            protocol: Protocol::MiningProtocol,
            min_version: 2,
            max_version: 2,
            flags: 1,
            endpoint_host: b"localhost".to_vec().try_into().unwrap(),
            endpoint_port: 34254,
            vendor: b"vendor".to_vec().try_into().unwrap(),
            hardware_version: vec![1].try_into().unwrap(),
            firmware: vec![2].try_into().unwrap(),
            device_id: vec![3].try_into().unwrap(),
        };
        let mut original: CSetupConnection = message.clone().into();
        let mut cloned = original.clone();
        original.endpoint_host.as_mut_slice()[0] = b'X';
        drop(original);
        assert_eq!(cloned.to_rust_rep_mut().unwrap(), message);

        let error = SetupConnectionError {
            flags: 7,
            error_code: vec![4].try_into().unwrap(),
        };
        let original: CSetupConnectionError = error.clone().into();
        let mut cloned = original.clone();
        drop(original);
        assert_eq!(cloned.to_rust_rep_mut().unwrap(), error);
    }

    #[test]
    fn template_c_messages_round_trip() {
        let template = NewTemplate {
            template_id: 1,
            future_template: true,
            version: 2,
            coinbase_tx_version: 2,
            coinbase_prefix: vec![1, 2].try_into().unwrap(),
            coinbase_tx_input_sequence: 3,
            coinbase_tx_value_remaining: 4,
            coinbase_tx_outputs_count: 1,
            coinbase_tx_outputs: vec![5, 6].try_into().unwrap(),
            coinbase_tx_locktime: 7,
            merkle_path: Seq0255::new(vec![[8; 32].into()]).unwrap(),
        };
        let mut c_template: CNewTemplate = template.clone().into();
        assert_eq!(c_template.to_rust_rep_mut().unwrap(), template);

        let txs = RequestTransactionDataSuccess {
            template_id: 1,
            excess_data: vec![2].try_into().unwrap(),
            transaction_list: Seq064K::new(vec![vec![3, 4].try_into().unwrap()]).unwrap(),
        };
        let mut c_txs: CRequestTransactionDataSuccess = txs.clone().into();
        assert_eq!(c_txs.to_rust_rep_mut().unwrap(), txs);

        let error = RequestTransactionDataError {
            template_id: 1,
            error_code: vec![2].try_into().unwrap(),
        };
        let mut c_error: CRequestTransactionDataError = error.clone().into();
        assert_eq!(c_error.to_rust_rep_mut().unwrap(), error);

        let prev_hash = SetNewPrevHash {
            template_id: 1,
            prev_hash: [2; 32].into(),
            header_timestamp: 3,
            n_bits: 4,
            target: [5; 32].into(),
        };
        let mut c_prev_hash: CSetNewPrevHash = prev_hash.clone().into();
        assert_eq!(c_prev_hash.to_rust_rep_mut().unwrap(), prev_hash);

        let solution = SubmitSolution {
            template_id: 1,
            version: 2,
            header_timestamp: 3,
            header_nonce: 4,
            coinbase_tx: vec![5, 6].try_into().unwrap(),
        };
        let mut c_solution: CSubmitSolution = solution.clone().into();
        assert_eq!(c_solution.to_rust_rep_mut().unwrap(), solution);
    }

    #[test]
    fn c_vector_growth_preserves_capacity_and_free_clears_handles() {
        // SAFETY: Every appended vector transfers a unique Rust-owned allocation.
        let mut vectors = unsafe { init_cvec2() };
        for n in 0..20_u8 {
            unsafe {
                cvec2_push(&mut vectors, vec![n].into());
            }
        }
        assert_eq!(vectors.as_mut_slice().len(), 20);
        free_vec_2(&mut vectors);
        free_vec_2(&mut vectors);
        assert!(vectors.as_mut_slice().is_empty());

        let mut bytes = [1, 2];
        let mut shared = CVec::as_shared_buffer(&mut bytes);
        free_vec(&mut shared);
        assert_eq!(bytes, [1, 2]);
    }
}
