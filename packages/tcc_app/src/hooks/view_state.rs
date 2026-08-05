//! View state hooks

use freya::prelude::*;
use freya::router::RouterContext;
use std::collections::HashMap;

pub struct PersistedView {
    pub name: String,
}

pub fn use_view_state() -> PersistedView {
    PersistedView {
        name: "default".to_string(),
    }
}

pub mod state {
    use freya::query::{MutationStateData, UseMutation, UseQuery};

    pub fn settled_or_loading<Q: freya::query::QueryCapability>(
        query: &UseQuery<Q>,
    ) -> Option<Q::Ok> {
        match &*query.read().state() {
            freya::query::QueryStateData::Settled { res: Ok(data), .. } => Some(data.clone()),
            freya::query::QueryStateData::Loading { res: Some(data), .. } => Some(data.clone()),
            _ => None,
        }
    }
}