use std::collections::HashMap;

const MEMMAP: &[&str] = &[
    "TEXT_ENABLE",
    "TEXT_CHAR_OFFSET",
    "TEXT_BUFFER_ADDR",
    "TEXT_BUFFER_LEN",
    "COLOR_BUFFER_ADDR",
    "COLOR_BUFFER_LEN",
    "ROM_BANK",
    "VIDEO_ENABLE",
    "VIDEO_COLORMAP_ADDR",
    "VIDEO_SPRITE_BUFFER_ADDR",
    "VIDEO_SPRITE_COUNT",
    "AUDIO_ENABLE",
    "AUDIO_BUFF_ADDR",
    "AUDIO_BUFF_END",
    "AUDIO_BUFF_POS",
    "AUDIO_AMP",
    "AUDIO_STRIDE",
    "AUDIO_WRAP",
    "NET_RECV_STATUS",
    "NET_RECV_BUFFER_ADDR",
    "NET_RECV_BUFFER_LEN",
    "NET_SEND_STATUS",
    "NET_SEND_BUFFER_ADDR",
    "NET_SEND_BUFFER_LEN",
    "INPUT_MOUSE_X",
    "INPUT_MOUSE_Y",
    "INPUT_MOUSE_BUTTON",
    "INPUT_TEXTCHAR",
    "INPUT_REALTIME",
    "ARR_INPUT_GAMEPADS",
];

pub fn add_memmap_constants(constants: &mut HashMap<String, f64>) {
    for (addr, name) in MEMMAP.iter().enumerate() {
        constants.insert(format!("${}", name.to_string()), addr as f64);
    }
}
