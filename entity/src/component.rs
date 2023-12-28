use crate::entity::EntityId;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt::Debug;

/// Due to current language restrictions, components cannot contain non-static lifetimes.
pub trait Component: 'static {}

pub struct Storage<T: Component> {
    /// Maps entity IDs to indices in the data vector.
    map: HashMap<EntityId, usize>,
    /// Maps indices in the vector to entity ID.
    reverse_map: Vec<EntityId>,
    /// Raw components.
    /// These are stored next to each other in a vector to improve performance.
    data: Vec<T>,
}

impl<T: Component + 'static> Storage<T> {
    /// Creates a new storage and stores the given component in it.
    pub fn with(entity: EntityId, component: T) -> Box<dyn TypelessStorage> {
        Box::new(Self {
            map: HashMap::from([(entity, 0)]),
            reverse_map: vec![entity],
            data: vec![component],
        })
    }

    pub fn insert(&mut self, entity: EntityId, component: T) -> Option<T> {
        if let Some(index) = self.map.get(&entity) {
            // Replace component with new one.
            Some(std::mem::replace(&mut self.data[*index], component))
        } else {
            let index = self.data.len();
            self.map.insert(entity, index);
            self.reverse_map.push(entity);
            self.data.push(component);
            None
        }
    }

    pub fn get(&self, entity: EntityId) -> Option<&T> {
        let index = self.map.get(&entity)?;
        Some(&self.data[*index])
    }

    pub fn get_mut(&mut self, entity: EntityId) -> Option<&mut T> {
        let index = self.map.get(&entity)?;
        Some(&mut self.data[*index])
    }
}

/// Used to turn [`Storage`] into a trait object without generic parameter.
trait TypelessStorage {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
    /// Drops all components owned by the given entity.
    /// Returns `false` if the store is now empty and should be discarded.
    fn despawn(&mut self, entity: EntityId) -> bool;
}

impl<T: Component + 'static> TypelessStorage for Storage<T> {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn despawn(&mut self, entity: EntityId) -> bool {
        if let Some(index) = self.map.remove(&entity) {
            // Remove component and move last component in the array to its place.
            self.data.swap_remove(index);
            // Change mapping for the affected entity.
            let modified_id = self.reverse_map[self.reverse_map.len() - 1];
            self.map.insert(modified_id, index);
            self.reverse_map.swap_remove(index);
        }

        self.data.is_empty()
    }
}

pub struct Components {
    map: HashMap<TypeId, Box<dyn TypelessStorage>>,
}

impl Components {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    /// Stores a component in the store for the given entity.
    ///
    /// If this component was already present for the entity it is replaced and the old component is returned.
    pub fn insert<T: Component + 'static>(&mut self, entity: EntityId, component: T) -> Option<T> {
        let type_id = TypeId::of::<T>();

        if let Some(store) = self.map.get_mut(&type_id) {
            let downcast: &mut Storage<T> = store.as_any_mut().downcast_mut().unwrap();
            downcast.insert(entity, component)
        } else {
            self.map.insert(type_id, Storage::with(entity, component));
            None
        }
    }

    pub fn get<T: Component + 'static>(&self, entity: EntityId) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let downcast: &Storage<T> = self.map.get(&type_id)?.as_any().downcast_ref().unwrap();
        downcast.get(entity)
    }

    pub fn get_mut<T: Component + 'static>(&mut self, entity: EntityId) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let downcast: &mut Storage<T> = self.map.get_mut(&type_id)?.as_any_mut().downcast_mut().unwrap();
        downcast.get_mut(entity)
    }

    pub fn despawn(&mut self, entity: EntityId) {
        self.map.retain(|type_id, store| !store.despawn(entity));
    }
}
