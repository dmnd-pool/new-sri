use crate::{
    binary_sv2::{self, GetSize, Seq0255, Sv2Option},
    mining_sv2::{NewExtendedMiningJob, SubmitSharesExtended},
};
use core::convert::TryInto;
use std::hint::black_box;

fn mining_job() -> NewExtendedMiningJob<'static> {
    NewExtendedMiningJob {
        channel_id: 1,
        job_id: 1,
        min_ntime: Sv2Option::new(Some(1)),
        version: 0x20000000,
        version_rolling_allowed: true,
        merkle_path: Seq0255::new(vec![[1; 32].into(); 12]).unwrap(),
        coinbase_tx_prefix: vec![1; 100].try_into().unwrap(),
        coinbase_tx_suffix: vec![2; 500].try_into().unwrap(),
    }
}

fn extended_share() -> SubmitSharesExtended<'static> {
    SubmitSharesExtended {
        channel_id: 7,
        sequence_number: 8,
        job_id: 9,
        nonce: 10,
        ntime: 11,
        version: 0x20000000,
        extranonce: vec![12; 28].try_into().unwrap(),
    }
}

#[test]
fn mining_message_size_does_not_allocate() {
    let job = mining_job();
    let allocations = crate::test_allocator::count_allocations(|| {
        assert_eq!(black_box(&job).get_size(), 1_007);
    });
    assert_eq!(allocations, 0, "message size must not copy the message");
}

#[test]
fn mining_job_encoding_keeps_the_allocation_budget() {
    let job = mining_job();
    let allocations = crate::test_allocator::count_allocations(|| {
        let bytes = binary_sv2::to_bytes(black_box(&job).clone()).unwrap();
        assert_eq!(bytes.len(), 1_007);
        black_box(bytes);
    });
    assert!(
        allocations <= 23,
        "old message encoding used 23 allocations; got {}",
        allocations
    );
}

#[test]
fn mining_job_encoding_uses_flat_fields() {
    use binary_sv2::encodable::EncodableField;

    let fields: EncodableField = mining_job().into();
    match &fields {
        EncodableField::Struct(fields) => assert!(
            fields
                .iter()
                .all(|field| matches!(field, EncodableField::Primitive(_))),
            "nested fields add allocation and traversal work to each mining job"
        ),
        EncodableField::Primitive(_) => panic!("a mining job has multiple fields"),
    }
}

#[test]
fn share_decoding_does_not_allocate_field_lists() {
    let original = extended_share();
    let mut bytes = binary_sv2::to_bytes(original.clone()).unwrap();
    let allocations = crate::test_allocator::count_allocations(|| {
        let decoded: SubmitSharesExtended = binary_sv2::from_bytes(black_box(&mut bytes)).unwrap();
        assert_eq!(decoded, original);
    });
    assert_eq!(
        allocations, 1,
        "only the owned extranonce needs an allocation"
    );
}

#[test]
fn share_decoding_rejects_incomplete_or_large_extranonce() {
    let mut bytes = binary_sv2::to_bytes(extended_share()).unwrap();
    for length in 0..bytes.len() {
        let mut incomplete = bytes[..length].to_vec();
        assert!(binary_sv2::from_bytes::<SubmitSharesExtended>(&mut incomplete).is_err());
    }
    bytes[24] = 33;
    bytes.extend([0; 5]);
    assert!(binary_sv2::from_bytes::<SubmitSharesExtended>(&mut bytes).is_err());
}
