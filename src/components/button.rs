use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use apricot::{app::App, rectangle::Rectangle, render_core::TextureId};

#[derive(Debug)]
/// Represents different events that could be generated by GUI components
pub enum Event {
    ButtonClicked(&'static str),
}

/// Queue for storing events generated by GUI components
pub struct EventQueue {
    queue: Mutex<VecDeque<Event>>,
}

impl EventQueue {
    /// Create a new EventQueue
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    /// Adds a new element to the end of the queue
    pub fn push(&self, event: Event) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(event);
    }

    /// Removes and returns the first event in the queue, or None
    pub fn pop(&self) -> Option<Event> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
}

/// Represents a clickable button
pub struct Button {
    /// Unique identifier for the button
    id: &'static str,
    /// The rectangle defining the button's position and size
    rect: Rectangle,
    /// The texture ID for the normal state of the button
    texture_id: TextureId,
    /// The texture ID for the hovered state of the button
    hovered_texture_id: TextureId,
    /// The event queue that stores events triggered by this button
    event_queue: Arc<EventQueue>,
}

impl Button {
    /// Creates a new button
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

    /// Checks if the button is being hovered and clicked
    pub fn update(&mut self, app: &App) {
        let is_hovered = self.rect.contains_point(&app.mouse_pos);
        if is_hovered && app.mouse_left_clicked {
            self.event_queue.push(Event::ButtonClicked(self.id));
        }
    }

    /// Renders a button to the screen
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