use std::{fmt::Debug, hash::Hash};
pub trait Identity: Default + Debug + Clone + Hash + PartialEq + Eq {}
