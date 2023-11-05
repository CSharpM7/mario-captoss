use crate::imports::imports_acmd::*;
use super::*;

pub unsafe extern "C" fn captoss_start_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    captoss_start_snap(weapon);
    0.into()
}

pub unsafe extern "C" fn captoss_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

pub unsafe extern "C" fn captoss_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("haved"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(captoss_start_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_start_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn captoss_start_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if !captoss_owner_is_mario(weapon) {
        return 0.into();
    }
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    let owner = get_owner_boma(weapon);
    if StatusModule::status_kind(owner) != *FIGHTER_STATUS_KIND_SPECIAL_S {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    captoss_start_snap(weapon);
    0.into()
}

pub unsafe extern "C" fn captoss_start_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x:0.0,y:0.0,z:0.0}, 0);
    PostureModule::add_pos(weapon.module_accessor, &Vector3f{x:10.0*lr,y:-3.0,z:0.0});
    0.into()
}
unsafe extern "C" fn captoss_start_snap(weapon: &mut smashline::L2CWeaponCommon) {
    if !captoss_owner_is_mario(weapon) {
        return;
    }
    let owner = get_owner_boma(weapon);
    let mut ownerPos = VECTOR_ZERO;
    let mut capPos = VECTOR_ZERO;
    let lr = PostureModule::lr(owner);
    let owner_offset = ModelModule::joint_global_offset_from_top(owner, Hash40{hash: hash40("havel")}, &mut ownerPos);  
    let cap_offset = ModelModule::joint_global_offset_from_top(weapon.module_accessor, Hash40{hash: hash40("have")}, &mut capPos);       
    let newPos = Vector3f{x: PostureModule::pos_x(owner) + ownerPos.x - capPos.x - (2.0*lr), y: PostureModule::pos_y(owner) + ownerPos.y - (capPos.y/1.5), z: PostureModule::pos_z(owner) + ownerPos.z- capPos.z};
    PostureModule::set_pos(weapon.module_accessor, &newPos);

    
    let mut vec =Vector3f{x: 0.0, y: 0.0, z: 0.0};
    let offset = ModelModule::joint_global_rotation(owner,Hash40::new("havel"),&mut vec,false);
    let rot = Vector3f{x: vec.x, y: 0.0, z: 0.0};
    PostureModule::set_rot(
        weapon.module_accessor,
        &rot,
        0
    );
}

pub fn install() {    
    Agent::new("mario_captoss")
        .status(Init, CAPTOSS_STATUS_KIND_START, captoss_start_init)
        .status(Pre, CAPTOSS_STATUS_KIND_START, captoss_start_pre)
        .status(Main, CAPTOSS_STATUS_KIND_START, captoss_start_main)
        .status(Exec, CAPTOSS_STATUS_KIND_START, captoss_start_exec)
        .status(End, CAPTOSS_STATUS_KIND_START, captoss_start_end)
        .install();
}
