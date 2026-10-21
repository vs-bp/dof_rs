use crate::*;


/* -------------------------------------------------------------------------- */
/*                                 Structures                                 */
/* -------------------------------------------------------------------------- */
#[derive(Clone)]
pub struct DataExportSimulation {
    pub t: f64,
    pub m: f64,
    pub i_longitudinal: f64,
    pub i_rotational: f64,
    pub cg: f64,
    pub thrust: f64,
    pub l_ref: f64,
    pub _a_ref: f64,
}
#[derive(Clone)]
pub struct DataExportAnalysis {
    pub mach: f64,
    pub cp: f64,
    pub cna: f64,
    pub cd: f64
}

#[derive(Clone)]
pub struct ConfigurationState {
    pub dt_ascent: f64,
    pub dt_descent: f64,
    pub cd_main: f64,
    pub cd_drogue: f64,
    pub diameter_main_in: f64,
    pub diameter_drogue_in: f64,
    pub altitude_main_ft    : f64,
    pub wind_heading_deg: f64,
    pub wind_velocity_mph: f64,
    pub rail_length: f64,
    pub launch_angle: f64,
    pub launch_heading: f64,
    pub control_alg_p: f64,
    pub control_alg_d: f64,
    pub control_roll_start: f64,
    pub control_roll_target: f64,
    pub control_slew_rate: f64,
}

#[derive(Clone)]
pub struct SimulationState {
    pub t: f64,
    pub dt: f64,
    pub xe: Vector3<f64>,
    pub ve: Vector3<f64>,
    pub ae: Vector3<f64>,
    pub fb: Vector3<f64>,
    pub fb_thrust: Vector3<f64>,
    pub fb_airframe: Vector3<f64>,
    pub fb_controls: Vector3<f64>,
    pub mb: Vector3<f64>,
    pub mb_airframe: Vector3<f64>,
    pub mb_controls: Vector3<f64>,
    pub ab: Vector3<f64>,
    pub wb: Vector3<f64>,
    pub dcm_be: Matrix3<f64>,
    pub dcm_eb: Matrix3<f64>,
    pub eul: Vector3<f64>,
    pub eul_rate: Vector3<f64>,
    pub q: Vector4<f64>,
    pub i: Matrix3<f64>,
    pub m: f64,
    pub cg: f64,
    pub air_rho: f64,
    pub air_p: f64,
    pub air_a: f64,
    pub mach: f64,
    pub cna: f64,
    pub cd: f64,
    pub cp: f64,
    pub vb: Vector3<f64>,
    pub incidence: f64,
    pub sideslip: f64,
    pub a_ref: f64,
    pub l_ref: f64,
    pub control_alg_m_factor: f64,
    pub control_alg_a_desired: f64,
    pub control_alg_angle_target: f64,
    pub control_alg_angle_true: f64
}


