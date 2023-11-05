use crate::imports::imports_agent::*;
pub unsafe extern "C" fn is_cappy(object_boma: *mut BattleObjectModuleAccessor) -> bool {
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