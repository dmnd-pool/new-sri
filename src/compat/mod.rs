//! Adapters retaining the roles-logic API while delegating wire formats to SRI v1.12.0.

pub mod binary_sv2;
pub mod common_messages_sv2;
pub(crate) mod const_sv2;
pub(crate) mod ffi;
pub mod framing_sv2;
pub mod job_declaration_sv2;
pub mod mining_sv2;
pub mod template_distribution_sv2;

pub(crate) mod stratum_common {
    pub use bitcoin;
}
