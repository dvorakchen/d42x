use std::sync::Arc;

use axum::extract::FromRef;
use tokio::sync::RwLock;

use crate::business::{
    category::{CategoryRepository, PanicCategoryRepo},
    meme::{MemeRepository, PanicMemeRepository},
    suggests::{PanicSuggestRepository, SuggestRepository},
};

#[derive(Clone)]
pub struct AppStates {
    pub meme_repo: MemeRepoSSType,
    pub cate_repo: CategoryRepoSSType,
    pub suggest_repo: SuggestRepoSSType,
}

impl FromRef<AppStates> for MemeRepoSSType {
    fn from_ref(input: &AppStates) -> Self {
        Arc::clone(&input.meme_repo)
    }
}

impl FromRef<AppStates> for CategoryRepoSSType {
    fn from_ref(input: &AppStates) -> Self {
        Arc::clone(&input.cate_repo)
    }
}

impl FromRef<AppStates> for SuggestRepoSSType {
    fn from_ref(input: &AppStates) -> Self {
        Arc::clone(&input.suggest_repo)
    }
}

pub type CategoryRepoSSType = Arc<RwLock<CategoryRepoSS>>;

pub trait IntoRepoSSType<T> {
    fn into_shared(self) -> T;
}

pub struct CategoryRepoSS {
    pub repo: Box<dyn CategoryRepository + 'static + Sync + Send>,
}

impl CategoryRepoSS {
    pub fn new(repo: impl CategoryRepository + 'static + Sync + Send) -> Self {
        Self {
            repo: Box::new(repo),
        }
    }

    pub fn non() -> Self {
        Self::new(PanicCategoryRepo)
    }
}

impl IntoRepoSSType<CategoryRepoSSType> for CategoryRepoSS {
    fn into_shared(self) -> CategoryRepoSSType {
        Arc::new(RwLock::new(self))
    }
}

pub type MemeRepoSSType = Arc<MemeRepoSS>;

pub struct MemeRepoSS {
    pub repo: Box<dyn MemeRepository + 'static + Sync + Send>,
}

impl MemeRepoSS {
    pub fn new(repo: impl MemeRepository + 'static + Sync + Send) -> Self {
        Self {
            repo: Box::new(repo),
        }
    }

    pub fn non() -> Self {
        Self::new(PanicMemeRepository)
    }
}

impl IntoRepoSSType<MemeRepoSSType> for MemeRepoSS {
    fn into_shared(self) -> MemeRepoSSType {
        Arc::new(self)
    }
}

pub type SuggestRepoSSType = Arc<SuggestRepoSS>;

pub struct SuggestRepoSS {
    pub repo: Box<dyn SuggestRepository + 'static + Sync + Send>,
}

impl SuggestRepoSS {
    pub fn new(repo: impl SuggestRepository + 'static + Sync + Send) -> Self {
        Self {
            repo: Box::new(repo),
        }
    }

    pub fn non() -> Self {
        Self::new(PanicSuggestRepository)
    }
}

impl IntoRepoSSType<SuggestRepoSSType> for SuggestRepoSS {
    fn into_shared(self) -> SuggestRepoSSType {
        Arc::new(self)
    }
}
