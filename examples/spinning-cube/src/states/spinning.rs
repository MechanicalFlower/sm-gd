use gdnative::api::MeshInstance;
use gdnative::prelude::*;

use sm_gd::*;

use crate::{
    states::Idle,
    resource::CubeResource,
};

#[derive(Debug)]
pub struct Spinning;

impl GodotState for Spinning {
    type Owner = MeshInstance;
    type Resource = CubeResource;

    fn init(&self, owner: &Self::Owner, resource: &mut Self::Resource) {}

    fn ready(&self, owner: &Self::Owner, resource: &mut Self::Resource) {}

    fn input(&self, owner: &Self::Owner, resource: &mut Self::Resource, event: Ref<InputEvent>) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        None
    }

    fn update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        None
    }

    fn physics_update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>> {
        // Update the mesh materal color
        resource.time += delta;
        resource.color(owner);

        // Rotate the cube
        let rotation = owner.rotation();
        let old_sign = rotation.y.signum();

        owner.rotate_y((resource.rotate_speed * delta).into());

        let rotation = owner.rotation();
        let new_sign = rotation.y.signum();

        // If the cube has rotated by half we go back to the Idle state
        if new_sign != old_sign {
            return Some(Box::new(Idle));
        }

        None
    }

    fn integrate_forces(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) {}
}
