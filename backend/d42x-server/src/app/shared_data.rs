use std::sync::Arc;

use crate::business::{
    category::{CategoryRepository, PanicCategoryRepo},
    meme::{MemeRepository, PanicMemeRepository},
};

pub type CategoryRepoSSType = Arc<CategoryRepoSS>;

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
        Arc::new(self)
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
