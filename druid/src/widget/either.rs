// Copyright 2019 The xi-editor Authors.
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

//! A widget that switches dynamically between two child views.

use crate::kurbo::{Point, Rect, Size};
use crate::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, PaintCtx, UpdateCtx, Widget, WidgetPod,
};

/// A widget that switches between two possible child views.
pub struct Either<T: Data, S> {
    closure: Box<dyn Fn(&T, &Env) -> bool>,
    true_branch: WidgetPod<T, S, Box<dyn Widget<T, S>>>,
    false_branch: WidgetPod<T, S, Box<dyn Widget<T, S>>>,
    current: bool,
}

impl<T: Data, S> Either<T, S> {
    /// Create a new widget that switches between two views.
    ///
    /// The given closure is evaluated on data change. If its value is `true`, then
    /// the `true_branch` widget is shown, otherwise `false_branch`.
    pub fn new(
        closure: impl Fn(&T, &Env) -> bool + 'static,
        true_branch: impl Widget<T, S> + 'static,
        false_branch: impl Widget<T, S> + 'static,
    ) -> Either<T, S> {
        Either {
            closure: Box::new(closure),
            true_branch: WidgetPod::new(true_branch).boxed(),
            false_branch: WidgetPod::new(false_branch).boxed(),
            current: false,
        }
    }
}

impl<T: Data, S> Widget<T, S> for Either<T, S> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, style_parent: &mut S, env: &Env) {
        if self.current {
            self.true_branch.event(ctx, event, data, style_parent, env)
        } else {
            self.false_branch.event(ctx, event, data, style_parent, env)
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: Option<&T>, data: &T, env: &Env) {
        let current = (self.closure)(data, env);
        if current != self.current {
            self.current = current;
            ctx.invalidate();
            // TODO: more event flow to request here.
        }
        if self.current {
            self.true_branch.update(ctx, data, env);
        } else {
            self.false_branch.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        if self.current {
            let size = self.true_branch.layout(layout_ctx, bc, data, env);
            self.true_branch
                .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
            size
        } else {
            let size = self.false_branch.layout(layout_ctx, bc, data, env);
            self.false_branch
                .set_layout_rect(Rect::from_origin_size(Point::ORIGIN, size));
            size
        }
    }

    fn paint(&mut self, paint_ctx: &mut PaintCtx, data: &T, env: &Env) {
        if self.current {
            self.true_branch.paint(paint_ctx, data, env);
        } else {
            self.false_branch.paint(paint_ctx, data, env);
        }
    }
}
