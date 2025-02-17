// Copyright 2020 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT

mod params;
mod state;

pub use self::params::*;
pub use self::state::State;
use crate::{
    make_map, ACCOUNT_ACTOR_CODE_ID, INIT_ACTOR_CODE_ID, MARKET_ACTOR_CODE_ID, MINER_ACTOR_CODE_ID,
    POWER_ACTOR_CODE_ID, SYSTEM_ACTOR_ADDR,
};
use address::Address;
use cid::Cid;
use ipld_blockstore::BlockStore;
use message::Message;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use runtime::{ActorCode, Runtime};
use vm::{ActorError, ExitCode, MethodNum, Serialized, METHOD_CONSTRUCTOR};

/// Init actor methods available
#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    Exec = 2,
}

/// Init actor
pub struct Actor;
impl Actor {
    /// Init actor constructor
    pub fn constructor<BS, RT>(rt: &mut RT, params: ConstructorParams) -> Result<(), ActorError>
    where
        BS: BlockStore,
        RT: Runtime<BS>,
    {
        let sys_ref: &Address = &SYSTEM_ACTOR_ADDR;
        rt.validate_immediate_caller_is(std::iter::once(sys_ref))?;
        let mut empty_map = make_map(rt.store());
        let root = empty_map.flush().map_err(|err| {
            rt.abort(
                ExitCode::ErrIllegalState,
                format!("failed to construct state: {}", err),
            )
        })?;

        rt.create(&State::new(root, params.network_name))?;

        Ok(())
    }

    /// Exec init actor
    pub fn exec<BS, RT>(rt: &mut RT, params: ExecParams) -> Result<ExecReturn, ActorError>
    where
        BS: BlockStore,
        RT: Runtime<BS>,
    {
        rt.validate_immediate_caller_accept_any();
        let caller_code = rt
            .get_actor_code_cid(rt.message().from())
            .expect("no code for actor");
        if !can_exec(&caller_code, &params.code_cid) {
            return Err(rt.abort(
                ExitCode::ErrForbidden,
                format!(
                    "called type {} cannot exec actor type {}",
                    &caller_code, &params.code_cid
                ),
            ));
        }

        // Compute a re-org-stable address.
        // This address exists for use by messages coming from outside the system, in order to
        // stably address the newly created actor even if a chain re-org causes it to end up with
        // a different ID.
        let robust_address = rt.new_actor_address()?;

        // Allocate an ID for this actor.
        // Store mapping of pubkey or actor address to actor ID
        let id_address: Address = rt.transaction::<State, _, _>(|s, rt| {
            s.map_address_to_new_id(rt.store(), &robust_address)
                .map_err(|e| {
                    ActorError::new(ExitCode::ErrIllegalState, format!("exec failed {}", e))
                })
        })??;

        // Create an empty actor
        rt.create_actor(&params.code_cid, &id_address)?;

        // Invoke constructor
        rt.send(
            &id_address,
            METHOD_CONSTRUCTOR,
            &params.constructor_params,
            &rt.message().value().clone(),
        )
        .map_err(|err| rt.abort(err.exit_code(), "constructor failed"))?;

        Ok(ExecReturn {
            id_address,
            robust_address,
        })
    }
}

impl ActorCode for Actor {
    fn invoke_method<BS, RT>(
        &self,
        rt: &mut RT,
        method: MethodNum,
        params: &Serialized,
    ) -> Result<Serialized, ActorError>
    where
        BS: BlockStore,
        RT: Runtime<BS>,
    {
        match FromPrimitive::from_u64(method) {
            Some(Method::Constructor) => {
                Self::constructor(rt, params.deserialize()?)?;
                Ok(Serialized::default())
            }
            Some(Method::Exec) => {
                let res = Self::exec(rt, params.deserialize()?)?;
                Ok(Serialized::serialize(res)?)
            }
            _ => {
                // Method number does not match available, abort in runtime
                Err(rt.abort(ExitCode::SysErrInvalidMethod, "Invalid method"))
            }
        }
    }
}

fn can_exec(caller: &Cid, exec: &Cid) -> bool {
    // TODO spec also checks for an undefined Cid, see if this should be supported
    if exec == &*ACCOUNT_ACTOR_CODE_ID
        || exec == &*INIT_ACTOR_CODE_ID
        || exec == &*POWER_ACTOR_CODE_ID
        || exec == &*MARKET_ACTOR_CODE_ID
        || exec == &*MINER_ACTOR_CODE_ID
    {
        exec == &*MINER_ACTOR_CODE_ID && caller == &*POWER_ACTOR_CODE_ID
    } else {
        true
    }
}
