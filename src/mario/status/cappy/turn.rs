use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_TURN)]
unsafe fn captoss_turn_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {

    /*
    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("accel"));
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
    let speed_param = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_SPEED);
    let angle = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE); */
    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("brake_x"))*3.5;
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
    let speed_current = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    let lr = PostureModule::lr(weapon.module_accessor);
    PostureModule::set_lr(weapon.module_accessor, lr);
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
        *FS_SUCCEEDS_KEEP_EFFECT,
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
    GroundModule::set_ignore_boss(weapon.module_accessor, true);
    GroundModule::set_passable_check(weapon.module_accessor, false);
    GroundModule::set_collidable(weapon.module_accessor, false);
    JostleModule::set_status(weapon.module_accessor, false);

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

        let owner = get_owner_boma(weapon);
        let owner_status = StatusModule::status_kind(owner);

        //println!("Distance: {dis} / {min_dis}");
        if dis <= min_dis-4.0
        //&& ![FIGHTER_MARIO_STATUS_KIND_CAPJUMP].contains(&owner_status) 
        {
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
    if captoss_delete_if_orphaned(weapon) {
        return 0.into();
    }
    KineticModule::clear_speed_all(weapon.module_accessor);

    let sum_speed_len = KineticModule::get_sum_speed_length(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
    let turn_speed = 1.5;

    let owner = get_owner_boma(weapon);
    let owner_pos = *PostureModule::pos(owner);
    let owner_offset_y = WorkModule::get_param_float(owner, hash40("height"), 0) / 2.0;

    let pos = *PostureModule::pos(weapon.module_accessor);
    let offset_y = 1.25;

    let mut direction_full = Vector2f{x:owner_pos.x-pos.x, y: (owner_pos.y+owner_offset_y)-(pos.y+offset_y)};
    let direction_len = sv_math::vec2_length(direction_full.x,direction_full.y);
    let direction = Vector2f{x:direction_full.x/direction_len,y:direction_full.y/direction_len};
    //let direction = sv_math::vec2_normalize(direction_full.x,direction_full.y);

    let new_speed_len = 3.0;//sum_speed_len.abs();
    let new_speed_x = direction.x*new_speed_len;
    let new_speed_y = direction.y*new_speed_len;

    PostureModule::set_lr(weapon.module_accessor, direction_full.x.signum());
    let lr = PostureModule::lr(weapon.module_accessor);

    let mut lr_fix = 1.0;
    if owner_pos.x > pos.x && lr > 0.0 {
        lr_fix = -1.0;
    }
    else if owner_pos.x < pos.x && lr < 0.0 {
        lr_fix = -1.0;
    }
    //SET_SPEED_EX(weapon,new_speed_x,new_speed_y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    PostureModule::add_pos(weapon.module_accessor, &Vector3f{x:new_speed_x,y:new_speed_y,z:0.0});
    //WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL
    

    println!("Dir: {}/{}, Speed: {}/{}",direction.x,direction.y,new_speed_x*lr_fix,new_speed_y);
    /* 

    let mut direction_full = Vector2f{x:owner_pos.x-pos.x, y: owner_pos.y-pos.y};
    let direction = sv_math::vec2_normalize(direction_full.x,direction_full.y);
    println!("Pos: {},{} Owner: {}, Dir: {},{}",pos.x,pos.y,owner_pos.x,owner_pos.y,direction.x,direction.y);

    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y= KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let normal_speed = sv_math::vec2_normalize(speed_x,speed_y);

    //let mut rotateAmount = sv_math::vec3_cross(direction.x, direction.y, 0.0, normal_speed.x,normal_speed.y, 0.0);
    let mut rotateAmount = 0.2;
    let vec_lerp = Vector2f{x: lerp(normal_speed.x,direction.x,0.2), y: lerp(normal_speed.y,direction.y, rotateAmount)};

    let new_speed_len = sum_speed_len.max(speed_max);
    let new_speed = Vector2f{x:vec_lerp.x*new_speed_len, y: vec_lerp.y*new_speed_len};
    //println!("Speed: {},{} Dir: {},{}",new_speed.x,new_speed.y,direction.x,direction.y);

    SET_SPEED_EX(weapon,new_speed.x,new_speed.y, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    
    let current_follow_dist = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    let turn_follow_dist = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("turn_follow_dist"));
    if current_follow_dist <= turn_follow_dist {
        WorkModule::set_int(weapon.module_accessor, 0, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);
    }
    if WorkModule::get_int(weapon.module_accessor, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME) < 0 {
        //bruh idk
    }*/
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
