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
}
