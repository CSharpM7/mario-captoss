use crate::imports::imports_acmd::*;

unsafe extern "C" fn sound_capjump(agent: &mut smashline::L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_item_boomerang_throw"));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_jump03"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        //macros::PLAY_SEQUENCE(agent, Hash40::new("seq_mario_rnd_jump"));
        PLAY_VC(agent, Hash40::new("vc_mario_appeal02"), 0.5);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
    }
}

unsafe extern "C" fn effect_capjump(agent: &mut smashline::L2CAgentBase) {
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_capjump(agent: &mut smashline::L2CAgentBase) {
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_bounce"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {    
    Agent::new("mario")
        .sound_acmd("sound_specialairsjump", sound_capjump)
        .effect_acmd("effect_specialairsjump", effect_capjump)
        .expression_acmd("expression_specialairsjump", expression_capjump)
        .install();
}
