use std::default::Default;

use axum::{
    http::{header::AUTHORIZATION, HeaderName, HeaderValue},
    middleware,
    routing::post,
    Router,
};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{debug, info};

use crate::controllers::admin::log_in;

pub struct App {
    address: String,
    cors: String,
}

impl App {
    pub async fn run(self) {
        let listener = Self::bind_listener(&self.address).await;

        let api_routes = Router::new()
            .nest("/admin", Self::admin_routes())
            .nest("/client", Self::client_routes());
        let app = Router::new()
            .nest("/api", api_routes)
            .nest_service(
                "/admin-view",
                tower_http::services::ServeDir::new("admin-view"),
            )
            .layer(self.cors())
            // .layer(Extension(Self::init_db().await.expect("init db error")))
            .layer(middleware::from_fn(crate::middleware::cipher_middleware));

        axum::serve(listener, app).await.unwrap();
    }

    fn client_routes() -> Router {
        Router::new()
    }

    fn admin_routes() -> Router {
        Router::new().route("/login", post(log_in))
    }

    fn cors(&self) -> CorsLayer {
        info!("Cors allow origins: {}", self.cors);

        const ALL_ORIGINS: &str = "*";
        let allow_orgin = if self.cors == ALL_ORIGINS {
            debug!("cors allow any");
            AllowOrigin::any()
        } else {
            AllowOrigin::from(self.cors.clone().parse::<HeaderValue>().unwrap())
        };

        CorsLayer::new().allow_origin(allow_orgin).allow_headers([
            AUTHORIZATION,
            HeaderName::from_lowercase(b"x-date").unwrap(),
        ])
    }

    #[cfg(debug_assertions)]
    async fn bind_listener(address: &String) -> TcpListener {
        debug!("Debug environment");

        use listenfd::ListenFd;
        let mut listenfd = ListenFd::from_env();
        match listenfd.take_tcp_listener(0).unwrap() {
            // if we are given a tcp listener on listen fd 0, we use that one
            Some(listener) => {
                debug!("Hot Reloading");
                listener.set_nonblocking(true).unwrap();
                TcpListener::from_std(listener).unwrap()
            }
            // otherwise fall back to local listening
            None => TcpListener::bind(address).await.unwrap(),
        }
    }

    #[cfg(not(debug_assertions))]
    async fn bind_listener(address: &String) -> TcpListener {
        TcpListener::bind(address).await.unwrap()
    }
}

#[derive(Default)]
pub struct AppBuilder {
    address: String,
    cors: String,
}

impl AppBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_address(&mut self, address: String) -> &mut Self {
        self.address = address;
        self
    }

    pub fn set_cors(&mut self, cors: String) -> &mut Self {
        self.cors = cors;
        self
    }

    pub fn build(self) -> App {
        App {
            address: self.address,
            cors: self.cors,
        }
    }
}
