use crate::render::WgpuRenderer;
use nih_plug::{editor::Editor, params::internals::ParamPtr};

pub struct WgpuEditor {
    // pub param: ParamPtr,
}

impl WgpuEditor {}

impl Editor for WgpuEditor {
    fn spawn(
        &self,
        parent: nih_plug::prelude::ParentWindowHandle,
        _context: std::sync::Arc<dyn nih_plug::prelude::GuiContext>,
    ) -> Box<dyn std::any::Any + Send> {
        let renderer = WgpuRenderer::start(parent);
        Box::new(renderer)
    }

    fn size(&self) -> (u32, u32) {
        (512, 512)
    }

    fn set_scale_factor(&self, _factor: f32) -> bool {
        true
    }

    fn param_value_changed(&self, _id: &str, _normalized_value: f32) {}

    fn param_modulation_changed(&self, _id: &str, _modulation_offset: f32) {}

    fn param_values_changed(&self) {}
}
