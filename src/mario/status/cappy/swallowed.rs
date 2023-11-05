use crate::imports::imports_status::*;
use super::*;
pub const NEXT_STATUS: i32 = CAPTOSS_STATUS_KIND_HOLD;

pub unsafe extern "C" fn captoss_swallowed_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);

    GroundModule::set_rhombus_offset(weapon.module_accessor, &Vector2f::new(0.0, 2.0));
    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("brake_x"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));

    let speed_x = speed_max*lr;
    let speed_y = 0.0;
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y
    );
    let max_x = speed_max;
    let max_y = 0.0;
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        max_x,
        max_y
    );
    //let accel_x = -accel*lr*angle.cos();
    let accel_x = 0.0;
    let accel_y = 0.0;
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        accel_x,
        accel_y
    );

    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

pub unsafe extern "C" fn captoss_swallowed_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        *FS_SUCCEEDS_KEEP_ROT_Y_LR | *FS_SUCCEEDS_KEEP_EFFECT,
    );
    0.into()
}

pub unsafe extern "C" fn captoss_swallowed_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    WorkModule::off_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HOP);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if StopModule::is_stop(weapon.module_accessor){
        captoss_ground_check(weapon);
    }

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(captoss_swallowed_main_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(captoss_swallowed_main_status_loop as *const () as _))
}
unsafe extern "C" fn captoss_swallowed_main_substatus(weapon: &mut L2CWeaponCommon, param_3: L2CValue) -> L2CValue {
    0.into()
}

unsafe extern "C" fn captoss_swallowed_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let died = captoss_dec_life(weapon);
    if died {
        //smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        //captoss_effect_disappear(weapon);
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));
    let speed_min_mul = speed_min*1.0;

    if captoss_reflect_check(weapon) {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 0.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::reflect_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        //StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    {
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 0.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::reflect_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_HOP, false);
    }
    else if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_UP | *GROUND_TOUCH_FLAG_DOWN) as u32)
    {
        LANDING_EFFECT(weapon, Hash40::new("sys_landing_smoke_s"), Hash40::new("top"), 0, -2, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        let bound_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("floor_bound_x_mul"));
        KineticModule::reflect_speed(weapon.module_accessor,  &Vector3f{x: 0.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        KineticModule::reflect_accel(weapon.module_accessor,  &Vector3f{x: 0.0, y: 1.0, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        //KineticModule::mul_speed(weapon.module_accessor, &Vector3f{x: 1.0, y: -1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
        //KineticModule::mul_accel(weapon.module_accessor, &Vector3f{x: 1.0, y: -1.0, z: 1.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_ALL);
    }

    0.into()
}
pub unsafe extern "C" fn captoss_swallowed_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub unsafe extern "C" fn captoss_swallowed_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    

    Agent::new("mario_captoss")
        .status(Init, CAPTOSS_STATUS_KIND_SWALLOWED, captoss_swallowed_init)
        .status(Pre, CAPTOSS_STATUS_KIND_SWALLOWED, captoss_swallowed_pre)
        .status(Main, CAPTOSS_STATUS_KIND_SWALLOWED, captoss_swallowed_main)
        .status(Exec, CAPTOSS_STATUS_KIND_SWALLOWED, captoss_swallowed_exec)
        .status(End, CAPTOSS_STATUS_KIND_SWALLOWED, captoss_swallowed_end)
        .install();

}
