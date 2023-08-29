use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOLD)]
unsafe fn captoss_hold_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        println!("Isabelle hold?");
        return 0.into();
    }
    println!("HOLD: Init");

    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("brake_x"));
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));
    let speed_current = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let brake = accel;
    let lr = PostureModule::lr(weapon.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_current
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -accel
    );
    WorkModule::set_int(weapon.module_accessor, 180, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_INT_GRAVITY_FRAME);
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
    MotionModule::change_motion_inherit_frame_keep_rate(weapon.module_accessor, Hash40::new("hold"), -1.0,1.0,0.0);
    weapon.fastshift(L2CValue::Ptr(captoss_hold_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_hold_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if sum_speed_len < 0.1 {
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
    /*
    if captoss_distance_to_owner(weapon) < 12.0 {
        PostureModule::add_pos(owner_boma, &Vector3f{x: 0.0, y: 2.0, z: 0.0});
        KineticModule::add_speed_outside(owner_boma,0, &Vector3f{x: 0.0, y: 2.0, z: 0.0});
        let owner = get_fighter_common_from_accessor(&mut *owner_boma);
        let owner_object = owner.battle_object;
        if VarModule::is_flag(owner_object, mario::instance::flag::CAPJUMP_ENABLED){
            VarModule::off_flag(owner_object, mario::instance::flag::CAPJUMP_ENABLED);
            WorkModule::on_flag(owner_boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
            //owner.change_status(FIGHTER_MARIO_STATUS_KIND_CAPJUMP.into(), false.into()); 
            StatusModule::change_status_force(owner_boma, *FIGHTER_MARIO_STATUS_KIND_NUM+FIGHTER_MARIO_STATUS_KIND_CAPJUMP, false);
            StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_TURN, false);
            KineticModule::clear_speed_all(weapon.module_accessor);
        }
        else{
            smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            captoss_effect_disappear(weapon);
        }
    }*/
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
