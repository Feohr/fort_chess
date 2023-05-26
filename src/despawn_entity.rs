//! despawn entity module.
//!
//! Holds the `DespawnEntity` trait.
/*████Constants and Declarations█████████████████████████████████████████████████████████████████*/

use bevy::prelude::{Entity, Commands, Query, With, Component, DespawnRecursiveExt};

/// To despawn entities with a a specific component.
pub(crate) trait DespawnEntity<T>
    where
        T: Component,
{
    fn despawn_entity(&mut self, query: &Query<Entity, With<T>>);
}

/*████Functions██████████████████████████████████████████████████████████████████████████████████*/

impl<T> DespawnEntity<T> for Commands<'_,'_> 
    where
        T: Component,
{
    /// To despawn entities recursively whenever called.
    #[inline]
    fn despawn_entity(
        &mut self,
        query:      &Query<Entity, With<T>>,
    ) {
        query.iter().for_each(|entity| self.entity(entity).despawn_recursive());
    }
}
