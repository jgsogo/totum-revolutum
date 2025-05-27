use super::OXRWrapper;
use crate::{Error, Result};
use finances_app_models::AppState as AppStateProto;
use tauri::async_runtime::Mutex;
use tauri::State;

#[tauri::command]
pub async fn get_fx_spot(
    state: State<'_, AppStateProto>,
    oxr_client: State<'_, Mutex<Option<OXRWrapper>>>,
    quoted_ccy: &str,
) -> Result<f32> {
    let mut oxr_client = oxr_client.lock().await;

    if let Some(ref mut client) = *oxr_client {
        let base_ccy = state.base_ccy()?.to_string();
        let fx_spot = client.fx_spot(&base_ccy, quoted_ccy).await?;
        Ok(fx_spot)
    } else {
        Err(Error::Other("Not OXR client available".to_string()))
    }
}