/* -------------------------------------------------------------------------- */
/*                          Implementation Functions                          */
/* -------------------------------------------------------------------------- */
//
/* -------------------------- Atmosphere Modelling -------------------------- */
// Returns density, pressure, speed of sound from altitude.
pub fn troposphere_air_properties(h: f64) -> (f64, f64, f64) {
    let air_gamma = 1.401;
    let air_rho = 1.225 * f64::powf(1.0 - 0.0000225576956446 * h, 4.25580352933);
    let air_p = 101325.0 * f64::powf(1.0 - 0.0000225576956446 * h, 5.25580352933);
    let air_a = f64::sqrt(air_gamma * air_p / air_rho);
    return (air_rho, air_p, air_a);
}
/* ------------------------------- Aerodynamics ------------------------------ */
// Returns (incidence, sideslip) from Vb.
pub fn aero_angles_from_vb(vb: Vector3<f64>) -> (f64, f64) {
    let incidence: f64 = f64::atan2(vb.z, vb.x);
    let sideslip: f64 = f64::atan2(vb.y, vb.x);
    return (incidence, sideslip); 
}
/* -------------------------------- Rotations ------------------------------- */
pub fn quaternion_to_euler(q: Vector4<f64>) -> Vector3<f64> {
    let mut eul: Vector3<f64> = Vector3::new(0.0, 0.0, 0.0);
    let qw: f64 = q.x;
    let qx: f64 = q.y;
    let qy: f64 = q.z;
    let qz: f64 = q.w;
    eul.x = f64::atan2(2.0*(qy*qz + qw*qx), qw*qw - qx*qx - qy*qy + qz*qz);
    eul.y = f64::asin(-2.0*(qx*qz - qw*qy));
    eul.z = f64::atan2(2.0*(qx*qy + qw*qz), qw*qw + qx*qx - qy*qy - qz*qz);
    return eul;
}
pub fn euler_to_quaternion(eul: Vector3<f64>) -> Vector4<f64> {
    let cx = f64::cos(eul.x/2.0);
    let cy = f64::cos(eul.y/2.0);
    let cz = f64::cos(eul.z/2.0);
    let sx = f64::sin(eul.x/2.0);
    let sy = f64::sin(eul.y/2.0);
    let sz = f64::sin(eul.z/2.0);
    Vector4::new(
        cx*cy*cz + sx*sy*sz,
        sx*cy*cz - cx*sy*sz,
        cx*sy*cz + sx*cy*sz,
        cx*cy*sz - sx*sy*cz
    )
}
/* ------------------------ Direction-Cosine-Matrices ----------------------- */
pub fn quaternion_to_dcm_be(q: Vector4<f64>) -> Matrix3<f64> {
    let qw: f64 = q.x;
    let qx: f64 = q.y;
    let qy: f64 = q.z;
    let qz: f64 = q.w;
    return Matrix3::new(
        qw*qw + qx*qx - qy*qy - qz*qz, 2.0*(qx*qy + qw*qz), 2.0*(qx*qz - qw*qy),
        2.0*(qx*qy - qw*qz), qw*qw - qx*qx + qy*qy - qz*qz, 2.0*(qy*qz + qw*qx),
        2.0*(qx*qz + qw*qy), 2.0*(qy*qz - qw*qx), qw*qw - qx*qx - qy*qy + qz*qz
    );
}
pub fn quaternion_to_dcm_eb(q: Vector4<f64>) -> Matrix3<f64> {
    let qw: f64 = q.x;
    let qx: f64 = q.y;
    let qy: f64 = q.z;
    let qz: f64 = q.w;
    return Matrix3::new(
        qw*qw + qx*qx - qy*qy - qz*qz, 2.0*(qx*qy + qw*qz), 2.0*(qx*qz - qw*qy),
        2.0*(qx*qy - qw*qz), qw*qw - qx*qx + qy*qy - qz*qz, 2.0*(qy*qz + qw*qx),
        2.0*(qx*qz + qw*qy), 2.0*(qy*qz - qw*qx), qw*qw - qx*qx - qy*qy + qz*qz
    ).transpose();
}
// Use an intermediate conversion to quaternion instead of direct conversion.
// fn euler_to_dcm_be(eul: Vector3<f64>) -> Matrix3<f64> {
//     let q: Vector4<f64> = euler_to_quaternion(eul);
//     return quaternion_to_dcm_be(q);
// }
// fn euler_to_dcm_eb(eul: Vector3<f64>) -> Matrix3<f64> {
//     let q: Vector4<f64> = euler_to_quaternion(eul);
//     return quaternion_to_dcm_eb(q);
// }
/* --------------------- Rotation Rates and Integration --------------------- */
pub fn body_rate_to_euler_rate(wb: Vector3<f64>, eul: Vector3<f64>) -> Vector3<f64> {
    return Matrix3::new(
        1.0, f64::sin(eul.x) * f64::tan(eul.y), f64::cos(eul.x) * f64::tan(eul.y),
        0.0, f64::cos(eul.x), -f64::sin(eul.x),
        0.0, f64::sin(eul.x) / f64::cos(eul.y), f64::cos(eul.x) / f64::cos(eul.y)
    ) * wb;
}
pub fn integrate_body_rates_from_moment(i: Matrix3<f64>, wb: Vector3<f64>, mb: Vector3<f64>, dt: f64) -> Vector3<f64> {
    return wb + (i.try_inverse().unwrap() * (mb - wb.cross(&(i*wb)))) * dt;
}
pub fn integrate_quaternion_from_body_rates(wb: Vector3<f64>, q: Vector4<f64>, dt: f64) -> Vector4<f64> {
    let bp: f64 = wb.x;
    let bq: f64 = wb.y;
    let br: f64 = wb.z;
    let q_transformation_matrix: Matrix4<f64> = Matrix4::new(
        0.0 , -bp,  -bq,   -br,
        bp,   0.0,  -br,   -bq, 
        bq,   -br,  0.0,   bp, 
        br,   bq,   -bp,   0.0
    );
    let qdot: Vector4<f64> = 0.5 * q_transformation_matrix * q;
    return (q + qdot*dt).normalize();
}

