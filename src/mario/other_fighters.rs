use crate::imports::imports_agent::*;

unsafe fn is_cappy(object_boma: *mut BattleObjectModuleAccessor) -> bool {
    if utility::get_kind(&mut *object_boma) == *WEAPON_KIND_KOOPAJR_CANNONBALL {
        let owner_id = WorkModule::get_int(object_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
        let owner_boma = smash::app::sv_battle_object::module_accessor(owner_id);
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_MARIO {
            return true;
        }
    }
    return false;
}

unsafe fn ac_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    
    if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_N_SEARCH {
        let object_id = WorkModule::get_int(fighter.module_accessor,*FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID) as u32;
        if object_id == 0 || object_id == 0x50000000 {return;}
        let object_boma = sv_battle_object::module_accessor(object_id);
        if is_cappy(object_boma) {
            WorkModule::set_int(fighter.module_accessor, 0x50000000, *FIGHTER_MURABITO_INSTANCE_WORK_ID_INT_TARGET_OBJECT_ID);

            let weapon = get_fighter_common_from_accessor(object_boma);
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

unsafe fn rosa_update(fighter: &mut L2CFighterCommon) {
    let boma = fighter.module_accessor;
    let status_kind = StatusModule::status_kind(boma);

    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        let object_id = WorkModule::get_int(fighter.module_accessor,FIGHTER_ROSETTA_STATUS_SPECIAL_LW_INT_CAPTURE_OBJECT_ID) as u32;
        let object = get_battle_object_from_id(object_id);
        let object_boma = sv_battle_object::module_accessor(object_id);
        println!("Hmmm");
        if is_cappy(object_boma) {
            println!("Cappy?");
            let cappy_status = StatusModule::status_kind(object_boma);
            if cappy_status != CAPTOSS_STATUS_KIND_SWALLOWED {
                println!("Swallow");
                StatusModule::change_status_force(object_boma, CAPTOSS_STATUS_KIND_SWALLOWED,false);
            }
        }
    }
}


#[line("murabito", main)]
fn murabito_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        ac_update(fighter);
    }
}
#[line("shizue", main)]
fn shizue_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        ac_update(fighter);
    }
}
#[line("rosetta", main)]
fn rosetta_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        rosa_update(fighter);
    }
}

pub fn install() {
    murabito_frame::install();
    shizue_frame::install();
    //rosetta_frame::install();
}