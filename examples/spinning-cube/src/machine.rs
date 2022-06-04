use gdnative::api::MeshInstance;

use sm_gd::*;

use crate::resource::CubeResource;

// Create a new `CubeMachine` struct, which will be our state machine
// object. It keeps track of the `State`.
#[derive(Debug)]
pub struct CubeMachine {
    pub state: Box<dyn GodotStateTraits<Owner = MeshInstance, Resource = CubeResource>>,
}

// Implement the `Machine` trait, allowing us to update the current `state().
impl GodotMachine for CubeMachine {
    type Owner = MeshInstance;
    type Resource = CubeResource;

    fn state(&mut self, state: Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>) {
        self.state = state
    }
}

// Implement the `Initializer` trait, to allow a new machine to be initialised
// using the `new` associated function, given a valid state marked with the
// `GodotInitialState` marker trait.
impl GodotInitializer for CubeMachine {
    type Owner = MeshInstance;
    type Resource = CubeResource;
    
    fn new(state: impl GodotInitialState<Owner = MeshInstance, Resource = CubeResource>) -> Self {
        CubeMachine { state: Box::new(state) }
    }
}