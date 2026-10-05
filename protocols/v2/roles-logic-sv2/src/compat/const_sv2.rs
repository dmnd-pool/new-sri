//! Legacy constant names, sourced from the current protocol crates.

pub use ::common_messages_sv2::*;
pub use ::job_declaration_sv2::*;
pub use ::mining_sv2::*;
pub use ::template_distribution_sv2::*;

pub use ::job_declaration_sv2::{
    CHANNEL_BIT_PUSH_SOLUTION as CHANNEL_BIT_SUBMIT_SOLUTION_JD,
    MESSAGE_TYPE_PUSH_SOLUTION as MESSAGE_TYPE_SUBMIT_SOLUTION_JD,
};
pub use ::mining_sv2::{
    CHANNEL_BIT_OPEN_EXTENDED_MINING_CHANNEL_SUCCESS as CHANNEL_BIT_OPEN_EXTENDED_MINING_CHANNEL_SUCCES,
    MESSAGE_TYPE_OPEN_EXTENDED_MINING_CHANNEL_SUCCESS as MESSAGE_TYPE_OPEN_EXTENDED_MINING_CHANNEL_SUCCES,
};
pub use ::template_distribution_sv2::{
    CHANNEL_BIT_COINBASE_OUTPUT_CONSTRAINTS as CHANNEL_BIT_COINBASE_OUTPUT_DATA_SIZE,
    MESSAGE_TYPE_COINBASE_OUTPUT_CONSTRAINTS as MESSAGE_TYPE_COINBASE_OUTPUT_DATA_SIZE,
};
