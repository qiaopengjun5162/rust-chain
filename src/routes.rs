use axum::{
    Extension, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use ethers::{
    contract::ContractError,
    providers::{Http, Middleware, Provider, ProviderError},
};
use thiserror::Error;
use tracing::info;

use crate::counter::Counter;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(": ContractError {0}")]
    ContractError(#[from] ContractError<Provider<Http>>),
    #[error(": ProviderError {0}")]
    ProviderError(#[from] ProviderError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = match self {
            Self::ContractError(e) => format!("Contract Error: {}", e),
            Self::ProviderError(e) => format!("Provider Error: {}", e),
        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

pub async fn handle_number(
    Extension(counter): Extension<Counter>,
) -> Result<Json<String>, ApiError> {
    info!("Getting number");

    let number = counter.get_number().await?;
    Ok(Json(number.to_string()))
}

pub async fn handle_block_number(
    Extension(counter): Extension<Counter>,
) -> Result<Json<String>, ApiError> {
    info!("Getting block number");
    let block_number: u64 = counter.client.get_block_number().await?.as_u64();

    Ok(Json(block_number.to_string()))
}
