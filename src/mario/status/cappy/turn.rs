use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_TURN)]
unsafe fn captoss_turn_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {

    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("accel"));
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
    let speed_param = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_SPEED);
    let angle = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    let speed_current = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let lr = PostureModule::lr(weapon.module_accessor);
    PostureModule::set_lr(weapon.module_accessor, lr);

    let roty = if lr < 0.0 {0.0} else {180.0};
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x:0.0,y:roty,z:0.0}, 0);

    println!("TURN: Init (Lr: {lr})");

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
        speed_max*lr
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -accel*lr
    );
    /* 
    let speed_rot = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("rot_speed"));
    //let speed_rot = KineticModule::get_sum_speed_length(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        speed_rot
    );*/
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_TURN)]
unsafe fn captoss_turn_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    println!("TURN: PRE");
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

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_TURN)]
unsafe fn captoss_turn_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    println!("TURN: MAIN");
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_INFLICTION);
    WorkModule::off_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_TO_HOP);
    if StopModule::is_stop(weapon.module_accessor){
        captoss_ground_check(weapon);
    }
    //AttackModule::clear_all(weapon.module_accessor);
    MotionModule::change_motion_inherit_frame_keep_rate(weapon.module_accessor as _, Hash40::new("turn"), -1.0,1.0,0.0);
    weapon.fastshift(L2CValue::Ptr(captoss_turn_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_turn_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if StopModule::is_stop(weapon.module_accessor){
        return 0.into();
    }

    let correct = GroundModule::get_correct(weapon.module_accessor);
    let has_link = LinkModule::is_link(weapon.module_accessor, *LINK_NO_ARTICLE);
    if has_link {
        let parent = LinkModule::get_parent_id(weapon.module_accessor, *LINK_NO_ARTICLE,true);
        let reflect = WorkModule::is_flag(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLAG_REFLECT);
        let parent_pos = get_parent_pos(weapon);
        let pos_x = PostureModule::pos_x(weapon.module_accessor);
        let pos_y = PostureModule::pos_y(weapon.module_accessor);
        let dis = sv_math::vec2_distance(parent_pos.x,parent_pos.y,pos_x,pos_y);
        let min_dis = if !reflect {13.0} else {11.0};

        //println!("Distance: {dis} / {min_dis}");
        if dis <= min_dis {
            smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return 0.into();
        }
    }
    else{
        //println!("Lmao no link");
    }
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }

    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_TURN)]
unsafe fn captoss_turn_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
    let speed_mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_mul"));
    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("accel"));
    let speed_min_mul = speed_min*1.0;
    
    let current_follow_dist = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    let turn_follow_dist = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("turn_follow_dist"));
    if current_follow_dist <= turn_follow_dist {
        WorkModule::set_int(weapon.module_accessor, 0, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);
    }
    if WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME) < 0 {
        //bruh idk
    }
    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_TURN)]
unsafe fn captoss_turn_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
pub fn install() {    
    captoss_turn_init::install();
    captoss_turn_pre::install();
    captoss_turn_main::install();
    captoss_turn_exec::install();
    captoss_turn_end::install();
}
