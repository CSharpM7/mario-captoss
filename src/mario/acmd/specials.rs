use crate::imports::imports_acmd::*;

unsafe extern "C" fn game_specials(agent: &mut smashline::L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS, false, -1);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 7.0+2.0);
    if macros::is_excute(agent) {
        macros::SEARCH(agent, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(agent.module_accessor, 9.0+2.0, *FIGHTER_MARIO_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(agent.lua_state_agent, 9.0+1.0);
    if macros::is_excute(agent) {
        search!(agent, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        let damage_mul = 1.5; //WorkModule::get_param_float(agent.module_accessor, hash40("param_reflector"), hash40("attack_mul"));
        let speed_mul = 1.0; //WorkModule::get_param_float(agent.module_accessor, hash40("param_reflector"), hash40("speed_mul"));
        let life_mul = 1.0; //WorkModule::get_param_float(agent.module_accessor, hash40("param_reflector"), hash40("life_mul"));
        let reflect_max = 999;
        shield!(agent, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, hash40("throw"), 6.5, 0.0, 3.0, -1.0, 0.0, 3.0, -1.0, 
        damage_mul, speed_mul, 50, false, life_mul, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);

    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        if ArticleModule::is_exist(agent.module_accessor, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS) 
        && crate::mario::FORCE_FLY {
            let cappy = get_article_boma(agent.module_accessor, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS);
            StatusModule::change_status_force(cappy, CAPTOSS_STATUS_KIND_FLY, false);
        }
        if crate::mario::SHOOT {
            ArticleModule::shoot_exist(agent.module_accessor, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        //shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIO_REFLECTOR_KIND_MANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        shield!(agent, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
}


unsafe extern "C" fn effect_specials(agent: &mut smashline::L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("mario_supermant_flash"), Hash40::new("top"), 0, 8.0, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        if StatusModule::situation_kind(agent.module_accessor) == *SITUATION_KIND_GROUND {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

pub fn install() {    
    Agent::new("mario")
        .game_acmd("game_specials", game_specials)
        .game_acmd("game_specialairs", game_specials)
        .effect_acmd("effect_specials", effect_specials)
        .effect_acmd("effect_specialairs", effect_specials)
        .install();
}
