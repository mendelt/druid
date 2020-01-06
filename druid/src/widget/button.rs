// Copyright 2018 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A button widget.

use crate::kurbo::Size;
use crate::widget::{Label, LabelText, Container};
use crate::{
    BoxConstraints, BoxedWidget, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget,
};

/// A button with a text label.
pub struct Button<T> {
    /// A closure that will be invoked when the button is clicked.
    on_clicked: Box<dyn Fn(&mut EventCtx, &mut T, &Env)>,
    template: Box<dyn Fn(&mut EventCtx, &mut T) -> Widget<ButtonState>>,
    inner: BoxedWidget<ButtonState>,
}

#[derive(Clone, Data)]
struct ButtonState {
    clicked: bool,
}

impl<T: Data + 'static> Button<T> {
    pub fn new(tempate: impl Fn(&ButtonState, &Env) ) -> Self {

    }

    /// Create a new textbutton
    pub fn textButton(text: impl Into<LabelText>) -> Button<T> {
        Button {
            template: |state, env| {
                // TODO: Determine background brush based on state.clicked
                Container::new(Label::new(text))
                    .border(brush: impl Into<PaintBrush>, width: f64)
                    .background(brush: impl Into<PaintBrush>)
            },
            action: None
        }
    }

    pub fn on_clicked(self, handler: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Button<T> {
        self.on_clicked = Some(handler);
        self
    }
}

impl<T: Data> Widget<T> for Button<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, &mut self.state, env)
        // TODO: determine if state has changed, act accordingly
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: Option<&T>, data: &T, env: &Env) {
        self.inner.update(ctx, None, &self.state, env)
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        bc.debug_check("Button");

        self.inner.layout(layout_ctx, bc, &self.state, env)
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        // Dynamically recreate inner, then repaint it.
        self.inner = WidgetPod::new(self.template(&self.state, env));
        self.inner.paint(paint_ctx, &self.state, env)
    }
}
