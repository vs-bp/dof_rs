use eframe::egui;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};

mod plot1;
mod plot2;
mod plot3;
mod plot4;

/* -------------------------------------------------------------------------- */
/*                                 Structures                                 */
/* -------------------------------------------------------------------------- */
#[derive(Clone)]
struct DataExportSimulation {
    t: f64,
    m: f64,
    
    i_longitudinal: f64,
    i_rotational: f64,

    cg: f64,

    thrust: f64,

    l_ref: f64,
    a_ref: f64,
}
#[derive(Clone)]
struct DataExportAnalysis {
    mach: f64,
    cp: f64,
    cna: f64,
    cd: f64
}

#[derive(Clone)]
struct ConfigurationState {
    rail_length: f64,
    launch_angle: f64,
    launch_heading: f64,
    control_alg_p: f64,
    control_alg_d: f64
}

#[derive(Clone)]
struct SimulationState {
    t: f64,
    dt: f64,
    
    xe: Vector3<f64>,
    ve: Vector3<f64>,
    ae: Vector3<f64>,
    
    fb: Vector3<f64>,
    fb_thrust: Vector3<f64>,
    fb_airframe: Vector3<f64>,
    fb_controls: Vector3<f64>,

    mb: Vector3<f64>,
    mb_airframe: Vector3<f64>,
    mb_controls: Vector3<f64>,

    ab: Vector3<f64>,
    wb: Vector3<f64>,

    dcm_be: Matrix3<f64>,
    dcm_eb: Matrix3<f64>,

    eul: Vector3<f64>,
    eul_rate: Vector3<f64>,
    q: Vector4<f64>,

    i: Matrix3<f64>,
    m: f64,
    cg: f64,

    air_rho: f64,
    air_p: f64,
    air_a: f64,
    mach: f64,

    cna: f64,
    cd: f64,
    cp: f64,

    vb: Vector3<f64>,
    incidence: f64,
    sideslip: f64,

    a_ref: f64,
    l_ref: f64,

    control_alg_m_factor: f64,
    control_alg_a_desired: f64,
    control_alg_angle: f64
}


