use crate::imports::imports_agent::*;
pub const SUB_STATUS3:                     i32 = 0x13;
pub const SUB_STATUS2:                     i32 = 0x14;
pub const SUB_STATUS:                      i32 = 0x15;
pub const VECTOR3_ZERO : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

extern "C"{
    /// gets whether we are in training mode
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}

pub unsafe fn get_entry_from_boma(boma: *mut BattleObjectModuleAccessor) -> u32 {
    return WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as u32
}
pub unsafe fn get_entry(fighter: &mut L2CAgentBase) -> u32 {
    return get_entry_from_boma(fighter.module_accessor);
}

pub unsafe fn get_owner_boma(weapon: &mut L2CAgentBase) -> *mut BattleObjectModuleAccessor {
    return &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
}

pub unsafe fn get_article_boma(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int) -> *mut BattleObjectModuleAccessor {
    let article = ArticleModule::get_article(boma, article_type);
    let object_id = smash::app::lua_bind::Article::get_battle_object_id(article) as u32;
    return sv_battle_object::module_accessor(object_id);
}
pub unsafe fn snap_article(boma: *mut BattleObjectModuleAccessor, article_type: skyline::libc::c_int, fighterBone: Hash40, articleBone: Hash40) {
    let mut pos = Vector3f{x:0.0,y:0.0,z:0.0};
    let offset = ModelModule::joint_global_offset_from_top(boma, fighterBone, &mut pos);        
    let newPos = Vector3f{x: PostureModule::pos_x(boma) + pos.x, y: PostureModule::pos_y(boma) + pos.y + 0.0, z: PostureModule::pos_z(boma) + pos.z};
    let article_boma = get_article_boma(boma, article_type);
    ArticleModule::set_pos(boma, article_type, newPos);
    ModelModule::set_joint_translate(article_boma, articleBone, &newPos, true,false);
}


pub unsafe fn change_status_by_situation(fighter: &mut L2CFighterBase, status_ground: L2CValue, status_air: L2CValue, clear_cat: L2CValue) {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        fighter.change_status(status_air,clear_cat);
    }
    else{
        fighter.change_status(status_ground,clear_cat);
    }
}

pub fn get_fighter_common_from_accessor<'a>(boma: &'a mut BattleObjectModuleAccessor) -> &'a mut L2CFighterCommon {
    unsafe {
        let lua_module = *(boma as *mut BattleObjectModuleAccessor as *mut u64).add(0x190 / 8);
        std::mem::transmute(*((lua_module + 0x1D8) as *mut *mut L2CFighterCommon))
    }
}
pub unsafe fn lerp(a: f32, b: f32, t: f32) -> f32 {
    return a + ((b-a)*t);
}