use crate::imports::imports_acmd::*;

unsafe extern "C" fn sound_capcatch(agent: &mut smashline::L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_item_boomerang_catch"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_mario_appeal_s06"));
    }
}

unsafe extern "C" fn effect_capcatch(agent: &mut smashline::L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        //macros::EFFECT(agent, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn expression_capcatch(agent: &mut smashline::L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_grapple"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {    
    Agent::new("mario")
        .sound_acmd("sound_specialsend", sound_capcatch)
        .sound_acmd("sound_specialairsend", sound_capcatch)
        .effect_acmd("effect_specialsend", effect_capcatch)
        .effect_acmd("effect_specialairsend", effect_capcatch)
        .expression_acmd("expression_specialsend", expression_capcatch)
        .expression_acmd("expression_specialairsend", expression_capcatch)
        .install();
}
