use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_JUMP)]
unsafe fn captoss_jump_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NONE,
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

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_JUMP)]
unsafe fn captoss_jump_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    println!("Cappy was jumped on!");
    if StopModule::is_stop(weapon.module_accessor){
        //captoss_ground_check(weapon);
    }
    AttackModule::clear_all(weapon.module_accessor);
    MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("jump"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::clear_speed_all(weapon.module_accessor);

    weapon.fastshift(L2CValue::Ptr(captoss_jump_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_jump_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    KineticModule::clear_speed_all(weapon.module_accessor);
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    //captoss_check_recapture(weapon);
    if MotionModule::is_end(weapon.module_accessor) {
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_TURN, false);
    }
    if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR){
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL);
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_JUMP)]
unsafe fn captoss_jump_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    captoss_jump_pre::install();
    captoss_jump_main::install();
    captoss_jump_end::install();

}
