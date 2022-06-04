use gdnative::prelude::*;

pub trait GodotStateTraits:
    GodotState + Sync + Send + std::any::Any + std::fmt::Debug
{
    /// Returns `self` as `&mut dyn Any`
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// State is a virtual base class that allows structs to be
/// used as states in a state machine.
pub trait GodotState: std::fmt::Debug {
    type Owner: GodotObject;
    type Resource: NativeClass;

    /// Virtual function. Called by the state machine upon changing the active state.
    fn init(&self, owner: &Self::Owner, resource: &mut Self::Resource);

    /// Virtual function. Corresponds to the `_ready()` callback.
    fn ready(&self, owner: &Self::Owner, resource: &mut Self::Resource);

    /// Virtual function. Corresponds to the `_input()` callback.
    fn input(&self, owner: &Self::Owner, resource: &mut Self::Resource, event: Ref<InputEvent>) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>>;

    /// Virtual function. Corresponds to the `_process()` callback.
    fn update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>>;
    
    /// Virtual function. Corresponds to the `_physics_process()` callback.
    fn physics_update(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32) -> Option<Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>>;
    
    /// Virtual function. Corresponds to the `_integrate_forces()` callback.
    fn integrate_forces(&self, owner: &Self::Owner, resource: &mut Self::Resource, delta: f32);
}

impl<T> GodotStateTraits for T
where
    T: GodotState + std::any::Any + Sync + Send + std::fmt::Debug,
{
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// InitialState is a custom marker trait that allows a state to be used as
/// the initial state in a state machine. This trait is a superset of the
/// `GodotStateTraits` trait.
pub trait GodotInitialState: GodotStateTraits {}

/// Machine provides the method required to query a state machine for its
/// current state.
pub trait GodotMachine: std::fmt::Debug {
    type Owner: GodotObject;
    type Resource: NativeClass;

    /// state allows you to update the current state of the state machine.
    fn state(&mut self, state: Box<dyn GodotStateTraits<Owner = Self::Owner, Resource = Self::Resource>>);
}

/// Initializer defines the `new` method on a machine, that accepts any state
/// marked as `GodotInitialState`, and returns a new machine.
pub trait GodotInitializer {
    type Owner: GodotObject;
    type Resource: NativeClass;

    /// new initialises a new machine, based on the provided `GodotInitialState` as
    /// input.
    fn new(state: impl GodotInitialState<Owner = Self::Owner, Resource = Self::Resource>) -> Self;
}