use crate::imports::imports_agent::*;

pub unsafe extern "C" fn hat_mesh_visibility(fighter: &mut L2CFighterCommon, boma: *mut BattleObjectModuleAccessor, status_kind: i32) {
    let motion_kind = MotionModule::motion_kind(boma);
    let capcatch = //MotionModule::frame(boma) <= 10.0 && 
    StatusModule::status_kind(boma) == FIGHTER_MARIO_STATUS_KIND_CAPCATCH;
    if [hash40("appeal_s_r"), hash40("appeal_s_l"), hash40("appeal_s_l")].contains(&motion_kind) || capcatch {
        return;
    }
    let hatless = VarModule::is_flag(fighter.battle_object, mario::instance::flag::HATLESS);

    if ArticleModule::is_exist(boma, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS) {
        if !VarModule::is_flag(fighter.battle_object, mario::instance::flag::HATLESS){
            VarModule::on_flag(fighter.battle_object, mario::instance::flag::HATLESS);
        }
        ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    else{
        if VarModule::is_flag(fighter.battle_object, mario::instance::flag::HATLESS) {
            VarModule::dec_int(fighter.battle_object, mario::instance::int::CAP_TIMER);
            if VarModule::get_int(fighter.battle_object, mario::instance::int::CAP_TIMER) <= 0 {
                VarModule::off_flag(fighter.battle_object, mario::instance::flag::HATLESS);
                EFFECT_FOLLOW(fighter,Hash40::new("sys_item_arrival"),Hash40::new("hat"),0,0,0,0,0,0,0.35,false);
            }
        }
    }

    ModelModule::set_mesh_visibility(boma, Hash40::new("mario_hathead"), !hatless);
    ModelModule::set_mesh_visibility(boma, Hash40::new("mario_nohat"), (hatless&&!capcatch));
}

pub unsafe extern "C" fn mario_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);

    hat_mesh_visibility(fighter,boma,status_kind);
}

pub fn install() {
    Agent::new("mario")
        .on_line(Main, mario_update)
        .install();
}