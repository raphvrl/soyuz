use glam::{Mat4, Vec3};

#[derive(Debug, Clone)]
pub enum ProjectionType {
    Orthographic {
        width: f32,
        height: f32,
        near: f32,
        far: f32,
    },
    Perspective {
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    },
    Frustum {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

impl ProjectionType {
    pub fn to_matrix(&self) -> Mat4 {
        match self {
            ProjectionType::Orthographic {
                width,
                height,
                near,
                far,
            } => Mat4::orthographic_rh(
                -width / 2.0,
                width / 2.0,
                -height / 2.0,
                height / 2.0,
                *near,
                *far,
            ),
            ProjectionType::Perspective {
                fov,
                aspect,
                near,
                far,
            } => Mat4::perspective_rh(fov.to_radians(), *aspect, *near, *far),
            ProjectionType::Frustum {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => Mat4::frustum_rh(*left, *right, *bottom, *top, *near, *far),
        }
    }
}

pub struct Camera {
    pub position: Vec3,
    pub up: Vec3,
    pub yaw: f32,
    pub pitch: f32,

    projection: ProjectionType,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    view_proj_matrix: Mat4,
    dirty: bool,
}

impl Camera {
    pub fn new_perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let position = Vec3::new(0.0, 0.0, 5.0);
        let yaw: f32 = -90.0;
        let pitch: f32 = 0.0;
        let up = Vec3::Y;

        let projection = ProjectionType::Perspective {
            fov,
            aspect,
            near,
            far,
        };

        let projection_matrix = projection.to_matrix();
        let view_matrix =
            Self::compute_view_matrix(&position, yaw.to_radians(), pitch.to_radians(), &up);
        let view_proj_matrix = projection_matrix * view_matrix;

        Self {
            position,
            yaw,
            pitch,
            up,
            projection,
            view_matrix,
            projection_matrix,
            view_proj_matrix,
            dirty: false,
        }
    }

    pub fn new_orthographic(width: f32, height: f32, near: f32, far: f32) -> Self {
        let position = Vec3::new(0.0, 0.0, 0.0);
        let yaw_degrees: f32 = -90.0;
        let pitch_degrees: f32 = 0.0;
        let up = Vec3::Y;

        let projection = ProjectionType::Orthographic {
            width,
            height,
            near,
            far,
        };

        let projection_matrix = projection.to_matrix();
        let view_matrix = Self::compute_view_matrix(
            &position,
            yaw_degrees.to_radians(),
            pitch_degrees.to_radians(),
            &up,
        );
        let view_proj_matrix = projection_matrix * view_matrix;

        Self {
            position,
            yaw: yaw_degrees,
            pitch: pitch_degrees,
            up,
            projection,
            view_matrix,
            projection_matrix,
            view_proj_matrix,
            dirty: false,
        }
    }

    pub fn new_frustum(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let position = Vec3::new(0.0, 0.0, 0.0);
        let yaw_degrees: f32 = -90.0;
        let pitch_degrees: f32 = 0.0;
        let up = Vec3::Y;

        let projection = ProjectionType::Frustum {
            left,
            right,
            bottom,
            top,
            near,
            far,
        };

        let projection_matrix = projection.to_matrix();
        let view_matrix = Self::compute_view_matrix(
            &position,
            yaw_degrees.to_radians(),
            pitch_degrees.to_radians(),
            &up,
        );
        let view_proj_matrix = projection_matrix * view_matrix;

        Self {
            position,
            yaw: yaw_degrees,
            pitch: pitch_degrees,
            up,
            projection,
            view_matrix,
            projection_matrix,
            view_proj_matrix,
            dirty: false,
        }
    }

    pub fn rotate(&mut self, x_offset_degrees: f32, y_offset_degrees: f32) {
        self.yaw += x_offset_degrees;
        self.pitch -= y_offset_degrees.clamp(-89.0, 89.0);
        self.dirty = true;
    }

    pub fn forward(&self) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();
        Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        )
        .normalize()
    }

    pub fn right(&self) -> Vec3 {
        self.forward().cross(self.up).normalize()
    }

    fn compute_view_matrix(position: &Vec3, yaw_rad: f32, pitch_rad: f32, up: &Vec3) -> Mat4 {
        let forward = Vec3::new(
            yaw_rad.cos() * pitch_rad.cos(),
            pitch_rad.sin(),
            yaw_rad.sin() * pitch_rad.cos(),
        )
        .normalize();
        let target = *position + forward;
        Mat4::look_at_rh(*position, target, *up)
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
        self.dirty = true;
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.position += offset;
        self.dirty = true;
    }

    pub fn set_up(&mut self, up: Vec3) {
        self.up = up;
        self.dirty = true;
    }

    pub fn set_projection(&mut self, projection: ProjectionType) {
        self.projection = projection;
        self.projection_matrix = self.projection.to_matrix();
        self.dirty = true;
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        if let ProjectionType::Perspective {
            aspect: ref mut current_aspect,
            ..
        } = self.projection
        {
            *current_aspect = aspect;
            self.projection_matrix = self.projection.to_matrix();
            self.dirty = true;
        }
    }

    pub fn set_orthographic_size(&mut self, width: f32, height: f32) {
        if let ProjectionType::Orthographic {
            width: ref mut current_width,
            height: ref mut current_height,
            ..
        } = self.projection
        {
            *current_width = width;
            *current_height = height;
            self.projection_matrix = self.projection.to_matrix();
            self.dirty = true;
        }
    }

    pub fn update(&mut self) {
        if self.dirty {
            self.view_matrix = Self::compute_view_matrix(
                &self.position,
                self.yaw.to_radians(),
                self.pitch.to_radians(),
                &self.up,
            );
            self.view_proj_matrix = self.projection_matrix * self.view_matrix;
            self.dirty = false;
        }
    }

    pub fn view_matrix(&self) -> &Mat4 {
        &self.view_matrix
    }

    pub fn projection_matrix(&self) -> &Mat4 {
        &self.projection_matrix
    }

    pub fn view_proj_matrix(&self) -> &Mat4 {
        &self.view_proj_matrix
    }

    pub fn projection_type(&self) -> &ProjectionType {
        &self.projection
    }

    pub fn projection_type_mut(&mut self) -> &mut ProjectionType {
        self.dirty = true;
        &mut self.projection
    }
}
