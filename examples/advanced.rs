use fusion_rust_bindings::{
    FusionAhrs, FusionAhrsGetEarthAcceleration, FusionAhrsGetQuaternion, FusionAhrsInitialise,
    FusionAhrsSetSettings, FusionAhrsSettings, FusionAhrsUpdate, FusionCalibrationInertial,
    FusionCalibrationMagnetic, FusionConvention_FusionConventionNwu, FusionEuler, FusionMatrix,
    FusionOffset, FusionOffsetInitialise, FusionOffsetUpdate, FusionQuaternionToEuler,
    FusionVector,
};

use std::time::Instant;

const SAMPLE_RATE: u32 = 100; /* replace this with actual sample rate */

fn main() {
    /* Define calibration (replace with actual calibration data if available) */
    let gyroscopeMisalignment = FusionMatrix {
        array: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };
    let gyroscopeSensitivity = FusionVector {
        array: [1.0, 1.0, 1.0],
    };
    let gyroscopeOffset = FusionVector {
        array: [0.0, 0.0, 0.0],
    };
    let accelerometerMisalignment = FusionMatrix {
        array: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };
    let accelerometerSensitivity = FusionVector {
        array: [1.0, 1.0, 1.0],
    };
    let accelerometerOffset = FusionVector {
        array: [0.0, 0.0, 0.0],
    };
    let softIronMatrix = FusionMatrix {
        array: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };
    let hardIronOffset = FusionVector {
        array: [0.0, 0.0, 0.0],
    };

    /* Initialise algorithms */
    let mut offset = FusionOffset::default();
    let mut ahrs = FusionAhrs::default();

    unsafe {
        FusionOffsetInitialise(&mut offset, SAMPLE_RATE);
        FusionAhrsInitialise(&mut ahrs);
    }

    /* Set AHRS algorithm settings */
    let settings = FusionAhrsSettings {
        convention: FusionConvention_FusionConventionNwu,
        gain: 0.5,
        gyroscopeRange: 2000.0, /* replace this with actual gyroscope range in degrees/s */
        accelerationRejection: 10.0,
        magneticRejection: 10.0,
        recoveryTriggerPeriod: 5 * SAMPLE_RATE, /* 5 seconds */
    };

    unsafe {
        FusionAhrsSetSettings(&mut ahrs, &settings);
    }

    let mut initial = true;
    let mut previousTimestamp = Instant::now();
    let mut delta_time = 0.0;

    /* This loop should repeat each time new gyroscope data is available */
    loop {
        /* Acquire latest sensor data */
        let mut gyroscope = FusionVector {
            array: [0.0, 0.0, 0.0],
        };
        let mut accelerometer = FusionVector {
            array: [0.0, 0.0, 1.0],
        };
        let mut magnetometer = FusionVector {
            array: [1.0, 0.0, 0.0],
        };

        unsafe {
            /* Apply calibration */
            gyroscope = FusionCalibrationInertial(
                gyroscope,
                gyroscopeMisalignment,
                gyroscopeSensitivity,
                gyroscopeOffset,
            );
            accelerometer = FusionCalibrationInertial(
                accelerometer,
                accelerometerMisalignment,
                accelerometerSensitivity,
                accelerometerOffset,
            );
            magnetometer = FusionCalibrationMagnetic(magnetometer, softIronMatrix, hardIronOffset);

            /* Update gyroscope offset correction algorithm */
            gyroscope = FusionOffsetUpdate(&mut offset, gyroscope);
        }

        /* Calculate delta time (in seconds) to account for gyroscope sample clock error */
        let timestamp = Instant::now();
        if initial {
            initial = false;
        } else {
            let delta_time = timestamp
                .saturating_duration_since(previousTimestamp)
                .as_secs_f32();
        }
        previousTimestamp = timestamp;

        unsafe {
            /* Update gyroscope AHRS algorithm */
            FusionAhrsUpdate(
                &mut ahrs,
                gyroscope,
                accelerometer,
                magnetometer,
                delta_time,
            );

            /* Get algorithm outputs */
            let euler = FusionQuaternionToEuler(FusionAhrsGetQuaternion(&ahrs));
            let earth = FusionAhrsGetEarthAcceleration(&ahrs);

            println!(
                "Roll {:.1}, Pitch {:.1}, Yaw {:.1}, X {:.1}, Y {:.1}, Z {:.1}\n",
                euler.angle.roll,
                euler.angle.pitch,
                euler.angle.yaw,
                earth.axis.x,
                earth.axis.y,
                earth.axis.z
            );
        }
    }
}
