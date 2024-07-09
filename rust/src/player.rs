use godot::engine::{CharacterBody2D, ICharacterBody2D, InputEvent, InputEventMouseButton, InputEventMouseMotion};
use godot::global::MouseButton;
use godot::obj::{Base, WithBaseField};
use godot::register::GodotClass;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: f64,
    target_move: Vector2,
    base: Base<CharacterBody2D>,
    acceleration: f32,
    #[export]
    max_acceleration: f32,
    is_moving: bool
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            speed: 750.0,
            target_move: Vector2::new(600.0, 300.0),
            base,
            acceleration: 0.0,
            max_acceleration: 1.5,
            is_moving: false
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let event1 = event.clone();
        match event.try_cast::<InputEventMouseButton>() {
            Ok(e) => {
                if e.get_button_index() == MouseButton::RIGHT {
                    self.is_moving = e.is_pressed();
                }
            }
            Err(_) => {}
        }
        match event1.try_cast::<InputEventMouseMotion>() {
            Ok(_) => {
                self.target_move = self.base().get_global_mouse_position();
            }
            Err(_) => {}
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        
        let target_move = self.target_move;
        

        if self.is_moving && self.acceleration <= self.max_acceleration{
            self.acceleration += 0.02;
        }
        if !self.is_moving {
            self.acceleration -= 0.01;
        }
        if self.acceleration < 0.0 || self.base().get_position().distance_to(target_move) < 10.0{
            self.acceleration = 0.0;
        }
        
        self.base_mut().look_at(target_move);
        let velocity = self.base().get_transform().a * self.speed as f32 * self.acceleration;
        self.base_mut().set_velocity(velocity);

        self.base_mut().move_and_slide();
    }
}