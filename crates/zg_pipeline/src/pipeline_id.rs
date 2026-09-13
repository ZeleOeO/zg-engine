#[repr(usize)]
#[derive(Debug, Copy, Clone)]
pub enum PipelineID {
    MAIN = 0,
    LIGHT = 1,
}
