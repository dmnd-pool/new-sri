//! Compile and exercise the public API used by the original pool.
use bitcoin::{Amount, ScriptBuf, TxOut};
use roles_logic_sv2::{
    channel_logic::channel_factory::{ExtendedChannelKind, PoolChannelFactory},
    common_properties::{CommonDownstreamData, IsDownstream, IsMiningDownstream},
    handlers::mining::{ParseDownstreamMiningMessages, SendTo, SupportedChannelTypes},
    job_creator::JobsCreators,
    mining_sv2::*,
    parsers::{IsSv2Message, Mining},
    routing_logic::NoRouting,
    selectors::NullDownstreamMiningSelector,
    template_distribution_sv2::NewTemplate,
    utils::{GroupId, Mutex},
    Error,
};
use std::{convert::TryInto, sync::Arc};

#[derive(Debug)]
struct Pool {
    factory: PoolChannelFactory,
}

impl IsDownstream for Pool {
    fn get_downstream_mining_data(&self) -> CommonDownstreamData {
        CommonDownstreamData {
            header_only: false,
            work_selection: true,
            version_rolling: true,
        }
    }
}
impl IsMiningDownstream for Pool {}

impl ParseDownstreamMiningMessages<(), NullDownstreamMiningSelector, NoRouting> for Pool {
    fn get_channel_type(&self) -> SupportedChannelTypes {
        SupportedChannelTypes::GroupAndExtended
    }

    fn is_work_selection_enabled(&self) -> bool {
        true
    }

    fn handle_open_standard_mining_channel(
        &mut self,
        message: OpenStandardMiningChannel,
        _upstream: Option<Arc<Mutex<()>>>,
    ) -> Result<SendTo<()>, Error> {
        let id = self.factory.new_standard_id_for_hom();
        let messages = self.factory.add_standard_channel(
            message.request_id.as_u32(),
            message.nominal_hash_rate,
            true,
            id,
        )?;
        Ok(SendTo::Multiple(
            messages
                .into_iter()
                .map(|m| SendTo::Respond(m.into_static()))
                .collect(),
        ))
    }

    fn handle_open_extended_mining_channel(
        &mut self,
        message: OpenExtendedMiningChannel,
    ) -> Result<SendTo<()>, Error> {
        let messages = self.factory.new_extended_channel(
            message.request_id,
            message.nominal_hash_rate,
            message.min_extranonce_size,
        )?;
        Ok(SendTo::Multiple(
            messages.into_iter().map(SendTo::Respond).collect(),
        ))
    }

    fn handle_update_channel(&mut self, _: UpdateChannel) -> Result<SendTo<()>, Error> {
        Ok(SendTo::None(None))
    }
    fn handle_submit_shares_standard(
        &mut self,
        _: SubmitSharesStandard,
    ) -> Result<SendTo<()>, Error> {
        Ok(SendTo::None(None))
    }
    fn handle_submit_shares_extended(
        &mut self,
        _: SubmitSharesExtended,
    ) -> Result<SendTo<()>, Error> {
        Ok(SendTo::None(None))
    }
    fn handle_set_custom_mining_job(
        &mut self,
        message: SetCustomMiningJob,
    ) -> Result<SendTo<()>, Error> {
        Ok(SendTo::Respond(Mining::SetCustomMiningJobSuccess(
            self.factory
                .on_new_set_custom_mining_job(message.into_static()),
        )))
    }
}

