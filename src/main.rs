use eframe::egui;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};

mod plot1;
mod plot2;
mod plot3;
mod plot4;

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
    i_inv: Matrix3<f64>,
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
    l_ref: f64
}

fn eul2quat(eul: Vector3<f64>) -> Vector4<f64> {
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
    /*                      Configuration and Initialization                      */
    /* -------------------------------------------------------------------------- */
    let config_rail_length: f64 = 3.0;
    let config_launch_angle: f64 = (5_f64).to_radians();
    let config_launch_heading: f64 = (150_f64).to_radians();
    let config_eul: Vector3<f64> = Vector3::new(
        f64::to_radians(0.0), 
        -f64::cos(config_launch_heading) * config_launch_angle, 
        f64::sin(config_launch_heading) * config_launch_angle
    );

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
        eul: config_eul, 
        eul_rate: Vector3::new(0.0, 0.0, 0.0),
        q: eul2quat(config_eul),
        i: Matrix3::identity(),
        i_inv: Matrix3::identity(),
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
    };
    let mut state_log: Vec<SimulationState> = vec![];

    /* -------------------------------------------------------------------------- */
    /*                                  Main Loop                                 */
    /* -------------------------------------------------------------------------- */
    let mut loop_exit: bool = false;
    while !loop_exit {
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
            0.0, 0.0, data_simulation_last.i_longitudinal);
        state.cg = data_simulation_last.cg;
        state.fb_thrust = Vector3::new(data_simulation_last.thrust, 0.0, 0.0);
        state.a_ref = data_simulation_last.a_ref;
        state.l_ref = data_simulation_last.l_ref;

        // Air property determination. (Troposphere estimation)
        let air_gamma = 1.401;
        state.air_rho = 1.225 * f64::powf(1.0 - 0.0000225576956446 * state.xe.x, 4.25580352933);
        state.air_p = 101325.0 * f64::powf(1.0 - 0.0000225576956446 * state.xe.x, 5.25580352933);
        state.air_a = f64::sqrt(air_gamma * state.air_p / state.air_rho);
        state.mach = state.ve.magnitude() / state.air_a;

        // Find last relevant analysis data point.
        let mut i = 0;
        for point in &data_analysis[..data_analysis.len()-2] {
            if point.mach < state.mach { i += 1; }
            else { break; }
        }
        let data_analysis_last = data_analysis[i].clone();
        state.cna = data_analysis_last.cna;
        state.cd = data_analysis_last.cd;
        state.cp = data_analysis_last.cp;
        
        // Calculate DCMbe and DCMeb from new rotation.
        let qw: f64 = state.q.x;
        let qx: f64 = state.q.y;
        let qy: f64 = state.q.z;
        let qz: f64 = state.q.w;
        state.dcm_be = Matrix3::new(
            qw*qw + qx*qx - qy*qy - qz*qz, 2.0*(qx*qy + qw*qz), 2.0*(qx*qz - qw*qy),
            2.0*(qx*qy - qw*qz), qw*qw - qx*qx + qy*qy - qz*qz, 2.0*(qy*qz + qw*qx),
            2.0*(qx*qz + qw*qy), 2.0*(qy*qz - qw*qx), qw*qw - qx*qx - qy*qy + qz*qz
        );
        state.dcm_eb = state.dcm_be.transpose();
        
        // Get body velocity and euler angle representation.
        state.vb = state.dcm_be * state.ve;
        state.eul.x = f64::atan2(2.0*(qy*qz + qw*qx), qw*qw - qx*qx - qy*qy + qz*qz);
        state.eul.y = f64::asin(-2.0*(qx*qz - qw*qy));
        state.eul.z = f64::atan2(2.0*(qx*qy + qw*qz), qw*qw + qx*qx - qy*qy - qz*qz);

        state.eul_rate = Matrix3::new(
            1.0, f64::sin(state.eul.x) * f64::tan(state.eul.y), f64::cos(state.eul.x) * f64::tan(state.eul.y),
            0.0, f64::cos(state.eul.x), -f64::sin(state.eul.x),
            0.0, f64::sin(state.eul.x) / f64::cos(state.eul.y), f64::cos(state.eul.x) / f64::cos(state.eul.y)
        ) * state.wb;

        // Force and moment determination.
        state.incidence = f64::atan2(state.vb.z, state.vb.x);
        state.sideslip = f64::atan2(state.vb.y, state.vb.x);
        if state.vb.magnitude() < 0.001 { 
            // No forces if no airspeed.
            state.fb_airframe = Vector3::new(0.0, 0.0, 0.0); 
            state.mb_airframe = Vector3::new(0.0, 0.0, 0.0); 
        } else {        
            state.fb_airframe = (-0.5*state.air_rho*state.vb.magnitude_squared()*state.a_ref*state.cna) * Vector3::new(0.0, state.sideslip, state.incidence);
            state.fb_airframe += (-0.5*state.air_rho*state.vb.magnitude_squared()*state.a_ref*state.cd) * state.vb.normalize();
            state.mb_airframe = Vector3::new(state.cg - state.cp, 0.0, 0.0).cross(&state.fb_airframe);
            
            // Force onstrained if on rail.
            if state.xe.magnitude() < config_rail_length {
                state.fb_airframe.y = 0.0;
                state.fb_airframe.z = 0.0;
                state.mb_airframe = Vector3::new(0.0, 0.0,0.0);
            }
        }
        state.fb = state.fb_thrust + state.fb_airframe + state.fb_controls;
        state.ab = state.fb / state.m;
        state.mb = state.mb_airframe + state.mb_controls;
    
        // Gravity is calculated in earth axes, transformed to body axes where
        // if on the rail y and z axes are constrained, and then transformed back to earth axes.
        let mut grav: Vector3<f64> = Vector3::new(-9.80665, 0.0,0.0);
        grav = state.dcm_be * grav;
        if state.xe.magnitude() < config_rail_length {
            grav.y = 0.0;
            grav.z = 0.0;
        }
        grav = state.dcm_eb * grav;

        // Applying moments to calculate new body rate.
        state.wb += (state.i_inv * (state.mb - state.wb.cross(&(state.i*state.wb)))) * state.dt;

        // Integrate quaternion from new body rate.
        let p: f64 = state.wb.x;
        let q: f64 = state.wb.y;
        let r: f64 = state.wb.z;
        let q_transformation_matrix = Matrix4::new(
            0.0 , -p, -q, -r,
            p,  0.0,  -r, -q, 
            q, -r,  0.0,   p, 
            r,  q, -p,  0.0
        );
        let qdot: Vector4<f64> = 0.5 * q_transformation_matrix * state.q;
        state.q = (state.q + qdot*state.dt).normalize();

        // Integrate acceleration to position and velocity.
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
