use axum::Server;
use std::net::SocketAddr;

use crate::{error::{Result, MulibError}, router::create_router};

pub async fn start_server() -> Result<()> {
    let router = create_router().await?;
    let socket_addr: SocketAddr = "0.0.0.0:3000".parse().map_err(|_| {MulibError::ServerError})?;
    Server::bind(&socket_addr)
            .serve(router.into_make_service())
            .await.map_err(|_| { MulibError::ServerError })?;
    Ok(())
}
