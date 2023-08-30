mod start;
mod haved;
mod fly;
mod hold;
mod hop;
mod turn;
mod jump;
use crate::imports::imports_agent::*;

pub fn install() {
    start::install();
    haved::install();
    fly::install();
    hold::install();
    hop::install();
    turn::install();
    jump::install();
}


unsafe extern "C" fn captoss_ground_check(weapon: &mut smashline::L2CWeaponCommon) -> bool{

    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
    {
        weapon.clear_lua_stack();
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        return true;
    }
    /* 
    let died = captoss_dec_life(weapon);
    if died {
        weapon.clear_lua_stack();
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }*/
    return false;
}
unsafe extern "C" fn captoss_dec_life(weapon: &mut smashline::L2CWeaponCommon) ->bool{
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    return life <= 0;
}
unsafe extern "C" fn captoss_delete_if_orphaned(weapon: &mut smashline::L2CWeaponCommon) -> bool{
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
    let mut toreturn = false;
    if !sv_battle_object::is_active(owner_id) {
        toreturn = true;
    }
    else{
        let owner_boma = get_owner_boma(weapon);
        let status = StatusModule::status_kind(owner_boma);
        if [*FIGHTER_STATUS_KIND_DEAD,*FIGHTER_STATUS_KIND_REBIRTH].contains(&status) {
            toreturn = true;
        }
    }
    if toreturn == true {
        weapon.clear_lua_stack();
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        captoss_effect_disappear(weapon);
    }
    return toreturn;
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
unsafe extern "C" fn captoss_distance_to_owner(weapon: &mut smashline::L2CWeaponCommon) -> f32 {
    let owner_boma = get_owner_boma(weapon);
    let owner_offset_y = WorkModule::get_param_float(owner_boma, hash40("height"), 0) / 2.0;
    let parent_pos = *PostureModule::pos(owner_boma);

    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);
    let offset_y = 1.25;

    return sv_math::vec2_distance(parent_pos.x,parent_pos.y+owner_offset_y,pos_x,pos_y+offset_y);
}
unsafe extern "C" fn captoss_owner_is_mario(weapon: &mut smashline::L2CWeaponCommon) -> bool {
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
	return sv_battle_object::kind(owner_id) == *FIGHTER_KIND_MARIO
}
unsafe extern "C" fn captoss_check_recapture(weapon: &mut smashline::L2CWeaponCommon) -> bool {
    let is_reflected = WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL);
    let min_dis = if !is_reflected {11.0} else {9.0};
    let cap_status = StatusModule::status_kind(weapon.module_accessor);

    if captoss_distance_to_owner(weapon) < min_dis {
        let owner_boma = get_owner_boma(weapon);
        let owner = get_fighter_common_from_accessor(&mut *owner_boma);
        let owner_object = owner.battle_object;
        let owner_status = StatusModule::status_kind(owner_boma);
        let owner_frame = MotionModule::frame(owner_boma);
        let can_cap = WorkModule::is_flag(owner_boma,*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);
        
        if [*FIGHTER_STATUS_KIND_CATCH_WAIT,*FIGHTER_STATUS_KIND_CATCH_ATTACK,*FIGHTER_STATUS_KIND_THROW].contains(&owner_status) 
        && cap_status == CAPTOSS_STATUS_KIND_HOLD {
            return false;
        }
        let is_damaged = is_damage_status(owner_boma) || is_captured_status(owner_boma);
        if VarModule::is_flag(owner_object, mario::instance::flag::CAPJUMP_ENABLED)
        //if WorkModule::is_flag(owner_boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP)
        && (cap_status == CAPTOSS_STATUS_KIND_HOLD || (owner_status == *FIGHTER_MARIO_STATUS_KIND_NUM + FIGHTER_MARIO_STATUS_KIND_CAPDIVE && can_cap)) 
        && !is_damaged 
        && StatusModule::prev_status_kind(weapon.module_accessor, 0) != CAPTOSS_STATUS_KIND_JUMP {
            if captoss_distance_to_owner(weapon) < min_dis-3.0 {
                VarModule::off_flag(owner_object, mario::instance::flag::CAPJUMP_ENABLED);
                WorkModule::off_flag(owner_boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
                let pos = *PostureModule::pos(weapon.module_accessor);
                let owner_pos = *PostureModule::pos(owner_boma);
                //PostureModule::add_pos(owner_boma, &Vector3f{x: 0.0, y: 2.0, z: 0.0});
                PostureModule::set_pos(owner_boma,&Vector3f{x: owner_pos.x, y: pos.y+1.0, z: owner_pos.z});
                WorkModule::on_flag(owner_boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
                //owner.change_status(FIGHTER_MARIO_STATUS_KIND_CAPJUMP.into(), false.into()); 
                StatusModule::change_status_force(owner_boma, *FIGHTER_MARIO_STATUS_KIND_NUM+FIGHTER_MARIO_STATUS_KIND_CAPJUMP, false);

                if cap_status == CAPTOSS_STATUS_KIND_HOLD {
                    StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_JUMP, false);
                    KineticModule::clear_speed_all(weapon.module_accessor);
                }
                return true;
            }
        }
        else {
            smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            captoss_effect_reappear(weapon);
            macros::PLAY_SE(weapon, Hash40::new("se_item_boomerang_catch"));
            return true;
        }
    }
    return false;
}

unsafe extern "C" fn captoss_effect_disappear(weapon: &mut smashline::L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(
        weapon.module_accessor,
        Hash40::new("sys_erace_smoke"),
        &Vector3f{x:pos.x,y:pos.y+2.0,z:pos.z},
        &Vector3f{x:0.0,y:0.0,z:0.0},
        0.625,
        0,
        -1,
        false,
        0
    );
}
unsafe extern "C" fn captoss_effect_reappear(weapon: &mut smashline::L2CWeaponCommon) {
    let pos = PostureModule::pos(weapon.module_accessor);
    
    if captoss_delete_if_orphaned(weapon) {
        return;
    }
    let owner = get_owner_boma(weapon);

    EffectModule::req_follow(
        owner,
        Hash40::new("sys_item_arrival"),
        Hash40::new("hat"),
        &VECTOR_ZERO,
        &VECTOR_ZERO,
        0.35,
        false,
        0,
        0,
        0,
        0,
        0,
        false,
        false
    );
}