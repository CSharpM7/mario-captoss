use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOLD)]
unsafe fn captoss_hold_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    println!("HOLD: Init");

    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("accel"));
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed"));
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));
    let speed_current = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let brake = accel/2.0;
    let lr = PostureModule::lr(weapon.module_accessor);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_current
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_min*lr
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0//-accel
    );
    sv_kinetic_energy!(
        set_brake, 
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 
        brake, 
        0.0
    );
    sv_kinetic_energy!(
        set_stable_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0
    );
    /* 
    let speed_rot = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("rot_speed"));
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        speed_rot*lr
    );*/

    WorkModule::set_int(weapon.module_accessor, 180, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOLD)]
unsafe fn captoss_hold_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    println!("HOLD: PRE");
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
    println!("HOLD: MAIN");
    if StopModule::is_stop(weapon.module_accessor){
        //captoss_ground_check(weapon);
    }
    AttackModule::clear_all(weapon.module_accessor);
    //MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("hold"), 0.0, 1.0, false, 0.0, false, false);
    //MotionModule::change_motion_inherit_frame_keep_rate(weapon.module_accessor as _, Hash40::new("hold"), -1.0,1.0,0.0);
    weapon.fastshift(L2CValue::Ptr(captoss_hold_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_hold_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    let owner_boma = get_owner_boma(weapon);
    if ControlModule::check_button_off(owner_boma,*CONTROL_PAD_BUTTON_SPECIAL)
    && ControlModule::check_button_off(owner_boma,*CONTROL_PAD_BUTTON_SPECIAL_RAW){
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_TURN, false);
        return 0.into()
    }
    if captoss_distance_to_owner(weapon) < 12.0 {
        PostureModule::add_pos(owner_boma, &Vector3f{x: 0.0, y: 2.0, z: 0.0});
        KineticModule::add_speed_outside(owner_boma,0, &Vector3f{x: 0.0, y: 2.0, z: 0.0});
        //StatusModule::change_status_force(owner_boma, FIGHTER_MARIO_STATUS_KIND_CAPJUMP, false);
        StatusModule::change_status_request_from_script(owner_boma, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_TURN, false);
    }
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if (WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE) <= 0){
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_TURN, false);
    }
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
