// Copyright © 2025 David Haig
// SPDX-License-Identifier: MIT

extern crate alloc;

use alloc::rc::{Rc, Weak};
use core::cell::RefCell;

use i_slint_core::{
    api::PlatformError,
    item_rendering,
    items::FontMetrics,
    lengths::{LogicalLength, LogicalRect, LogicalSize},
    renderer::RendererSealed,
    window::{WindowAdapter, WindowInner},
};

pub struct Stm32Renderer {
    maybe_window_adapter: RefCell<Option<Weak<dyn WindowAdapter>>>,
}

impl Stm32Renderer {
    pub fn render(&self) -> Result<(), PlatformError> {
        let Some(window) = self.maybe_window_adapter.borrow().as_ref().and_then(|w| w.upgrade())
        else {
            return Ok(());
        };
        let window_inner = WindowInner::from_pub(window.window());

        let mut renderer = self.partial_rendering_state.create_partial_renderer(buffer_renderer);
        let window_adapter = renderer.window_adapter.clone();

        window_inner.draw_contents(|components| {
            for (component, origin) in components {
                item_rendering::render_component_items(
                    component,
                    &mut renderer,
                    *origin,
                    &window_adapter,
                );
            }
        });

        Ok(())
    }
}

#[doc(hidden)]
impl RendererSealed for Stm32Renderer {
    fn text_size(
        &self,
        _font_request: i_slint_core::graphics::FontRequest,
        _text: &str,
        _max_width: Option<i_slint_core::lengths::LogicalLength>,
        _scale_factor: i_slint_core::lengths::ScaleFactor,
        _text_wrap: i_slint_core::items::TextWrap,
    ) -> LogicalSize {
        LogicalSize { ..Default::default() }
    }

    fn font_metrics(
        &self,
        _font_request: i_slint_core::graphics::FontRequest,
        _scale_factor: i_slint_core::lengths::ScaleFactor,
    ) -> FontMetrics {
        FontMetrics::default()
    }

    fn text_input_byte_offset_for_position(
        &self,
        _text_input: std::pin::Pin<&i_slint_core::items::TextInput>,
        _pos: i_slint_core::lengths::LogicalPoint,
        _font_request: i_slint_core::graphics::FontRequest,
        _scale_factor: i_slint_core::lengths::ScaleFactor,
    ) -> usize {
        0
    }

    fn text_input_cursor_rect_for_byte_offset(
        &self,
        _text_input: std::pin::Pin<&i_slint_core::items::TextInput>,
        _byte_offset: usize,
        _font_request: i_slint_core::graphics::FontRequest,
        _scale_factor: i_slint_core::lengths::ScaleFactor,
    ) -> LogicalRect {
        LogicalRect::default()
    }

    fn default_font_size(&self) -> LogicalLength {
        LogicalLength::default()
    }

    fn set_window_adapter(
        &self,
        _window_adapter: &std::rc::Rc<dyn i_slint_core::window::WindowAdapter>,
    ) {
        // do nothing
    }
}
