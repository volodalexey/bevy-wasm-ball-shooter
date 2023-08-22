pub const CAMERA_SPEED: f32 = 10.0;
pub const CAMERA_ROTATION_SPEED: f32 = 0.05;
pub const CAMERA_SCALE: f32 = 0.01;
pub const COLLISION_SNAP_COOLDOWN_TIME: f32 = 5.0; // seconds
pub const SIZE: f32 = 23.0;
pub const ROW_HEIGHT: f32 = 1.5 * SIZE;
pub const EMPTY_PLAYGROUND_ROWS: i32 = 10;
pub const ALL_PLAYGROUND_ROWS: i32 = 16;
pub const FILL_PLAYGROUND_ROWS: i32 = ALL_PLAYGROUND_ROWS - EMPTY_PLAYGROUND_ROWS;
pub const EMPTY_PLAYGROUND_HEIGHT: f32 = ROW_HEIGHT * EMPTY_PLAYGROUND_ROWS as f32;
pub const PROJECTILE_SPEED: f32 = 1000.;
pub const MIN_PROJECTILE_SNAP_VELOCITY: f32 = 20.0;
pub const MIN_PROJECTILE_SNAP_DOT: f32 = 0.2;
pub const INNER_RADIUS_COEFF: f32 = 0.866025404; // √3 / 2
pub const BALL_RADIUS: f32 = INNER_RADIUS_COEFF * SIZE;
pub const NEXT_PROJECTILE_SIZE: f32 = 10.0;
pub const NEXT_PROJECTILE_RADIUS: f32 = INNER_RADIUS_COEFF * NEXT_PROJECTILE_SIZE;
pub const BALL_DIAMETER: f32 = BALL_RADIUS * 2.0;
pub const PROJECTILE_SPAWN_BOTTOM: f32 = 40.0;
pub const NEXT_PROJECTILE_SPAWN_BOTTOM: f32 = 20.0;
pub const NEXT_PROJECTILE_SPAWN_SIDE: f32 = 40.0;
pub const PROJECTILE_SHOOT_BOTTOM: f32 = PROJECTILE_SPAWN_BOTTOM + ROW_HEIGHT * 2.0;
pub const GAME_OVER_BOTTOM: f32 = PROJECTILE_SPAWN_BOTTOM + ROW_HEIGHT;
pub const MIN_CLUSTER_SIZE: usize = 3;
pub const START_LEVEL: u32 = 1;
pub const MAX_LEVEL: u32 = 16;
pub const CAST_RAY_TRIES: u32 = 10;
pub const CAST_RAY_VELOCITY: f32 = BALL_RADIUS;
pub const CAST_RAY_VELOCITY_TOLERANCE: f32 = 0.1;
pub const CAST_RAY_MAX_TOI: f32 = EMPTY_PLAYGROUND_HEIGHT * 4.0 / CAST_RAY_VELOCITY;
pub const CAST_RAY_BOUNCE_Y_ADD: f32 = BALL_RADIUS / 3.9; // devergence between physics actual result and cast ray bounce point
pub const AIM_TARGET_Z_INDEX: f32 = 2.0;
pub const AIM_LINE_Z_INDEX: f32 = 2.0;
pub const LINE_WIDTH: f32 = 4.0;
pub const LINE_Z_INDEX: f32 = 1.0;
pub const WALL_X_WIDTH: f32 = 10.0;
pub const WALL_Y_HEIGHT: f32 = 2500.0;
pub const MOVE_DOWN_TIME: f32 = 2.0;
pub const MOVE_DOWN_TOLERANCE: f32 = 0.1;
pub const MAX_APPEAR_TIME: f32 = 2.0;
pub const APPEAR_TOLERANCE: f32 = 0.1;
pub const CLUSTER_TOLERANCE: f32 = BALL_DIAMETER + 3.0;
pub const CELL_SIZE: f32 = BALL_DIAMETER * 2.0;
pub const MAGNETIC_DISTANCE_STRONG: f32 = BALL_DIAMETER * 1.6;
pub const MAGNETIC_FACTOR_STRONG: f32 = 100.0;
pub const MAGNETIC_DISTANCE_WEAK: f32 = BALL_DIAMETER * 4.0;
pub const MAGNETIC_FACTOR_WEAK: f32 = 10.0;
pub const CLUSTER_CHECK_COOLDOWN_TIME: f32 = 1.0; // seconds
pub const MAX_GRID_BALL_SPEED: f32 = 30.0;
pub const LOCK_POSITION_TOLERANCE: f32 = 1.0;
