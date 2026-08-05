//! Text input component

use freya::prelude::*;
use freya::text_edit::TextEdit;

#[derive(PartialEq)]
pub struct TextInput {
    placeholder: String,
    value: State<String>,
    on_change: Option<EventHandler<String>>,
    on_submit: Option<EventHandler<()>>,
}

impl TextInput {
    pub fn new() -> Self {
        Self {
            placeholder: String::new(),
            value: use_state(String::new),
            on_change: None,
            on_submit: None,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn value(mut self, value: State<String>) -> Self {
        self.value = value;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(String) + 'static) -> Self {
        self.on_change = Some(EventHandler::new(handler));
        self
    }

    pub fn on_submit(mut self, handler: impl Fn(()) + 'static) -> Self {
        self.on_submit = Some(EventHandler::new(handler));
        self
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for TextInput {
    fn render(&self) -> impl IntoElement {
        let value = self.value.clone();
        let on_change = self.on_change.clone();
        let on_submit = self.on_submit.clone();
        
        TextEdit::new()
            .placeholder(self.placeholder.clone())
            .text(value.read().clone())
            .on_change(move |text| {
                value.set(text.clone());
                if let Some(handler) = &on_change {
                    handler(text);
                }
            })
            .on_submit(move |_| {
                if let Some(handler) = &on_submit {
                    handler(());
                }
            })
            .font_size(14.)
            .padding(Gaps::new_symmetric(12., 8.))
            .corner_radius(CornerRadius::new_all(8.))
            .background(crate::theme::colors::component_bg())
            .border(
                Border::new()
                    .fill(crate::theme::colors::component_border())
                    .width(1.)
                    .alignment(BorderAlignment::Inner),
            )
    }
}