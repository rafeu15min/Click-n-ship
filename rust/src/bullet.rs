use godot::engine::{Area2D, IArea2D};
use godot::register::GodotClass;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Bullet {
    #[export]
    speed: f64,
    base: Base<Area2D>
}

#[godot_api]
impl IArea2D for Bullet {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            speed: 1000.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut position = self.base().get_global_transform().origin; // Get current position
        position = position + self.base().get_transform().a * self.speed as f32 * delta as f32;
        self.base_mut().set_global_transform(Transform2D::from_angle_origin(0.0,position));
    }

    #[cfg(FALSE)] 
    #[func]
    fn on_collision(&mut self, item: Gd<Area2D>) {
        if item.is_in_group("tiny_meteor"){
            body.queue_free();
        }
        self.queue_free();
    }
}


