use serde::{Deserialize, Serialize};
#[derive(Default, Debug, Clone, Deserialize, Serialize)]
pub struct Stats {
    pub rbase_nodes: usize,
    pub search_nodes: usize,
    pub bad_iso: usize,
    pub good_iso: usize,
    pub bad_canonical: usize,
    pub refiner_calls: usize,
}
