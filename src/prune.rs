

pub trait Prunable {
    type Pruned;

    fn prune(Self) -> Self::Pruned;
}
