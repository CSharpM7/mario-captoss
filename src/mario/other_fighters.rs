use crate::imports::imports_agent::*;

pub unsafe extern "C" fn ac_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    
    if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let object_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        if object_id == 0 || object_id == 0x50000000 {return;}
        let object_boma = sv_battle_object::module_accessor(object_id);
        if is_cappy(object_boma) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_FAILURE,false);
            WorkModule::set_int(fighter.module_accessor, 0x50000000, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);

            let mario_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_ACTIVATE_FOUNDER_ID) as u32;
            if sv_battle_object::is_active(mario_id) {
                let mario = get_battle_object_from_id(mario_id);
                let mario_boma = (*mario).module_accessor;
                let life = WorkModule::get_param_int(mario_boma, hash40("param_captoss"), hash40("life"));
                VarModule::set_int(mario, mario::instance::int::CAP_TIMER,life);
            }

            let weapon = get_fighter_common_from_accessor(object_boma);
            macros::STOP_SE(weapon, Hash40::new("se_item_boomerang_throw"));
            smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            let pos = *PostureModule::pos(object_boma);
            EffectModule::req(
                object_boma,
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
    }
}

const FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID: i32 = 0x11000006;

pub unsafe extern "C" fn rosa_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let object_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID) as u32;
        let object = get_battle_object_from_id(object_id);
        let object_boma = sv_battle_object::module_accessor(object_id);
        if is_cappy(object_boma) {
            let cappy_status = StatusModule::status_kind(object_boma);
            if cappy_status != CAPTOSS_STATUS_KIND_SWALLOWED {
                StatusModule::change_status_force(object_boma, CAPTOSS_STATUS_KIND_SWALLOWED,false);
            }
        }
    }
}

pub fn install() {
    Agent::new("murabito")
        .on_line(Main, ac_update)
        .install();
    Agent::new("shizue")
        .on_line(Main, ac_update)
        .install();
}