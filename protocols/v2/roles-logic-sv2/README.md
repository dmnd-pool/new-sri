# roles_logic_sv2

Standalone copy of `../stratum/protocols/v2/roles-logic-sv2`, retaining its pool,
proxy, routing, job creation, and handler APIs while using the protocol libraries
from [SRI v1.12.0](https://github.com/stratum-mining/stratum/releases/tag/v1.12.0).
The source snapshot is from the local Stratum checkout at
`f037e6db3568454cdf7973aa6f457dd41d0856e1`, including its pool-specific channel
restoration, job variants, and fanout methods.

## Use in a pool

```toml
[dependencies]
roles_logic_sv2 = { path = "../roles-logic-sv2" }
```

Keep using the existing `roles_logic_sv2` modules, message struct literals,
handler traits, parser enum variants, and `into_static()` / `as_static()` methods.
`Target`, `Extranonce`, and `ExtendedExtranonce` remain under
`roles_logic_sv2::mining_sv2`. The `prop_test` and `disable_nopanic` features remain
available. `prop_test` also retains the template message generators.

The adapters in `src/compat` keep the old Rust interface and delegate wire codecs
to the released crates. They retain owned message data because SRI now separates
borrowed and owned types. Standard-channel request IDs still borrow the received
payload so proxy ID remapping updates relayed bytes.

### Direct dependency imports

SRI's new binary and message types are different Rust types. If your pool imports
those types directly from another dependency, use the matching compatibility
imports at the roles-logic boundary:

```rust
use roles_logic_sv2::{binary_sv2, mining_sv2, template_distribution_sv2};
```

Conversions to and from SRI's current owned messages are also available through
`From` / `Into`. Bitcoin types remain compatible with `bitcoin` 0.32.

Transport code using the old external `codec_sv2::Frame` / `StandardEitherFrame`
API needs adaptation to codec 7; this library cannot restore names inside another
crate. Every parser enum converts into the current `framing_sv2::MessageFrame`.
The additive `roles_logic_sv2::framing_sv2` facade also provides the old generic
`Sv2Frame<T, B>`, which implements the current codec's `EncodableFrame` trait.
Received current `SerializedFrame` values convert into that compatibility frame.
Factory and handler APIs do not require a rewrite.

## Release dependencies

| Crate | Version |
| --- | --- |
| `binary_sv2` | 7.0.0 |
| `derive_codec_sv2` (transitive) | 2.0.0 |
| `common_messages_sv2` | 9.0.0 |
| `mining_sv2` | 12.0.0 |
| `job_declaration_sv2` | 10.0.0 |
| `template_distribution_sv2` | 7.0.0 |
| `framing_sv2` | 8.0.0 |
| `codec_sv2` (test dependency) | 7.0.0 |
| `noise_sv2` (via the test codec) | 2.0.0 |

There are no dependencies on the sibling checkout or old SRI protocol versions.
The former `const_sv2` constants now come from the current subprotocol crates;
`stratum-common` is replaced by its underlying Bitcoin dependency. Role logic is
retained here, so this is not a replacement with the unrelated `channels_sv2`,
`handlers_sv2`, or `parsers_sv2` APIs or a backport of all their behavioral changes.

## Protocol changes with compatibility defaults

Keeping a Rust field does not put a removed field back on the wire:

| Legacy interface | SRI v1.12.0 behavior |
| --- | --- |
| `CoinbaseOutputDataSize` | Encodes `CoinbaseOutputConstraints`, reserving `min(size * 20, 20_000)` additional sigops conservatively. The current `CoinbaseOutputConstraints` type is also exposed for an explicit budget. |
| `OpenExtendedMiningChannelSuccess` | Encodes group ID zero; decoding discards the new group ID. The retained factories dispatch extended work by individual channel. Group-aware transport must retain the current SRI message separately. |
| `AllocateMiningJobTokenSuccess` | Encodes the token and outputs. Decoding derives the old size from output bytes and sets `async_mining_allowed = true`. |
| `SetCustomMiningJob` | The removed reward and extranonce-size fields decode as zero. Complete output amounts are preserved when reconstructing a job; extranonce length comes from channel state. Nonzero locally supplied legacy rewards still override the first output. |
| `DeclareMiningJob::tx_list` | Represents **wtxids** on the current wire. A caller providing txids must compute wtxids from its transactions; hashes alone cannot be converted. |
| `set_async_job_nogotiation()` | Retains its name but sets the current JD full-template-mode flag. Flag validation follows the current protocol. |
| `SubmitSolutionJd` | Encodes `PushSolution`, with the current nonce/ntime field order. |
| `Mining::Reconnect` | Retains its enum variant and uses the current common-protocol message ID `0x04`. |

The existing Rust-facing C message wrappers and exported C conversion/free helpers
are retained as well. Raw C handles retain their ownership contract: owned buffers
must be freed exactly once.

## Verification

```sh
cargo fmt --all -- --check
cargo test --locked
cargo test --locked --all-features
```

Tests retain the original mining round, channel restoration, fanout, and job
variant checks. Additional tests compare all 41 message schemas with independent
current-wire fixtures, exercise the released codec, preserve proxy request-ID
mutation, roundtrip C wrappers, and run a pool-style handler and custom job against
current SRI messages. One upstream job-dispatcher test remains marked ignored.
`tests/pool_api.rs` checks the API used by the sibling pool; it is not a full
end-to-end run of that pool's networking stack.

Licensed under MIT OR Apache-2.0; see `LICENSE-MIT` and `LICENSE-APACHE`.