/* -------------------------------------------------------------------------- */
/*                          Implementation Functions                          */
/* -------------------------------------------------------------------------- */
//
/* -------------------------- Atmosphere Modelling -------------------------- */
// Returns density, pressure, speed of sound from altitude.
fn troposphere_air_properties(h: f64) -> (f64, f64, f64) {
    let air_gamma = 1.401;
    let air_rho = 1.225 * f64::powf(1.0 - 0.0000225576956446 * h, 4.25580352933);
    let air_p = 101325.0 * f64::powf(1.0 - 0.0000225576956446 * h, 5.25580352933);
    let air_a = f64::sqrt(air_gamma * air_p / air_rho);
    return (air_rho, air_p, air_a);
}
/* ------------------------------- Aerodynamics ------------------------------ */
// Returns (incidence, sideslip) from Vb.
fn aero_angles_from_vb(vb: Vector3<f64>) -> (f64, f64) {
    let incidence: f64 = f64::atan2(vb.z, vb.x);
    let sideslip: f64 = f64::atan2(vb.y, vb.x);
    return (incidence, sideslip); 
}
/* -------------------------------- Rotations ------------------------------- */
fn quaternion_to_euler(q: Vector4<f64>) -> Vector3<f64> {
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
fn euler_to_quaternion(eul: Vector3<f64>) -> Vector4<f64> {
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
fn quaternion_to_dcm_be(q: Vector4<f64>) -> Matrix3<f64> {
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
fn quaternion_to_dcm_eb(q: Vector4<f64>) -> Matrix3<f64> {
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
fn body_rate_to_euler_rate(wb: Vector3<f64>, eul: Vector3<f64>) -> Vector3<f64> {
    return Matrix3::new(
        1.0, f64::sin(eul.x) * f64::tan(eul.y), f64::cos(eul.x) * f64::tan(eul.y),
        0.0, f64::cos(eul.x), -f64::sin(eul.x),
        0.0, f64::sin(eul.x) / f64::cos(eul.y), f64::cos(eul.x) / f64::cos(eul.y)
    ) * wb;
}
fn integrate_body_rates_from_moment(i: Matrix3<f64>, wb: Vector3<f64>, mb: Vector3<f64>, dt: f64) -> Vector3<f64> {
    return wb + (i.try_inverse().unwrap() * (mb - wb.cross(&(i*wb)))) * dt;
}
fn integrate_quaternion_from_body_rates(wb: Vector3<f64>, q: Vector4<f64>, dt: f64) -> Vector4<f64> {
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
/*                          Implementation Definition                         */
/* -------------------------------------------------------------------------- */
//
fn simulate(data_simulation: &Vec<DataExportSimulation>, data_analysis: &Vec<DataExportAnalysis>, state_log: &mut Vec<SimulationState>, config: &ConfigurationState) {
    /* -------------------------------------------------------------------------- */
    /*                                   Config                                   */
    /* -------------------------------------------------------------------------- */
    let mut state: SimulationState = SimulationState { 
        t: 0.0, 
        dt: 0.01,
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
            -f64::cos(config.launch_heading) * config.launch_angle, 
            f64::sin(config.launch_heading) * config.launch_angle
        ),
        eul_rate: Vector3::new(0.0, 0.0, 0.0),
        q: euler_to_quaternion(Vector3::new(
            f64::to_radians(0.0), 
            -f64::cos(config.launch_heading) * config.launch_angle, 
            f64::sin(config.launch_heading) * config.launch_angle
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
        control_alg_angle: 0.0
    };
    
    /* -------------------------------------------------------------------------- */
    /*                                  Main Loop                                 */
    /* -------------------------------------------------------------------------- */
    let mut loop_exit: bool = false;
    while !loop_exit {
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
        state.a_ref = data_simulation_last.a_ref;
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
        // Calculate control delection.
        // TODO Model discrepancy between regression and true values.
        let cma_c1: f64 = 3.224098 / 5.0 * f64::powf(10.0, -2.0);
        let cma_c2: f64 = 3.165564 / 5.0 * f64::powf(10.0, -2.0);
        let cma_c3: f64 = 8.027749 / 5.0 * f64::powf(10.0, -3.0);
        state.control_alg_m_factor = cma_c1*state.mach*state.mach*state.mach + cma_c2*state.mach*state.mach + cma_c3*state.mach;
        state.control_alg_a_desired = config.control_alg_p * state.eul.x + config.control_alg_d * state.eul_rate.x;
        let control_alg_m_desired: f64 = state.control_alg_a_desired * state.i.m11;
        if state.mach > 0.05 { state.control_alg_angle = f64::clamp(control_alg_m_desired / state.control_alg_m_factor, -15.0, 15.0); }
        else { state.control_alg_angle = 0.0; }
        
        // Calculate applied moment and force from control deflection.
        // Normal force is negligible for a single small canard, looking only at moment.
        state.fb_controls = Vector3::new(0.0, 0.0, 0.0);
        state.mb_controls = Vector3::new(state.control_alg_angle * state.control_alg_m_factor, 0.0, 0.0);

        // Calculate DCMbe and DCMeb from rotation.
        state.dcm_be = quaternion_to_dcm_be(state.q);
        state.dcm_eb = quaternion_to_dcm_eb(state.q);
        
        // Get body velocity and euler angle representations.
        state.vb = state.dcm_be * state.ve;
        state.eul = quaternion_to_euler(state.q);
        state.eul_rate = body_rate_to_euler_rate(state.wb, state.eul);

        // Vehicle aero forces and moments.
        (state.incidence, state.sideslip) = aero_angles_from_vb(state.vb);
        state.fb_airframe = (-0.5*state.air_rho*state.vb.magnitude_squared()*state.a_ref*state.cna) * Vector3::new(0.0, state.sideslip, state.incidence);
        state.fb_airframe += (-0.5*state.air_rho*state.vb.magnitude_squared()*state.a_ref*state.cd) * state.vb.normalize();
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
        state.wb = integrate_body_rates_from_moment(state.i, state.wb, state.mb, state.dt);
        state.q = integrate_quaternion_from_body_rates(state.wb, state.q, state.dt);

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
        if state.ve.x < -1.0 { loop_exit = true; }
        if state.t > 30.0 { loop_exit = true; }

        // Save state for future viewing.
        state_log.push(state.clone());

        // Step time.
        state.t += state.dt;
    }
} 

fn main() -> eframe::Result {
    /* -------------------------------------------------------------------------- */
    /*                                Data Reading                                */
    /* -------------------------------------------------------------------------- */
    let mut data_simulation: Vec<DataExportSimulation> = vec![];
    let mut data_analysis: Vec<DataExportAnalysis> = vec![];

    let file_simulation: String = std::fs::read_to_string("C:/Users/kcast/OneDrive/Desktop/Code/dof_rs/input/simulation.csv").unwrap();
    let file_analysis: String = std::fs::read_to_string("C:/Users/kcast/OneDrive/Desktop/Code/dof_rs/input/analysis.csv").unwrap();

    for line in file_simulation.lines().collect::<Vec<&str>>() {
        let words: Vec<&str> = line.split(',').collect();
        data_simulation.push(DataExportSimulation { 
            t: words[0].parse::<f64>().unwrap(), 
            m: words[21].parse::<f64>().unwrap() * 0.02834952,
            i_longitudinal: words[23].parse::<f64>().unwrap() * 0.0421401128809,
            i_rotational: words[24].parse::<f64>().unwrap() * 0.0421401128809,
            cg: words[27].parse::<f64>().unwrap() * 0.0254, 
            thrust: words[29].parse::<f64>().unwrap(), 
            l_ref: words[53].parse::<f64>().unwrap() * 0.0254,
            a_ref: words[54].parse::<f64>().unwrap() * 0.00064516
        });
    }
    for line in file_analysis.lines().collect::<Vec<&str>>() {
        let words: Vec<&str> = line.split(',').collect();
        data_analysis.push(DataExportAnalysis {
            mach: words[0].parse::<f64>().unwrap(),
            cp: words[1].parse::<f64>().unwrap() * 0.0254,
            cna: words[2].parse::<f64>().unwrap(),
            cd: words[7].parse::<f64>().unwrap()
        });
    }

    /* -------------------------------------------------------------------------- */
    /*                             Initial Simulation                             */
    /* -------------------------------------------------------------------------- */
    let mut state_log: Vec<SimulationState> = vec![];
    let mut config: ConfigurationState = ConfigurationState { 
        rail_length: 3.0, 
        launch_angle: (5_f64).to_radians(), 
        launch_heading: (5_f64).to_radians(), 
        control_alg_p: -5.0,
        control_alg_d: -5.0
    };
    simulate(&data_simulation, &data_analysis, &mut state_log, &mut config);

    /* -------------------------------------------------------------------------- */
    /*                                     GUI                                    */
    /* -------------------------------------------------------------------------- */
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true).with_fullscreen(false),
        ..Default::default()
    };

    /* ------------------------------- Egui Window ------------------------------ */
    eframe::run_simple_native("dof_rs", options, move |ctx, _frame| {
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            // ui.horizontal(|ui| {
            //     ui.label("P Gain");
            //     ui.add(egui::widgets::DragValue::new(&config.control_alg_p));
            // })
            ui.horizontal(|ui| {
                plot1::show(&state_log, ui);
                plot2::show(&state_log, ui);
            });
            ui.horizontal(|ui| {
                plot3::show(&state_log, ui);
                plot4::show(&state_log, ui);
            });
        });
    })
}
