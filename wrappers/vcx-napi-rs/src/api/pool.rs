use libvcx_core::api_vcx::api_global::pool;
use libvcx_core::aries_vcx::indy::ledger::pool::PoolConfig;
use libvcx_core::errors::error::{LibvcxError, LibvcxErrorKind};
use libvcx_core::serde_json;
use napi_derive::napi;

use crate::error::to_napi_err;

// implement fn open_main_pool and close_main_pool using  layer functions, make sure the function
// is async if the respective  layer is async
#[napi]
async fn open_main_pool(pool_config: String) -> napi::Result<()> {
    let pool_config = serde_json::from_str::<PoolConfig>(&pool_config)
        .map_err(|err| LibvcxError::from_msg(LibvcxErrorKind::InvalidJson, format!("Serialization error: {:?}", err)))
        .map_err(to_napi_err)?;
    pool::open_main_pool(&pool_config).await.map_err(to_napi_err)?;
    Ok(())
}

#[napi]
async fn close_main_pool() -> napi::Result<()> {
    pool::close_main_pool().await.map_err(to_napi_err)
}
