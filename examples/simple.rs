use fusion_rust_bindings::{
    FusionAhrs, FusionAhrsGetQuaternion, FusionAhrsInitialise, FusionAhrsUpdateNoMagnetometer,
    FusionQuaternionToEuler, FusionVector,
};

const SAMPLE_PERIOD: f32 = 0.01; /* replace this with actual sample period */

fn main() {
    let mut ahrs: FusionAhrs = FusionAhrs::default();
    unsafe { FusionAhrsInitialise(&mut ahrs) };

    /* This loop should repeat each time new gyroscope data is available */
    loop {
        let gyroscope: FusionVector = FusionVector {
            array: [0.0, 0.0, 0.0],
        }; /* replace this with actual gyroscope data in degrees/s */

        let accelerometer: FusionVector = FusionVector {
            array: [0.0, 0.0, 1.0],
        }; /* replace this with actual accelerometer data in g */

        unsafe {
            FusionAhrsUpdateNoMagnetometer(&mut ahrs, gyroscope, accelerometer, SAMPLE_PERIOD);
            let euler = FusionQuaternionToEuler(FusionAhrsGetQuaternion(&ahrs));
            println!(
                "Roll {:.1} Pitch {:.1} Yaw {:.1}\n",
                euler.angle.roll, euler.angle.pitch, euler.angle.yaw
            );
        }
    }
}
