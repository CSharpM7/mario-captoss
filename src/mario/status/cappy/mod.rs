mod start;
mod haved;
mod fly;
mod hold;
mod hop;
mod turn;
mod jump;
mod swallowed;
mod pocket;
use crate::imports::imports_status::*;
use super::*;

pub fn install() {
    start::install();
    haved::install();
    fly::install();
    hold::install();
    hop::install();
    swallowed::install();
    turn::install();
    jump::install();
    pocket::install();
}


unsafe extern "C" fn captoss_ground_check(weapon: &mut smashline::L2CWeaponCommon) -> bool{

    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
    {
        weapon.clear_lua_stack();
        macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
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
unsafe extern "C" fn captoss_attacked_check(weapon: &mut smashline::L2CWeaponCommon) -> bool{

    let hit_normal = AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_HIT);
    let hit_attack = AttackModule::is_infliction(weapon.module_accessor, *COLLISION_KIND_MASK_ATTACK);
    //println!("Hit: {hit_normal} Attack: {hit_attack}");
    if hit_attack
    {
        return true;
    }
    return false;
}

unsafe extern "C" fn captoss_swallowed_check(weapon: &mut smashline::L2CWeaponCommon) -> bool{
    let swallowed = WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED);
    if swallowed {
        //new status
        StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_SWALLOWED, false);
    }
    return swallowed;
}
unsafe extern "C" fn captoss_reflect_check(weapon: &mut smashline::L2CWeaponCommon) -> bool{
    let reflected = AttackModule::is_infliction(weapon.module_accessor,*COLLISION_KIND_MASK_REFLECTOR);
    let was_reflected = WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL);

    let toReturn = (reflected && !was_reflected);
    if toReturn == true {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL);
    }
    return toReturn;
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
        macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
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
    let mut min_dis = if !is_reflected {11.0} else {9.0};
    let cap_status = StatusModule::status_kind(weapon.module_accessor);
    let cap_team = TeamModule::team_no(weapon.module_accessor);
    let owner_boma = get_owner_boma(weapon);
    let owner_scale = PostureModule::scale(owner_boma);
    let owner_team = TeamModule::team_no(owner_boma);
    min_dis *= owner_scale;

    if captoss_distance_to_owner(weapon) < min_dis 
    && cap_team == owner_team
    {
        let owner = get_fighter_common_from_accessor(&mut *owner_boma);
        let owner_object = owner.battle_object;
        let owner_status = StatusModule::status_kind(owner_boma);
        let owner_frame = MotionModule::frame(owner_boma);

        let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(owner_boma,Hash40::new_raw(MotionModule::motion_kind(owner_boma)),false) as f32;
        let can_cap = owner_frame <= cancel_frame; //WorkModule::is_flag(owner_boma,*FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_CONTINUE);

        let speed_current = WorkModule::get_float(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLOAT_CHARGE);
        let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_max"));
        //Give some leeway to be able to Cap Jump
        let turn_cap = (cap_status == CAPTOSS_STATUS_KIND_TURN && speed_current.abs() < speed_max*0.375);
        
        if [*FIGHTER_STATUS_KIND_CATCH_WAIT,*FIGHTER_STATUS_KIND_CATCH_ATTACK,*FIGHTER_STATUS_KIND_THROW].contains(&owner_status) 
        && cap_status == CAPTOSS_STATUS_KIND_HOLD {
            return false;
        }
        let is_damaged = is_damage_status(owner_boma) || is_captured_status(owner_boma);
        let owner_speed_y = KineticModule::get_sum_speed_y(owner_boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let can_catch = WorkModule::is_enable_transition_term_group(owner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP)
        || WorkModule::is_enable_transition_term_group(owner_boma, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL)
        || [FIGHTER_MARIO_STATUS_KIND_CAPDIVE,*FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&owner_status);

        let can_transition = can_catch && (![*FIGHTER_STATUS_KIND_JUMP,*FIGHTER_STATUS_KIND_JUMP_AERIAL,*FIGHTER_STATUS_KIND_FLY,
            FIGHTER_MARIO_STATUS_KIND_CAPDIVE].contains(&owner_status));

        //println!("CapStatus: {cap_status} CanCap: {can_cap} CanTrans: {can_transition}");
        if VarModule::is_flag(owner_object, mario::instance::flag::CAPJUMP_ENABLED)
        && (cap_status == CAPTOSS_STATUS_KIND_HOLD || (turn_cap) || (owner_status == FIGHTER_MARIO_STATUS_KIND_CAPDIVE && can_cap)) 
        && !is_damaged 
        && can_catch
        {
            if captoss_distance_to_owner(weapon) < min_dis-3.0 {
                if VarModule::is_flag(owner_object, mario::instance::flag::CAPJUMP_ENABLED) {
                    //VarModule::on_flag(weapon.battle_object, mario_cappy::instance::flag::CAPDIVE_ENABLE_ON_RETURN);
                }
                VarModule::off_flag(owner_object, mario::instance::flag::CAPJUMP_ENABLED);
                WorkModule::off_flag(owner_boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
                let pos = *PostureModule::pos(weapon.module_accessor);
                let owner_pos = *PostureModule::pos(owner_boma);
                PostureModule::set_pos(owner_boma,&Vector3f{x: owner_pos.x, y: pos.y+1.0, z: owner_pos.z});
                WorkModule::on_flag(owner_boma, *FIGHTER_MARIO_INSTANCE_WORK_ID_FLAG_SPECIAL_S_HOP);
                StatusModule::change_status_force(owner_boma, FIGHTER_MARIO_STATUS_KIND_CAPJUMP, false);

                StatusModule::change_status_force(weapon.module_accessor, CAPTOSS_STATUS_KIND_JUMP, false);
                KineticModule::clear_speed_all(weapon.module_accessor);
                return true;
            }
        }
        else {
            if can_transition {
                StatusModule::change_status_force(owner_boma, FIGHTER_MARIO_STATUS_KIND_CAPCATCH, false);
                VarModule::off_flag(owner_object, mario::instance::flag::HATLESS);
            }
            else{
                captoss_effect_reappear(weapon);
                macros::PLAY_SE(weapon, Hash40::new("se_item_boomerang_catch"));
            }

            macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
            EffectModule::kill_all(weapon.module_accessor, *EFFECT_SUB_ATTRIBUTE_NONE as u32, true, false);
            smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            return true;
        }
    }
    return false;
}

unsafe extern "C" fn captoss_effect_disappear(weapon: &mut smashline::L2CWeaponCommon) {
    macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
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
    return;
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