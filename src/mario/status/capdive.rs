use crate::imports::imports_agent::*;

#[smashline::new_status("mario", FIGHTER_MARIO_STATUS_KIND_CAPDIVE)]
unsafe fn capdive_pre(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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

#[smashline::new_status("mario", FIGHTER_MARIO_STATUS_KIND_CAPDIVE)]
unsafe fn capdive_main(fighter: &mut smashline::L2CFighterCommon) -> L2CValue {
    println!("Capdive!");
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("escape_air"), 0.0, 1.0, false, 0.0, false, false);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);

    let lr = PostureModule::lr(fighter.module_accessor);
    let dive_speed_x = 1.9*lr;
    let dive_speed_y = 1.5;
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        dive_speed_x,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);

    fighter.sub_shift_status_main(L2CValue::Ptr(capdive_main_status_loop as *const () as _))
}

unsafe extern "C" fn capdive_main_status_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }
    }
    else if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        WorkModule::set_float(fighter.module_accessor,14.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        return 0.into();
    }
    
    0.into()
}

#[smashline::new_status("mario", FIGHTER_MARIO_STATUS_KIND_CAPDIVE)]
unsafe fn capdive_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    capdive_pre::install();
    capdive_main::install();
    capdive_end::install();
}
