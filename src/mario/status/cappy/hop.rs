use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("life"));
    let speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_speed_x"));
    let speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_speed_y"));
    let speed_y_limit = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_limit_speed_y"));
    let accel_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_accel_y"));
    let lr = PostureModule::lr(weapon.module_accessor);
    WorkModule::set_int(weapon.module_accessor, life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x*lr
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -accel_y
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0
    );
    let rot_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_rot_mul"));
    //KineticModule::mul_accel(weapon.module_accessor, &Vector3f {x: rot_mul, y: rot_mul, z: rot_mul }, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_KOOPAJR_CANNONBALL_HOP,
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

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    println!("Hop!");
    //MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("hop"), 0.0, 1.0, false, 0.0, false, false);
    //AttackModule::clear_all(weapon.module_accessor);
    weapon.fastshift(L2CValue::Ptr(captoss_hop_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_hop_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let currentRate = MotionModule::rate(weapon.module_accessor);
    let newRate = lerp(currentRate,0.0,0.1);
    MotionModule::set_rate(weapon.module_accessor, newRate);
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let died = captoss_dec_life(weapon);
    if died {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        captoss_effect_disappear(weapon);
        return 0.into();
    }
    if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_KOOPAJR_CANNONBALL_HOP);
    }
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        //KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_KOOPAJR_CANNONBALL_HOP);
    }
    
    0.into()
}

pub fn install() {    
    captoss_hop_init::install();
    captoss_hop_pre::install();
    captoss_hop_main::install();
    captoss_hop_end::install();
    captoss_hop_exec::install();
}
