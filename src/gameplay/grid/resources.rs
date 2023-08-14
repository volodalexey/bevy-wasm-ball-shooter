use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result},
};

use bevy::{
    prelude::{info, Entity, Resource, Vec2},
    time::{Timer, TimerMode},
};
use hexx::{Hex, HexLayout, HexOrientation, OffsetHexMode};

use crate::gameplay::constants::{
    COLLISION_SNAP_COOLDOWN_TIME, FILL_PLAYGROUND_ROWS, MAX_LEVEL, SIZE,
};

#[derive(Default, Debug, Copy, Clone)]
pub struct Bound {
    pub x: f32,
    pub y: f32,
    pub axi_q: i32,
    pub axi_r: i32,
    pub init_odd_off_q: i32,
    pub init_even_off_q: i32,
    pub off_q: i32,
    pub off_r: i32,
}

#[derive(Default)]
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
            "Bounds {}<x>{} {}<y>{} q={} r={} axi({}<q>{}, {}<r>{}) off({}<q>{}, {}<r>{}) odd({}<>{}) even({}<>{})",
            self.mins.x,
            self.maxs.x,
            self.mins.y,
            self.maxs.y,
            self.cols,
            self.rows,
            self.mins.axi_q,
            self.maxs.axi_q,
            self.mins.axi_r,
            self.maxs.axi_r,
            self.mins.off_q,
            self.maxs.off_q,
            self.mins.off_r,
            self.maxs.off_r,
            self.mins.init_odd_off_q,
            self.maxs.init_odd_off_q,
            self.mins.init_even_off_q,
            self.maxs.init_even_off_q,
        )
    }
}

#[derive(Resource)]
pub struct Grid {
    pub init_cols: i32,
    pub init_rows: i32,
    pub last_active_row: i32,
    pub offset_mode: OffsetHexMode,
    pub layout: HexLayout,
    pub storage: HashMap<Hex, Entity>,
    // https://leetless.de/posts/spatial-hashing-vs-ecs/
    // Cell = HashMap<Entity, Position>
    // HashMap<CellIndex, Cell>
    pub bounds: Bounds,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            init_cols: 0,
            init_rows: 0,
            last_active_row: 0,
            offset_mode: OffsetHexMode::OddRows,
            layout: HexLayout {
                orientation: HexOrientation::Pointy,
                hex_size: hexx::Vec2::ONE * SIZE,
                invert_x: false,
                invert_y: false,
                origin: Vec2::ZERO,
            },
            storage: Default::default(),
            bounds: Default::default(),
        }
    }
}

impl Grid {
    pub fn calc_init_cols_rows(&mut self, level: u32) {
        self.init_cols = match level > MAX_LEVEL / 2 {
            false => 7,
            true => 9,
        };
        self.init_rows = match level {
            1 => 1,
            2 => 6,
            3 => 8,
            4 => 6,
            5 => 8,
            6 => 10,
            7 => 12,
            8 => 14,
            9 => 16,
            10 => 18,
            11 => 20,
            12 => 22,
            13 => 24,
            14 => 26,
            15 => 28,
            MAX_LEVEL => 30,
            _ => 0,
        };
        let fill_rows = FILL_PLAYGROUND_ROWS;
        self.last_active_row = -(match self.init_rows < fill_rows {
            true => self.init_rows,
            false => fill_rows,
        } - 1);
    }

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

    pub fn empty_neighbors(&self, hex: Hex) -> Vec<Hex> {
        hex.all_neighbors()
            .iter()
            .filter_map(|&neighbor_hex| match self.get(neighbor_hex) {
                Some(_) => None,
                None => {
                    // println!("neighbor_hex({}, {})", neighbor_hex.x, neighbor_hex.y);
                    Some(neighbor_hex)
                }
            })
            .collect::<Vec<Hex>>()
    }

