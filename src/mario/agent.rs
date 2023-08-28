use crate::imports::imports_agent::*;

unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_MARIO {
        return;
    }
}
#[event("mario_capptoss", initialize)]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
        println!("Agent init");
        //GetVarManager::reset_var_module_by_object_id(fighter.battle_object_id, false);
    }
}
#[event(start)]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}

pub fn install() {
    agent_init::install();
    agent_reset::install();
}