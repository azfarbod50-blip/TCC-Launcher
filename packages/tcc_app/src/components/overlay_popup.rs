//! Overlay popup component

use freya::prelude::*;

#[derive(PartialEq)]
pub struct OverlayPopup {
    position: Option<Position>,
    on_close: Option<EventHandler<()>>,
    child: Element,
}

impl OverlayPopup {
    pub fn new() -> Self {
        Self {
            position: None,
            on_close: None,
            child: rsx! {},
        }
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }

    pub fn on_close(mut self, handler: impl Fn(()) + 'static) -> Self {
        self.on_close = Some(EventHandler::new(handler));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = child.into_element();
        self
    }
}

impl Default for OverlayPopup {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for OverlayPopup {
    fn render(&self) -> impl IntoElement {
        let on_close = self.on_close.clone();
        
        rect()
            .width(Size::fill())
            .height(Size::fill())
            .position(self.position.unwrap_or(Position::Center))
            .background(Color::from_argb(128, 0, 0, 0))
            .on_press(move |_| {
                if let Some(handler) = &on_close {
                    handler(());
                }
            })
            .child(self.child.clone())
    }
}