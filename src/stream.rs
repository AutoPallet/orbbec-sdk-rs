//! Stream module
use crate::{
    CameraDistortion, CameraIntrinsic, Format,
    error::OrbbecError,
    sys::stream::{OBStreamProfile, OBStreamProfileList},
};

/// Stream profile trait
pub trait StreamProfile: AsRef<OBStreamProfile> {}
/// Video Stream profile
pub struct VideoStreamProfile {
    inner: OBStreamProfile,
}

impl VideoStreamProfile {
    pub(crate) fn new(inner: OBStreamProfile) -> Self {
        VideoStreamProfile { inner }
    }

    pub(crate) fn inner(&self) -> &OBStreamProfile {
        &self.inner
    }

    /// Get the camera intrinsic parameters for this video stream profile
    pub fn get_intrinsic(&self) -> Result<CameraIntrinsic, OrbbecError> {
        self.inner
            .get_video_intrinsic()
            .map(CameraIntrinsic::from)
            .map_err(OrbbecError::from)
    }

    /// Get the camera distortion parameters for this video stream profile
    pub fn get_distortion(&self) -> Result<CameraDistortion, OrbbecError> {
        self.inner
            .get_video_distortion()
            .map(CameraDistortion::from)
            .map_err(OrbbecError::from)
    }

    #[cfg(feature = "nalgebra")]
    /// Get the extrinsic for source stream to target stream
    /// ### Arguments
    /// * `target` - The target video stream profile.
    /// ### Returns
    /// The transform (in millimeters) from the source stream to the target stream as a 3D isometry.
    pub fn get_extrinsic_to(
        &self,
        target: &VideoStreamProfile,
    ) -> Result<nalgebra::Isometry3<f32>, OrbbecError> {
        self.inner
            .get_extrinsic_to(target.inner())
            .map_err(OrbbecError::from)
            .map(|t| {
                let rot = nalgebra::Rotation3::from_matrix_unchecked(nalgebra::Matrix3::new(
                    t.rot[0], t.rot[1], t.rot[2], t.rot[3], t.rot[4], t.rot[5], t.rot[6], t.rot[7],
                    t.rot[8],
                ));
                let trans = nalgebra::Translation3::new(t.trans[0], t.trans[1], t.trans[2]);
                nalgebra::Isometry3::from_parts(trans, rot.into())
            })
    }

    /// Get the width of this video stream profile in pixels
    pub fn width(&self) -> u32 {
        self.inner
            .get_video_width()
            .map_err(OrbbecError::from)
            .unwrap()
    }

    /// Get the height of this video stream profile in pixels
    pub fn height(&self) -> u32 {
        self.inner
            .get_video_height()
            .map_err(OrbbecError::from)
            .unwrap()
    }

    /// Get the frame rate of this video stream profile
    pub fn fps(&self) -> u32 {
        self.inner
            .get_video_fps()
            .map_err(OrbbecError::from)
            .unwrap()
    }

    /// Get the pixel format of this stream profile
    pub fn format(&self) -> Result<Format, OrbbecError> {
        self.inner.get_format().map_err(OrbbecError::from)
    }
}

impl AsRef<OBStreamProfile> for VideoStreamProfile {
    fn as_ref(&self) -> &OBStreamProfile {
        &self.inner
    }
}
impl StreamProfile for VideoStreamProfile {}

/// List of video stream profiles
pub struct StreamProfileList {
    inner: OBStreamProfileList,
}

impl StreamProfileList {
    pub(crate) fn new(inner: OBStreamProfileList) -> Self {
        StreamProfileList { inner }
    }

