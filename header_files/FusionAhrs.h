/**
 *
 * The MIT License (MIT)
 *
 * Copyright (c) 2021 x-io Technologies
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 * @file FusionAhrs.h
 * @author Seb Madgwick
 * @brief AHRS algorithm to combine gyroscope, accelerometer, and magnetometer
 * measurements into a single measurement of orientation relative to the Earth.
 */

#ifndef FUSION_AHRS_H
#define FUSION_AHRS_H

//------------------------------------------------------------------------------
// Includes

#include "FusionConvention.h"
#include "FusionMath.h"
#include <stdbool.h>

//------------------------------------------------------------------------------
// Definitions

/**
 * @brief AHRS algorithm settings.
 */
typedef struct {
    FusionConvention convention;
    float gain;
    float gyroscopeRange;
    float accelerationRejection;
    float magneticRejection;
    unsigned int recoveryTriggerPeriod;
} FusionAhrsSettings;

/**
 * @brief AHRS algorithm structure.  Structure members are used internally and
 * must not be accessed by the application.
 */
typedef struct {
    FusionAhrsSettings settings;
    FusionQuaternion quaternion;
    FusionVector accelerometer;
    bool initialising;
    float rampedGain;
    float rampedGainStep;
    bool angularRateRecovery;
    FusionVector halfAccelerometerFeedback;
    FusionVector halfMagnetometerFeedback;
    bool accelerometerIgnored;
    int accelerationRecoveryTrigger;
    int accelerationRecoveryTimeout;
    bool magnetometerIgnored;
    int magneticRecoveryTrigger;
    int magneticRecoveryTimeout;
} FusionAhrs;

/**
 * @brief AHRS algorithm internal states.
 */
typedef struct {
    float accelerationError;
    bool accelerometerIgnored;
    float accelerationRecoveryTrigger;
    float magneticError;
    bool magnetometerIgnored;
    float magneticRecoveryTrigger;
} FusionAhrsInternalStates;

/**
 * @brief AHRS algorithm flags.
 */
typedef struct {
    bool initialising;
    bool angularRateRecovery;
    bool accelerationRecovery;
    bool magneticRecovery;
} FusionAhrsFlags;

//------------------------------------------------------------------------------
// Function declarations

void FusionAhrsInitialise(FusionAhrs *const ahrs);

void FusionAhrsReset(FusionAhrs *const ahrs);

void FusionAhrsSetSettings(FusionAhrs *const ahrs, const FusionAhrsSettings *const settings);

void FusionAhrsUpdate(FusionAhrs *const ahrs, const FusionVector gyroscope, const FusionVector accelerometer, const FusionVector magnetometer, const float deltaTime);

void FusionAhrsUpdateNoMagnetometer(FusionAhrs *const ahrs, const FusionVector gyroscope, const FusionVector accelerometer, const float deltaTime);

void FusionAhrsUpdateExternalHeading(FusionAhrs *const ahrs, const FusionVector gyroscope, const FusionVector accelerometer, const float heading, const float deltaTime);

FusionQuaternion FusionAhrsGetQuaternion(const FusionAhrs *const ahrs);

void FusionAhrsSetQuaternion(FusionAhrs *const ahrs, const FusionQuaternion quaternion);

FusionVector FusionAhrsGetGravity(const FusionAhrs *const ahrs);

FusionVector FusionAhrsGetLinearAcceleration(const FusionAhrs *const ahrs);

FusionVector FusionAhrsGetEarthAcceleration(const FusionAhrs *const ahrs);

FusionAhrsInternalStates FusionAhrsGetInternalStates(const FusionAhrs *const ahrs);

FusionAhrsFlags FusionAhrsGetFlags(const FusionAhrs *const ahrs);

void FusionAhrsSetHeading(FusionAhrs *const ahrs, const float heading);

#endif

//------------------------------------------------------------------------------
// End of file
