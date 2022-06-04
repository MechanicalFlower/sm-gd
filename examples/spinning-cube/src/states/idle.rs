use gdnative::api::MeshInstance;
use gdnative::prelude::*;

use sm_gd::*;

use crate::{
    states::Spinning,
    resource::CubeResource,
};

#[derive(Debug)]
pub struct Idle;

impl GodotInitialState for Idle {}

impl GodotState for Idle {
    type Owner = MeshInstance;
    type Resource = CubeResource;

    fn ready(&self, owner: &Self::Owner, resource: &mut Self::Resource) {
        resource.color(owner);
    }

    fn input(&self, owner: &Self::Owner, resource: &mut Self::Resource, event: Ref<InputEvent>) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        let event = unsafe { event.assume_safe() };
        
        // Go to the Spinning state when user click on Enter
        if event.is_action_pressed("ui_accept", false, false) {
            return Some(Box::new(Spinning));
        }
        
        None
    }
}
