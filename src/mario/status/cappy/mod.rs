mod start;
mod haved;
mod fly;
mod hold;
mod hop;
mod turn;
use crate::imports::imports_agent::*;

pub fn install() {
    start::install();
    haved::install();
    fly::install();
    hold::install();
    hop::install();
    turn::install();
}


unsafe extern "C" fn captoss_ground_check(weapon: &mut smashline::L2CWeaponCommon) -> bool{
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32)
    {
        println!("Touched!");
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
unsafe extern "C" fn get_parent_pos(weapon: &mut smashline::L2CWeaponCommon) -> Vector3f{
    let owner_boma = get_owner_boma(weapon);
    return *PostureModule::pos(owner_boma);
}
unsafe extern "C" fn captoss_distance_to_owner(weapon: &mut smashline::L2CWeaponCommon) -> f32 {
    let parent_pos = get_parent_pos(weapon);
    let pos_x = PostureModule::pos_x(weapon.module_accessor);
    let pos_y = PostureModule::pos_y(weapon.module_accessor);

    return sv_math::vec2_distance(parent_pos.x,parent_pos.y,pos_x,pos_y);
}