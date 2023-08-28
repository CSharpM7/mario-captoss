use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("hop_life"));
    let speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_speed_x"));
    let speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_speed_y"));
    let speed_y_limit = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_limit_speed_y"));
    let accel_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_accel_y"));
    let lr = PostureModule::lr(weapon.module_accessor);
    println!("New life: {life}");
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
    KineticModule::mul_accel(weapon.module_accessor, &Vector3f {x: rot_mul, y: rot_mul, z: rot_mul }, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    EffectModule::detach_all(weapon.module_accessor, 5);
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        *FS_SUCCEEDS_KEEP_ROT_Y_LR | *FS_SUCCEEDS_KEEP_EFFECT,
    );
    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    println!("hop: MAIN");
    MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("hop"), 0.0, 1.0, false, 0.0, false, false);
    //AttackModule::clear_all(weapon.module_accessor);
    weapon.fastshift(L2CValue::Ptr(captoss_hop_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_hop_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_HOP)]
unsafe fn captoss_hop_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let died = captoss_dec_life(weapon);
    if !died {
        return 0.into();
    }
    
    let pos = PostureModule::pos(weapon.module_accessor);
    EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_erace_smoke"),
        pos,
        &Vector3f{x:0.0,y:0.0,z:0.0},
        1.0,
        0,
        -1,
        false,
        0
    );
    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    0.into()
}

pub fn install() {    
    captoss_hop_init::install();
    captoss_hop_pre::install();
    captoss_hop_main::install();
    captoss_hop_end::install();
    captoss_hop_exec::install();
}
