use crate::imports::imports_agent::*;

#[smashline::new_status("mario", FIGHTER_MARIO_STATUS_KIND_CAPJUMP)]
unsafe fn capjump_pre(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    println!("Capjump!");
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_HI_JUMP,
        *GROUND_CORRECT_KIND_AIR as u32,
        //app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[smashline::new_status("mario", FIGHTER_MARIO_STATUS_KIND_CAPJUMP)]
unsafe fn capjump_main(fighter: &mut smashline::L2CFighterCommon) -> L2CValue {
    println!("Capjump!");
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("jump_aerial_f"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(capjump_main_status_loop as *const () as _))
}

unsafe extern "C" fn capjump_main_status_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        /* 
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }*/
    }
    else if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(
            L2CValue::I32(*FIGHTER_STATUS_KIND_FALL),
            L2CValue::Bool(false)
        );
        return 0.into();
    }
    
    0.into()
}

#[smashline::new_status("mario", FIGHTER_MARIO_STATUS_KIND_CAPJUMP)]
unsafe fn capjump_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    capjump_pre::install();
    capjump_main::install();
    capjump_end::install();

}
