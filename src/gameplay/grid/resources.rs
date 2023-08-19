use bevy::{
    prelude::{Resource, Vec2},
    time::{Timer, TimerMode},
};
use hexx::{HexLayout, HexOrientation, OffsetHexMode};

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

#[derive(Resource)]
pub struct Grid {
    pub init_cols: i32,
    pub init_rows: i32,
    pub last_active_row: i32,
    pub offset_mode: OffsetHexMode,
    pub layout: HexLayout,
    pub bounds: Bounds,
}

impl Default for Grid {
    fn default() -> Self {
        let layout = HexLayout {
            orientation: HexOrientation::Pointy,
            hex_size: hexx::Vec2::ONE * SIZE,
            invert_x: false,
            invert_y: false,
            origin: Vec2::ZERO,
        };
        Self {
            init_cols: 0,
            init_rows: 0,
            last_active_row: 0,
            offset_mode: OffsetHexMode::OddRows,
            layout,
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
            2 => 2,
            3 => 4,
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

    pub fn clear(&mut self) {
        self.layout.origin = hexx::Vec2::ZERO;
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
