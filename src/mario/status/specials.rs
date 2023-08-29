use crate::imports::imports_agent::*;


#[status("mario",FIGHTER_STATUS_KIND_SPECIAL_S)]
unsafe fn specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ArticleModule::is_exist(fighter.module_accessor, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS) {
        println!("New status!");
        //StatusModule::set_status_kind_interrupt(fighter.module_accessor,*FIGHTER_MARIO_STATUS_KIND_NUM + FIGHTER_MARIO_STATUS_KIND_CAPDIVE);
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_MARIO_STATUS_KIND_NUM+FIGHTER_MARIO_STATUS_KIND_CAPDIVE, false);
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0
    );
    0.into()
}

pub fn install() {
    specials_pre::install();
}