pub mod middlewares;
pub mod shared_data;

use crate::controllers::{
    admin::{
        change_password, check_logged_in, delete_meme, list_memes, log_in, post_memes,
        update_categories,
    },
    client::{
        interaction::{get_interactions, like_increase, unlike_increase},
        ui::{create_suggest, get_categories, get_paginated_memes, meme_detail},
    },
};
use axum::{
    Router,
    http::{
        HeaderName, HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    middleware,
    routing::{delete, get, post, put},
};
use middlewares::{CipherLayer, jwt_auth_middleware};
use shared_data::{
    AccountRepoSS, AccountRepoSSType, AppStates, CategoryRepoSS, CategoryRepoSSType,
    IntoRepoSSType, MemeRepoSS, MemeRepoSSType, SuggestRepoSS, SuggestRepoSSType,
};
use soft_aes::aes::AES_BLOCK_SIZE;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{debug, info};

pub struct App {
    listener: TcpListener,
    router: Router,
}

impl App {
    pub async fn run(self) {
        axum::serve(self.listener, self.router).await.unwrap();
    }
}
pub struct AppBuilder {
    address: String,
    cors: String,
    account_repo: Option<AccountRepoSSType>,
    category_repo: Option<CategoryRepoSSType>,
    meme_repo: Option<MemeRepoSSType>,
    suggest_repo: Option<SuggestRepoSSType>,
    aes_key: String,
    aes_iv: [u8; AES_BLOCK_SIZE],
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            address: String::new(),
            cors: String::new(),
            account_repo: None,
            category_repo: None,
            meme_repo: None,
            suggest_repo: None,
            aes_key: String::new(),
            aes_iv: [0; 16],
        }
    }

    pub fn address(mut self, address: String) -> Self {
        self.address = address;
        self
    }

    pub fn cors(mut self, cors: String) -> Self {
        self.cors = cors;
        self
    }

    pub fn account_repo(mut self, repo: impl IntoRepoSSType<AccountRepoSSType>) -> Self {
        self.account_repo = Some(repo.into_shared());
        self
    }

    pub fn category_repo(mut self, repo: impl IntoRepoSSType<CategoryRepoSSType>) -> Self {
        self.category_repo = Some(repo.into_shared());
        self
    }

    pub fn meme_repo(mut self, repo: impl IntoRepoSSType<MemeRepoSSType>) -> Self {
        self.meme_repo = Some(repo.into_shared());
        self
    }

    pub fn suggest_repo(mut self, repo: impl IntoRepoSSType<SuggestRepoSSType>) -> Self {
        self.suggest_repo = Some(repo.into_shared());
        self
    }

    pub fn aes_key(mut self, aes_key: String) -> Self {
        self.aes_key = aes_key;
        self
    }

    pub fn aes_iv(mut self, aes_iv: [u8; AES_BLOCK_SIZE]) -> Self {
        self.aes_iv = aes_iv;
        self
    }

    pub async fn build(mut self) -> App {
        let listener = self.get_bind_listener().await;

        let app_state = self.build_state();

        let api_routes = Router::new()
            .nest(
                "/admin",
                Router::new()
                    .route("/check-logged-in", get(check_logged_in))
                    .route("/login", post(log_in))
                    .route("/change-password", put(change_password))
                    .route("/categories", get(get_categories))
                    .route("/categories/{meme_id}", put(update_categories))
                    .route("/post-memes", post(post_memes))
                    .route("/memes", get(list_memes))
                    .route("/memes/{id}", delete(delete_meme)),
            )
            .nest(
                "/client",
                Router::new()
                    .route("/categories", get(get_categories))
                    .route("/memes", get(get_paginated_memes))
                    .route("/memes/interactions", post(get_interactions))
                    .route("/memes/{id}", get(meme_detail))
                    .route("/memes/{id}/like", put(like_increase))
                    .route("/memes/{id}/unlike", put(unlike_increase))
                    .route("/suggests", post(create_suggest)),
            )
            .with_state(app_state.clone());
        // .with_state(meme_repo);

        let router = Router::new()
            // .route("/", get(home))
            .nest("/api", api_routes)
            .nest_service(
                "/assets",
                tower_http::services::ServeDir::new("wwwroot/assets"),
            )
            .nest_service(
                "/favicon.ico",
                tower_http::services::ServeDir::new("wwwroot/favicon.ico"),
            );

        let cors_layer = self.build_cors();

        // setup middlewares
        let router = router.layer(
            ServiceBuilder::new()
                .layer(cors_layer)
                // .layer(middleware::from_fn(crate::middleware::cipher_middleware))
                .layer(CipherLayer::new(self.aes_key.clone(), self.aes_iv.clone()))
                .layer(middleware::from_fn(jwt_auth_middleware)),
        );

        axum::serve(listener, router).await.unwrap();

        todo!()
        // App { listener, router }
    }

    fn build_state(&mut self) -> AppStates {
        let acc_repo = if let Some(acc_repo) = self.account_repo.take() {
            acc_repo
        } else {
            AccountRepoSS::non().into_shared()
        };

        let cate_repo = if let Some(cate_repo) = self.category_repo.take() {
            cate_repo
        } else {
            CategoryRepoSS::non().into_shared()
        };

        let meme_repo = if let Some(meme_repo) = self.meme_repo.take() {
            meme_repo
        } else {
            MemeRepoSS::non().into_shared()
        };

        let suggest_repo = if let Some(suggest_repo) = self.suggest_repo.take() {
            suggest_repo
        } else {
            SuggestRepoSS::non().into_shared()
        };

        AppStates {
            account_repo: acc_repo,
            cate_repo,
            meme_repo,
            suggest_repo,
        }
    }

    fn build_cors(&self) -> CorsLayer {
        info!("Cors allow origins: {}", self.cors);
        const HOST_SEPARATE: &str = ";";
        const ALL_ORIGINS: &str = "*";

        let allow_orgin = if self.cors == ALL_ORIGINS {
            debug!("cors allow any");
            AllowOrigin::any()
        } else {
            let hosts: Vec<_> = self
                .cors
                .split(HOST_SEPARATE)
                .map(|host| HeaderValue::from_str(host).unwrap())
                .collect();
            AllowOrigin::list(hosts)
        };

        CorsLayer::new()
            .allow_origin(allow_orgin)
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([
                AUTHORIZATION,
                CONTENT_TYPE,
                HeaderName::from_lowercase(b"x-date").unwrap(),
            ])
    }

    #[cfg(debug_assertions)]
    async fn get_bind_listener(&self) -> TcpListener {
        debug!("Debug environment");

        use listenfd::ListenFd;
        use tokio::net::TcpListener;
        let mut listenfd = ListenFd::from_env();
        match listenfd.take_tcp_listener(0).unwrap() {
            // if we are given a tcp listener on listen fd 0, we use that one
            Some(listener) => {
                debug!("Hot Reloading");
                listener.set_nonblocking(true).unwrap();
                TcpListener::from_std(listener).unwrap()
            }
            // otherwise fall back to local listening
            None => TcpListener::bind(self.address.clone()).await.unwrap(),
        }
    }

    #[cfg(not(debug_assertions))]
    async fn get_bind_listener(&self) -> TcpListener {
        TcpListener::bind(self.address.clone()).await.unwrap()
    }
}
