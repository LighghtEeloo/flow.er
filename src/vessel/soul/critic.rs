use crate::util::*;


#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Critic {
    FlowNodeNotFoundError,
    FlowNodeExistError,
    FlowNodeMismatchError,
}

pub use Critic::*;

