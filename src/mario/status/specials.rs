use crate::imports::imports_status::*;

pub unsafe extern "C" fn specials_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let mul_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_start_mul_spd_x"));
        let start_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_start_air_acl_x"));
        let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_acl_y"));
        let speed_x = sum_speed_x/mul_x;
        
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            ENERGY_STOP_RESET_TYPE_AIR,
            speed_x,
            0.0,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_brake,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_STOP,
            start_accel_x,
            0.0
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_spd_y"));

        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
            0.0,
            speed_y,
            0.0,
            0.0,
            0.0
        );
        sv_kinetic_energy!(
            set_accel,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
            -accel_y
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    0.into()
}

pub unsafe extern "C" fn specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, mario::instance::flag::HATLESS) {
        StatusModule::change_status_force(fighter.module_accessor, FIGHTER_MARIO_STATUS_KIND_CAPDIVE, false);
    }
    else{
        VarModule::on_flag(fighter.battle_object, mario::instance::flag::CAPDIVE_ENABLED);
    }

    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );

    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        (*FIGHTER_STATUS_ATTR_START_TURN | *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S) as u32,
        0
    );
    0.into()
}

pub unsafe extern "C" fn specials_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    let lr = PostureModule::lr(fighter.module_accessor);
    let rot = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("rot"),
        rot,
        true
    );
    let throw = &mut Vector3f{x: 0.0, y: 0.0, z: 0.0};
    ModelModule::joint_global_position(
        fighter.module_accessor,
        Hash40::new("throw"),
        throw,
        true
    );
    //let new_pos = Vector3f{x: rot.x, y: throw.y, z: throw.z};

    let startframe = 6.0;
    let endframe = 22.0;
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut offset = 6.7;
    let delta = (frame-startframe)/(endframe-startframe);
    let maxdist = 26.0;

    if startframe <= frame && frame <= endframe {
        let new_z = offset + (delta*maxdist);
        let new_pos = Vector3f{x: rot.x+(new_z*lr), y: rot.y-0.5, z: throw.z};
        ModelModule::set_joint_translate(
            fighter.module_accessor,
            Hash40::new("throw"),
            &new_pos,
            true,
            false
        );
    }


    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
        let air_accel_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y_stable"), 0);
        let air_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);

        let special_s_attack_spd_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_spd_y"));
        let special_s_attack_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_acl_y"));
        let special_s_attack_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_attack_max_y"));

        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_S_FLAG_HOP);
                /*
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let speed_y = app::sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                */

                sv_kinetic_energy!(
                    set_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    special_s_attack_spd_y //0.0
                );
            }
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -special_s_attack_acl_y
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                special_s_attack_max_y
            );
            let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else {
            sv_kinetic_energy!(
                set_accel,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                -air_accel_y
            );
            sv_kinetic_energy!(
                set_accel_x_mul,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                air_accel_x
            );
            sv_kinetic_energy!(
                set_limit_speed,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                air_accel_y_stable
            );
        } 
    }
    0.into()
}

pub fn install() {
    Agent::new("mario")
        .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_init)
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_pre)
        .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, specials_exec)
        .install();
}