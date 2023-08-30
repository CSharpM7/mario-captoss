pub const FIGHTER_MARIO_STATUS_KIND_LAST: i32 = 0x1e3;
pub const FIGHTER_MARIO_STATUS_KIND_CAPJUMP: i32 = 0;
pub const FIGHTER_MARIO_STATUS_KIND_CAPDIVE: i32 = 1;

pub const FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS: i32 = 6;
pub const CAPTOSS_STATUS_KIND_START: i32 = 0;
pub const CAPTOSS_STATUS_KIND_HAVED: i32 = 1;
pub const CAPTOSS_STATUS_KIND_FLY: i32 = 2;
pub const CAPTOSS_STATUS_KIND_TURN: i32 = 3;
pub const CAPTOSS_STATUS_KIND_HOP: i32 = 4;
pub const CAPTOSS_STATUS_KIND_HOLD: i32 = 5;
pub const CAPTOSS_STATUS_KIND_SWALLOWED: i32 = 6;
pub const CAPTOSS_STATUS_KIND_JUMP: i32 = 7;

pub mod mario {
    pub mod instance {
        pub mod flag {
            pub const CAPJUMP_ENABLED: i32 = 0x0100;
        }
        pub mod int {
            pub const SPECIAL_LW_ROTATIONS: i32 = 0x0100;
        }
    }
}