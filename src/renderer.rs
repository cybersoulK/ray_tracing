use std::f32::consts::PI;

use glam::{UVec2, Vec3, Quat, Vec2, Vec4};



pub trait Light {
    fn get_light_ray(&self, point: Vec3, normal: Vec3) -> Option<Ray>;
    fn check_shadow(&self, point: Vec3, intersected_point: Vec3) -> bool;
    fn intensity(&self, point: Vec3, normal: Vec3) -> f32;
}

pub struct SpotLight {
    pub position: Vec3,
    pub intensity: f32
}

impl Light for SpotLight {
    fn get_light_ray(&self, point: Vec3, normal: Vec3) -> Option<Ray> {

        let light_normal = (self.position - point).normalize();

        if light_normal.angle_between(normal) < PI / 2.0 {
            Some(Ray { position: point, direction: light_normal })
        }
        else { None }
    }

    fn check_shadow(&self, point: Vec3, intersected_point: Vec3) -> bool {
        self.position.distance(point) > intersected_point.distance(point)
    }

    fn intensity(&self, point: Vec3, normal: Vec3) -> f32 {

        let by_distance = self.intensity / ((self.position - point).length()).sqrt();
        let by_angle = 1.0 - (self.position - point).angle_between(normal) / (PI / 2.0);

        by_distance * by_angle
    }
}


pub struct DirectionalLight {
    pub direction: Vec3,
    pub intensity: f32
}

impl Light for DirectionalLight {
    fn get_light_ray(&self, point: Vec3, normal: Vec3) -> Option<Ray> {
        
        let light_normal = self.direction * -1.0;

        if light_normal.angle_between(normal) < PI / 2.0 {
            Some(Ray { position: point, direction: light_normal })
        }
        else { None }
    }

    fn check_shadow(&self, _point: Vec3, _intersected_point: Vec3) -> bool {
        true
    }

    fn intensity(&self, _point: Vec3, normal: Vec3) -> f32 {

        let by_angle = 1.0 - (self.direction * -1.0).angle_between(normal) / (PI / 2.0);

        self.intensity * by_angle
    }
}



pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<f32>;
    fn get_normal(&self, point: Vec3) -> Vec3;
}


pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f32> {
        
        let a = ray.direction.powf(2.0);
        let b = 2.0 * ray.direction * (ray.position - self.position);
        let c = ray.position.powf(2.0) + self.position.powf(2.0) - 2.0 * ray.position * self.position;

        let a = a.x + a.y + a.z;
        let b = b.x + b.y + b.z;
        let c = c.x + c.y + c.z - self.radius.powf(2.0);


        let partial_t = b.powf(2.0) - 4.0 * a * c;

        if partial_t < 0.0 {
            None
        }
        else {
            let partial_t_sqrt = partial_t.sqrt();

            let t1 = (-b + partial_t_sqrt) / 2.0 * a;
            let t2 = (-b - partial_t_sqrt) / 2.0 * a;

            let t = t1.min(t2);
            Some(t)
        }
    }

    fn get_normal(&self, point: Vec3) -> Vec3 {
        (point - self.position).normalize()
    }
}


pub struct Camera {
    pub position: Vec3,
    pub rotation: Quat,

    ///fov_y in radians
    pub fov_y: f32,
    pub near_z: f32,
}


pub fn get_cursor_world_position(cursor_position: Vec2, camera: &Camera, screen_size: Vec2, z_depth: f32) -> Vec3 {

    let Camera { position, rotation, fov_y, near_z } = *camera;

    let scale = fov_y.atan() * near_z;
    let scale = glam::vec3(scale, scale, 1.0);

    let camera_matrix = glam::Mat4::from_scale_rotation_translation(scale, rotation, position);


    let screen_size_f32 = glam::vec2(screen_size.x as f32, screen_size.y as f32);


    let Vec2 { x, y } = cursor_position;

    let pixel_position = glam::vec3(x as f32 - screen_size_f32.x / 2.0, (y as f32 - screen_size_f32.y / 2.0) * -1.0, 0.1) / glam::vec3(screen_size_f32.y, screen_size_f32.y, 1.0);
    let pixel_world_position = camera_matrix.project_point3(pixel_position);

    let direction = (pixel_world_position - position).normalize();


    (z_depth - position.distance(pixel_position))* direction
}


