// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

use super::errors::Error;
use super::manager::SyncManager;
use blocks::{Block, FullTipset, Tipset};
use chain::ChainStore;
use cid::{Cid, Codec, Error as CidError, Version};
use libp2p::core::PeerId;
use multihash::Multihash;
use raw_block::RawBlock;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Syncer<'a> {
    // TODO add ability to send msg to all subscribers indicating incoming blocks
    // TODO add state manager
    // TODO add block sync

    // manages sync buckets
    sync_manager: SyncManager<'a>,
    // access and store tipsets / blocks / messages
    chain_store: ChainStore<'a>,
    // the known genesis tipset
    _genesis: Tipset,
    // self peerId
    _own: PeerId,
}

impl<'a> Syncer<'a> {
    /// TODO add constructor

    /// informs the syncer about a new potential tipset
    /// This should be called when connecting to new peers, and additionally
    /// when receiving new blocks from the network
    fn inform_new_head(&self, from: PeerId, fts: FullTipset) -> Result<(), Error> {
        // check if full block is nil and if so return error
        if fts.blocks().is_empty() {
            return Err(Error::NoBlocks);
        }
        // validate message data
        for block in fts.blocks() {
            self.validate_msg_data(block)?;
        }
        // TODO send pubsub message indicating incoming blocks
        // TODO Add peer to blocksync

        // compare target_weight to heaviest weight stored; ignore otherwise
        let best_weight = self.chain_store.heaviest_tipset().blocks()[0].weight();
        let target_weight = fts.blocks()[0].to_header().weight();

        if !target_weight.lt(&best_weight) {
            // Store incoming block header
            self.chain_store.persist_headers(&fts.tipset()?)?;
            // Set peer head
            self.sync_manager.set_peer_head(from, fts.tipset()?);
        }
        // incoming tipset from miners does not appear to be better than our best chain, ignoring for now
        Ok(())
    }

    fn validate_msg_data(&self, block: &Block) -> Result<(), Error> {
        let sm_root = self.compute_msg_data(block)?;
        // TODO change message_receipts to messages() once #192 is in
        if block.to_header().message_receipts() != &sm_root {
            return Err(Error::InvalidRoots);
        }

        self.chain_store.put_messages(block.bls_msgs())?;
        self.chain_store.put_messages(block.secp_msgs())?;

        Ok(())
    }
    fn compute_msg_data(&self, block: &Block) -> Result<Cid, CidError> {
        // TODO compute message roots

        let _bls_cids = cids_from_messages(block.bls_msgs())?;
        let _secp_cids = cids_from_messages(block.secp_msgs())?;

        // TODO temporary until AMT structure is implemented
        // see Lotus implementation https://github.com/filecoin-project/lotus/blob/master/chain/sync.go#L338
        // will return a new CID representing both message roots
        let hash = Multihash::from_bytes(vec![0, 0]);
        Ok(Cid::new(Codec::DagCBOR, Version::V1, hash.unwrap()))
    }
    /// Should match up with 'Semantical Validation' in validation.md in the spec
    pub fn validate(&self, block: Block) -> Result<(), Error> {
        /* TODO block validation essentially involves 7 main checks: 
            1. time_check: Must have a valid timestamp
            2. winner_check: Must verify it contains the winning ticket 
            3. message_check: All messages in the block must be valid
            4. miner_check: Must be from a valid miner 
            5. block_sig_check: Must have a valid signature by the miner address of the final ticket
            6. verify_ticket_vrf: Must be generated from the smallest ticket in the parent tipset and from same miner
            7. verify_election_proof_check: Must include an election proof which is a valid signature by the miner address of the final ticket
        */

        // get header from full block
        let header = block.to_header();

        // TODO retrieve base_tipset from load_fts() when #196 comes in
        // temporary fix below; will replace with load_fts()
        let base_tipset = Tipset::new(vec!(*header))?;

        // check if block has been signed
        if header.signature().bytes().is_empty() {
            return  Err(Error::Blockchain("Signature is nil in header".to_string()));
        }
    
        /*
        1. time_check rules:
          - must include a timestamp not in the future
          - must have a valid timestamp
          - must be later than the earliest parent block time plus appropriate delay, which is BLOCK_DELAY
        */

        // Timestamp checks 
        // TODO include allowable clock drift
        let time_now =  match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) =>  n.as_secs(),
            Err(_) => return Err(Error::Validation("SystemTime before UNIX EPOCH!".to_string())),
        };
        if time_now < header.timestamp() {
            return Err(Error::Validation("Timestamp from future".to_string()));
        };
        // TODO: Block Delay is a default constant set in this doc https://github.com/filecoin-project/specs/blob/6ab401c0b92efb6420c6e198ec387cf56dc86057/validation.md
        // It is not mentioned how Block Delay can be changed so assume default value for now  
        // Also different time in Lotus implementation; will use Lotus implementation at 45 secs rather than 30 secs      
        const FIXED_BLOCK_DELAY: u64 = 45;
        // TODO add Sub trait to ChainEpoch type when it becomes u64 and re-work below for readability
        if header.timestamp() < base_tipset.min_timestamp()?+FIXED_BLOCK_DELAY*(*header.epoch() - *base_tipset.tip_epoch()) {
            return Err(Error::Validation("Block was generated too soon".to_string()));
        }

        /*
        TODO 
        2. winner_check rules:
          - TODOs missing the following pieces of data to validate ticket winner
                - miner slashing
                - miner power storage
                - miner sector size
                - fn is_ticket_winner()
          - See lotus check here for more details: https://github.com/filecoin-project/lotus/blob/master/chain/sync.go#L522
        */

        /*
        TODO
        3. message_check rules:
            - All messages in the block must be valid
            - The execution of each message, in the order they are in the block,
             must produce a receipt matching the corresponding one in the receipt set of the block
            - The resulting state root after all messages are applied, must match the one in the block

           TODOs missing the following pieces of data to validate messages
           - check_block_messages -> see https://github.com/filecoin-project/lotus/blob/master/chain/sync.go#L705 
        */
        
        /*
        TODO
        4. miner_check rules:
            - Ensure miner is valid; miner_is_valid -> see https://github.com/filecoin-project/lotus/blob/master/chain/sync.go#L460
        */

        /*
        TODO
        5. block_sig_check rules:
            - Must have a valid signature by the miner address of the final ticket
            
            TODOs missing the following pieces of data
            - check_block_sigs -> see https://github.com/filecoin-project/lotus/blob/master/chain/types/blockheader_cgo.go#L13
        */

        /*
        TODO
        6. verify_ticket_vrf rules:
            - the ticket must be generated from the smallest ticket in the parent tipset
            - all tickets in the ticket array must have been generated by the same miner

           TODOs
           - Complete verify_vrf -> see https://github.com/filecoin-project/lotus/blob/master/chain/gen/gen.go#L600 
        */

        /*
        TODO
        7. verify_election_proof_check rules: 
            - Must include an election proof which is a valid signature by the miner address of the final ticket

            TODOs
            - verify_election_proof -> see https://github.com/filecoin-project/lotus/blob/master/chain/sync.go#L650
        */

        Ok(())
    }
}

pub fn cids_from_messages<T: RawBlock>(messages: &[T]) -> Result<Vec<Cid>, CidError> {
    messages.iter().map(RawBlock::cid).collect()
}
