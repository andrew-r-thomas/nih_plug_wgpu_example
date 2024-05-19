use std::sync::Arc;

use crate::{render::WgpuRenderer, NihPlugWgpuExampleParams};
use nih_plug::{editor::Editor, params::internals::ParamPtr};

pub struct WgpuEditor {
    pub params: Arc<NihPlugWgpuExampleParams>,
}

impl WgpuEditor {}

impl Editor for WgpuEditor {
    fn spawn(
        &self,
        parent: nih_plug::prelude::ParentWindowHandle,
        context: std::sync::Arc<dyn nih_plug::prelude::GuiContext>,
    ) -> Box<dyn std::any::Any + Send> {
        let renderer = WgpuRenderer::start(parent, context.clone(), self.params.clone());
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