/* -------------------------------------------------------------------------- */
/*                                  Algorithm                                 */
/* -------------------------------------------------------------------------- */
pub fn simulate(data_simulation: &Vec<DataExportSimulation>,  data_analysis: &Vec<DataExportAnalysis>, state_log: &mut Vec<SimulationState>, config: &mut ConfigurationState) {
        state_log.clear();

    /* -------------------------------------------------------------------------- */
    /*                                   Config                                   */
    /* -------------------------------------------------------------------------- */
    let mut state: SimulationState = SimulationState { 
        t: 0.0, 
        dt: config.dt_ascent,
        xe: Vector3::new(0.0, 0.0, 0.0), 
        ve: Vector3::new(0.0, 0.0, 0.0), 
        ae: Vector3::new(0.0, 0.0, 0.0),
        fb: Vector3::new(0.0, 0.0, 0.0), 
        fb_thrust: Vector3::new(0.0, 0.0, 0.0),
        fb_airframe: Vector3::new(0.0, 0.0, 0.0),
        fb_controls: Vector3::new(0.0, 0.0, 0.0),
        mb: Vector3::new(0.0, 0.0, 0.0),
        mb_airframe: Vector3::new(0.0, 0.0, 0.0),
        mb_controls: Vector3::new(0.0, 0.0, 0.0),
        ab: Vector3::new(0.0, 0.0, 0.0),
        wb: Vector3::new(0.0, 0.0, 0.0), 
        dcm_be: Matrix3::identity(), 
        dcm_eb: Matrix3::identity(), 
        eul: Vector3::new(
            f64::to_radians(0.0), 
            -f64::cos(config.launch_heading.to_radians()) * config.launch_angle.to_radians(), 
            f64::sin(config.launch_heading.to_radians()) * config.launch_angle.to_radians()
        ),
        eul_rate: Vector3::new(0.0, 0.0, 0.0),
        q: euler_to_quaternion(Vector3::new(
            f64::to_radians(0.0), 
            -f64::cos(config.launch_heading.to_radians()) * config.launch_angle.to_radians(), 
            f64::sin(config.launch_heading.to_radians()) * config.launch_angle.to_radians()
        )),
        i: Matrix3::identity(),
        m: 1.0,
        cg: 0.0,
        air_rho: 1.225,
        air_p: 101325.0,
        air_a: 343.0,
        mach: 0.0,
        cna: 0.0,
        cd: 0.0,
        cp: 0.0,
        vb: Vector3::new(0.0, 0.0, 0.0),
        incidence: 0.0,
        sideslip: 0.0,
        a_ref: 0.0,
        l_ref: 0.0,
        control_alg_m_factor: 0.0,
        control_alg_a_desired: 0.0,
        control_alg_angle_target: 0.0,
        control_alg_angle_true: 0.0,
    };

    /* -------------------------------------------------------------------------- */
    /*                                  Main Loop                                 */
    /* -------------------------------------------------------------------------- */
    let mut loop_exit: bool = false;
    let mut apogee_reached: bool = false;
    let mut main_deployed: bool = false;
    while !loop_exit {
        // Timestep determined by flags.
        if apogee_reached { state.dt = config.dt_descent; }
        else { state.dt = config.dt_ascent; }

        /* ------------------ Find mass and aero values from refs. ------------------ */
        // Find last relevant simulation data point.
        let mut i = 0;
        for point in &data_simulation[..data_simulation.len()-2] {
            if point.t < state.t { i += 1; }
            else { break; }
        }
        let data_simulation_last = data_simulation[i].clone();
        state.m = data_simulation_last.m;
        state.i = Matrix3::new(
            data_simulation_last.i_rotational, 0.0, 0.0, 
            0.0, data_simulation_last.i_longitudinal, 0.0, 
            0.0, 0.0, data_simulation_last.i_longitudinal
        );
        state.cg = data_simulation_last.cg;
        state.fb_thrust = Vector3::new(data_simulation_last.thrust, 0.0, 0.0);
        state.a_ref = 3.14159265359 * f64::powi(data_simulation_last.l_ref / 2.0,2);
        state.l_ref = data_simulation_last.l_ref;

        // Find last relevant analysis data point.
        // Uses mach number determined from velocity and current air properties from altitude.
        (state.air_rho, state.air_p, state.air_a) = troposphere_air_properties(state.xe.x);
        state.mach = state.ve.magnitude() / state.air_a;
        let mut i = 0;
        for point in &data_analysis[..data_analysis.len()-2] {
            if point.mach < state.mach { i += 1; }
            else { break; }
        }
        let data_analysis_last = data_analysis[i].clone();
        state.cna = data_analysis_last.cna;
        state.cd = data_analysis_last.cd;
        state.cp = data_analysis_last.cp;

        /* --------------------------- Force/Moment Determination -------------------------- */
        // Determine roll target.
        let roll_target = if state.t > config.control_roll_start { config.control_roll_target.to_radians() } else { 0.0 };

        // Desired control moment.
        state.control_alg_a_desired = config.control_alg_p * (state.eul.x - roll_target) + config.control_alg_d * state.eul_rate.x;
        let control_alg_m_desired: f64 = state.control_alg_a_desired * state.i.m11;

        // Determine a value cma from mach such that mfac * angle gives the moment applied at full atmospheric pressure.
        // Then multiply this by pressure in atmospheres such that it gives moment applied accounting for altitude.
        let cma_c1: f64 = -6.788666 * f64::powf(10.0, -4.0);
        let cma_c2: f64 = 3.455015 * f64::powf(10.0, -2.0);
        let cma_c3: f64 = -8.024854 * f64::powf(10.0, -3.0);
        state.control_alg_m_factor = cma_c3 * state.mach.powi(3) + cma_c2 * state.mach.powi(2) + cma_c1 * state.mach;
        // state.control_alg_m_factor *= state.air_rho / 101325.0;

        // Determine angle from CMa.
        if state.mach > 0.01 { state.control_alg_angle_target = control_alg_m_desired / state.control_alg_m_factor; }
        else { state.control_alg_angle_target = 0.0; }

        // Simulate servo slew rate towards target angle.
        let slew_target: f64 = state.control_alg_angle_target.clamp(-15.0, 15.0).round();
        let slew_target_offset: f64 = state.control_alg_angle_true - slew_target;
        if f64::abs(slew_target_offset) < config.control_slew_rate * state.dt {
            state.control_alg_angle_true = slew_target;
        } else {
            if slew_target_offset > 0.0 { state.control_alg_angle_true -= config.control_slew_rate * state.dt; }
            else { state.control_alg_angle_true += config.control_slew_rate * state.dt; }
        }
        
        // Calculate applied moment and force from control deflection.
        // Normal force is negligible for a single small canard, looking only at moment.
        state.fb_controls = Vector3::new(0.0, 0.0, 0.0);
        state.mb_controls = Vector3::new(state.control_alg_angle_true * state.control_alg_m_factor, 0.0, 0.0);

        // Calculate DCMbe and DCMeb from rotation.
        state.dcm_be = quaternion_to_dcm_be(state.q);
        state.dcm_eb = quaternion_to_dcm_eb(state.q);
        
        // Get body velocity and euler angle representations.
        state.vb = state.dcm_be * state.ve;
        state.eul = quaternion_to_euler(state.q);
        state.eul_rate = body_rate_to_euler_rate(state.wb, state.eul);

        // Vehicle aero forces and moments.
        // Airframe normal and drag forces only considered if before apogee.
        // Drogue and main drag forces only considered if appropriate flag triggered.
        //
        // Wind is subtracted from vb to get airspeed rather than ground speed.
        let wind_vb: Vector3<f64> = state.dcm_be * Vector3::new(0.0, config.wind_heading_deg.to_radians().sin(), config.wind_heading_deg.to_radians().cos()) * config.wind_velocity_mph * 0.44704;
        (state.incidence, state.sideslip) = aero_angles_from_vb(state.vb - wind_vb);
        state.fb_airframe = Vector3::new(0.0, 0.0, 0.0);
        if !apogee_reached { state.fb_airframe += (-0.5*state.air_rho*(state.vb - wind_vb).magnitude_squared()*state.a_ref*state.cna) * Vector3::new(0.0, state.sideslip, state.incidence); }
        if !apogee_reached { state.fb_airframe += (-0.5*state.air_rho*(state.vb - wind_vb).magnitude_squared()*state.a_ref*state.cd) * (state.vb - wind_vb).normalize(); }
        if apogee_reached { state.fb_airframe += (-0.5*state.air_rho*(state.vb - wind_vb).magnitude_squared()*0.000506707479097*config.diameter_drogue_in*config.diameter_drogue_in*config.cd_drogue) * (state.vb - wind_vb).normalize(); }
        if main_deployed { state.fb_airframe += (-0.5*state.air_rho*(state.vb - wind_vb).magnitude_squared()*0.000506707479097*config.diameter_main_in*config.diameter_main_in*config.cd_main) * (state.vb - wind_vb).normalize(); }
        state.mb_airframe = Vector3::new(state.cg - state.cp, 0.0, 0.0).cross(&state.fb_airframe);

        // Override to zero at low airspeeds to avoid NAN.
        if state.vb.magnitude() < 0.001 { 
            state.fb_airframe = Vector3::new(0.0, 0.0, 0.0); 
            state.mb_airframe = Vector3::new(0.0, 0.0, 0.0);
        }
        // Override lateral axes and rotation to zero while on rail.
        if state.xe.magnitude() < config.rail_length {
            state.fb_airframe.y = 0.0;
            state.fb_airframe.z = 0.0;
            state.mb_airframe = Vector3::new(0.0, 0.0,0.0);
        }

        // Sum forces and moments.
        state.fb = state.fb_thrust + state.fb_airframe + state.fb_controls;
        state.mb = state.mb_airframe + state.mb_controls;

        /* ------------------------------- Integration ------------------------------ */
        // Integrate new rotation from moments.
        // Paused after apogee.
        if !apogee_reached {
            state.wb = integrate_body_rates_from_moment(state.i, state.wb, state.mb, state.dt);
            state.q = integrate_quaternion_from_body_rates(state.wb, state.q, state.dt);
        }

        // Gravity is calculated in earth axes, transformed to body axes where
        // if on the rail y and z axes are constrained, and then transformed back to earth axes.
        let mut grav: Vector3<f64> = Vector3::new(-9.80665, 0.0,0.0);
        grav = state.dcm_be * grav;
        if state.xe.magnitude() < config.rail_length {
            grav.y = 0.0;
            grav.z = 0.0;
        }
        grav = state.dcm_eb * grav;

        // Integrate acceleration to position and velocity.
        state.ab = state.fb / state.m;
        state.ae = state.dcm_eb * state.ab;
        state.ve += state.ae*state.dt;
        state.ve += grav*state.dt;
        state.xe += state.ve*state.dt;

        // Simulation stop condition.
        if state.ve.x < -1.0 { apogee_reached = true; }
        if apogee_reached && state.xe.x < config.altitude_main_ft * 0.3048 { main_deployed = true; }
        if apogee_reached && state.xe.x < -1.0 { loop_exit = true; }

        // Save state for future viewing.
        state_log.push(state.clone());

        // Step time.
        state.t += state.dt;
    }
}

