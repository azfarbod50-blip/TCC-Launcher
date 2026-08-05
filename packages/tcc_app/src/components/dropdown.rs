//! Dropdown component

use freya::prelude::*;

#[derive(PartialEq)]
pub struct Dropdown<T> {
    items: Vec<(String, T)>,
    selected: State<Option<T>>,
    on_select: Option<EventHandler<T>>,
    placeholder: String,
}

impl<T: Clone + PartialEq + 'static> Dropdown<T> {
    pub fn new(items: Vec<(String, T)>) -> Self {
        Self {
            items,
            selected: use_state(|| None),
            on_select: None,
            placeholder: "Select...".to_string(),
        }
    }

    pub fn selected(mut self, selected: State<Option<T>>) -> Self {
        self.selected = selected;
        self
    }

    pub fn on_select(mut self, handler: impl Fn(T) + 'static) -> Self {
        self.on_select = Some(EventHandler::new(handler));
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
}

impl<T: Clone + PartialEq + 'static> Default for Dropdown<T> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<T: Clone + PartialEq + 'static> Component for Dropdown<T> {
    fn render(&self) -> impl IntoElement {
        let selected = self.selected.clone();
        let on_select = self.on_select.clone();
        let mut open = use_state(|| false);
        
        let selected_text = selected.read().as_ref()
            .and_then(|s| self.items.iter().find(|(_, v)| v == s).map(|(k, _)| k.clone()))
            .unwrap_or_else(|| self.placeholder.clone());
        
        let mut dropdown = rect()
            .horizontal()
            .cross_align(Alignment::Center)
            .spacing(8.)
            .padding(Gaps::new_symmetric(12., 8.))
            .corner_radius(CornerRadius::new_all(8.))
            .background(crate::theme::colors::component_bg())
            .border(
                Border::new()
                    .fill(crate::theme::colors::component_border())
                    .width(1.)
                    .alignment(BorderAlignment::Inner),
            )
            .on_press(move |_| open.set(!*open.read()))
            .child(
                label()
                    .text(selected_text)
                    .font_size(14.)
                    .color(crate::theme::colors::fg_primary()),
            )
            .child(
                Icon::new(IconType::ChevronDown)
                    .size(16.)
                    .color(crate::theme::colors::fg_secondary()),
            );

        if *open.read() {
            dropdown = dropdown.child(
                rect()
                    .vertical()
                    .width(Size::fill())
                    .position(Position::new_global().top(40.).left(0.))
                    .background(crate::theme::colors::page_elevated())
                    .border(
                        Border::new()
                            .fill(crate::theme::colors::component_border())
                            .width(1.)
                            .alignment(BorderAlignment::Inner),
                    )
                    .corner_radius(CornerRadius::new_all(8.))
                    .child(
                        self.items.iter().map(|(label, value)| {
                            let value = value.clone();
                            let on_select = on_select.clone();
                            let selected = selected.clone();
                            let open = open.clone();
                            
                            rect()
                                .width(Size::fill())
                                .padding(Gaps::new_symmetric(12., 8.))
                                .on_press(move |_| {
                                    selected.set(Some(value.clone()));
                                    if let Some(handler) = &on_select {
                                        handler(value);
                                    }
                                    open.set(false);
                                })
                                .child(
                                    label()
                                        .text(label.clone())
                                        .font_size(14.)
                                        .color(crate::theme::colors::fg_primary()),
                                )
                        })
                    ),
            );
        }

        dropdown.into_element()
    }
}