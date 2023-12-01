use gpui::{AnyView, DefiniteLength};

use crate::prelude::*;
use crate::{ButtonCommon, ButtonLike, ButtonSize, ButtonStyle, Label, LineHeightStyle};

#[derive(IntoElement)]
pub struct Button {
    base: ButtonLike,
    label: SharedString,
    label_color: Option<Color>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            base: ButtonLike::new(id),
            label: label.into(),
            label_color: None,
        }
    }

    pub fn color(mut self, label_color: impl Into<Option<Color>>) -> Self {
        self.label_color = label_color.into();
        self
    }
}

impl Selectable for Button {
    fn selected(mut self, selected: bool) -> Self {
        self.base = self.base.selected(selected);
        self
    }
}

impl Disableable for Button {
    fn disabled(mut self, disabled: bool) -> Self {
        self.base = self.base.disabled(disabled);
        self
    }
}

impl Clickable for Button {
    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut WindowContext) + 'static,
    ) -> Self {
        self.base = self.base.on_click(handler);
        self
    }
}

impl FixedWidth for Button {
    fn width(mut self, width: DefiniteLength) -> Self {
        self.base = self.base.width(width);
        self
    }

    fn full_width(mut self) -> Self {
        self.base = self.base.full_width();
        self
    }
}

impl ButtonCommon for Button {
    fn id(&self) -> &ElementId {
        self.base.id()
    }

    fn style(mut self, style: ButtonStyle) -> Self {
        self.base = self.base.style(style);
        self
    }

    fn size(mut self, size: ButtonSize) -> Self {
        self.base = self.base.size(size);
        self
    }

    fn tooltip(mut self, tooltip: impl Fn(&mut WindowContext) -> AnyView + 'static) -> Self {
        self.base = self.base.tooltip(tooltip);
        self
    }
}

impl RenderOnce for Button {
    type Rendered = ButtonLike;

    fn render(self, _cx: &mut WindowContext) -> Self::Rendered {
        let label_color = if self.base.disabled {
            Color::Disabled
        } else if self.base.selected {
            Color::Selected
        } else {
            self.label_color.unwrap_or_default()
        };

        self.base.child(
            Label::new(self.label)
                .color(label_color)
                .line_height_style(LineHeightStyle::UILabel),
        )
    }
}
