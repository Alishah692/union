use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use ibc_union_light_client::{
    msg::{InstantiateMsg, QueryMsg},
    IbcClientError,
};

use crate::client::TendermintLightClient;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, IbcClientError<TendermintLightClient>> {
    ibc_union_light_client::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    ibc_union_light_client::query::<TendermintLightClient>(deps, env, msg).map_err(Into::into)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MigrateMsg {}

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
