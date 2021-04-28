mod join;
mod pow;

pub use super::Permutation;
use join::Join;
use pow::Pow;

pub trait PermBuilder: Clone {
    /// Builds from a Vec
    fn lazy_from_vec(values: Vec<usize>) -> Permutation {
        Permutation::from_vec(values)
    }

    /// Computes f(x)
    fn build_apply(&self, x: usize) -> usize;

    /// TODO: What is more efficient (a * b)^-1 or (b^-1 * a^-1)? If the latter join can benefit from a
    /// ad hominem implementation
    fn build_inv(&self) -> Permutation {
        self.collapse().inv()
    }

    /// Computes g^x
    fn build_pow(&self, x: isize) -> Pow<Self> {
        Pow::new(self.clone(), x)
    }

    /// Computes g / f = g * f^-1. Note that here an alternative is to compute .inv() directly but since
    /// Most others operations are lazy I prefer this approach a bit.
    fn build_divide<InPerm: PermBuilder>(&self, other: &InPerm) -> Join<Self, Pow<InPerm>> {
        Join::new(self.clone(), Pow::new(other.clone(), -1))
    }

    /// Very general type that allows to join permutations depending on how efficient we want them
    fn build_multiply<InPerm: PermBuilder>(&self, next: &InPerm) -> Join<Self, InPerm> {
        Join::new(self.clone(), next.clone())
    }

    /// Unfold all the layers and make a single permutation (Note, often it will be wanted to store this)
    fn collapse(&self) -> Permutation;
}