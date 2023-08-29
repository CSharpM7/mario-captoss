use crate::imports::imports_agent::*;

unsafe fn mario_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let motion_kind = MotionModule::motion_kind(boma);
    /*
    barrel_timer(fighter,boma,status_kind);
    barrel_air_despawn(fighter,boma,status_kind,motion_kind);
    */
    if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind){
        return;
    }
    if ArticleModule::is_exist(boma, FIGHTER_MARIO_GENERATE_ARTICLE_CAPTOSS) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIO_GENERATE_ARTICLE_CAPPY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ModelModule::set_mesh_visibility(boma, Hash40::new("mario_hathead"), false);
        ModelModule::set_mesh_visibility(boma, Hash40::new("mario_nohat"), true);
    }
    else{
        ModelModule::set_mesh_visibility(boma, Hash40::new("mario_hathead"), true);
        ModelModule::set_mesh_visibility(boma, Hash40::new("mario_nohat"), false);
    }
}

#[line("mario", main)]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        mario_update(fighter);
    }
}

pub fn install() {
    mario_frame::install();
}