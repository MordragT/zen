#![feature(return_position_impl_trait_in_trait)]

use std::{fmt::Debug, hash::Hash, num::NonZeroU8};

use bevy::{
    prelude::{Component, Entity, Handle, Image},
    utils::HashMap,
};

pub struct InventoryBuilder<K: ItemKind> {
    items: HashMap<K, HashMap<Entity, Option<NonZeroU8>>>,
}

impl<K: ItemKind> InventoryBuilder<K> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add(mut self, kind: K, items: impl IntoIterator<Item = Entity>) -> Self {
        let items = items.into_iter().map(|entity| (entity, None)).collect();
        self.items.insert(kind, items);
        self
    }

    pub fn build(self) -> Inventory<K> {
        Inventory { items: self.items }
    }
}

pub struct Inventory<K: ItemKind> {
    items: HashMap<K, HashMap<Entity, Option<NonZeroU8>>>,
}

impl<K: ItemKind> Inventory<K> {}

pub trait Item {
    fn name(&self) -> &String;
    fn description(&self) -> &String;
    fn logo(&self) -> Handle<Image>;
    fn kind(&self) -> impl ItemKind;
}

pub trait ItemKind: Component + Copy + Clone + Eq + Hash + Debug + Default {}

#[cfg(test)]
mod test {
    use bevy::{
        prelude::{App, Commands},
        DefaultPlugins,
    };

    #[test]
    fn test_inventory() {
        App::new()
            .add_plugins(DefaultPlugins)
            .add_startup_system(setup)
            .run();
    }

    fn setup(mut commands: Commands) {}
}
