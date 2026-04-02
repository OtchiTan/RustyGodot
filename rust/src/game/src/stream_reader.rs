use common::stream_reader::StreamReader;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct GDStreamReader {
    base: Base<RefCounted>,
    pub stream_reader: Option<StreamReader>,
}

#[godot_api]
impl IRefCounted for GDStreamReader {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            base,
            stream_reader: None,
        }
    }
}
