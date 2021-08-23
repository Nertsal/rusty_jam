use macroquad::prelude::{draw_circle, draw_poly_lines, Color};

use super::*;

impl<'a, 'b> Game<'a, 'b> {
    pub fn draw(&mut self) {
        let transforms = self.world.read_component::<Transform>();
        let rigidbodies = self.world.read_component::<Rigidbody>();
        let colors = self.world.read_component::<ColorComponent>();
        let healths = self.world.read_component::<Health>();
        for (entity, transform, rigidbody, color) in
            (&self.world.entities(), &transforms, &rigidbodies, &colors).join()
        {
            if let Some(health) = healths.get(entity) {
                draw_circle(
                    transform.position.x,
                    transform.position.y,
                    health.fraction() * rigidbody.radius,
                    color.color,
                );
            }
            draw_circle_outline(transform.position, rigidbody.radius, color.color);
        }
    }
}

fn draw_circle_outline(position: Vec2, radius: f32, color: Color) {
    draw_poly_lines(position.x, position.y, 50, radius, 0.0, 0.2, color);
}
