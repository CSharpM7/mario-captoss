pub const FIGHTER_MARIO_STATUS_KIND_LAST: i32 = 0x1e3; //0x1e3;
pub const FIGHTER_MARIO_STATUS_KIND_CAPJUMP: i32 = FIGHTER_MARIO_STATUS_KIND_LAST+1;
pub const FIGHTER_MARIO_STATUS_KIND_CAPDIVE: i32 = FIGHTER_MARIO_STATUS_KIND_LAST+2;
pub const FIGHTER_MARIO_STATUS_KIND_CAPCATCH: i32 = FIGHTER_MARIO_STATUS_KIND_LAST+3;

pub const FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS: i32 = 6;
pub const CAPTOSS_STATUS_KIND_START: i32 = 0;
pub const CAPTOSS_STATUS_KIND_HAVED: i32 = 1; //unused
pub const CAPTOSS_STATUS_KIND_FLY: i32 = 2;
pub const CAPTOSS_STATUS_KIND_TURN: i32 = 3;
pub const CAPTOSS_STATUS_KIND_HOP: i32 = 4;
pub const CAPTOSS_STATUS_KIND_HOLD: i32 = 5;
pub const CAPTOSS_STATUS_KIND_SWALLOWED: i32 = 6;
pub const CAPTOSS_STATUS_KIND_JUMP: i32 = 7;
pub const CAPTOSS_STATUS_KIND_POCKET: i32 = 8; //unused

pub mod mario {
    pub mod instance {
        pub mod flag {
            pub const HATLESS: i32 = 0x0100;
            pub const CAPJUMP_ENABLED: i32 = 0x0101;
            pub const CAPDIVE_ENABLED: i32 = 0x0102;
            pub const CAPDIVE_ENABLE_ON_RETURN: i32 = 0x0103;
        }
        pub mod int {
            pub const CAP_TIMER: i32 = 0x0100;
        }
    }
}
pub mod mario_cappy {
    pub mod instance {
        pub mod flag {
            pub const CAPDIVE_ENABLE_ON_RETURN: i32 = 0x0100;
        }
    }
}