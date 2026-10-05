//! # Stratum V2 Roles-Logic Library
//!
//! roles_logic_sv2 provides the core logic and utilities for implementing roles in the Stratum V2
//! (Sv2) protocol, such as miners, pools, and proxies. It abstracts message handling, channel
//! management, job creation, and routing logic, enabling efficient and secure communication across
//! upstream and downstream connections.
//!
//! ## Usage
//!
//! To include this crate in your project, run:
//! ```bash
//! $ cargo add roles_logic_sv2
//! ```
//!
//! ## Build Options
//!
//! This crate can be built with the following features:
//!
//! - `prop_test`: Enables the retained template and job property tests.
extern crate alloc;

pub mod channel_logic;
pub mod common_properties;
mod compat;
pub mod errors;
pub mod handlers;
pub mod job_creator;
pub mod job_dispatcher;
pub mod parsers;
pub mod routing_logic;
pub mod selectors;
pub mod utils;
pub use compat::{
    binary_sv2, common_messages_sv2, framing_sv2, job_declaration_sv2, mining_sv2,
    template_distribution_sv2,
};
pub use errors::Error;
