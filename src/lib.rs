use crate::plugin::{create_timer, get_mod_dir, log_message, start_timer, Events, LOG_INFO};
use crate::server::main_http_thread;
use crate::server_info::ServerInfoHandle;

use std::os::raw::{c_int, c_ulonglong, c_ushort};
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use tokio::runtime::Runtime;
use warp::Future;

mod plugin;
mod server;
mod server_info;

#[derive(Clone)]
struct ServerHandle(Arc<RwLock<Server>>, Rc<RwLock<Runtime>>);

extern "C" fn tick() -> c_ulonglong {
    let server_handle: &mut ServerHandle = unsafe { EVENTS_INSTANCE.as_mut() }.unwrap();
    server_handle.clone().with(|server| {
        let timer = server.timer;
        server.tick += 1;
        server_handle.block_on(async {
            server.info.update_players(server.tick % 20 == 0).await;
        });

        server.tick %= 1000;

        start_timer(timer)
    });

    0
}

impl ServerHandle {
    fn with<O>(&self, mut block: impl FnMut(&mut Server) -> O) -> O {
        let mut guard = self.0.write().unwrap();
        block(&mut guard)
    }

    fn block_on<F: Future>(&mut self, future: F) -> F::Output {
        self.1.write().unwrap().block_on(future)
    }
}

#[derive(Clone, Debug)]
struct Server {
    info: ServerInfoHandle,
    timer: c_int,
    tick: u64,
}

impl Server {
    fn into_handle(self, runtime: Rc<RwLock<Runtime>>) -> ServerHandle {
        ServerHandle(Arc::new(RwLock::new(self)), runtime)
    }
}

impl Events for ServerHandle {
    fn new() -> Self {
        Server {
            info: ServerInfoHandle::new(),
            timer: -1,
            tick: 0,
        }
        .into_handle(Rc::new(RwLock::new(
            Runtime::new().expect("Failed to create Tokio runtime"),
        )))
    }

    fn on_any(&mut self, event_name: &str) {
        log_message(
            plugin::LOG_VERBOSE,
            format!("Got event: {}", event_name).as_str(),
        )
    }

    fn on_gui_action(&mut self, player_id: u16, message_box_id: i32, data: Option<&str>) {
        self.clone().with(|server| {
            self.block_on(async {
                server
                    .info
                    .gui_action(player_id, message_box_id, data)
                    .await;
            });
        });
    }

    fn on_player_connect(&mut self, player_id: c_ushort) {
        self.clone().with(|server| -> () {
            self.block_on(async {
                server.info.add_player(player_id).await;
            });
        })
    }

    fn on_player_disconnect(&mut self, player_id: c_ushort) {
        self.clone().with(|server| {
            self.block_on(async {
                server.info.remove_player(player_id).await;
            })
        })
    }

    fn on_server_init(&mut self) {
        log_message(
            LOG_INFO,
            format!(
                concat!(
                    "Loaded ",
                    env!("CARGO_PKG_NAME"),
                    " ",
                    env!("CARGO_PKG_VERSION"),
                    " (pwd: {:?}, mod_dir: {})"
                ),
                std::env::current_dir(),
                get_mod_dir()
            )
            .as_str(),
        );
    }

    fn on_server_post_init(&mut self) {
        let mut info = {
            self.with(|server| {
                server.timer = create_timer(tick, 50);
                log_message(
                    LOG_INFO,
                    format!("nwahttp tick timer registered with id {}", server.timer).as_str(),
                );
                start_timer(server.timer);
                server.info.clone()
            })
        };

        std::thread::spawn(move || {
            let info_clone = info.clone();
            info.block_on(async { main_http_thread(info_clone).await });
        });
        log_message(LOG_INFO, "Started HTTP thread");
    }
}

use_events!(ServerHandle);