pub fn show(ui: &mut egui::Ui, data_simulation: &Vec<DataExportSimulation>,  data_analysis: &Vec<DataExportAnalysis>, state_log: &mut Vec<SimulationState>, config: &mut ConfigurationState) {
    ui.horizontal(|ui| {
        ui.label("(P, D) Gains");
        if
            ui.add(egui::widgets::DragValue::new(&mut config.control_alg_p).speed(0.05)).changed() ||
            ui.add(egui::widgets::DragValue::new(&mut config.control_alg_d).speed(0.05)).changed()
        {
            simulate(&data_simulation, &data_analysis, state_log, config);
        }
        ui.separator();
        ui.label("(dps) Control Slew Rate");
        if
            ui.add(egui::widgets::DragValue::new(&mut config.control_slew_rate).speed(5.0)).changed()
        {
            simulate(&data_simulation, &data_analysis, state_log, config);
        }
        ui.separator();
        ui.label("(Time, Degrees) Roll Program");
        if
            ui.add(egui::widgets::DragValue::new(&mut config.control_roll_start).speed(0.05)).changed() ||
            ui.add(egui::widgets::DragValue::new(&mut config.control_roll_target).speed(0.25)).changed()
        {
            simulate(&data_simulation, &data_analysis, state_log, config);
        }
    });
    ui.horizontal(|ui| {
        ui.label("(Pitch deg, Bearing deg) Launch Angles");
        if
            ui.add(egui::widgets::DragValue::new(&mut config.launch_angle).speed(0.05)).changed() ||
            ui.add(egui::widgets::DragValue::new(&mut config.launch_heading).speed(0.25)).changed()
        {
            simulate(&data_simulation, &data_analysis, state_log, config);
        }
        ui.separator();
        ui.label("Diameters (drogue in, main in)");
        if
            ui.add(egui::widgets::DragValue::new(&mut config.diameter_drogue_in).speed(0.05)).changed() ||
            ui.add(egui::widgets::DragValue::new(&mut config.diameter_main_in).speed(0.05)).changed()
        {
            simulate(&data_simulation, &data_analysis, state_log, config);
        }
        ui.separator();
        ui.label("Main Deploy Altitude (ft)");
        if
            ui.add(egui::widgets::DragValue::new(&mut config.altitude_main_ft).speed(0.5)).changed()
        {
            simulate(&data_simulation, &data_analysis, state_log, config);
        }
        ui.separator();
        ui.label("Wind (heading deg, velocity mph)");
        if
            ui.add(egui::widgets::DragValue::new(&mut config.wind_heading_deg).speed(0.5)).changed() ||
            ui.add(egui::widgets::DragValue::new(&mut config.wind_velocity_mph).speed(0.1)).changed()
        {
            simulate(&data_simulation, &data_analysis, state_log, config);
        }
    });
    ui.horizontal(|ui| {
        plot1::show(&state_log, ui);
        plot2::show(&state_log, ui);
    });
    ui.horizontal(|ui| {
        plot3::show(&state_log, ui);
        plot4::show(&state_log, ui);
    });
}