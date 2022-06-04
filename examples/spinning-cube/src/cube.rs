use gdnative::api::MeshInstance;
use gdnative::prelude::*;

use sm_gd::*;

use crate::{
    states::Idle,
    machine::CubeMachine,
    resource::CubeResource
};

#[derive(gdnative::derive::NativeClass)]
#[inherit(MeshInstance)]
pub struct Cube {
    #[property]
    cube_resource: Instance<CubeResource>,
    state_machine: CubeMachine,
}

#[methods]
impl Cube {
    fn new(_owner: &MeshInstance) -> Self {
        Cube {
            cube_resource: Instance::<CubeResource, Unique>::new().into_shared(),
            // Set first state to Idle
            state_machine: CubeMachine::new(Idle),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &MeshInstance) {
        owner.set_physics_process(true);

        // Initialize first state 
        let resource = unsafe { self.cube_resource.assume_safe() };
        resource.map_mut(|r, _o| self.state_machine.state.ready(owner, r)).unwrap();
    }

    #[export]
    fn _input(&mut self, owner: &MeshInstance, event: Ref<InputEvent>) {
        let resource = unsafe { self.cube_resource.assume_safe() };
        resource.map_mut(|r, _o| {
            self.state_machine.state.input(owner, r, event).map(|new_state| {
                // Update current state
                self.state_machine.state(new_state);
                self.state_machine.state.init(owner, r)
            })
        }).unwrap();
    }

    #[export]
    fn _physics_process(&mut self, owner: &MeshInstance, delta: f32) {
        let resource = unsafe { self.cube_resource.assume_safe() };
        resource.map_mut(|r, _o| {
            self.state_machine.state.physics_update(owner, r, delta).map(|new_state| {
                // Update current state
                self.state_machine.state(new_state);
                self.state_machine.state.init(owner, r)
            })
        }).unwrap();
    }
}
