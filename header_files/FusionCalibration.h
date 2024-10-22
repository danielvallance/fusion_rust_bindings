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
 * @file FusionCalibration.h
 * @author Seb Madgwick
 * @brief Gyroscope, accelerometer, and magnetometer calibration models.
 */

#ifndef FUSION_CALIBRATION_H
#define FUSION_CALIBRATION_H

//------------------------------------------------------------------------------
// Includes

#include "FusionMath.h"

//------------------------------------------------------------------------------
// Inline functions

/**
 * @brief Gyroscope and accelerometer calibration model.
 * @param uncalibrated Uncalibrated measurement.
 * @param misalignment Misalignment matrix.
 * @param sensitivity Sensitivity.
 * @param offset Offset.
 * @return Calibrated measurement.
 */
static inline FusionVector FusionCalibrationInertial(const FusionVector uncalibrated, const FusionMatrix misalignment, const FusionVector sensitivity, const FusionVector offset) {
    return FusionMatrixMultiplyVector(misalignment, FusionVectorHadamardProduct(FusionVectorSubtract(uncalibrated, offset), sensitivity));
}

/**
 * @brief Magnetometer calibration model.
 * @param uncalibrated Uncalibrated measurement.
 * @param softIronMatrix Soft-iron matrix.
 * @param hardIronOffset Hard-iron offset.
 * @return Calibrated measurement.
 */
static inline FusionVector FusionCalibrationMagnetic(const FusionVector uncalibrated, const FusionMatrix softIronMatrix, const FusionVector hardIronOffset) {
    return FusionMatrixMultiplyVector(softIronMatrix, FusionVectorSubtract(uncalibrated, hardIronOffset));
}

#endif

//------------------------------------------------------------------------------
// End of file
