use std::fmt::Debug;

use tracing::info;

use super::state::State;

/// Method of choosing which cell (ignoring cells of size 1) to branch on.
/// All techniques use 'earliest cell' as the final tie-breaking strategy
pub enum Selector {
    /// Smallest cell
    Smallest,
    /// Largest cell
    Largest,
    /// First cell
    First,
    /// Cell where the values are non-trivially connected to the most other cells
    MostConnected,
    /// Smallest cell, breaking ties by choosing MostConnected
    SmallestMostConnected,
}

fn find_best_cell<F: Copy, T: Ord + Debug>(state: &State, func: F) -> usize
where
    F: Fn(&State, usize) -> T,
{
    *state
        .domain
        .partition()
        .base_cells() // Get cells to branch on
        .iter()
        .filter(|&&i| state.domain.partition().cell(i).len() > 1)
        .min_by_key(|&&value| func(state, value))
        .unwrap()
}

pub fn select_branching_cell(state: &State) -> usize {
    let choice = Selector::First;
    let cell = match choice {
        Selector::Smallest => find_best_cell(state, |s, i| s.domain.partition().cell(i).len()),
        Selector::Largest => {
            find_best_cell(state, |s, i| -(s.domain.partition().cell(i).len() as isize))
        }
        Selector::First => find_best_cell(state, |_, _| 0),
        _ => unimplemented!(),
    };

    info!(
        "Choosing to branch on cell {:?} from {:?}",
        cell,
        state.domain.partition().extended_as_list_set()
    );
    cell
}
