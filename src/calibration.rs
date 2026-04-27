//! Helpers for interpreting Orbbec calibration parameters.
use crate::sys::orb::{OBCameraDistortion, OBCameraDistortionModel, OBCameraIntrinsic};

#[cfg(feature = "nalgebra")]
use nalgebra::{Matrix2, Matrix3, Vector2};

/// Camera intrinsic parameters.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraIntrinsic {
    /// Focal length in x direction (pixels).
    pub fx: f32,
    /// Focal length in y direction (pixels).
    pub fy: f32,
    /// Optical center abscissa (pixels).
    pub cx: f32,
    /// Optical center ordinate (pixels).
    pub cy: f32,
    /// Image width (pixels).
    pub width: i16,
    /// Image height (pixels).
    pub height: i16,
}

impl From<OBCameraIntrinsic> for CameraIntrinsic {
    fn from(ob: OBCameraIntrinsic) -> Self {
        Self {
            fx: { ob.fx },
            fy: { ob.fy },
            cx: { ob.cx },
            cy: { ob.cy },
            width: { ob.width },
            height: { ob.height },
        }
    }
}

impl From<CameraIntrinsic> for OBCameraIntrinsic {
    fn from(c: CameraIntrinsic) -> Self {
        OBCameraIntrinsic {
            fx: c.fx,
            fy: c.fy,
            cx: c.cx,
            cy: c.cy,
            width: c.width,
            height: c.height,
        }
    }
}

/// Camera distortion parameters (Brown-Conrady K6 by default).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CameraDistortion {
    /// Radial distortion factor 1.
    pub k_1: f32,
    /// Radial distortion factor 2.
    pub k_2: f32,
    /// Radial distortion factor 3.
    pub k_3: f32,
    /// Radial distortion factor 4.
    pub k_4: f32,
    /// Radial distortion factor 5.
    pub k_5: f32,
    /// Radial distortion factor 6.
    pub k_6: f32,
    /// Tangential distortion factor 1.
    pub p_1: f32,
    /// Tangential distortion factor 2.
    pub p_2: f32,
    /// Distortion model.
    pub model: OBCameraDistortionModel,
}

impl From<OBCameraDistortion> for CameraDistortion {
    fn from(ob: OBCameraDistortion) -> Self {
        Self {
            k_1: { ob.k_1 },
            k_2: { ob.k_2 },
            k_3: { ob.k_3 },
            k_4: { ob.k_4 },
            k_5: { ob.k_5 },
            k_6: { ob.k_6 },
            p_1: { ob.p_1 },
            p_2: { ob.p_2 },
            model: { ob.model },
        }
    }
}

impl From<CameraDistortion> for OBCameraDistortion {
    fn from(c: CameraDistortion) -> Self {
        OBCameraDistortion {
            k_1: c.k_1,
            k_2: c.k_2,
            k_3: c.k_3,
            k_4: c.k_4,
            k_5: c.k_5,
            k_6: c.k_6,
            p_1: c.p_1,
            p_2: c.p_2,
            model: c.model,
        }
    }
}

impl CameraIntrinsic {
    /// Create a new camera intrinsic.
    /// ### Arguments
    /// * `fx` - Focal length in x direction (pixels).
    /// * `fy` - Focal length in y direction (pixels).
    /// * `cx` - Optical center abscissa (pixels).
    /// * `cy` - Optical center ordinate (pixels).
    /// * `width` - Image width (pixels).
    /// * `height` - Image height (pixels).
    pub const fn new(fx: f32, fy: f32, cx: f32, cy: f32, width: i16, height: i16) -> Self {
        CameraIntrinsic {
            fx,
            fy,
            cx,
            cy,
            width,
            height,
        }
    }

    #[cfg(feature = "nalgebra")]
    /// Create a new camera intrinsic from a 3x3 matrix.
    /// The matrix is expected to be in the following format:
    /// ```text
    /// [ fx  0  cx ]
    /// [  0 fy  cy ]
    /// [  0  0   1 ]
    /// ```
    /// ### Arguments
    /// * `matrix` - The 3x3 matrix.
    pub fn from_matrix(matrix: Matrix3<f32>, width: i16, height: i16) -> Self {
        let fx = matrix[(0, 0)];
        let fy = matrix[(1, 1)];
        let cx = matrix[(0, 2)];
        let cy = matrix[(1, 2)];
        Self::new(fx, fy, cx, cy, width, height)
    }

    #[cfg(feature = "nalgebra")]
    /// Convert the camera intrinsic to a 3x3 matrix.
    pub fn to_matrix(&self) -> Matrix3<f32> {
        Matrix3::new(self.fx, 0.0, self.cx, 0.0, self.fy, self.cy, 0.0, 0.0, 1.0)
    }

