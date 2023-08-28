use crate::imports::imports_acmd::*;

#[smashline::acmd("mario_captoss", ["game_fly"])]
pub unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 70, 40, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 40, 0, 60, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

#[smashline::acmd("mario_captoss", ["effect_fly","effect_turn","effect_hold"])]
pub unsafe extern "C" fn effect_fly(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.module_accessor;
    for _ in 1..i32::MAX {
        if is_excute(agent) {
            macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("sys_spin_wind"), Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 1, 0, 0, -90, 0, 0.5, true, *EF_FLIP_YZ);
        }
        wait(agent.lua_state_agent, 5.0);
    }
}
#[smashline::acmd("mario_captoss", ["game_turn"])]
unsafe fn game_turn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 70, 40, 0, 50, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

pub fn install() {    
    game_fly::install();
    effect_fly::install();

    game_turn::install();
}
