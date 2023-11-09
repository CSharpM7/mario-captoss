use crate::imports::imports_status::*;
use super::*;
//Unused

pub unsafe extern "C" fn captoss_pocket_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    println!("Pocket init");
    WorkModule::set_int(weapon.module_accessor, 120,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    KineticModule::clear_speed_all(weapon.module_accessor);
    KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_STOP);
    
    0.into()
}

pub unsafe extern "C" fn captoss_pocket_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    0.into()
}

pub unsafe extern "C" fn captoss_pocket_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    println!("Pocket main");
    PostureModule::set_pos(weapon.module_accessor, &Vector3f{x:0.0,y:1000.0,z:0.0});

    EffectModule::detach_all(weapon.module_accessor, 5);
    AttackModule::clear_all(weapon.module_accessor);
    weapon.fastshift(L2CValue::Ptr(captoss_pocket_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_pocket_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let died = captoss_dec_life(weapon);
    if died {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        captoss_effect_disappear(weapon);
        captoss_effect_reappear(weapon);
    }
    0.into()
}

pub unsafe extern "C" fn captoss_pocket_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    Agent::new("mario_captoss")
        .status(Init, CAPTOSS_STATUS_KIND_POCKET, captoss_pocket_init)
        .status(Pre, CAPTOSS_STATUS_KIND_POCKET, captoss_pocket_pre)
        .status(Main, CAPTOSS_STATUS_KIND_POCKET, captoss_pocket_main)
        .status(End, CAPTOSS_STATUS_KIND_POCKET, captoss_pocket_end)
        .install();
}
