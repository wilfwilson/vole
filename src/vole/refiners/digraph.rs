use super::super::state::State;
use super::Refiner;
use crate::perm::Permutation;
use crate::vole::trace;
use crate::{datastructures::digraph::Digraph, vole::backtracking::Backtrack};



pub struct DigraphStabilizer {
    digraph: Digraph,
}

impl DigraphStabilizer {
    pub fn new(digraph: Digraph) -> DigraphStabilizer {
        DigraphStabilizer { digraph }
    }
}

impl<T: State> Refiner<T> for DigraphStabilizer {
    fn name(&self) -> String {
        format!("DigraphStabilizer of {:?}", self.digraph)
    }

    fn check(&self, p: &Permutation) -> bool {
        (&self.digraph) ^ p == self.digraph
    }

    fn refine_begin(&mut self, _state: &mut T) -> trace::Result<()> {
        //  state.refine_partition_by(|x| self.digraph.contains(x))?;
        Ok(())
    }
}

impl Backtrack for DigraphStabilizer {
    fn bt_save(&mut self) {}
    fn bt_revert(&mut self) {}
}