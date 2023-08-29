#![allow(non_snake_case)]

use smash::lib::lua_const::BATTLE_OBJECT_CATEGORY_FIGHTER;

use {
    smash::{
        app:: *,
    },
    //custom_var::*,
    //crate::data::gamemode::*,
    crate::imports::imports_agent::*
};

pub unsafe fn can_install(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let cat = smash::app::utility::get_category(&mut *module_accessor);
    if cat == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let fighter_kind = utility::get_kind(&mut *module_accessor);
        if fighter_kind == *FIGHTER_KIND_MARIO {
            return true;
        }
    }
    /* 
    else if cat == *BATTLE_OBJECT_CATEGORY_WEAPON {
        let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);  
        let owner_kind = utility::get_kind(&mut *owner_boma);
        if owner_kind == *FIGHTER_KIND_KOOPA {
            return true;
        }
    }
    */
    
    return false;
}


#[skyline::hook(offset = 0x3af9f0)]
pub unsafe fn battleobjectmoduleaccessor__start_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    original!()(module_accessor, param_1);
    let object_id = (*module_accessor).battle_object_id;
    if can_install(module_accessor) {
        GetVarManager::reset_var_module_by_object_id(object_id, false);
    }
}

#[skyline::hook(offset = 0x3afde0)]
pub unsafe fn battleobjectmoduleaccessor__end_modules(module_accessor: *mut BattleObjectModuleAccessor, param_1: u32) {
    let object_id = (*module_accessor).battle_object_id;
    if can_install(module_accessor) {
        GetVarManager::remove_var_module_by_object_id(object_id);
    }
    original!()(module_accessor, param_1)
}

pub fn install() {
    //if is_HDR() {return;}
    skyline::install_hooks!(
        battleobjectmoduleaccessor__start_modules,
        battleobjectmoduleaccessor__end_modules
    );
}