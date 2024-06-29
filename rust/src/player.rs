use godot::engine::{CharacterBody2D, ICharacterBody2D, InputEvent, InputEventMouseButton, InputEventMouseMotion};
use godot::global::MouseButton;
use godot::obj::Base;
use godot::register::GodotClass;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player{
    #[export]
    speed: f64,
    #[var]
    target: Vector2,
    target_move: Vector2,
    base: Base<CharacterBody2D>
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {        
        Self {
            speed: 600.0,
            target: Vector2::new( 600.0, 300.0),
            target_move: Vector2::new( 600.0, 300.0),
            base
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let event1 = event.clone();
        match event.try_cast::<InputEventMouseButton>() {
            Ok(e) => {
               if e.get_button_index() == MouseButton::RIGHT && e.is_pressed(){
                   self.target = self.base().get_global_mouse_position();
               }
            }
            Err(_) => {}
        };
        match event1.try_cast::<InputEventMouseMotion>(){
            Ok(_)=>{
                self.target_move = self.base().get_global_mouse_position();
            }
            Err(_) => {}
        }

    }
    
    fn physics_process(&mut self, delta: f64) {
        let target = self.target;
        let target_move = self.target_move;
        let velocity = self.base().get_position().direction_to(target).normalized() * self.speed as f32;
        self.base_mut().set_velocity(velocity);
        self.base_mut().look_at(target_move);
        if self.base().get_position().distance_to(target) > 10.0  {
            self.base_mut().move_and_slide();
        }
        let new_position = self.base().get_position() + velocity * delta as f32;
        self.base_mut().set_velocity(new_position);
    }


}