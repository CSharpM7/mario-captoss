use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_capdive(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.75);
    FT_MOTION_RATE_RANGE(agent,1.0,12.0,8.0);
    frame(agent.lua_state_agent, 12.0);
    FT_MOTION_RATE_RANGE(agent,12.0,36.0,20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 45, 35, 0, 80, 3.5, 1.9, -1.6, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 45, 35, 0, 80, 3.0, 0.0, 5.0, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::set_power(agent.module_accessor, 0, 6.0,false);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 36.0);
    FT_MOTION_RATE(agent,1.0);
}
unsafe extern "C" fn game_capdiveair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    sv_kinetic_energy!(set_speed_mul, agent, FIGHTER_KINETIC_ENERGY_ID_MOTION, 1.125);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("neck"), 8.0, 45, 35, 0, 80, 3.5, 1.9, -1.6, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        //macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 45, 35, 0, 80, 3.0, 0.0, 5.0, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::set_power(agent.module_accessor, 0, 6.0,false);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 46.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
        WorkModule::set_float(agent.module_accessor, MotionModule::frame(agent.module_accessor),*FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
}
unsafe extern "C" fn effect_capdive(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_s_dash").hash {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
    }
}
unsafe extern "C" fn sound_capdive(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        PLAY_SE(agent,Hash40::new("se_mario_throw_b01"));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        PLAY_VC(agent,Hash40::new("vc_mario_passive"),0.5);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_s_dash").hash {
            macros::PLAY_SE(agent, Hash40::new("se_mario_rise"));
        }
    }
}
unsafe extern "C" fn expression_capdive(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        if MotionModule::motion_kind(agent.module_accessor) == Hash40::new("special_s_dash").hash {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

pub fn install() {
    Agent::new("mario")
        .acmd("game_specialsdash", game_capdive,Priority::Default)
        .acmd("game_specialairsdash", game_capdiveair,Priority::Default)
        .acmd("sound_specialsdash", sound_capdive,Priority::Default)
        .acmd("sound_specialairsdash", sound_capdive,Priority::Default)
        .acmd("effect_specialsdash", effect_capdive,Priority::Default)
        .acmd("effect_specialairsdash", effect_capdive,Priority::Default)
        .acmd("expression_specialsdash", expression_capdive,Priority::Default)
        .acmd("expression_specialairsdash", expression_capdive,Priority::Default)
        .install();
}