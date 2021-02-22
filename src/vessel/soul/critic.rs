use crate::util::*;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Critic {
    FlowNodeNotFoundError,
    FlowNodeExistError
}

pub use Critic::*;

