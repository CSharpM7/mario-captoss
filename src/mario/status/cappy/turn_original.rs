use crate::imports::imports_agent::*;
use super::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_TURN)]
unsafe fn captoss_turn_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    println!("TURN: Init");
    let founder = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let owner_boma = get_owner_boma(weapon);

    let follow_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("follow_frame"));
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed"));
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
    let accel = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("accel"));
    let speed_curr = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_SPEED);
    let angle = WorkModule::get_float(weapon.module_accessor,*WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_ANGLE);
    WorkModule::set_int(weapon.module_accessor, follow_frame, *WN_LINK_BOOMERANG_TURN_WORK_INT_FOLLOW_FRAME);

    let rot_node = *WN_LINK_BOOMERANG_POSTURE_ROT_NODE_TOPN;
    let rot_x = PostureModule::rot_x(weapon.module_accessor, rot_node);
    let rot_y = PostureModule::rot_x(weapon.module_accessor, rot_node);
    let rot_z = PostureModule::rot_x(weapon.module_accessor, rot_node);
    let angle_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("angle_x_turn"));
    PostureModule::set_rot(weapon.module_accessor, &Vector3f{x:rot_x, y:rot_x, z:rot_z}, rot_node);

    let max_dist = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("turn_dist"));
    WorkModule::set_float(weapon.module_accessor, max_dist, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    let angle_back = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("angle_x_back"));

    let frame = speed_min-speed_curr;
    //WorkModule::set_int(weapon.module_accessor, param_turn, *WN_LINK_BOOMERANG_TURN_WORK_INT_BACK_ROT_FRAME);
    //I have no fucking idea
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        0.0
    );
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        0.0
    );
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
    MotionModule::change_motion(weapon.module_accessor as _, Hash40::new("turn"), 0.0, 1.0, false, 0.0, false, false);
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
        if reflect == true {
            //uhhhh
        }
        else {
            let teamowner = TeamModule::team_owner_id(weapon.module_accessor);
            if teamowner == parent
            {
                println!("Has parent");
                let parent_pos = get_linked_parent_pos(weapon);
                let pos_x = PostureModule::pos_x(weapon.module_accessor);
                let pos_y = PostureModule::pos_y(weapon.module_accessor);
                let dis = sv_math::vec2_distance(parent_pos.x,parent_pos.y,pos_x,pos_y);
                if dis <= 9.0 {
                    //If you can return, then return
                    smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
                    return 0.into();
                }
            }
        }

        if correct != *GROUND_CORRECT_KIND_NONE {
            //Changes ground correct based on the position of the parent?
        }
    }
    /* 
    let max_dist = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("turn_dist"));
    let current_dist = WorkModule::get_float(weapon.module_accessor, *WN_LINK_BOOMERANG_INSTANCE_WORK_ID_FLOAT_TURN_DIST);
    if current_dist >= max_dist {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        return 0.into();
    }
    */
    if correct != *GROUND_CORRECT_KIND_NONE {
        if StatusModule::is_changing(weapon.module_accessor) {
            return 0.into();
        }
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
            GroundModule::set_correct(weapon.module_accessor,GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
        }
        if GroundModule::is_touch(weapon.module_accessor, (*GROUND_TOUCH_FLAG_DOWN | *GROUND_TOUCH_FLAG_UP)  as u32) {
            smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
            return 0.into();
        }
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

unsafe extern "C" fn get_linked_parent_pos(weapon: &mut smashline::L2CWeaponCommon) -> Vector3f{
    lua_args!(weapon,FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_X, LINK_NO_ARTICLE, true);
    sv_module_access::link(weapon.lua_state_agent);
    let parent_x = weapon.pop_lua_stack(1).get_f32();
    
    lua_args!(weapon,FL_MA_MSC_LINK_GET_PARENT_MODEL_NODE_GLOBAL_POSITION_Y, LINK_NO_ARTICLE, true);
    sv_module_access::link(weapon.lua_state_agent);
    let parent_y = weapon.pop_lua_stack(1).get_f32();

    return Vector3f{x: parent_x, y: parent_y, z: 0.0};
}

pub fn install() {    
    captoss_turn_init::install();
    captoss_turn_pre::install();
    captoss_turn_main::install();
    captoss_turn_exec::install();

}