    #[cfg(feature = "nalgebra")]
    #[inline]
    /// Convert a point in pixel coordinates to normalized coordinates.
    ///
    /// Applies `K⁻¹ · [u, v, 1]ᵀ`, which is `((u − cx) / fx, (v − cy) / fy)`.
    /// ### Arguments
    /// * `p` - The point (in pixel coordinates).
    /// ### Returns
    /// The point (in normalized coordinates).
    pub fn pixel_to_normalized(&self, p: Vector2<f32>) -> Vector2<f32> {
        Vector2::new((p.x - self.cx) / self.fx, (p.y - self.cy) / self.fy)
    }

    #[cfg(feature = "nalgebra")]
    #[inline]
    /// Convert a point in normalized coordinates to pixel coordinates.
    ///
    /// Applies `K · [x, y, 1]ᵀ`, which is `(fx · x + cx, fy · y + cy)`.
    /// ### Arguments
    /// * `p` - The point (in normalized coordinates).
    /// ### Returns
    /// The point (in pixel coordinates).
    pub fn normalized_to_pixel(&self, p: Vector2<f32>) -> Vector2<f32> {
        Vector2::new(self.fx * p.x + self.cx, self.fy * p.y + self.cy)
    }
}

impl CameraDistortion {
    /// Create a new camera distortion.
    /// The distortion model is assumed to be Brown-Conrady K6.
    ///
    /// ### Arguments
    /// * `k_1` - Radial distortion factor 1.
    /// * `k_2` - Radial distortion factor 2.
    /// * `k_3` - Radial distortion factor 3.
    /// * `k_4` - Radial distortion factor 4.
    /// * `k_5` - Radial distortion factor 5.
    /// * `k_6` - Radial distortion factor 6.
    /// * `p_1` - Tangential distortion factor 1.
    /// * `p_2` - Tangential distortion factor 2.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        k_1: f32,
        k_2: f32,
        k_3: f32,
        k_4: f32,
        k_5: f32,
        k_6: f32,
        p_1: f32,
        p_2: f32,
    ) -> Self {
        CameraDistortion {
            k_1,
            k_2,
            k_3,
            k_4,
            k_5,
            k_6,
            p_1,
            p_2,
            model: OBCameraDistortionModel::BrownConradyK6,
        }
    }

    #[cfg(feature = "nalgebra")]
    #[inline]
    /// Distort a point using the camera distortion.
    /// ### Arguments
    /// * `p` - The point (in normalized coordinates) to distort, before distortion.
    /// ### Returns
    /// The distorted point (in normalized coordinates).
    pub fn distort(&self, p: Vector2<f32>) -> Vector2<f32> {
        let x = p.x;
        let y = p.y;

        let r2 = x * x + y * y;
        let r4 = r2 * r2;
        let r6 = r4 * r2;

        let radial_num = 1.0 + self.k_1 * r2 + self.k_2 * r4 + self.k_3 * r6;
        let radial_den = 1.0 + self.k_4 * r2 + self.k_5 * r4 + self.k_6 * r6;
        let radial = radial_num / radial_den;

        let x_tan = 2.0 * self.p_1 * x * y + self.p_2 * (r2 + 2.0 * x * x);
        let y_tan = self.p_1 * (r2 + 2.0 * y * y) + 2.0 * self.p_2 * x * y;

        Vector2::new(x * radial + x_tan, y * radial + y_tan)
    }

    #[cfg(feature = "nalgebra")]
    #[inline]
    /// Distort a point using the camera distortion.
    /// ### Arguments
    /// * `p` - The point (in pixel coordinates) to distort, before distortion.
    /// ### Returns
    /// The distorted point (in pixel coordinates).
    pub fn distort_pixel(&self, calibration: &CameraIntrinsic, p: Vector2<f32>) -> Vector2<f32> {
        calibration.normalized_to_pixel(self.distort(calibration.pixel_to_normalized(p)))
    }

    #[cfg(feature = "nalgebra")]
    /// Compute the residual and Jacobian of the Brown-Conrady k6 model.
    /// ### Arguments
    /// * `p_u` - The undistorted point (in normalized coordinates).
    /// * `p_d` - The distorted point (in normalized coordinates).
    /// ### Returns
    /// A tuple containing the residual and Jacobian.
    fn residual_and_jacobian(
        &self,
        p_u: Vector2<f32>,
        p_d: Vector2<f32>,
    ) -> (Vector2<f32>, Matrix2<f32>) {
        let x = p_u.x;
        let y = p_u.y;

        let r2 = x * x + y * y;
        let r4 = r2 * r2;
        let r6 = r4 * r2;

        let num = 1.0 + self.k_1 * r2 + self.k_2 * r4 + self.k_3 * r6;
        let den = 1.0 + self.k_4 * r2 + self.k_5 * r4 + self.k_6 * r6;
        let radial = num / den;

        let dnum_dr2 = self.k_1 + 2.0 * self.k_2 * r2 + 3.0 * self.k_3 * r4;
        let dden_dr2 = self.k_4 + 2.0 * self.k_5 * r2 + 3.0 * self.k_6 * r4;
        let dradial_dr2 = (dnum_dr2 * den - num * dden_dr2) / (den * den);

        let dradial_dx = dradial_dr2 * 2.0 * x;
        let dradial_dy = dradial_dr2 * 2.0 * y;

        let x_tan = 2.0 * self.p_1 * x * y + self.p_2 * (r2 + 2.0 * x * x);
        let y_tan = self.p_1 * (r2 + 2.0 * y * y) + 2.0 * self.p_2 * x * y;

        let fx = x * radial + x_tan - p_d.x;
        let fy = y * radial + y_tan - p_d.y;
        let residual = Vector2::new(fx, fy);

        let dx_tan_dx = 2.0 * self.p_1 * y + 6.0 * self.p_2 * x;
        let dx_tan_dy = 2.0 * self.p_1 * x + 2.0 * self.p_2 * y;
        let dy_tan_dx = 2.0 * self.p_1 * x + 2.0 * self.p_2 * y;
        let dy_tan_dy = 6.0 * self.p_1 * y + 2.0 * self.p_2 * x;

        let j11 = radial + x * dradial_dx + dx_tan_dx;
        let j12 = x * dradial_dy + dx_tan_dy;
        let j21 = y * dradial_dx + dy_tan_dx;
        let j22 = radial + y * dradial_dy + dy_tan_dy;

        let jacobian = Matrix2::new(j11, j12, j21, j22);
        (residual, jacobian)
    }

    #[cfg(feature = "nalgebra")]
    /// Invert the Brown-Conrady k6 model in normalized coordinates.
    /// ### Warning: This is a very expensive operation.
    /// ### Arguments
    /// * `p_d` - The distorted point (in normalized coordinates).
    /// * `max_iters` - The maximum number of iterations.
    /// * `eps` - The epsilon tolerance.
    /// ### Returns
    /// A result containing the undistorted point and whether the inversion converged.
    pub fn try_undistort(&self, p_d: Vector2<f32>, max_iters: usize, eps: f32) -> UndistortResult {
        // Good default start for normal lenses: undistorted ~= distorted.
        let mut p = p_d;
        let mut iterations = 0;

        for i in 0..max_iters {
            iterations = i + 1;

            let (residual, jacobian) = self.residual_and_jacobian(p, p_d);
            let err = residual.norm();
            if err <= eps {
                return UndistortResult {
                    point_undistorted: p,
                    converged: true,
                    iterations,
                    residual_norm: err,
                };
            }

            let Some(step) = jacobian.lu().solve(&(-residual)) else {
                return UndistortResult {
                    point_undistorted: p,
                    converged: false,
                    iterations,
                    residual_norm: err,
                };
            };

            // Damped Newton / backtracking line search.
            let base_err2 = err * err;
            let mut alpha = 1.0;
            let mut accepted = false;

            while alpha >= 1.0 / 64.0 {
                let candidate = p + step * alpha;
                let cand_err2 = (self.distort(candidate) - p_d).norm_squared();
                if cand_err2 < base_err2 {
                    p = candidate;
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if !accepted {
                return UndistortResult {
                    point_undistorted: p,
                    converged: false,
                    iterations,
                    residual_norm: err,
                };
            }

            if step.norm() * alpha <= eps {
                let final_err = (self.distort(p) - p_d).norm();
                return UndistortResult {
                    point_undistorted: p,
                    converged: final_err <= eps,
                    iterations,
                    residual_norm: final_err,
                };
            }
        }

        let final_err = (self.distort(p) - p_d).norm();
        UndistortResult {
            point_undistorted: p,
            converged: final_err <= eps,
            iterations,
            residual_norm: final_err,
        }
    }

    #[cfg(feature = "nalgebra")]
    #[inline]
    /// Undistort a point in pixel coordinates using the camera distortion.
    /// ### Warning: This is a very expensive operation.
    /// ### Arguments
    /// * `point_distorted` - The distorted point (in pixel coordinates).
    /// ### Returns
    /// The undistorted point (in pixel coordinates).
    pub fn undistort_pixel(
        &self,
        calibration: &CameraIntrinsic,
        point_distorted: Vector2<f32>,
    ) -> Vector2<f32> {
        calibration.normalized_to_pixel(
            self.try_undistort(calibration.pixel_to_normalized(point_distorted), 10, 1e-6)
                .point_undistorted,
        )
    }

    #[cfg(feature = "nalgebra")]
    #[inline]
    /// ### Warning: This is a very expensive operation.
    pub fn undistort(&self, point_distorted: Vector2<f32>) -> Vector2<f32> {
        self.try_undistort(point_distorted, 5, 1e-6)
            .point_undistorted
    }
}

#[cfg(feature = "nalgebra")]
/// Result of undistortion of a point using the Brown-Conrady k6 model.
pub struct UndistortResult {
    /// The undistorted point (in normalized coordinates).
    pub point_undistorted: Vector2<f32>,
    /// Whether the inversion converged.
    pub converged: bool,
    /// The number of iterations.
    pub iterations: usize,
    /// Norm of the residual vector, which is the difference between the distorted and undistorted point.
    pub residual_norm: f32,
}

#[cfg(all(test, feature = "nalgebra"))]
mod tests {
    use super::*;

    const COLOR_CAMERA_INTRINSIC: CameraIntrinsic =
        CameraIntrinsic::new(1126.4753, 1126.2233, 951.83734, 502.78265, 1920, 1080);
    const COLOR_CAMERA_DISTORTION: CameraDistortion = CameraDistortion::new(
        0.0813251,
        -0.107933,
        0.0442094,
        0.0,
        0.0,
        0.0,
        -0.0000137227,
        0.0000265599,
    );

    const DEPTH_CAMERA_INTRINSIC: CameraIntrinsic =
        CameraIntrinsic::new(504.9072, 504.85593, 338.72913, 333.5183, 640, 576);
    const DEPTH_CAMERA_DISTORTION: CameraDistortion = CameraDistortion::new(
        11.37115,
        6.0372944,
        0.23659736,
        11.6887865,
        9.872374,
        1.4392949,
        4.627623e-5,
        2.1284488e-5,
    );

    #[test]
    fn test_undistort_color_camera() {
        // Iterate over a grid of points and undistort them, then redistort them and check if the result is close to the original point
        for x in 0..100 {
            for y in 0..100 {
                let distorted = Vector2::new(x as f32 / 100.0, y as f32 / 100.0);
                let undistorted = COLOR_CAMERA_DISTORTION
                    .try_undistort(distorted, 10, 1e-6)
                    .point_undistorted;
                let redistorted = COLOR_CAMERA_DISTORTION.distort(undistorted);
                let error = (redistorted - distorted).norm();
                assert!(error < 1e-6, "x: {x}, y: {y}, error: {error}");
            }
        }
    }

    #[test]
    fn test_undistort_pixels_color_camera() {
        for x in 0..COLOR_CAMERA_INTRINSIC.width {
            for y in 0..COLOR_CAMERA_INTRINSIC.height {
                let distorted = Vector2::new(x as f32, y as f32);
                let undistorted =
                    COLOR_CAMERA_DISTORTION.undistort_pixel(&COLOR_CAMERA_INTRINSIC, distorted);
                let redistorted =
                    COLOR_CAMERA_DISTORTION.distort_pixel(&COLOR_CAMERA_INTRINSIC, undistorted);
                let error = (redistorted - distorted).norm();
                assert!(
                    error < (1e-6 * COLOR_CAMERA_INTRINSIC.width as f32),
                    "x: {x}, y: {y}, error: {error}"
                );
            }
        }
    }

    #[test]
    fn test_undistort_depth_camera() {
        for x in 0..100 {
            for y in 0..100 {
                let distorted = Vector2::new(x as f32 / 100.0, y as f32 / 100.0);
                let undistorted = DEPTH_CAMERA_DISTORTION.undistort(distorted);
                let redistorted = DEPTH_CAMERA_DISTORTION.distort(undistorted);
                let error = (redistorted - distorted).norm();
                assert!(error < 1e-6, "x: {x}, y: {y}, error: {error}");
            }
        }
    }

    #[test]
    fn test_undistort_pixels_depth_camera() {
        for x in 0..DEPTH_CAMERA_INTRINSIC.width {
            for y in 0..DEPTH_CAMERA_INTRINSIC.height {
                let distorted = Vector2::new(x as f32, y as f32);
                let undistorted =
                    DEPTH_CAMERA_DISTORTION.undistort_pixel(&DEPTH_CAMERA_INTRINSIC, distorted);
                let redistorted =
                    DEPTH_CAMERA_DISTORTION.distort_pixel(&DEPTH_CAMERA_INTRINSIC, undistorted);
                let error = (redistorted - distorted).norm();
                assert!(
                    error < (1e-6 * DEPTH_CAMERA_INTRINSIC.width as f32),
                    "x: {x}, y: {y}, error: {error}"
                );
            }
        }
    }
}