#[test]
fn unchanged_pool_handler_accepts_current_wire_messages() {
    let factory = PoolChannelFactory::new(
        Arc::new(Mutex::new(GroupId::new())),
        ExtendedExtranonce::new(0..0, 0..5, 5..13),
        JobsCreators::new(13),
        1.0,
        ExtendedChannelKind::Pool,
        vec![TxOut {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::new(),
        }],
        b"/pool/".to_vec(),
    )
    .unwrap();
    let pool = Arc::new(Mutex::new(Pool { factory }));

    // The wire comes from the released mining crate, independently of the facade.
    let request = ::mining_sv2::OpenExtendedMiningChannelOwned {
        request_id: 42,
        user_identity: b"worker".to_vec().try_into().unwrap(),
        nominal_hash_rate: 100_000_000_000.0,
        max_target: vec![255; 32].try_into().unwrap(),
        min_extranonce_size: 8,
    };
    let mut payload = binary_sv2::to_bytes(request).unwrap();
    let response = Pool::handle_message_mining(
        pool.clone(),
        ::mining_sv2::MESSAGE_TYPE_OPEN_EXTENDED_MINING_CHANNEL,
        &mut payload,
        roles_logic_sv2::routing_logic::MiningRoutingLogic::None,
    )
    .unwrap();
    let SendTo::Multiple(messages) = response else {
        panic!("expected channel response")
    };
    let SendTo::Respond(Mining::OpenExtendedMiningChannelSuccess(success)) = &messages[0] else {
        panic!("expected extended channel success")
    };
    assert_eq!(success.request_id, 42);
    assert_eq!(success.extranonce_size, 8);
    let channel_id = success.channel_id;
    let mut encoded = binary_sv2::to_bytes(success.clone()).unwrap();
    let wire: ::mining_sv2::OpenExtendedMiningChannelSuccess =
        binary_sv2::from_bytes(&mut encoded).unwrap();
    assert_eq!(wire.channel_id, channel_id);
    assert_eq!(wire.group_channel_id, 0);

    // Legacy struct literals and pool-specific fanout extensions remain usable.
    let mut template = NewTemplate {
        template_id: 1,
        future_template: true,
        version: 0x2000_0000,
        coinbase_tx_version: 2,
        coinbase_prefix: vec![1, 17].try_into().unwrap(),
        coinbase_tx_input_sequence: u32::MAX,
        coinbase_tx_value_remaining: 3_1250_0000,
        coinbase_tx_outputs_count: 0,
        coinbase_tx_outputs: Vec::new().try_into().unwrap(),
        coinbase_tx_locktime: 0,
        merkle_path: vec![].into(),
    };
    let (job, channels, _) = pool
        .safe_lock(|pool| {
            pool.factory
                .on_new_template_for_extended_fanout(&mut template)
        })
        .unwrap()
        .unwrap();
    assert_eq!(channels, vec![channel_id]);
    assert!(job.is_future());
    let message = Mining::NewExtendedMiningJob(job);
    assert_eq!(
        message.message_type(),
        ::mining_sv2::MESSAGE_TYPE_NEW_EXTENDED_MINING_JOB
    );
    let mut encoded = binary_sv2::to_bytes(message).unwrap();
    let wire: ::mining_sv2::NewExtendedMiningJob = binary_sv2::from_bytes(&mut encoded).unwrap();
    assert!(wire.is_future());
    assert!(!wire.coinbase_tx_prefix.as_ref().is_empty());
}

#[test]
fn current_custom_job_keeps_complete_coinbase_outputs() {
    let outputs = vec![
        TxOut {
            value: Amount::from_sat(300_000_000),
            script_pubkey: ScriptBuf::new(),
        },
        TxOut {
            value: Amount::from_sat(12_500_000),
            script_pubkey: ScriptBuf::new(),
        },
    ];
    let output_bytes: Vec<u8> = outputs
        .iter()
        .flat_map(bitcoin::consensus::serialize)
        .collect();
    let wire = ::mining_sv2::SetCustomMiningJobOwned {
        channel_id: 1,
        request_id: 7,
        token: vec![1].try_into().unwrap(),
        version: 0x2000_0000,
        prev_hash: vec![0; 32].try_into().unwrap(),
        min_ntime: 1_700_000_000,
        nbits: 0x207fffff,
        coinbase_tx_version: 2,
        coinbase_prefix: vec![2, 0xe8, 3].try_into().unwrap(),
        coinbase_tx_input_n_sequence: u32::MAX,
        coinbase_tx_outputs: output_bytes.try_into().unwrap(),
        coinbase_tx_locktime: 0,
        merkle_path: vec![].try_into().unwrap(),
    };
    let mut bytes = binary_sv2::to_bytes(wire).unwrap();
    let legacy: SetCustomMiningJob = binary_sv2::from_bytes(&mut bytes).unwrap();
    assert_eq!(legacy.coinbase_tx_value_remaining, 0);
    let job = roles_logic_sv2::job_creator::extended_job_from_custom_job(&legacy, 0, 13).unwrap();
    let mut coinbase = job.coinbase_tx_prefix.to_vec();
    coinbase.extend_from_slice(&[0; 13]);
    coinbase.extend_from_slice(job.coinbase_tx_suffix.as_ref());
    let tx: bitcoin::Transaction = bitcoin::consensus::deserialize(&coinbase).unwrap();
    assert_eq!(tx.output, outputs);
    assert_eq!(&tx.input[0].script_sig.as_bytes()[..3], &[2, 0xe8, 3]);
}