    pub fn sort_neighbors(&self, neighbors: &mut Vec<Hex>, position: Vec2) {
        neighbors.sort_by(|a_hex, b_hex| {
            let a_hex = *a_hex;
            let b_hex = *b_hex;
            let a_hex_pos = self.layout.hex_to_world_pos(a_hex);
            let b_hex_pos = self.layout.hex_to_world_pos(b_hex);
            let a_distance = position.distance(a_hex_pos);
            let b_distance = position.distance(b_hex_pos);
            // println!(
            //     "a_hex({}, {}) a_pos({}, {}) a_dist({}) b_hex({}, {}) b_pos({}, {}) b_dist({})",
            //     a_hex.x,
            //     a_hex.y,
            //     a_hex_pos.x,
            //     a_hex_pos.y,
            //     a_distance,
            //     b_hex.x,
            //     b_hex.y,
            //     b_hex_pos.x,
            //     b_hex_pos.y,
            //     b_distance
            // );
            b_distance.total_cmp(&a_distance)
        });
    }

    pub fn check_update_bounds(&mut self) {
        if self.bounds.dirty {
            self.update_bounds();
        }
    }

    #[inline]
    pub fn update_bounds(&mut self) {
        // q
        let mut max_axi_q: i32 = 0;
        let mut min_axi_q: i32 = 0;
        let mut max_off_q: i32 = 0;
        let mut min_off_q: i32 = 0;
        // r
        let mut max_axi_r: i32 = 0;
        let mut min_axi_r: i32 = 0;
        let mut max_off_r: i32 = 0;
        let mut min_off_r: i32 = 0;
        // x
        let mut max_x: f32 = f32::MIN;
        let mut min_x: f32 = f32::MAX;
        // y
        let mut max_y: f32 = f32::MIN;
        let mut min_y: f32 = f32::MAX;
        for (&hex, _) in self.storage.iter() {
            let pos = self.layout.hex_to_world_pos(hex);
            let offset = hex.to_offset_coordinates(self.offset_mode);
            // q
            min_axi_q = min_axi_q.min(hex.x);
            max_axi_q = max_axi_q.max(hex.x);
            min_off_q = min_off_q.min(offset[0]);
            max_off_q = max_off_q.max(offset[0]);
            // r
            min_axi_r = min_axi_r.min(hex.y);
            max_axi_r = max_axi_r.max(hex.y);
            min_off_r = min_off_r.min(offset[1]);
            max_off_r = max_off_r.max(offset[1]);
            // x
            min_x = min_x.min(pos.x);
            max_x = max_x.max(pos.x);
            // y
            min_y = min_y.min(pos.y);
            max_y = max_y.max(pos.y);
        }

        let (sx, sy) = self.layout.hex_size.into();
        let half_side = self.init_cols / 2;

        self.bounds = Bounds {
            mins: Bound {
                x: if min_x == f32::MAX { 0.0 } else { min_x - sx },
                y: if min_y == f32::MAX { 0.0 } else { min_y - sy },
                axi_q: min_axi_q,
                axi_r: min_axi_r,
                init_odd_off_q: -half_side,
                init_even_off_q: -half_side,
                off_q: min_off_q,
                off_r: min_off_r,
            },
            maxs: Bound {
                x: if max_x == f32::MIN { 0.0 } else { max_x + sx },
                y: if max_y == f32::MIN { 0.0 } else { max_y + sy },
                axi_q: max_axi_q,
                axi_r: max_axi_r,
                init_odd_off_q: half_side,
                init_even_off_q: half_side - 1,
                off_q: max_off_q,
                off_r: max_off_r,
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
    pub fn print_sorted_offset(&mut self) {
        self.print_sorted(false, true, false, false)
    }

    #[allow(dead_code)]
    pub fn print_sorted_axial(&mut self) {
        self.print_sorted(false, false, true, false)
    }

    #[allow(dead_code)]
    pub fn print_sorted_position(&mut self) {
        self.print_sorted(false, false, false, true)
    }

    pub fn print_sorted(
        &mut self,
        print_id: bool,
        print_offset: bool,
        print_axial: bool,
        print_position: bool,
    ) {
        self.check_update_bounds();
        let mut s: Vec<(i32, (Hex, u32, [i32; 2]))> = Vec::with_capacity(self.storage.len());
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
        for (&hex, entity) in self.storage.iter() {
            let hex_offset = hex.to_offset_coordinates(self.offset_mode);
            let sort_value = hex_offset[1].abs() * y_mul_factor + hex_offset[0].abs();
            s.push((sort_value, (hex, entity.index(), hex_offset)));
        }
        s.sort_by(|x, y| x.0.cmp(&y.0));
        info!("Grid sorted----");
        for off_r in self.bounds.mins.off_r..=self.bounds.maxs.off_r {
            let mut result: Vec<String> = Vec::new();
            for off_q in self.bounds.mins.off_q..=self.bounds.maxs.off_q {
                let axi_hex = Hex::from_offset_coordinates([off_q, off_r], self.offset_mode);
                if let Some((_, (hex, id, hex_offset))) = s
                    .iter()
                    .find(|(_, (hex, _, _))| hex.x == axi_hex.x && hex.y == axi_hex.y)
                {
                    if print_id {
                        result.push(format!("id-({})", id));
                    }
                    if print_offset {
                        result.push(format!("off({}, {})", hex_offset[0], hex_offset[1]));
                    }
                    if print_axial {
                        result.push(format!("axi({}, {})", hex.x, hex.y));
                    }
                    if print_position {
                        let pos = self.layout.hex_to_world_pos(*hex);
                        result.push(format!("pos({}, {})", pos.x, pos.y));
                    }
                } else {
                    if print_id {
                        result.push("id-(--)".to_owned());
                    }
                    if print_offset {
                        result.push("off(-, -)".to_owned());
                    }
                    if print_axial {
                        result.push("axi(-, -)".to_owned());
                    }
                    if print_position {
                        result.push("pos(-, -)".to_owned());
                    }
                }
            }
            let is_even = (off_r + 1) & 1 == 0;
            match is_even {
                true => println!(" {}", result.join(" ")),
                false => println!("{}", result.join(" ")),
            }
        }
        info!("----Grid sorted");
    }
}

pub struct CheckAt {
    ms_time: u32,
    checked: bool,
}

impl CheckAt {
    pub fn new(ms_time: u32) -> Self {
        Self {
            ms_time,
            checked: false,
        }
    }
}

#[derive(Resource)]
pub struct CollisionSnapCooldown {
    pub timer: Timer,
    pub check_at: Vec<CheckAt>,
}

impl Default for CollisionSnapCooldown {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(COLLISION_SNAP_COOLDOWN_TIME, TimerMode::Repeating);
        timer.pause();
        Self {
            timer,
            check_at: vec![
                CheckAt::new(1000),
                CheckAt::new(2000),
                CheckAt::new(3000),
                CheckAt::new(4000),
            ],
        }
    }
}

impl CollisionSnapCooldown {
    pub fn start(&mut self) {
        self.restart();
        self.timer.unpause();
    }

    pub fn stop(&mut self) {
        self.timer.pause();
    }

    pub fn restart(&mut self) {
        self.timer.reset();
        self.timer.pause();
        self.check_at = (0u32..=(COLLISION_SNAP_COOLDOWN_TIME * 1000.0).round() as u32)
            // each 100 ms check
            .step_by(100)
            .map(|t| CheckAt::new(t))
            .collect::<Vec<CheckAt>>();
    }

    pub fn is_ready_for_check(&mut self, mut check_fn: impl FnMut() -> bool) -> bool {
        let elapsed_ms = self.timer.elapsed().as_millis() as u32;
        let mut is_ready = self.timer.finished();
        if !is_ready {
            for check_at in self.check_at.iter_mut() {
                if !check_at.checked && elapsed_ms > check_at.ms_time {
                    (*check_at).checked = true;
                    is_ready = check_fn();
                    // check only once per function call
                    break;
                }
            }
        }
        is_ready
    }
}
