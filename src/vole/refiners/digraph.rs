use std::sync::Arc;

use super::super::state::State;
use super::Refiner;
use crate::perm::Permutation;
use crate::vole::trace;
use crate::{datastructures::digraph::Digraph, vole::backtracking::Backtrack};

pub struct DigraphTransporter {
    digraph_left: Arc<Digraph>,
    digraph_right: Arc<Digraph>,
}

impl DigraphTransporter {
    pub fn new_stabilizer(digraph: Arc<Digraph>) -> Self {
        Self::new_transporter(digraph.clone(), digraph)
    }

    pub fn new_transporter(digraph_left: Arc<Digraph>, digraph_right: Arc<Digraph>) -> Self {
        Self {
            digraph_left,
            digraph_right,
        }
    }
}

impl Refiner for DigraphTransporter {
    fn name(&self) -> String {
        if self.is_group() {
            format!("DigraphTransporter of {:?}", self.digraph_left)
        } else {
            format!(
                "DigraphTransporter of {:?} -> {:?}",
                self.digraph_left, self.digraph_right
            )
        }
    }

    fn check(&self, p: &Permutation) -> bool {
        &(*self.digraph_left) ^ p == *self.digraph_right
    }

    fn refine_begin_left(&mut self, state: &mut State) -> trace::Result<()> {
        state.add_arc_graph(&self.digraph_left);
        Ok(())
    }

    fn refine_begin_right(&mut self, state: &mut State) -> trace::Result<()> {
        state.add_arc_graph(&self.digraph_right);
        Ok(())
    }

    fn is_group(&self) -> bool {
        Arc::ptr_eq(&self.digraph_left, &self.digraph_right)
    }
}

impl Backtrack for DigraphTransporter {
    fn save_state(&mut self) {}
    fn restore_state(&mut self) {}
}
