use std::collections::HashMap;

use bevy::prelude::{Entity, Resource, Vec2};

use crate::gameplay::hex::{Bounds, Coord, Layout};

#[derive(Resource, Default)]
pub struct Grid {
    pub layout: Layout,
    pub storage: HashMap<Coord, Entity>,
    /// World bounds. Updated by calling [update_bounds].
    pub bounds: Bounds,
    /// True if bounds haven't been updated since last modification.
    pub dirty: bool,
}

impl Grid {
    pub fn get(&self, hex: Coord) -> Option<&Entity> {
        self.storage.get(&hex)
    }

    pub fn set(&mut self, hex: Coord, entity: Option<Entity>) -> Option<Entity> {
        self.dirty = true;
        match entity {
            Some(entity) => self.storage.insert(hex.clone(), entity),
            None => self.storage.remove(&hex),
        }
    }

    pub fn dim(&self) -> (f32, f32) {
        (
            (self.bounds.mins.x - self.bounds.maxs.x).abs(),
            (self.bounds.mins.y - self.bounds.maxs.y).abs(),
        )
    }

    pub fn columns(&self) -> i32 {
        let (w, _) = self.dim();
        let (hw, _) = self.layout.hex_size();
        (w / hw / 2.).round() as i32
    }

    #[allow(dead_code)]
    pub fn rows(&self) -> i32 {
        let (_, h) = self.dim();
        let (_, hh) = self.layout.hex_size();
        (h / hh / 2.).round() as i32
    }

    pub fn neighbors(&self, hex: Coord) -> Vec<(Coord, &Entity)> {
        hex.neighbors()
            .iter()
            .filter_map(|&hex| match self.get(hex) {
                Some(entity) => Some((hex, entity)),
                None => None,
            })
            .collect::<Vec<(Coord, &Entity)>>()
    }

    // TODO: this is not that efficient, but should be fine for now.
    #[inline]
    pub fn update_bounds(&mut self) {
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        for (&hex, _) in self.storage.iter() {
            let pos = self.layout.to_world(hex);
            max_x = max_x.max(pos.x);
            max_y = max_y.max(pos.y);
            min_x = min_x.min(pos.x);
            min_y = min_y.min(pos.y);
        }

        let (sx, sy) = self.layout.hex_size();

        self.dirty = false;
        self.bounds = Bounds {
            mins: Vec2::new(min_x - sx, min_y - sy),
            maxs: Vec2::new(max_x + sx, max_y + sy),
        }
    }

    pub fn clear(&mut self) {
        self.storage.clear();
        self.update_bounds();
    }
}
