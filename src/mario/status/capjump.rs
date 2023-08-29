use crate::imports::imports_agent::*;

#[smashline::new_status("mario", FIGHTER_MARIO_STATUS_KIND_CAPJUMP)]
unsafe fn capjump_pre(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES),
        //app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
    let accel = 0.18;
    let distance = 1.1;
    //let jump_speed = KineticUtility::get_jump_speed_y(distance, accel);
    let jump_mul = 0.11;
    let jump_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("jump_initial_y"), 0)*jump_mul;
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

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
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_STOP,
        ENERGY_STOP_RESET_TYPE_AIR,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        ENERGY_CONTROLLER_RESET_TYPE_MOVE_AIR,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        jump_speed
    );
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_accel_y_stable
    );
    let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
    let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
    let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
    let special_hi_x_mul = 1.0;
    let air_max_speed_x_mul = 1.0;
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        air_accel_x_mul * special_hi_x_mul
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        air_accel_x_add * special_hi_x_mul
    );
    sv_kinetic_energy!(
        mul_x_speed_max,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_max_speed_x_mul
    ); 
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * special_hi_x_mul,
        0.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        air_speed_x_stable * special_hi_x_mul,
        0.0
    );
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.0
    );


    fighter.sub_shift_status_main(L2CValue::Ptr(capjump_main_status_loop as *const () as _))
}

unsafe extern "C" fn capjump_main_status_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else if MotionModule::frame(fighter.module_accessor) > 10.0 {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
    && fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    /* 
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() == false
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 0.into();
        }
    }
    */
    if fighter.is_situation(*SITUATION_KIND_GROUND) {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        WorkModule::set_float(fighter.module_accessor,10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
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
