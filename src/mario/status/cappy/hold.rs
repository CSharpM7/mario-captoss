use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOLD)]
unsafe fn captoss_hold_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        println!("Isabelle hold?");
        return 0.into();
    }
    let hold_frame_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("gravity_start_frame_max")) as i32;
    WorkModule::set_int(weapon.module_accessor, hold_frame_max, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);

    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOLD)]
unsafe fn captoss_hold_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOLD)]
unsafe fn captoss_hold_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    if StopModule::is_stop(weapon.module_accessor){
        //captoss_ground_check(weapon);
    }
    AttackModule::clear_all(weapon.module_accessor);
    //MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("hold"), 0.0, 1.0, false, 0.0, false, false);
    MotionModule::change_motion_inherit_frame_keep_rate(weapon.module_accessor, Hash40::new("hold"), -1.0,1.0,0.0);
    weapon.fastshift(L2CValue::Ptr(captoss_hold_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_hold_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    let speed_current_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_current_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_current_x.abs() < 0.1 {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0,
            speed_current_y
        );
    }
    if speed_current_y.abs() < 0.1 {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            speed_current_x,
            0
        );
    }
    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if sum_speed_len < 0.2 {
        KineticModule::clear_speed_all(weapon.module_accessor);
    }
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    let owner_boma = get_owner_boma(weapon);
    
    let hold_frame_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("gravity_start_frame_min")) as i32;
    let hold_frame_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("gravity_start_frame_max")) as i32;
    let hold_frame_current = WorkModule::get_int(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);

    if hold_frame_current < hold_frame_max-hold_frame_min {
        if ControlModule::check_button_off(owner_boma,*CONTROL_PAD_BUTTON_SPECIAL)
        && ControlModule::check_button_off(owner_boma,*CONTROL_PAD_BUTTON_SPECIAL_RAW){
            WorkModule::add_int(weapon.module_accessor, -20,*WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
            //StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_TURN, false);
            //return 0.into()
        }
    }
    if captoss_reflect_check(weapon) {
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    if captoss_swallowed_check(weapon) {
        return 0.into();
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
    if (hold_frame_current <= 0) {
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_TURN, false);
    } 
    captoss_check_recapture(weapon);
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOLD)]
unsafe fn captoss_hold_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    captoss_hold_init::install();
    captoss_hold_pre::install();
    captoss_hold_main::install();
    captoss_hold_end::install();

}
