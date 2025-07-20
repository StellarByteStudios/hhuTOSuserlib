use alloc::{boxed::Box, string::String};
use core::{any::type_name, fmt};

use crate::{
    gameengine::{
        collision::{circle::Circle, collisiontrait::Collider, rect::Rect},
        gameframelayer::GameFrameLayer,
        position::Position,
        velocity::Velocity,
    },
    graphix::picturepainting::pictures::frame::Frame,
};

pub struct GameObject {
    name: String,
    position: Position,
    velocity: Velocity,
    sprite: Frame,
    collider: Box<dyn Collider>,
}
impl GameObject {
    fn new(
        name: String,
        position: Position,
        velocity: Velocity,
        sprite: Frame,
        collider: Box<dyn Collider>,
    ) -> Self {
        Self {
            name,
            position,
            velocity,
            sprite,
            collider,
        }
    }

    /* Getter */
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_position(&self) -> Position {
        self.position
    }

    pub fn get_velocity(&self) -> Velocity {
        self.velocity
    }

    /* checks */
    pub fn is_moving(&self) -> bool {
        return !self.velocity.is_zero();
    }

    // Gibt den Namen des anderen zurück
    pub fn check_collision(&self, other: &GameObject) -> Option<String> {
        let colision = self.collider.test_collision(other.collider.as_ref());

        //kprintln!("Texte collision von {} und {} mit dem ergebnis {}", self.name, other.name, colision);
        if self.collider.test_collision(other.collider.as_ref()) {
            return Some(other.get_name());
        }
        return None;
    }

    /* Bewegung */
    pub fn transform(&mut self, position_add: &Position) {
        self.position = self.position + position_add.clone();
        self.collider.move_position(&position_add);
    }

    pub fn acelerate(&mut self, velocity_add: &Velocity) {
        self.velocity = self.velocity + velocity_add.clone();
    }

    pub fn set_new_pos(&mut self, new_position: &Position) {
        let dif = self.position - new_position.clone();
        self.position = new_position.clone();
        self.collider.set_position(new_position);
    }

    pub fn set_new_velocity(&mut self, new_velocity: &Velocity) {
        self.velocity = new_velocity.clone();
    }

    pub fn tick(&mut self) {
        self.position = self.position + self.velocity;
        let dif = Position::new(self.velocity.get_x() as i32, self.velocity.get_y() as i32);
        self.collider.move_position(&dif);
    }

    /* Anzeige */
    pub fn print_on_game_layer(&self, layer: &mut GameFrameLayer) {
        layer.draw_sprite_on_position(&self.position, &self.sprite);
    }

    pub fn visual_move(&mut self, layer: &mut GameFrameLayer, new_position: &Position) {
        // Angezeigtes erstmal löschen
        layer.delete_sprite_on_position(&self.position, &self.sprite);

        // Gameobject verschieben
        self.set_new_pos(new_position);

        // neu zeichnen
        layer.draw_sprite_on_position(&self.position, &self.sprite);
    }

    pub fn visual_transform(&mut self, layer: &mut GameFrameLayer, offset: &Position) {
        // Angezeigtes erstmal löschen
        layer.delete_sprite_on_position(&self.position, &self.sprite);

        // Gameobject verschieben
        self.transform(offset);

        // neu zeichnen
        layer.draw_sprite_on_position(&self.position, &self.sprite);
    }

    pub fn visual_tick(&mut self, layer: &mut GameFrameLayer) {
        // Angezeigtes erstmal löschen
        layer.delete_sprite_on_position(&self.position, &self.sprite);

        // Gameobject verschieben
        self.tick();

        // neu zeichnen
        layer.draw_sprite_on_position(&self.position, &self.sprite);
    }
}

impl fmt::Debug for GameObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GameObject")
            .field("name", &self.name)
            .field("position", &self.position)
            .field("velocity", &self.velocity)
            .field("sprite", &self.sprite)
            .field("collider", &type_name::<dyn Collider>())
            .finish()
    }
}

// * * Gameobject Factory * * //
pub struct GameObjectFactory {
    object: GameObject,
}
impl GameObjectFactory {
    pub fn new() -> GameObjectFactory {
        GameObjectFactory {
            object: GameObject::new(
                String::new(),
                Position::new_zero(),
                Velocity::new_zero(),
                Frame::new(0, 0),
                Box::new(Rect::new(Position::new_zero(), 0, 0)),
            ),
        }
    }

    pub fn set_name(mut self, name: String) -> Self {
        self.object.name = name;
        self
    }

    pub fn set_position(mut self, position: &Position) -> Self {
        self.object.position = position.clone();
        self.object.collider.set_position(position);
        self
    }

    pub fn set_velocity(mut self, velocity: Velocity) -> Self {
        self.object.velocity = velocity;
        self
    }

    pub fn set_sprite(mut self, sprite: Frame) -> Self {
        self.object.sprite = sprite;
        self
    }

    pub fn set_rectangle_collider(mut self, width: usize, height: usize) -> Self {
        self.object.collider = Box::new(Rect::new(self.object.position.clone(), width, height));
        self
    }

    pub fn set_circular_collider(mut self, radius: usize) -> Self {
        let center = Position::new(
            self.object.position.get_x() + radius as i32,
            self.object.position.get_y() + radius as i32,
        );
        self.object.collider = Box::new(Circle::new(center, radius));
        self
    }

    // Gibt das Gameobject ab und konsumiert die Factory
    pub fn create(self) -> GameObject {
        return self.object;
    }
}
