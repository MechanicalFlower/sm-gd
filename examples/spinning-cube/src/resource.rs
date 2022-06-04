use gdnative::api::{MeshInstance, Resource};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Resource)]
pub struct CubeResource {
    pub time: f32,
    #[property(default = 0.05)]
    pub rotate_speed: f32,
} 

#[methods]
impl CubeResource {
    pub fn new(_owner: &Resource) -> Self {
        CubeResource {
            time: 0.0,
            rotate_speed: 0.05,
        }
    }

    pub fn color(&self, owner: &MeshInstance) {
        use gdnative::api::SpatialMaterial;

        let rotation = owner.rotation();
        if let Some(mat) = owner.get_surface_material(0) {
            let mat = unsafe { mat.assume_safe() };
            let mat = mat.cast::<SpatialMaterial>().expect("Incorrect material");
            mat.set_albedo(Color::from_rgba(rotation.x.abs(), rotation.y.abs(), rotation.z.abs(), 1.0));
        }
    }
}