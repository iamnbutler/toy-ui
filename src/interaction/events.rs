//! Interaction event types and state

use super::ElementId;
use crate::layer::MouseButton;
use glam::Vec2;

/// Events generated by the interaction system
#[derive(Debug, Clone)]
pub enum InteractionEvent {
    /// Mouse entered an element
    MouseEnter { element_id: ElementId },

    /// Mouse left an element
    MouseLeave { element_id: ElementId },

    /// Mouse moved over an element
    MouseMove {
        element_id: ElementId,
        position: Vec2,
        local_position: Vec2,
    },

    /// Mouse button pressed on an element
    MouseDown {
        element_id: ElementId,
        button: MouseButton,
        position: Vec2,
        local_position: Vec2,
    },

    /// Mouse button released on an element
    MouseUp {
        element_id: ElementId,
        button: MouseButton,
        position: Vec2,
        local_position: Vec2,
    },

    /// Click event (mouse down + up on same element)
    Click {
        element_id: ElementId,
        button: MouseButton,
        position: Vec2,
        local_position: Vec2,
    },
}

/// Current interaction state of an element
#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    /// Whether the mouse is currently hovering over the element
    pub is_hovered: bool,

    /// Whether a mouse button is currently pressed on the element
    pub is_pressed: bool,
}

impl InteractionState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Handler trait for interaction events
pub trait InteractionHandler {
    /// Called when mouse enters the element
    fn on_mouse_enter(&mut self) {}

    /// Called when mouse leaves the element
    fn on_mouse_leave(&mut self) {}

    /// Called when mouse moves over the element
    fn on_mouse_move(&mut self, _position: Vec2, _local_position: Vec2) {}

    /// Called when mouse button is pressed on the element
    fn on_mouse_down(&mut self, _button: MouseButton, _position: Vec2, _local_position: Vec2) {}

    /// Called when mouse button is released on the element
    fn on_mouse_up(&mut self, _button: MouseButton, _position: Vec2, _local_position: Vec2) {}

    /// Called when element is clicked
    fn on_click(&mut self, _button: MouseButton, _position: Vec2, _local_position: Vec2) {}
}

/// Event handler closures for interactive elements
pub struct EventHandlers {
    pub on_mouse_enter: Option<Box<dyn FnMut()>>,
    pub on_mouse_leave: Option<Box<dyn FnMut()>>,
    pub on_mouse_move: Option<Box<dyn FnMut(Vec2, Vec2)>>,
    pub on_mouse_down: Option<Box<dyn FnMut(MouseButton, Vec2, Vec2)>>,
    pub on_mouse_up: Option<Box<dyn FnMut(MouseButton, Vec2, Vec2)>>,
    pub on_click: Option<Box<dyn FnMut(MouseButton, Vec2, Vec2)>>,
}

impl EventHandlers {
    pub fn new() -> Self {
        Self {
            on_mouse_enter: None,
            on_mouse_leave: None,
            on_mouse_move: None,
            on_mouse_down: None,
            on_mouse_up: None,
            on_click: None,
        }
    }

    /// Set the mouse enter handler
    pub fn on_mouse_enter<F>(mut self, handler: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.on_mouse_enter = Some(Box::new(handler));
        self
    }

    /// Set the mouse leave handler
    pub fn on_mouse_leave<F>(mut self, handler: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.on_mouse_leave = Some(Box::new(handler));
        self
    }

    /// Set the mouse move handler
    pub fn on_mouse_move<F>(mut self, handler: F) -> Self
    where
        F: FnMut(Vec2, Vec2) + 'static,
    {
        self.on_mouse_move = Some(Box::new(handler));
        self
    }

    /// Set the mouse down handler
    pub fn on_mouse_down<F>(mut self, handler: F) -> Self
    where
        F: FnMut(MouseButton, Vec2, Vec2) + 'static,
    {
        self.on_mouse_down = Some(Box::new(handler));
        self
    }

    /// Set the mouse up handler
    pub fn on_mouse_up<F>(mut self, handler: F) -> Self
    where
        F: FnMut(MouseButton, Vec2, Vec2) + 'static,
    {
        self.on_mouse_up = Some(Box::new(handler));
        self
    }

    /// Set the click handler
    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: FnMut(MouseButton, Vec2, Vec2) + 'static,
    {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Process an interaction event
    pub fn handle_event(&mut self, event: &InteractionEvent) {
        match event {
            InteractionEvent::MouseEnter { .. } => {
                if let Some(handler) = &mut self.on_mouse_enter {
                    handler();
                }
            }
            InteractionEvent::MouseLeave { .. } => {
                if let Some(handler) = &mut self.on_mouse_leave {
                    handler();
                }
            }
            InteractionEvent::MouseMove {
                position,
                local_position,
                ..
            } => {
                if let Some(handler) = &mut self.on_mouse_move {
                    handler(*position, *local_position);
                }
            }
            InteractionEvent::MouseDown {
                button,
                position,
                local_position,
                ..
            } => {
                if let Some(handler) = &mut self.on_mouse_down {
                    handler(*button, *position, *local_position);
                }
            }
            InteractionEvent::MouseUp {
                button,
                position,
                local_position,
                ..
            } => {
                if let Some(handler) = &mut self.on_mouse_up {
                    handler(*button, *position, *local_position);
                }
            }
            InteractionEvent::Click {
                button,
                position,
                local_position,
                ..
            } => {
                if let Some(handler) = &mut self.on_click {
                    handler(*button, *position, *local_position);
                }
            }
        }
    }
}

impl Default for EventHandlers {
    fn default() -> Self {
        Self::new()
    }
}
