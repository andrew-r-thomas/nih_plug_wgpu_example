use crate::render::WgpuRenderer;
use nih_plug::editor::Editor;

pub struct WgpuEditor {}

impl Editor for WgpuEditor {
    fn spawn(
        &self,
        parent: nih_plug::prelude::ParentWindowHandle,
        context: std::sync::Arc<dyn nih_plug::prelude::GuiContext>,
    ) -> Box<dyn std::any::Any + Send> {
        let renderer = WgpuRenderer::start(parent);
        Box::new(renderer)
    }

    fn size(&self) -> (u32, u32) {
        todo!()
    }

    fn set_scale_factor(&self, factor: f32) -> bool {
        todo!()
    }

    fn param_value_changed(&self, id: &str, normalized_value: f32) {
        todo!()
    }

    fn param_modulation_changed(&self, id: &str, modulation_offset: f32) {
        todo!()
    }

    fn param_values_changed(&self) {
        todo!()
    }
}