    /// Get the corresponding video stream profile through the passed parameters.
    ///
    /// On failure, the returned error is enriched with the list of available
    /// video stream profiles in this list (see [`Self::describe_video_profiles`]),
    /// so callers don't have to format that themselves.
    /// ### Arguments
    /// * `width` - Width of the stream in pixels
    /// * `height` - Height of the stream in pixels
    /// * `format` - Pixel format of the stream
    /// * `fps` - Frame rate of the stream in frames per second
    pub fn get_video_stream_profile(
        &self,
        width: u16,
        height: u16,
        format: Format,
        fps: u8,
    ) -> Result<VideoStreamProfile, OrbbecError> {
        match self
            .inner
            .get_video_stream_profile(width as i32, height as i32, format, fps as i32)
        {
            Ok(p) => Ok(VideoStreamProfile::new(p)),
            Err(err) => {
                let available = self.describe_video_profiles();
                let mut orbbec_err = OrbbecError::from(err);
                let prefix = format!(
                    "Requested video stream profile {width}x{height} @ {fps}fps ({format:?}) is not supported. Available video stream profiles:\n{available}\nUnderlying error",
                );
                let data = match &mut orbbec_err {
                    OrbbecError::Unknown(d)
                    | OrbbecError::StdException(d)
                    | OrbbecError::CameraDisconnected(d)
                    | OrbbecError::PlatformException(d)
                    | OrbbecError::InvalidValue(d)
                    | OrbbecError::WrongAPICallSequence(d)
                    | OrbbecError::NotImplemented(d)
                    | OrbbecError::IOException(d)
                    | OrbbecError::MemoryException(d)
                    | OrbbecError::UnsupportedOperation(d)
                    | OrbbecError::AccessDenied(d) => d,
                };
                data.message = format!("{prefix}: {}", data.message);
                Err(orbbec_err)
            }
        }
    }

    /// Render every video stream profile in this list as a human-readable
    /// `WxH @ FPSfps (FORMAT)` line, one per line. Used by
    /// [`Self::get_video_stream_profile`] to enrich its error message and
    /// surfaced publicly for callers that want to log the available
    /// profiles without triggering a lookup failure.
    pub fn describe_video_profiles(&self) -> String {
        let mut entries = Vec::new();
        for profile in self {
            match profile {
                Ok(p) => {
                    let format = p
                        .format()
                        .map(|f| format!("{f:?}"))
                        .unwrap_or_else(|_| "Unknown".to_string());
                    entries.push(format!(
                        "{}x{} @ {}fps ({format})",
                        p.width(),
                        p.height(),
                        p.fps()
                    ));
                }
                Err(err) => entries.push(format!("<error reading profile: {err}>")),
            }
        }
        if entries.is_empty() {
            "none reported".to_string()
        } else {
            entries.join("\n")
        }
    }

    /// Get the number of stream profiles in this list
    pub fn len(&self) -> Result<usize, OrbbecError> {
        self.inner
            .get_count()
            .map(|count| count as usize)
            .map_err(OrbbecError::from)
    }

    /// Return whether this list has no profiles
    pub fn is_empty(&self) -> Result<bool, OrbbecError> {
        Ok(self.len()? == 0)
    }

    /// Get the video stream profile at `index`
    pub fn get(&self, index: usize) -> Result<VideoStreamProfile, OrbbecError> {
        self.inner
            .get_stream_profile(index as u32)
            .map(VideoStreamProfile::new)
            .map_err(OrbbecError::from)
    }

    /// Get an iterator over the stream profiles in the list.
    pub fn iter(&self) -> StreamProfileListIterator<'_> {
        StreamProfileListIterator::new(self)
    }
}

/// An iterator over video stream profiles in a stream profile list
pub struct StreamProfileListIterator<'a> {
    profile_list: &'a StreamProfileList,
    index: usize,
    count: usize,
}

impl<'a> StreamProfileListIterator<'a> {
    fn new(profile_list: &'a StreamProfileList) -> Self {
        StreamProfileListIterator {
            profile_list,
            index: 0,
            count: profile_list.len().unwrap(),
        }
    }
}

impl<'a> Iterator for StreamProfileListIterator<'a> {
    type Item = Result<VideoStreamProfile, OrbbecError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            None
        } else {
            let profile = self.profile_list.get(self.index);
            self.index += 1;
            Some(profile)
        }
    }
}

impl<'a> IntoIterator for &'a StreamProfileList {
    type Item = Result<VideoStreamProfile, OrbbecError>;
    type IntoIter = StreamProfileListIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
