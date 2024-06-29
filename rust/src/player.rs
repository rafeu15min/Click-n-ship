use godot::engine::{
    CharacterBody2D, ICharacterBody2D, InputEvent, InputEventMouseButton, InputEventMouseMotion,
};
use godot::global::MouseButton;
use godot::obj::Base;
use godot::prelude::*;
use godot::register::GodotClass;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: f64,
    #[var]
    target: Vector2,
    target_move: Vector2,
    velocity: Vector2,
    base: Base<CharacterBody2D>,
}

//calma, vou fazer outro file pra camera
//safe

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 600.0,
            target: Vector2::new(600.0, 300.0),
            target_move: Vector2::new(600.0, 300.0),
            velocity: Vector2::ZERO,
            base,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let event1 = event.clone();
        match event.try_cast::<InputEventMouseButton>() {
            Ok(e) => {
                if e.get_button_index() == MouseButton::RIGHT && e.is_pressed() {
                    self.target = self.base().get_global_mouse_position();
                }
            }
            Err(_) => {}
        };
        match event1.try_cast::<InputEventMouseMotion>() {
            Ok(_) => {
                self.target_move = self.base().get_global_mouse_position();
            }
            Err(_) => {}
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        let friction = 0.98;
        let target_move = self.target_move;
        let target_velocity = self
            .base()
            .get_position()
            .direction_to(self.target)
            .normalized()
            * self.speed as f32;
        self.velocity = self
            .velocity
            .move_toward(target_velocity, (0.5 - friction) * self.speed as f32);
        let new_position = self.base().get_position() + self.velocity * _delta as f32;
        self.base_mut().look_at(target_move);
        if self.base().get_position().distance_to(self.target) > 10.0 {
            self.base_mut().set_position(new_position);
        }
    }
}
