use serde::{Deserialize, Serialize};
use sonoro_proto::pb::CameraParamInfo;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct CameraSettings {
    pub camera_mode: i32,
    pub pitch: f32,
    pub yaw: f32,
    pub roll: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub fov: i32,
    pub focus_distance: i32,
    pub blur_radius: i32,
    pub tilt_angle: i32,
    pub weather: i32,
    pub atmosphere: i32,
    pub wind_direction: i32,
}

impl CameraSettings {
    pub fn to_proto(&self) -> CameraParamInfo {
        CameraParamInfo {
            CameraMode: self.camera_mode,
            PitchValue: self.pitch,
            YawValue: self.yaw,
            RollValue: self.roll,
            PosX: self.pos_x,
            PosY: self.pos_y,
            PosZ: self.pos_z,
            Fov: self.fov,
            FocusDistance: self.focus_distance,
            BlurRadius: self.blur_radius,
            CameraTiltAngle: self.tilt_angle,
            Weather: self.weather,
            Atmosphere: self.atmosphere,
            WindDirection: self.wind_direction,
        }
    }

    pub fn from_proto(&mut self, proto: &CameraParamInfo) {
        self.camera_mode = proto.CameraMode;
        self.pitch = proto.PitchValue;
        self.yaw = proto.YawValue;
        self.roll = proto.RollValue;
        self.pos_x = proto.PosX;
        self.pos_y = proto.PosY;
        self.pos_z = proto.PosZ;
        self.fov = proto.Fov;
        self.focus_distance = proto.FocusDistance;
        self.blur_radius = proto.BlurRadius;
        self.tilt_angle = proto.CameraTiltAngle;
        self.weather = proto.Weather;
        self.atmosphere = proto.Atmosphere;
        self.wind_direction = proto.WindDirection;
    }
}
