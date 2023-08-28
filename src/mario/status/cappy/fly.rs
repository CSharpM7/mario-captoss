use crate::imports::imports_agent::*;
use super::*;
pub const NEXT_STATUS: i32 = CAPTOSS_STATUS_KIND_HOLD;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let founder = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = get_owner_boma(weapon);
    let lr = PostureModule::lr(owner_boma);
    PostureModule::set_lr(weapon.module_accessor, lr);
    PostureModule::set_scale(weapon.module_accessor, 1.375,false);
    
    let roty = if lr > 0.0 {0.0} else {180.0};
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x:0.0,y:roty,z:90.0}, 0);

    println!("FLY: Init (Lr: {lr})");

    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("accel"));
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
    let speed_param = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_SPEED);
    let angle = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);

    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed*lr
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_max
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -accel*lr
    );
    /*
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    let speed_rot = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("rot_speed"));
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        speed_rot*lr
    ); */
    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    println!("FLY: PRE");
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
        *FS_SUCCEEDS_KEEP_ROT_Y_LR,
    );
    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    println!("FLY: MAIN");
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP);
    if StopModule::is_stop(weapon.module_accessor){
        captoss_ground_check(weapon);
    }

    MotionModule::change_motion(weapon.module_accessor, Hash40::new("fly"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(captoss_fly_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_fly_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    MotionModule::update_dynamic_skeleton_without_complete_matrix(weapon.module_accessor);
    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));
    let speed_min_mul = speed_min*1.0;
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP)
    {
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_HOP, false);
        return 0.into();
    }
    if sum_speed_len <= speed_min_mul {
        if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_APPLY_FLY_SPEED) {
            println!("Turn?!?");
            //weapon.change_status(CAPTOSS_STATUS_KIND_HOP.into(),false.into());
            StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
            return 0.into();
        }
    }
    else{
        WorkModule::on_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_APPLY_FLY_SPEED);
    }
    if AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_HIT){
        WorkModule::on_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    }
    if WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION) {
        if StopModule::is_stop(weapon.module_accessor){
            let can_penetrate = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("is_penetration"))==0;
            if !can_penetrate {
                println!("Hit!");
                //StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
                //return 0.into();
            }
        }
    }
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    if captoss_ground_check(weapon) {
        //uhhh
        println!("Touched the ground?!?");
        //StatusModule::change_status_force(weapon.module_accessor, NEXT_STATUS, false);
        //return 0.into();
    }

    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_FLY)]
unsafe fn captoss_fly_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    captoss_fly_init::install();
    captoss_fly_pre::install();
    captoss_fly_main::install();
    captoss_fly_end::install();

}
