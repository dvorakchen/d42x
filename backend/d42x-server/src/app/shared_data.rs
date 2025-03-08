use std::sync::Arc;

use crate::business::category::{CategoryRepository, PanicCategoryRepo};

pub type CategoryRepoSSType = Arc<CategoryRepoSS>;

pub struct CategoryRepoSS {
    pub repo: Box<dyn CategoryRepository + 'static + Sync + Send>,
}

impl CategoryRepoSS {
    pub fn new_ext(
        repo: Box<dyn CategoryRepository + 'static + Sync + Send>,
    ) -> CategoryRepoSS {
        Self { repo }
    }

    pub fn non() -> CategoryRepoSS {
        Self::new_ext(Box::new(PanicCategoryRepo))
    }

    pub fn into(self) -> Arc<Self> {
        Arc::new(self)
    }
}
