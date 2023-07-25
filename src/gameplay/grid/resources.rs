use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

use bevy::prelude::{info, Entity, Resource};
use hexx::{Hex, HexLayout, HexOrientation, OffsetHexMode};

#[derive(Default, Debug, Copy, Clone)]
pub struct Bound {
    pub x: f32,
    pub y: f32,
    pub q: i32,
    pub r: i32,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Bounds {
    pub mins: Bound,
    pub maxs: Bound,
    pub cols: i32,
    pub rows: i32,
    pub dirty: bool,
}

impl Display for Bounds {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Bounds {}<x>{} {}<y>{} {}<q({})>{} {}<r({})>{}",
            self.mins.x,
            self.maxs.x,
            self.mins.y,
            self.maxs.y,
            self.mins.q,
            self.cols,
            self.maxs.q,
            self.mins.r,
            self.rows,
            self.maxs.r
        )
    }
}

#[derive(Resource)]
pub struct Grid {
    pub init_cols: i32,
    pub init_rows: i32,
    pub layout: HexLayout,
    pub storage: HashMap<Hex, Entity>,
    pub bounds: Bounds,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            init_cols: 0,
            init_rows: 0,
            layout: HexLayout {
                orientation: HexOrientation::Pointy,
                hex_size: hexx::Vec2::ONE,
                ..Default::default()
            },
            storage: Default::default(),
            bounds: Default::default(),
        }
    }
}

impl Grid {
    pub fn get(&self, hex: Hex) -> Option<&Entity> {
        self.storage.get(&hex)
    }

    pub fn set(&mut self, hex: Hex, entity: Entity) {
        self.bounds.dirty = true;
        self.storage.insert(hex.clone(), entity);
    }

    pub fn remove(&mut self, hex: &Hex) {
        self.bounds.dirty = true;
        self.storage.remove(hex);
    }

    pub fn dim(&self) -> (f32, f32) {
        (
            (self.bounds.mins.x - self.bounds.maxs.x).abs(),
            (self.bounds.mins.y - self.bounds.maxs.y).abs(),
        )
    }

    pub fn columns(&self) -> i32 {
        let (w, _) = self.dim();
        let (hw, _) = self.layout.hex_size.into();
        (w / hw / 2.).round() as i32
    }

    pub fn rows(&self) -> i32 {
        let (_, h) = self.dim();
        let (_, hh) = self.layout.hex_size.into();
        (h / hh / 2.).round() as i32
    }

    pub fn neighbors(&self, hex: Hex) -> Vec<(Hex, &Entity)> {
        hex.all_neighbors()
            .iter()
            .filter_map(|&hex| match self.get(hex) {
                Some(entity) => Some((hex, entity)),
                None => None,
            })
            .collect::<Vec<(Hex, &Entity)>>()
    }

    #[inline]
    pub fn update_bounds(&mut self) {
        // q
        let mut max_q: i32 = 0;
        let mut min_q: i32 = 0;
        // r
        let mut max_r: i32 = 0;
        let mut min_r: i32 = 0;
        // x
        let mut max_x: f32 = f32::MIN;
        let mut min_x: f32 = f32::MAX;
        // y
        let mut max_y: f32 = f32::MIN;
        let mut min_y: f32 = f32::MAX;
        for (&hex, _) in self.storage.iter() {
            let pos = self.layout.hex_to_world_pos(hex);
            // q
            min_q = min_q.min(hex.x);
            max_q = max_q.max(hex.x);
            // r
            min_r = min_r.min(hex.y);
            max_r = max_r.max(hex.y);
            // x
            min_x = min_x.min(pos.x);
            max_x = max_x.max(pos.x);
            // y
            min_y = min_y.min(pos.y);
            max_y = max_y.max(pos.y);
        }

        let (sx, sy) = self.layout.hex_size.into();

        self.bounds = Bounds {
            mins: Bound {
                x: if min_x == f32::MAX { 0.0 } else { min_x - sx },
                y: if min_y == f32::MAX { 0.0 } else { min_y - sy },
                q: min_q,
                r: min_r,
            },
            maxs: Bound {
                x: if max_x == f32::MIN { 0.0 } else { max_x + sx },
                y: if max_y == f32::MIN { 0.0 } else { max_y + sy },
                q: max_q,
                r: max_r,
            },
            cols: self.columns(),
            rows: self.rows(),
            dirty: false,
        }
    }

    pub fn clear(&mut self) {
        self.storage.clear();
        self.layout.origin = hexx::Vec2::ZERO;
        self.update_bounds();
    }

    #[allow(dead_code)]
    pub fn print_sorted(&mut self) {
        let mut s: Vec<(i32, Hex)> = Vec::new();
        let replaced: String = self
            .bounds
            .cols
            .to_string()
            .chars()
            .map(|x| match x {
                _ => '0',
            })
            .collect();
        let y_mul_factor: i32 = format!("1{}", replaced).parse().unwrap();
        for (&hex, _) in self.storage.iter() {
            let hex_offset = hex.to_offset_coordinates(OffsetHexMode::OddRows);
            let sort_value = hex_offset[1].abs() * y_mul_factor + hex_offset[0].abs();
            s.push((sort_value, hex));
        }
        s.sort_by(|x, y| x.0.cmp(&y.0));
        info!("Grid sorted----");
        for (_, hex) in s.iter() {
            let pos = self.layout.hex_to_world_pos(*hex);
            let hex_offet = hex.to_offset_coordinates(OffsetHexMode::OddRows);
            info!(
                "offset(x={}, y={}) axial(x={}, y={})  pos(x={}, y={}) ",
                hex_offet[0], hex_offet[1], hex.x, hex.y, pos.x, pos.y,
            );
        }
        info!("----Grid sorted");
    }
}
