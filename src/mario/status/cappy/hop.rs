use crate::imports::imports_status::*;
use super::*;

pub unsafe extern "C" fn captoss_hop_init(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    GroundModule::set_rhombus_offset(weapon.module_accessor, &Vector2f::new(0.0, 3.0));
    
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_captoss"), hash40("life"));
    //let speed_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_speed_x"));
    let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN).abs();
    let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    //let speed_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("hop_speed_y"));
    let speed_y_limit = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("limit_gravity"));
    let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("brake_x"));
    let accel_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("gravity"));
    let speed_min = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("speed_min"));

    let lr = PostureModule::lr(weapon.module_accessor);
    WorkModule::set_int(weapon.module_accessor, life,*WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    KineticModule::clear_speed_all(weapon.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x*lr,
        speed_y
    );
    sv_kinetic_energy!(
        set_accel,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        -accel_x*lr*0.5,
        -accel_y
    );
    sv_kinetic_energy!(
        set_limit_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
        speed_x,
        speed_y_limit*10.0
    );
    /*
    sv_kinetic_energy!(
        set_speed,
        weapon,
        WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL,
        speed_x,
        0.0
    ); */
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    //KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
    
    0.into()
}

pub unsafe extern "C" fn captoss_hop_pre(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor as _,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        smashline::skyline_smash::app::GroundCliffCheckKind(0),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    0.into()
}

pub unsafe extern "C" fn captoss_hop_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    EffectModule::detach_all(weapon.module_accessor, 5);
    //MotionModule::change_motion_inherit_frame_keep_rate(weapon.module_accessor, Hash40::new("hop"), -1.0,1.0,0.0);
    AttackModule::clear_all(weapon.module_accessor);
    weapon.fastshift(L2CValue::Ptr(captoss_hop_main_status_loop as *const () as _)).into()
}

unsafe extern "C" fn captoss_hop_main_status_loop(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let currentRate = MotionModule::rate(weapon.module_accessor);
    let lerpRate = if StatusModule::situation_kind(weapon.module_accessor) == *SITUATION_KIND_GROUND {0.1} else {0.05};
    let newRate = lerp(currentRate,0.0,lerpRate);
    MotionModule::set_rate(weapon.module_accessor, newRate);

    captoss_check_recapture(weapon);
    0.into()
}

pub unsafe extern "C" fn captoss_hop_end(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    0.into()
}

pub unsafe extern "C" fn captoss_hop_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    let died = captoss_dec_life(weapon);
    if died {
        smash_script::notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        captoss_effect_disappear(weapon);
        captoss_effect_reappear(weapon);
        return 0.into();
    }
    
    let speed_current_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_current_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if speed_current_x.abs() < 0.1 {
        sv_kinetic_energy!(
            set_speed,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0,
            speed_current_y
        );
        let accel_y = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("gravity"));
        sv_kinetic_energy!(
            set_accel,
            weapon,
            WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
            0.0,
            -accel_y
        );
        if StatusModule::situation_kind(weapon.module_accessor) == *SITUATION_KIND_GROUND {
            KineticModule::clear_speed_all(weapon.module_accessor);
            KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_ROT_NORMAL);
        }
    }
    if StatusModule::is_situation_changed(weapon.module_accessor) {
        if StatusModule::situation_kind(weapon.module_accessor) == *SITUATION_KIND_GROUND {
            LANDING_EFFECT(weapon, Hash40::new("sys_merikomi_smoke"), Hash40::new("rot"), 0, -0.5, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
            //macros::PLAY_SE(weapon, Hash40::new("se_item_sandbag_landing"));
            macros::PLAY_SE(weapon, Hash40::new("se_item_kusudama_landing"));
            
            let accel_x = WorkModule::get_param_float(weapon.module_accessor, hash40("param_captoss"), hash40("brake_x"));
            let lr = PostureModule::lr(weapon.module_accessor);
            sv_kinetic_energy!(
                set_accel,
                weapon,
                WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL,
                -accel_x*lr*2.0,
                0.0
            );
        }
    }
    
    0.into()
}

pub fn install() {    
    Agent::new("mario_captoss")
        .status(Init, CAPTOSS_STATUS_KIND_HOP, captoss_hop_init)
        .status(Pre, CAPTOSS_STATUS_KIND_HOP, captoss_hop_pre)
        .status(Main, CAPTOSS_STATUS_KIND_HOP, captoss_hop_main)
        .status(Exec, CAPTOSS_STATUS_KIND_HOP, captoss_hop_exec)
        .status(End, CAPTOSS_STATUS_KIND_HOP, captoss_hop_end)
        .install();
}
