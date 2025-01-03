use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use apricot::{app::App, rectangle::Rectangle, render_core::TextureId};

#[derive(Debug)]
pub enum Event {
    ButtonClicked(&'static str),
}

pub struct EventQueue {
    queue: Mutex<VecDeque<Event>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn push(&self, event: Event) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(event);
    }

    pub fn pop(&self) -> Option<Event> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
}

pub struct Button {
    id: &'static str,
    rect: Rectangle,
    texture_id: TextureId,
    hovered_texture_id: TextureId,
    event_queue: Arc<EventQueue>,
}

impl Button {
    pub fn new(
        id: &'static str,
        rect: Rectangle,
        texture_id: TextureId,
        hovered_texture_id: TextureId,
        event_queue: Arc<EventQueue>,
    ) -> Self {
        Self {
            id,
            rect,
            texture_id,
            hovered_texture_id,
            event_queue,
        }
    }

    pub fn update(&mut self, app: &App) {
        let is_hovered = self.rect.contains_point(&app.mouse_pos);
        if is_hovered && app.mouse_left_clicked {
            self.event_queue.push(Event::ButtonClicked(self.id));
        }
    }

    pub fn render(&mut self, app: &App) {
        let is_hovered = self.rect.contains_point(&app.mouse_pos);
        app.renderer.copy_texture(
            self.rect,
            if is_hovered {
                self.hovered_texture_id
            } else {
                self.texture_id
            },
            Rectangle::new(0.0, 0.0, 360.0, 360.0),
        );
    }
}
