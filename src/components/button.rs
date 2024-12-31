use apricot::{app::App, rectangle::Rectangle, render_core::TextureId};

pub struct Button {
    rect: Rectangle,
    texture_id: TextureId,
    hovered_texture_id: TextureId,
}

impl Button {
    pub fn new(rect: Rectangle, texture_id: TextureId, hovered_texture_id: TextureId) -> Self {
        Self {
            rect,
            texture_id,
            hovered_texture_id,
        }
    }

    pub fn update(&self, app: &App) {
        let turn_hovered = self.rect.contains_point(&app.mouse_pos);
        app.renderer.copy_texture(
            self.rect,
            if turn_hovered {
                self.hovered_texture_id
            } else {
                self.texture_id
            },
            Rectangle::new(0.0, 0.0, 360.0, 360.0),
        );
    }
}
