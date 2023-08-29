use crate::imports::imports_acmd::*;

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_START)]
unsafe fn captoss_start_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_NONE as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0,
    );
    0.into()
}

#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_START)]
unsafe fn captoss_start_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    weapon.fastshift(L2CValue::Ptr(captoss_start_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_start_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}
#[smashline::new_status("mario_captoss", CAPTOSS_STATUS_KIND_START)]
unsafe fn captoss_start_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub fn install() {    
    captoss_start_pre::install();
    captoss_start_main::install();
    captoss_start_end::install();
}