#[derive(Clone, Copy)]
pub struct Material {
    pub color: Vec4,
}


pub struct Scene {
    pub objects: Vec<(u32, Box<dyn Shape>, Material)>,
    pub lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
}


#[derive(Debug)]
pub struct Ray {
    position: Vec3,
    direction: Vec3,
}

impl Ray {
    fn get_point(&self, t: f32) -> Vec3 {
        self.position + self.direction * t
    }
}


pub fn render(scene: &Scene, buffer: &mut [u8], screen_size: UVec2) {

    let Camera { position, rotation, fov_y, near_z } = scene.camera;

    let scale = fov_y.atan() * near_z;
    let scale = glam::vec3(scale, scale, 1.0);

    let camera_matrix = glam::Mat4::from_scale_rotation_translation(scale, rotation, position);


    let mut rays = Vec::new();

    let screen_size_f32 = glam::vec2(screen_size.x as f32, screen_size.y as f32);

    for y in 0..screen_size.y {
        for x in 0..screen_size.x {

            let pixel_position = glam::vec3(x as f32 - screen_size_f32.x / 2.0, (y as f32 - screen_size_f32.y / 2.0) * -1.0, near_z) / glam::vec3(screen_size_f32.y, screen_size_f32.y, 1.0);
            let pixel_world_position = camera_matrix.project_point3(pixel_position);

            rays.push(Ray { 
                position: pixel_world_position,
                direction: (pixel_world_position - position).normalize(),
            });
        }
    }

    for (i, ray) in rays.into_iter().enumerate() {

        let i = i * 4;

        let color = trace_ray(scene, ray, 3) * 255.0;

        buffer[i + 0] = color.x as u8;
        buffer[i + 1] = color.y as u8;
        buffer[i + 2] = color.z as u8;
        buffer[i + 3] = color.w as u8;
    }
}


fn trace_ray(scene: &Scene, ray: Ray, max_bounce: usize) -> Vec4 {

    if let Some((t, entity_id, shape, material)) = intersect_ray(scene, &ray, None) {
    
        let point = ray.get_point(t);
        let normal = shape.get_normal(point);


        let mut intensity = scene.lights.iter().filter_map(|light| {

            if let Some(ray) = light.get_light_ray(point, normal) {

                let has_light = if let Some((t, _, _, _)) = intersect_ray(scene, &ray, Some(entity_id)) {

                    let intersected_point = ray.get_point(t);

                    light.check_shadow(point, intersected_point) == false
                }
                else { true };


                if has_light {
                    Some(light.intensity(point, normal) * material.color)
                }
                else { None }
            }
            else { None }
            
        }).sum::<Vec4>();


        if max_bounce != 0 {
            let direction = ray.direction - 2.0 * ray.direction.dot(normal) * normal;
            let ray = Ray { position: point, direction };

            intensity += trace_ray(scene, ray, max_bounce - 1) * 0.5;
        }

        
        intensity.min(glam::vec4(1.0, 1.0, 1.0, 1.0))
    }
    else { Vec4::ZERO } 
}


fn intersect_ray<'a>(scene: &'a Scene, ray: &Ray, exclude_id: Option<u32>) -> Option<(f32, u32, &'a Box<dyn Shape>, &'a Material)> {

    scene.objects.iter().filter_map(|(entity_id, shape, material)| {

        if let Some(exclude_id) = exclude_id {
            if exclude_id == *entity_id { return None }
        }

        if let Some(t) = shape.intersect(ray) {

            if t >= 0.0 { Some((t, *entity_id, shape, material)) }
            else { None }
        }
        else { None }

    }).min_by(|(a, _, _, _), (b, _, _, _)| a.total_cmp(b))
}