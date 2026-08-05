//! Tooltip component

use freya::prelude::*;

#[derive(PartialEq)]
pub struct Tooltip {
    text: String,
    child: Element,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            child: rsx! {},
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = child.into_element();
        self
    }
}

impl Component for Tooltip {
    fn render(&self) -> impl IntoElement {
        let mut show_tooltip = use_state(|| false);
        
        rect()
            .on_pointer_enter(move |_| show_tooltip.set(true))
            .on_pointer_leave(move |_| show_tooltip.set(false))
            .child(self.child.clone())
            .maybe(*show_tooltip.read(), |el| {
                el.child(
                    rect()
                        .position(Position::new_global().bottom(40.).center_horizontal())
                        .padding(Gaps::new_symmetric(8., 12.))
                        .background(crate::theme::colors::component_bg().with_a(240))
                        .corner_radius(CornerRadius::new_all(6.))
                        .border(
                            Border::new()
                                .fill(crate::theme::colors::component_border())
                                .width(1.)
                                .alignment(BorderAlignment::Inner),
                        )
                        .child(
                            label()
                                .text(self.text.clone())
                                .font_size(12.)
                                .color(crate::theme::colors::fg_primary()),
                        )
                )
            })
    }
}