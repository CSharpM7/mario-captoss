use crate::imports::imports_status::*;

pub unsafe extern "C" fn wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toReturn = fighter.status_Wait();
    fighter.sub_wait_common();
    fighter.sub_wait_motion_mtrans();
    fighter.sub_shift_status_main(L2CValue::Ptr(wait_main_loop as *const () as _))
}

unsafe extern "C" fn wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if VarModule::is_flag(fighter.battle_object, mario::instance::flag::HATLESS) {
        if motion_kind == hash40("wait_2") {
            //Too lazy to animate a wait_4 so just do wait_3 instead
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait_3"), 0.0, 1.0, false, 0.0, false, false);
        }
    }
    if fighter.status_Wait_Main().get_bool() {
        return 0.into();
    }
    0.into()
}

pub fn install() {
    Agent::new("mario")
        .status(Main, *FIGHTER_STATUS_KIND_WAIT, wait_main)
        .install();
}