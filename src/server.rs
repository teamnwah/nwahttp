use crate::plugin::*;
use crate::server_info::{Player, ServerInfoHandle};
use hyper::{header::CONTENT_TYPE, Body, Response};
use lazy_static::lazy_static;
use prometheus::{Encoder, TextEncoder};
use std::{net::SocketAddr, str::FromStr};
use warp;
use warp::ws::Ws;
use warp::{filters::path::end, Filter};

lazy_static! {
    static ref SERVER_VERSION: String = get_server_version();
}

fn get_info() -> String {
    format!(
        concat!(
            "server: tes3mp {}\n",
            concat!(
                "plugin: ",
                env!("CARGO_PKG_NAME"),
                " ",
                env!("CARGO_PKG_VERSION"),
                "\n"
            ),
            "about:\n",
            "  - https://github.com/TES3MP/openmw-tes3mp\n",
            "  - https://github.com/teamnwah/nwahttp\n",
        ),
        *SERVER_VERSION
    )
}

async fn list_players(info: ServerInfoHandle) -> Vec<Player> {
    info.get_players().await
}

pub async fn main_http_thread(info: ServerInfoHandle) {
    let fs = warp::fs::dir(get_mod_dir() + "/../www");

    let index = warp::path("info").and(warp::path::end()).map(|| get_info());

    let player_info = info.clone();
    let players = warp::path("api")
        .and(warp::path("players"))
        .and(end())
        .and_then(move || {
            let player_info = player_info.clone();
            async move {
                Ok(warp::reply::json(&list_players(player_info.clone()).await))
                    as Result<_, warp::Rejection>
            }
        });

    let server_info = info.clone();
    let player_websocket = warp::path("ws")
        .and(warp::path("players"))
        .and(warp::path::end())
        .and(warp::ws())
        .map(move |ws: Ws| {
            let server_info = server_info.clone();
            ws.on_upgrade(move |webs| {
                let server_info = server_info.clone();
                async move {
                    let server_info = server_info.clone();
                    server_info.add_websocket(webs).await;
                }
            })
        });

    let metrics_endpoint = warp::path("metrics").and(warp::path::end()).map(move || {
        let encoder = TextEncoder::new();
        let metric_families = prometheus::gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer).unwrap();
        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, encoder.format_type())
            .body(Body::from(buffer))
            .unwrap()
    });

    let endpoint = warp::get().and(
        index
            .or(players)
            .or(player_websocket)
            .or(metrics_endpoint)
            .or(fs),
    );

    warp::serve(endpoint)
        .run(SocketAddr::from_str("[::]:8787").expect("Invalid listen argument"))
        .await
}
