use crate::imports::imports_status::*;

pub unsafe extern "C" fn appeal_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toReturn = fighter.status_Appeal();

    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if VarModule::is_flag(fighter.battle_object, mario::instance::flag::HATLESS) {
        if [hash40("appeal_s_r"), hash40("appeal_s_l")].contains(&motion_kind){
            let new_taunt = if motion_kind == hash40("appeal_s_r") {Hash40::new("appeal_lw_r")} else {Hash40::new("appeal_lw_l")};
            MotionModule::change_motion(fighter.module_accessor, new_taunt, 0.0, 1.0, false, 0.0, false, false);
        }
    }

    toReturn
}

pub fn install() {
    Agent::new("mario")
        .status(Main, *FIGHTER_STATUS_KIND_APPEAL, appeal_main)
        .install();
}