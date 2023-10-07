use crate::imports::imports_agent::*;

unsafe extern "C" fn special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, mario::instance::flag::CAPDIVE_ENABLED) 
    && ArticleModule::is_exist(fighter.module_accessor, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS) {
        return false.into();
    }
    true.into()
}

unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD]) {
        //Re-enable capjump
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
        if !fighter.is_status_one_of(&[
        *FIGHTER_MARIO_STATUS_KIND_NUM+FIGHTER_MARIO_STATUS_KIND_CAPDIVE,*FIGHTER_MARIO_STATUS_KIND_NUM+FIGHTER_MARIO_STATUS_KIND_CAPJUMP]) {
            VarModule::on_flag(fighter.battle_object, mario::instance::flag::CAPJUMP_ENABLED);
            VarModule::on_flag(fighter.battle_object, mario::instance::flag::CAPDIVE_ENABLED);
        }
    }
    else if is_damage_status(fighter.module_accessor) {
        VarModule::on_flag(fighter.battle_object, mario::instance::flag::CAPDIVE_ENABLED);
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
    fighter.global_table[USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(special_s_callback as *const () as _));
    fighter.global_table[STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}
#[event("mario", initialize)]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[event(start)]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    agent_init::install();
    agent_reset::install();
}