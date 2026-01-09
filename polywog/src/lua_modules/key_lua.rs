use crate::input::Key;
use crate::lua::LuaModule;
use mlua::prelude::{LuaError, LuaResult};
use mlua::{FromLua, Integer, IntoLua, Lua, Value};

pub struct KeyModule;

impl LuaModule for KeyModule {
    const PATH: &'static str = "KeyCode";

    fn load(lua: &Lua) -> LuaResult<Value> {
        let m = lua.create_table()?;
        m.set("BACKQUOTE", Key::Backquote)?;
        m.set("BACKSLASH", Key::Backslash)?;
        m.set("BRACKET_LEFT", Key::BracketLeft)?;
        m.set("BRACKET_RIGHT", Key::BracketRight)?;
        m.set("COMMA", Key::Comma)?;
        m.set("DIGIT_0", Key::Digit0)?;
        m.set("DIGIT_1", Key::Digit1)?;
        m.set("DIGIT_2", Key::Digit2)?;
        m.set("DIGIT_3", Key::Digit3)?;
        m.set("DIGIT_4", Key::Digit4)?;
        m.set("DIGIT_5", Key::Digit5)?;
        m.set("DIGIT_6", Key::Digit6)?;
        m.set("DIGIT_7", Key::Digit7)?;
        m.set("DIGIT_8", Key::Digit8)?;
        m.set("DIGIT_9", Key::Digit9)?;
        m.set("EQUAL", Key::Equal)?;
        m.set("INTL_BACKSLASH", Key::IntlBackslash)?;
        m.set("INTL_RO", Key::IntlRo)?;
        m.set("INTL_YEN", Key::IntlYen)?;
        m.set("A", Key::A)?;
        m.set("B", Key::B)?;
        m.set("C", Key::C)?;
        m.set("D", Key::D)?;
        m.set("E", Key::E)?;
        m.set("F", Key::F)?;
        m.set("G", Key::G)?;
        m.set("H", Key::H)?;
        m.set("I", Key::I)?;
        m.set("J", Key::J)?;
        m.set("K", Key::K)?;
        m.set("L", Key::L)?;
        m.set("M", Key::M)?;
        m.set("N", Key::N)?;
        m.set("O", Key::O)?;
        m.set("P", Key::P)?;
        m.set("Q", Key::Q)?;
        m.set("R", Key::R)?;
        m.set("S", Key::S)?;
        m.set("T", Key::T)?;
        m.set("U", Key::U)?;
        m.set("V", Key::V)?;
        m.set("W", Key::W)?;
        m.set("X", Key::X)?;
        m.set("Y", Key::Y)?;
        m.set("Z", Key::Z)?;
        m.set("MINUS", Key::Minus)?;
        m.set("PERIOD", Key::Period)?;
        m.set("QUOTE", Key::Quote)?;
        m.set("SEMICOLON", Key::Semicolon)?;
        m.set("SLASH", Key::Slash)?;
        m.set("ALT_LEFT", Key::AltLeft)?;
        m.set("ALT_RIGHT", Key::AltRight)?;
        m.set("BACKSPACE", Key::Backspace)?;
        m.set("CAPS_LOCK", Key::CapsLock)?;
        m.set("CONTEXT_MENU", Key::ContextMenu)?;
        m.set("CONTROL_LEFT", Key::ControlLeft)?;
        m.set("CONTROL_RIGHT", Key::ControlRight)?;
        m.set("ENTER", Key::Enter)?;
        m.set("SUPER_LEFT", Key::SuperLeft)?;
        m.set("SUPER_RIGHT", Key::SuperRight)?;
        m.set("SHIFT_LEFT", Key::ShiftLeft)?;
        m.set("SHIFT_RIGHT", Key::ShiftRight)?;
        m.set("SPACE", Key::Space)?;
        m.set("TAB", Key::Tab)?;
        m.set("CONVERT", Key::Convert)?;
        m.set("KANA_MODE", Key::KanaMode)?;
        m.set("LANG1", Key::Lang1)?;
        m.set("LANG2", Key::Lang2)?;
        m.set("LANG3", Key::Lang3)?;
        m.set("LANG4", Key::Lang4)?;
        m.set("LANG5", Key::Lang5)?;
        m.set("NON_CONVERT", Key::NonConvert)?;
        m.set("DELETE", Key::Delete)?;
        m.set("END", Key::End)?;
        m.set("HELP", Key::Help)?;
        m.set("HOME", Key::Home)?;
        m.set("INSERT", Key::Insert)?;
        m.set("PAGE_DOWN", Key::PageDown)?;
        m.set("PAGE_UP", Key::PageUp)?;
        m.set("ARROW_DOWN", Key::ArrowDown)?;
        m.set("ARROW_LEFT", Key::ArrowLeft)?;
        m.set("ARROW_RIGHT", Key::ArrowRight)?;
        m.set("ARROW_UP", Key::ArrowUp)?;
        m.set("NUM_LOCK", Key::NumLock)?;
        m.set("NUMPAD_0", Key::Numpad0)?;
        m.set("NUMPAD_1", Key::Numpad1)?;
        m.set("NUMPAD_2", Key::Numpad2)?;
        m.set("NUMPAD_3", Key::Numpad3)?;
        m.set("NUMPAD_4", Key::Numpad4)?;
        m.set("NUMPAD_5", Key::Numpad5)?;
        m.set("NUMPAD_6", Key::Numpad6)?;
        m.set("NUMPAD_7", Key::Numpad7)?;
        m.set("NUMPAD_8", Key::Numpad8)?;
        m.set("NUMPAD_9", Key::Numpad9)?;
        m.set("NUMPAD_ADD", Key::NumpadAdd)?;
        m.set("NUMPAD_BACKSPACE", Key::NumpadBackspace)?;
        m.set("NUMPAD_CLEAR", Key::NumpadClear)?;
        m.set("NUMPAD_CLEAR_ENTRY", Key::NumpadClearEntry)?;
        m.set("NUMPAD_COMMA", Key::NumpadComma)?;
        m.set("NUMPAD_DECIMAL", Key::NumpadDecimal)?;
        m.set("NUMPAD_DIVIDE", Key::NumpadDivide)?;
        m.set("NUMPAD_ENTER", Key::NumpadEnter)?;
        m.set("NUMPAD_EQUAL", Key::NumpadEqual)?;
        m.set("NUMPAD_HASH", Key::NumpadHash)?;
        m.set("NUMPAD_MEMORY_ADD", Key::NumpadMemoryAdd)?;
        m.set("NUMPAD_MEMORY_CLEAR", Key::NumpadMemoryClear)?;
        m.set("NUMPAD_MEMORY_RECALL", Key::NumpadMemoryRecall)?;
        m.set("NUMPAD_MEMORY_STORE", Key::NumpadMemoryStore)?;
        m.set("NUMPAD_MEMORY_SUBTRACT", Key::NumpadMemorySubtract)?;
        m.set("NUMPAD_MULTIPLY", Key::NumpadMultiply)?;
        m.set("NUMPAD_PAREN_LEFT", Key::NumpadParenLeft)?;
        m.set("NUMPAD_PAREN_RIGHT", Key::NumpadParenRight)?;
        m.set("NUMPAD_STAR", Key::NumpadStar)?;
        m.set("NUMPAD_SUBTRACT", Key::NumpadSubtract)?;
        m.set("ESCAPE", Key::Escape)?;
        m.set("FN", Key::Fn)?;
        m.set("FN_LOCK", Key::FnLock)?;
        m.set("PRINT_SCREEN", Key::PrintScreen)?;
        m.set("SCROLL_LOCK", Key::ScrollLock)?;
        m.set("PAUSE", Key::Pause)?;
        m.set("BROWSER_BACK", Key::BrowserBack)?;
        m.set("BROWSER_FAVORITES", Key::BrowserFavorites)?;
        m.set("BROWSER_FORWARD", Key::BrowserForward)?;
        m.set("BROWSER_HOME", Key::BrowserHome)?;
        m.set("BROWSER_REFRESH", Key::BrowserRefresh)?;
        m.set("BROWSER_SEARCH", Key::BrowserSearch)?;
        m.set("BROWSER_STOP", Key::BrowserStop)?;
        m.set("EJECT", Key::Eject)?;
        m.set("LAUNCH_APP1", Key::LaunchApp1)?;
        m.set("LAUNCH_APP2", Key::LaunchApp2)?;
        m.set("LAUNCH_MAIL", Key::LaunchMail)?;
        m.set("MEDIA_PLAY_PAUSE", Key::MediaPlayPause)?;
        m.set("MEDIA_SELECT", Key::MediaSelect)?;
        m.set("MEDIA_STOP", Key::MediaStop)?;
        m.set("MEDIA_TRACK_NEXT", Key::MediaTrackNext)?;
        m.set("MEDIA_TRACK_PREVIOUS", Key::MediaTrackPrevious)?;
        m.set("POWER", Key::Power)?;
        m.set("SLEEP", Key::Sleep)?;
        m.set("AUDIO_VOLUME_DOWN", Key::AudioVolumeDown)?;
        m.set("AUDIO_VOLUME_MUTE", Key::AudioVolumeMute)?;
        m.set("AUDIO_VOLUME_UP", Key::AudioVolumeUp)?;
        m.set("WAKE_UP", Key::WakeUp)?;
        m.set("META", Key::Meta)?;
        m.set("HYPER", Key::Hyper)?;
        m.set("TURBO", Key::Turbo)?;
        m.set("ABORT", Key::Abort)?;
        m.set("RESUME", Key::Resume)?;
        m.set("SUSPEND", Key::Suspend)?;
        m.set("AGAIN", Key::Again)?;
        m.set("COPY", Key::Copy)?;
        m.set("CUT", Key::Cut)?;
        m.set("FIND", Key::Find)?;
        m.set("OPEN", Key::Open)?;
        m.set("PASTE", Key::Paste)?;
        m.set("PROPS", Key::Props)?;
        m.set("SELECT", Key::Select)?;
        m.set("UNDO", Key::Undo)?;
        m.set("HIRAGANA", Key::Hiragana)?;
        m.set("KATAKANA", Key::Katakana)?;
        m.set("F1", Key::F1)?;
        m.set("F2", Key::F2)?;
        m.set("F3", Key::F3)?;
        m.set("F4", Key::F4)?;
        m.set("F5", Key::F5)?;
        m.set("F6", Key::F6)?;
        m.set("F7", Key::F7)?;
        m.set("F8", Key::F8)?;
        m.set("F9", Key::F9)?;
        m.set("F10", Key::F10)?;
        m.set("F11", Key::F11)?;
        m.set("F12", Key::F12)?;
        m.set("F13", Key::F13)?;
        m.set("F14", Key::F14)?;
        m.set("F15", Key::F15)?;
        m.set("F16", Key::F16)?;
        m.set("F17", Key::F17)?;
        m.set("F18", Key::F18)?;
        m.set("F19", Key::F19)?;
        m.set("F20", Key::F20)?;
        m.set("F21", Key::F21)?;
        m.set("F22", Key::F22)?;
        m.set("F23", Key::F23)?;
        m.set("F24", Key::F24)?;
        m.set("F25", Key::F25)?;
        m.set("F26", Key::F26)?;
        m.set("F27", Key::F27)?;
        m.set("F28", Key::F28)?;
        m.set("F29", Key::F29)?;
        m.set("F30", Key::F30)?;
        m.set("F31", Key::F31)?;
        m.set("F32", Key::F32)?;
        m.set("F33", Key::F33)?;
        m.set("F34", Key::F34)?;
        m.set("F35", Key::F35)?;
        Ok(Value::Table(m))
    }
}

impl FromLua for Key {
    #[inline]
    fn from_lua(value: Value, _lua: &Lua) -> LuaResult<Self> {
        match value {
            Value::Integer(key) => Key::from_repr(key as usize)
                .ok_or_else(|| LuaError::runtime(format!("invalid key code [{key}]"))),
            value => Err(LuaError::runtime(format!("invalid key code [{value:?}]"))),
        }
    }
}

impl IntoLua for Key {
    #[inline]
    fn into_lua(self, _lua: &Lua) -> LuaResult<Value> {
        Ok(Value::Integer(self as Integer))
    }
}
