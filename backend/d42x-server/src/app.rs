use std::default::Default;

use axum::{
    Router,
    http::{HeaderName, HeaderValue, Method, header::AUTHORIZATION},
    middleware,
    routing::{get, post, put},
};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{debug, info};

use crate::controllers::{
    admin::{change_password, list_memes, log_in, post_memes},
    client::ui::{get_categories, home},
};

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
        let mut app = Router::new()
            .route("/", get(home))
            .nest("/api", api_routes)
            .nest_service("/wwwroot", tower_http::services::ServeDir::new("wwwroot"))
            .nest_service(
                "/favicon.ico",
                tower_http::services::ServeDir::new("wwwroot/favicon.ico"),
            );

        app = Self::custom_middleware(app).layer(self.cors());

        axum::serve(listener, app).await.unwrap();
    }

    fn client_routes() -> Router {
        Router::new().route("/categories", get(get_categories))
    }

    fn admin_routes() -> Router {
        Router::new()
            .route("/login", post(log_in))
            .route("/change-password", put(change_password))
            .route("/post-memes", post(post_memes))
            .route("/memes", get(list_memes))
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

        CorsLayer::new()
            .allow_origin(allow_orgin)
            .allow_methods([Method::GET, Method::POST, Method::PUT])
            .allow_headers([
                AUTHORIZATION,
                HeaderName::from_lowercase(b"x-date").unwrap(),
            ])
    }

    fn custom_middleware(router: Router) -> Router {
        router
            .layer(middleware::from_fn(crate::middleware::jwt_auth_middleware))
            .layer(middleware::from_fn(crate::middleware::cipher_middleware))
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
