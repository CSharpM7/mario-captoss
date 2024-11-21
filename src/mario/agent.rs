use crate::imports::imports_agent::*;

//Make diving Once Per Airtime
unsafe extern "C" fn special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, mario::instance::flag::CAPDIVE_ENABLED) {
    //&& ArticleModule::is_exist(fighter.module_accessor, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS) {
        return false.into();
    }
    true.into()
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let next_status_kind = StatusModule::status_kind_next(fighter.module_accessor);

    //Re-enable Cap movement on ground/death
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
        if !fighter.is_status_one_of(&[
        FIGHTER_MARIO_STATUS_KIND_CAPDIVE,FIGHTER_MARIO_STATUS_KIND_CAPJUMP]) {
            VarModule::on_flag(fighter.battle_object, mario::instance::flag::CAPJUMP_ENABLED);
            VarModule::on_flag(fighter.battle_object, mario::instance::flag::CAPDIVE_ENABLED);
        }
    }
    //Re-enable Cap dive on hit
    else if is_damage_status(fighter.module_accessor) {
        VarModule::on_flag(fighter.battle_object, mario::instance::flag::CAPDIVE_ENABLED);
    }
    //Reset hatless state
    if (&[
        *FIGHTER_STATUS_KIND_WIN,
        *FIGHTER_STATUS_KIND_LOSE,
        *FIGHTER_STATUS_KIND_ENTRY,
        *FIGHTER_STATUS_KIND_DEAD,
        *FIGHTER_STATUS_KIND_REBIRTH]).contains(&next_status_kind) || 
        !sv_information::is_ready_go() || lua_bind::FighterManager::is_result_mode(singletons::FighterManager())
    {
        VarModule::set_int(fighter.battle_object, mario::instance::int::CAP_TIMER,0); 
        VarModule::off_flag(fighter.battle_object, mario::instance::flag::HATLESS);
    }
    true.into()
}

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_MARIO {
        return;
    }
    GetVarManager::reset_var_module_by_object_id(fighter.battle_object_id, false);
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(special_s_callback as *const () as _));
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}
pub unsafe extern "C" fn agent_init(fighter: &mut L2CFighterCommon) {
    agent_start(fighter);
}
pub unsafe extern "C" fn agent_reset(fighter: &mut L2CFighterCommon) {
    agent_start(fighter);
}

pub fn install() {
    Agent::new("mario")
        .on_init(agent_init)
        .on_start(agent_reset)
        .install();
}