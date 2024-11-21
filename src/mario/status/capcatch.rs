use crate::imports::imports_status::*;

pub unsafe extern "C" fn capcatch_pre(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn capcatch_kinetics(fighter: &mut smashline::L2CFighterCommon, init: bool) {
    if !init {
        fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_FALL.into());
    }
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -air_accel_y*1.25
        );
        sv_kinetic_energy!(
            set_accel_x_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_accel_x * 0.5
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            air_accel_y_stable
        );
    }
}
pub unsafe extern "C" fn capcatch_main(fighter: &mut smashline::L2CFighterCommon) -> L2CValue {
    //fighter.sub_set_special_start_common_kinetic_setting(Hash40::new("param_special_s").into());
    capcatch_kinetics(fighter,true);

    let motion_g = Hash40::new("special_s_end");
    let motion_a = Hash40::new("special_air_s_end");
    let motion = if fighter.is_situation(*SITUATION_KIND_GROUND) {motion_g} else {motion_a};
    MotionModule::change_motion(fighter.module_accessor, motion, 0.0, 1.0, false, 0.0, false, false);

    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("mario_hathead"), true);
    ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("mario_nohat"), true);

    fighter.sub_shift_status_main(L2CValue::Ptr(capcatch_main_status_loop as *const () as _))
}

unsafe extern "C" fn capcatch_main_status_loop(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
        /*
        else if fighter.sub_air_check_stop_ceil().get_bool() {
            return 1.into();
        }*/
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status_by_situation(FIGHTER_STATUS_KIND_WAIT.into(), FIGHTER_STATUS_KIND_FALL.into(), true.into());
    }
    
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        let motion_g = Hash40::new("special_s_end");
        let motion_a = Hash40::new("special_air_s_end");
        fighter.sub_change_motion_by_situation(motion_g.into(), motion_a.into(), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        capcatch_kinetics(fighter,false);
    }
    0.into()
}

pub unsafe extern "C" fn capcatch_end(fighter: &mut smashline::L2CFighterCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {   
    Agent::new("mario")
        .status(Pre, FIGHTER_MARIO_STATUS_KIND_CAPCATCH, capcatch_pre)
        .status(Main, FIGHTER_MARIO_STATUS_KIND_CAPCATCH, capcatch_main)
        .status(End, FIGHTER_MARIO_STATUS_KIND_CAPCATCH, capcatch_end)
        .install();
}