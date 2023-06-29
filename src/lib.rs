#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
pub mod bindings;

use std::ffi::CString;

use bindings::{
    SDL_Event, SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_KEYUP,
    SDL_EventType_SDL_MOUSEBUTTONDOWN, SDL_EventType_SDL_MOUSEBUTTONUP,
    SDL_EventType_SDL_MOUSEMOTION, SDL_EventType_SDL_MOUSEWHEEL, SDL_EventType_SDL_TEXTINPUT,
    SDL_GL_GetDrawableSize, SDL_GetMouseState, SDL_GetPerformanceCounter,
    SDL_GetPerformanceFrequency, SDL_GetWindowFlags, SDL_GetWindowSize, SDL_KeyCode_SDLK_0,
    SDL_KeyCode_SDLK_1, SDL_KeyCode_SDLK_2, SDL_KeyCode_SDLK_3, SDL_KeyCode_SDLK_4,
    SDL_KeyCode_SDLK_5, SDL_KeyCode_SDLK_6, SDL_KeyCode_SDLK_7, SDL_KeyCode_SDLK_8,
    SDL_KeyCode_SDLK_9, SDL_KeyCode_SDLK_APPLICATION, SDL_KeyCode_SDLK_BACKQUOTE,
    SDL_KeyCode_SDLK_BACKSLASH, SDL_KeyCode_SDLK_BACKSPACE, SDL_KeyCode_SDLK_CAPSLOCK,
    SDL_KeyCode_SDLK_COMMA, SDL_KeyCode_SDLK_DELETE, SDL_KeyCode_SDLK_DOWN, SDL_KeyCode_SDLK_END,
    SDL_KeyCode_SDLK_EQUALS, SDL_KeyCode_SDLK_ESCAPE, SDL_KeyCode_SDLK_F1, SDL_KeyCode_SDLK_F10,
    SDL_KeyCode_SDLK_F11, SDL_KeyCode_SDLK_F12, SDL_KeyCode_SDLK_F2, SDL_KeyCode_SDLK_F3,
    SDL_KeyCode_SDLK_F4, SDL_KeyCode_SDLK_F5, SDL_KeyCode_SDLK_F6, SDL_KeyCode_SDLK_F7,
    SDL_KeyCode_SDLK_F8, SDL_KeyCode_SDLK_F9, SDL_KeyCode_SDLK_HOME, SDL_KeyCode_SDLK_INSERT,
    SDL_KeyCode_SDLK_KP_0, SDL_KeyCode_SDLK_KP_1, SDL_KeyCode_SDLK_KP_2, SDL_KeyCode_SDLK_KP_3,
    SDL_KeyCode_SDLK_KP_4, SDL_KeyCode_SDLK_KP_5, SDL_KeyCode_SDLK_KP_6, SDL_KeyCode_SDLK_KP_7,
    SDL_KeyCode_SDLK_KP_8, SDL_KeyCode_SDLK_KP_9, SDL_KeyCode_SDLK_KP_DIVIDE,
    SDL_KeyCode_SDLK_KP_ENTER, SDL_KeyCode_SDLK_KP_EQUALS, SDL_KeyCode_SDLK_KP_MINUS,
    SDL_KeyCode_SDLK_KP_MULTIPLY, SDL_KeyCode_SDLK_KP_PERIOD, SDL_KeyCode_SDLK_KP_PLUS,
    SDL_KeyCode_SDLK_LALT, SDL_KeyCode_SDLK_LCTRL, SDL_KeyCode_SDLK_LEFT,
    SDL_KeyCode_SDLK_LEFTBRACKET, SDL_KeyCode_SDLK_LGUI, SDL_KeyCode_SDLK_LSHIFT,
    SDL_KeyCode_SDLK_MINUS, SDL_KeyCode_SDLK_NUMLOCKCLEAR, SDL_KeyCode_SDLK_PAGEDOWN,
    SDL_KeyCode_SDLK_PAGEUP, SDL_KeyCode_SDLK_PAUSE, SDL_KeyCode_SDLK_PERIOD,
    SDL_KeyCode_SDLK_PRINTSCREEN, SDL_KeyCode_SDLK_QUOTE, SDL_KeyCode_SDLK_RALT,
    SDL_KeyCode_SDLK_RCTRL, SDL_KeyCode_SDLK_RETURN, SDL_KeyCode_SDLK_RGUI, SDL_KeyCode_SDLK_RIGHT,
    SDL_KeyCode_SDLK_RIGHTBRACKET, SDL_KeyCode_SDLK_RSHIFT, SDL_KeyCode_SDLK_SCROLLLOCK,
    SDL_KeyCode_SDLK_SEMICOLON, SDL_KeyCode_SDLK_SLASH, SDL_KeyCode_SDLK_SPACE,
    SDL_KeyCode_SDLK_TAB, SDL_KeyCode_SDLK_UP, SDL_KeyCode_SDLK_a, SDL_KeyCode_SDLK_b,
    SDL_KeyCode_SDLK_c, SDL_KeyCode_SDLK_d, SDL_KeyCode_SDLK_e, SDL_KeyCode_SDLK_f,
    SDL_KeyCode_SDLK_g, SDL_KeyCode_SDLK_h, SDL_KeyCode_SDLK_i, SDL_KeyCode_SDLK_j,
    SDL_KeyCode_SDLK_k, SDL_KeyCode_SDLK_l, SDL_KeyCode_SDLK_m, SDL_KeyCode_SDLK_n,
    SDL_KeyCode_SDLK_o, SDL_KeyCode_SDLK_p, SDL_KeyCode_SDLK_q, SDL_KeyCode_SDLK_r,
    SDL_KeyCode_SDLK_s, SDL_KeyCode_SDLK_t, SDL_KeyCode_SDLK_u, SDL_KeyCode_SDLK_v,
    SDL_KeyCode_SDLK_w, SDL_KeyCode_SDLK_x, SDL_KeyCode_SDLK_y, SDL_KeyCode_SDLK_z, SDL_Keymod,
    SDL_Keymod_KMOD_ALT, SDL_Keymod_KMOD_CTRL, SDL_Keymod_KMOD_GUI, SDL_Keymod_KMOD_SHIFT,
    SDL_Scancode_SDL_SCANCODE_A, SDL_Scancode_SDL_SCANCODE_BACKSPACE, SDL_Scancode_SDL_SCANCODE_C,
    SDL_Scancode_SDL_SCANCODE_DELETE, SDL_Scancode_SDL_SCANCODE_DOWN,
    SDL_Scancode_SDL_SCANCODE_END, SDL_Scancode_SDL_SCANCODE_ESCAPE,
    SDL_Scancode_SDL_SCANCODE_HOME, SDL_Scancode_SDL_SCANCODE_KP_ENTER,
    SDL_Scancode_SDL_SCANCODE_LEFT, SDL_Scancode_SDL_SCANCODE_PAGEDOWN,
    SDL_Scancode_SDL_SCANCODE_PAGEUP, SDL_Scancode_SDL_SCANCODE_RIGHT,
    SDL_Scancode_SDL_SCANCODE_SPACE, SDL_Scancode_SDL_SCANCODE_TAB, SDL_Scancode_SDL_SCANCODE_UP,
    SDL_Scancode_SDL_SCANCODE_V, SDL_Scancode_SDL_SCANCODE_X, SDL_Scancode_SDL_SCANCODE_Y,
    SDL_Scancode_SDL_SCANCODE_Z, SDL_ShowCursor, SDL_WarpMouseInWindow, SDL_Window,
    SDL_WindowFlags_SDL_WINDOW_INPUT_FOCUS, SDL_WindowFlags_SDL_WINDOW_MINIMIZED,
    SDL_bool_SDL_FALSE, SDL_bool_SDL_TRUE, SDL_BUTTON_LEFT, SDL_BUTTON_MIDDLE, SDL_BUTTON_RIGHT,
    SDL_BUTTON_X1, SDL_BUTTON_X2, SDL_Cursor, SDL_CreateSystemCursor, SDL_SystemCursor_SDL_SYSTEM_CURSOR_ARROW, SDL_SystemCursor_SDL_SYSTEM_CURSOR_IBEAM, SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEALL, SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENS, SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEWE, SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENESW, SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENWSE, SDL_SystemCursor_SDL_SYSTEM_CURSOR_HAND, SDL_SystemCursor_SDL_SYSTEM_CURSOR_NO, SDL_SetClipboardText, SDL_Rect, SDL_SetTextInputRect,
};
use imgui_sys::{
    ImGuiKey_F10, ImGuiKey_None, ImGuiMod_Alt, ImGuiMod_Ctrl, ImGuiMod_Shift, ImGuiMod_Super,
    ImGuiMouseCursor_None, ImGuiMouseCursor_Arrow, ImGuiMouseCursor_TextInput, ImGuiMouseCursor_ResizeAll, ImGuiMouseCursor_ResizeNS, ImGuiMouseCursor_ResizeNESW, ImGuiMouseCursor_ResizeNWSE, ImGuiMouseCursor_ResizeEW, ImGuiMouseCursor_Hand, ImGuiMouseCursor_NotAllowed, cty, ImGuiPlatformImeData,
};

use imgui_sys::{
    ImGuiKey_0, ImGuiKey_1, ImGuiKey_2, ImGuiKey_3, ImGuiKey_4, ImGuiKey_5, ImGuiKey_6, ImGuiKey_7,
    ImGuiKey_8, ImGuiKey_9, ImGuiKey_A, ImGuiKey_Apostrophe, ImGuiKey_B, ImGuiKey_Backslash,
    ImGuiKey_Backspace, ImGuiKey_C, ImGuiKey_CapsLock, ImGuiKey_Comma, ImGuiKey_D, ImGuiKey_Delete,
    ImGuiKey_DownArrow, ImGuiKey_E, ImGuiKey_End, ImGuiKey_Enter, ImGuiKey_Equal, ImGuiKey_Escape,
    ImGuiKey_F, ImGuiKey_F1, ImGuiKey_F11, ImGuiKey_F12, ImGuiKey_F2, ImGuiKey_F3, ImGuiKey_F4,
    ImGuiKey_F5, ImGuiKey_F6, ImGuiKey_F7, ImGuiKey_F8, ImGuiKey_F9, ImGuiKey_G,
    ImGuiKey_GraveAccent, ImGuiKey_H, ImGuiKey_Home, ImGuiKey_I, ImGuiKey_Insert, ImGuiKey_J,
    ImGuiKey_K, ImGuiKey_Keypad0, ImGuiKey_Keypad1, ImGuiKey_Keypad2, ImGuiKey_Keypad3,
    ImGuiKey_Keypad4, ImGuiKey_Keypad5, ImGuiKey_Keypad6, ImGuiKey_Keypad7, ImGuiKey_Keypad8,
    ImGuiKey_Keypad9, ImGuiKey_KeypadAdd, ImGuiKey_KeypadDecimal, ImGuiKey_KeypadDivide,
    ImGuiKey_KeypadEnter, ImGuiKey_KeypadEqual, ImGuiKey_KeypadMultiply, ImGuiKey_KeypadSubtract,
    ImGuiKey_L, ImGuiKey_LeftAlt, ImGuiKey_LeftArrow, ImGuiKey_LeftBracket, ImGuiKey_LeftCtrl,
    ImGuiKey_LeftShift, ImGuiKey_LeftSuper, ImGuiKey_M, ImGuiKey_Menu, ImGuiKey_Minus, ImGuiKey_N,
    ImGuiKey_NumLock, ImGuiKey_O, ImGuiKey_P, ImGuiKey_PageDown, ImGuiKey_PageUp, ImGuiKey_Pause,
    ImGuiKey_Period, ImGuiKey_PrintScreen, ImGuiKey_Q, ImGuiKey_R, ImGuiKey_RightAlt,
    ImGuiKey_RightArrow, ImGuiKey_RightBracket, ImGuiKey_RightCtrl, ImGuiKey_RightShift,
    ImGuiKey_RightSuper, ImGuiKey_S, ImGuiKey_ScrollLock, ImGuiKey_Semicolon, ImGuiKey_Slash,
    ImGuiKey_Space, ImGuiKey_T, ImGuiKey_Tab, ImGuiKey_U, ImGuiKey_UpArrow, ImGuiKey_V, ImGuiKey_W,
    ImGuiKey_X, ImGuiKey_Y, ImGuiKey_Z,
};

use imgui::{internal::RawCast, Context, Io, Key};

use crate::bindings::{SDL_free, SDL_GetClipboardText};

pub struct ImGuiSDL2 {
    mouse_press: [bool; 5],
    frequency: u64,
    time: u64,
    last_cursor: Option<*const SDL_Cursor>,
    clipboard_data: Option<*mut cty::c_char>,
    mouse_cursors: Vec<*mut SDL_Cursor>,
}

impl ImGuiSDL2 {
    pub fn new(imgui: &mut Context) -> Self {
        let io = imgui.io_mut();
        io.key_map[Key::Tab as usize] = SDL_Scancode_SDL_SCANCODE_TAB;
        io.key_map[Key::LeftArrow as usize] = SDL_Scancode_SDL_SCANCODE_LEFT;
        io.key_map[Key::RightArrow as usize] = SDL_Scancode_SDL_SCANCODE_RIGHT;
        io.key_map[Key::UpArrow as usize] = SDL_Scancode_SDL_SCANCODE_UP;
        io.key_map[Key::DownArrow as usize] = SDL_Scancode_SDL_SCANCODE_DOWN;
        io.key_map[Key::PageUp as usize] = SDL_Scancode_SDL_SCANCODE_PAGEUP;
        io.key_map[Key::PageDown as usize] = SDL_Scancode_SDL_SCANCODE_PAGEDOWN;
        io.key_map[Key::Home as usize] = SDL_Scancode_SDL_SCANCODE_HOME;
        io.key_map[Key::End as usize] = SDL_Scancode_SDL_SCANCODE_END;
        io.key_map[Key::Delete as usize] = SDL_Scancode_SDL_SCANCODE_DELETE;
        io.key_map[Key::Backspace as usize] = SDL_Scancode_SDL_SCANCODE_BACKSPACE;
        io.key_map[Key::Enter as usize] = SDL_Scancode_SDL_SCANCODE_KP_ENTER;
        io.key_map[Key::Escape as usize] = SDL_Scancode_SDL_SCANCODE_ESCAPE;
        io.key_map[Key::Space as usize] = SDL_Scancode_SDL_SCANCODE_SPACE;
        io.key_map[Key::A as usize] = SDL_Scancode_SDL_SCANCODE_A;
        io.key_map[Key::C as usize] = SDL_Scancode_SDL_SCANCODE_C;
        io.key_map[Key::V as usize] = SDL_Scancode_SDL_SCANCODE_V;
        io.key_map[Key::X as usize] = SDL_Scancode_SDL_SCANCODE_X;
        io.key_map[Key::Y as usize] = SDL_Scancode_SDL_SCANCODE_Y;
        io.key_map[Key::Z as usize] = SDL_Scancode_SDL_SCANCODE_Z;

        io.backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;
        

        
        let mut mouse_cursors: Vec<*mut SDL_Cursor> = Vec::with_capacity(9);

        let mut result = Self {
            mouse_press: [false; 5],
            frequency: unsafe { SDL_GetPerformanceFrequency() },
            time: unsafe { SDL_GetPerformanceCounter() },
            last_cursor: None,
            clipboard_data: None,
            mouse_cursors,
        };

        unsafe {
            //io.raw_mut().BackendPlatformName = CString::new("imgui-impl-sdl2-sys").unwrap().as_ptr();
            io.raw_mut().BackendPlatformUserData = &result as *const _ as *mut cty::c_void;

            io.raw_mut().SetClipboardTextFn = Some(Self::set_clipboard_text);
            io.raw_mut().GetClipboardTextFn = Some(Self::get_clipboard_text);
            io.raw_mut().ClipboardUserData = 0 as *mut cty::c_void;

            io.raw_mut().SetPlatformImeDataFn = Some(Self::set_platform_ime_data);

            result.mouse_cursors[ImGuiMouseCursor_Arrow as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_ARROW);
            result.mouse_cursors[ImGuiMouseCursor_TextInput as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_IBEAM);
            result.mouse_cursors[ImGuiMouseCursor_ResizeAll as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEALL);
            result.mouse_cursors[ImGuiMouseCursor_ResizeNS as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENS);
            result.mouse_cursors[ImGuiMouseCursor_ResizeEW as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEWE);
            result.mouse_cursors[ImGuiMouseCursor_ResizeNESW as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENESW);
            result.mouse_cursors[ImGuiMouseCursor_ResizeNWSE as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENWSE);
            result.mouse_cursors[ImGuiMouseCursor_Hand as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_HAND);
            result.mouse_cursors[ImGuiMouseCursor_NotAllowed as usize] = SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_NO);
        }

        result
    }

    unsafe fn get_backend_data() -> Option<*mut Self> {
        match imgui_sys::igGetCurrentContext() as usize {
            0 => None,
            _ => {
                Some((*imgui_sys::igGetIO()).BackendPlatformUserData as *mut Self)
            }
        }
    }

    unsafe extern "C" fn set_clipboard_text(user_data: *mut cty::c_void, text: *const cty::c_char) {
        SDL_SetClipboardText(text);
    }

    unsafe extern "C" fn set_platform_ime_data(_: *mut imgui_sys::ImGuiViewport, data: *mut ImGuiPlatformImeData) {
        let data = &*data;
        if data.WantVisible {
            let rect = SDL_Rect {
                x: data.InputPos.x as i32,
                y: data.InputPos.y as i32,
                w: 1,
                h: data.InputLineHeight as i32,
            };
            SDL_SetTextInputRect(&rect as *const SDL_Rect);
        }
    }

    unsafe extern "C" fn get_clipboard_text(_: *mut cty::c_void) -> *const cty::c_char {
        assert!(Self::get_backend_data().is_some());

        let backend = &mut *Self::get_backend_data().unwrap();

        if let Some(data) = backend.clipboard_data {
            SDL_free(data as *const _ as *mut cty::c_void);
        }

        backend.clipboard_data = Some(SDL_GetClipboardText());
        backend.clipboard_data.unwrap()
    }

    fn update_mouse(&mut self, imgui: &mut Context, window: &mut SDL_Window) {
        let io = imgui.io_mut();

        if io.want_set_mouse_pos {
            unsafe { SDL_WarpMouseInWindow(window, io.mouse_pos[0] as i32, io.mouse_pos[1] as i32) }
        } else {
            io.mouse_pos[0] = -f32::MAX;
            io.mouse_pos[1] = -f32::MAX;
        }

        let (mut mx, mut my) = (0i32, 0i32);
        let mouse_buttons = unsafe { SDL_GetMouseState(&mut mx, &mut my) };
        io.mouse_down[0] = self.mouse_press[0] || (mouse_buttons & SDL_BUTTON_LEFT) != 0;
        io.mouse_down[1] = self.mouse_press[1] || (mouse_buttons & SDL_BUTTON_RIGHT) != 0;
        io.mouse_down[2] = self.mouse_press[2] || (mouse_buttons & SDL_BUTTON_MIDDLE) != 0;
        io.mouse_down[3] = self.mouse_press[3] || (mouse_buttons & SDL_BUTTON_X1) != 0;
        io.mouse_down[4] = self.mouse_press[4] || (mouse_buttons & SDL_BUTTON_X2) != 0;

        self.mouse_press[0] = false;
        self.mouse_press[1] = false;
        self.mouse_press[2] = false;
        self.mouse_press[3] = false;
        self.mouse_press[4] = false;

        if unsafe { SDL_GetWindowFlags(window as *mut SDL_Window) }
            & SDL_WindowFlags_SDL_WINDOW_INPUT_FOCUS
            != 0
        {
            io.mouse_pos[0] = mx as f32;
            io.mouse_pos[1] = my as f32;
        }
    }

    fn update_modifiers(&self, io: &mut Io, modifiers: SDL_Keymod) {
        unsafe {
            imgui_sys::ImGuiIO_AddKeyEvent(
                io.raw_mut(),
                ImGuiMod_Ctrl,
                (modifiers & SDL_Keymod_KMOD_CTRL) != 0,
            );

            imgui_sys::ImGuiIO_AddKeyEvent(
                io.raw_mut(),
                ImGuiMod_Shift,
                (modifiers & SDL_Keymod_KMOD_SHIFT) != 0,
            );
            imgui_sys::ImGuiIO_AddKeyEvent(
                io.raw_mut(),
                ImGuiMod_Alt,
                (modifiers & SDL_Keymod_KMOD_ALT) != 0,
            );

            imgui_sys::ImGuiIO_AddKeyEvent(
                io.raw_mut(),
                ImGuiMod_Super,
                (modifiers & SDL_Keymod_KMOD_GUI) != 0,
            );
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn keycode_to_imgui_key(keycode: i32) -> u32 {
        let keycode = keycode as u32;
        match keycode {
            SDL_KeyCode_SDLK_TAB => ImGuiKey_Tab,
            SDL_KeyCode_SDLK_LEFT => ImGuiKey_LeftArrow,
            SDL_KeyCode_SDLK_RIGHT => ImGuiKey_RightArrow,
            SDL_KeyCode_SDLK_UP => ImGuiKey_UpArrow,
            SDL_KeyCode_SDLK_DOWN => ImGuiKey_DownArrow,
            SDL_KeyCode_SDLK_PAGEUP => ImGuiKey_PageUp,
            SDL_KeyCode_SDLK_PAGEDOWN => ImGuiKey_PageDown,
            SDL_KeyCode_SDLK_HOME => ImGuiKey_Home,
            SDL_KeyCode_SDLK_END => ImGuiKey_End,
            SDL_KeyCode_SDLK_INSERT => ImGuiKey_Insert,
            SDL_KeyCode_SDLK_DELETE => ImGuiKey_Delete,
            SDL_KeyCode_SDLK_BACKSPACE => ImGuiKey_Backspace,
            SDL_KeyCode_SDLK_SPACE => ImGuiKey_Space,
            SDL_KeyCode_SDLK_RETURN => ImGuiKey_Enter,
            SDL_KeyCode_SDLK_ESCAPE => ImGuiKey_Escape,
            SDL_KeyCode_SDLK_QUOTE => ImGuiKey_Apostrophe,
            SDL_KeyCode_SDLK_COMMA => ImGuiKey_Comma,
            SDL_KeyCode_SDLK_MINUS => ImGuiKey_Minus,
            SDL_KeyCode_SDLK_PERIOD => ImGuiKey_Period,
            SDL_KeyCode_SDLK_SLASH => ImGuiKey_Slash,
            SDL_KeyCode_SDLK_SEMICOLON => ImGuiKey_Semicolon,
            SDL_KeyCode_SDLK_EQUALS => ImGuiKey_Equal,
            SDL_KeyCode_SDLK_LEFTBRACKET => ImGuiKey_LeftBracket,
            SDL_KeyCode_SDLK_BACKSLASH => ImGuiKey_Backslash,
            SDL_KeyCode_SDLK_RIGHTBRACKET => ImGuiKey_RightBracket,
            SDL_KeyCode_SDLK_BACKQUOTE => ImGuiKey_GraveAccent,
            SDL_KeyCode_SDLK_CAPSLOCK => ImGuiKey_CapsLock,
            SDL_KeyCode_SDLK_SCROLLLOCK => ImGuiKey_ScrollLock,
            SDL_KeyCode_SDLK_NUMLOCKCLEAR => ImGuiKey_NumLock,
            SDL_KeyCode_SDLK_PRINTSCREEN => ImGuiKey_PrintScreen,
            SDL_KeyCode_SDLK_PAUSE => ImGuiKey_Pause,
            SDL_KeyCode_SDLK_KP_0 => ImGuiKey_Keypad0,
            SDL_KeyCode_SDLK_KP_1 => ImGuiKey_Keypad1,
            SDL_KeyCode_SDLK_KP_2 => ImGuiKey_Keypad2,
            SDL_KeyCode_SDLK_KP_3 => ImGuiKey_Keypad3,
            SDL_KeyCode_SDLK_KP_4 => ImGuiKey_Keypad4,
            SDL_KeyCode_SDLK_KP_5 => ImGuiKey_Keypad5,
            SDL_KeyCode_SDLK_KP_6 => ImGuiKey_Keypad6,
            SDL_KeyCode_SDLK_KP_7 => ImGuiKey_Keypad7,
            SDL_KeyCode_SDLK_KP_8 => ImGuiKey_Keypad8,
            SDL_KeyCode_SDLK_KP_9 => ImGuiKey_Keypad9,
            SDL_KeyCode_SDLK_KP_PERIOD => ImGuiKey_KeypadDecimal,
            SDL_KeyCode_SDLK_KP_DIVIDE => ImGuiKey_KeypadDivide,
            SDL_KeyCode_SDLK_KP_MULTIPLY => ImGuiKey_KeypadMultiply,
            SDL_KeyCode_SDLK_KP_MINUS => ImGuiKey_KeypadSubtract,
            SDL_KeyCode_SDLK_KP_PLUS => ImGuiKey_KeypadAdd,
            SDL_KeyCode_SDLK_KP_ENTER => ImGuiKey_KeypadEnter,
            SDL_KeyCode_SDLK_KP_EQUALS => ImGuiKey_KeypadEqual,
            SDL_KeyCode_SDLK_LCTRL => ImGuiKey_LeftCtrl,
            SDL_KeyCode_SDLK_LSHIFT => ImGuiKey_LeftShift,
            SDL_KeyCode_SDLK_LALT => ImGuiKey_LeftAlt,
            SDL_KeyCode_SDLK_LGUI => ImGuiKey_LeftSuper,
            SDL_KeyCode_SDLK_RCTRL => ImGuiKey_RightCtrl,
            SDL_KeyCode_SDLK_RSHIFT => ImGuiKey_RightShift,
            SDL_KeyCode_SDLK_RALT => ImGuiKey_RightAlt,
            SDL_KeyCode_SDLK_RGUI => ImGuiKey_RightSuper,
            SDL_KeyCode_SDLK_APPLICATION => ImGuiKey_Menu,
            SDL_KeyCode_SDLK_0 => ImGuiKey_0,
            SDL_KeyCode_SDLK_1 => ImGuiKey_1,
            SDL_KeyCode_SDLK_2 => ImGuiKey_2,
            SDL_KeyCode_SDLK_3 => ImGuiKey_3,
            SDL_KeyCode_SDLK_4 => ImGuiKey_4,
            SDL_KeyCode_SDLK_5 => ImGuiKey_5,
            SDL_KeyCode_SDLK_6 => ImGuiKey_6,
            SDL_KeyCode_SDLK_7 => ImGuiKey_7,
            SDL_KeyCode_SDLK_8 => ImGuiKey_8,
            SDL_KeyCode_SDLK_9 => ImGuiKey_9,
            SDL_KeyCode_SDLK_a => ImGuiKey_A,
            SDL_KeyCode_SDLK_b => ImGuiKey_B,
            SDL_KeyCode_SDLK_c => ImGuiKey_C,
            SDL_KeyCode_SDLK_d => ImGuiKey_D,
            SDL_KeyCode_SDLK_e => ImGuiKey_E,
            SDL_KeyCode_SDLK_f => ImGuiKey_F,
            SDL_KeyCode_SDLK_g => ImGuiKey_G,
            SDL_KeyCode_SDLK_h => ImGuiKey_H,
            SDL_KeyCode_SDLK_i => ImGuiKey_I,
            SDL_KeyCode_SDLK_j => ImGuiKey_J,
            SDL_KeyCode_SDLK_k => ImGuiKey_K,
            SDL_KeyCode_SDLK_l => ImGuiKey_L,
            SDL_KeyCode_SDLK_m => ImGuiKey_M,
            SDL_KeyCode_SDLK_n => ImGuiKey_N,
            SDL_KeyCode_SDLK_o => ImGuiKey_O,
            SDL_KeyCode_SDLK_p => ImGuiKey_P,
            SDL_KeyCode_SDLK_q => ImGuiKey_Q,
            SDL_KeyCode_SDLK_r => ImGuiKey_R,
            SDL_KeyCode_SDLK_s => ImGuiKey_S,
            SDL_KeyCode_SDLK_t => ImGuiKey_T,
            SDL_KeyCode_SDLK_u => ImGuiKey_U,
            SDL_KeyCode_SDLK_v => ImGuiKey_V,
            SDL_KeyCode_SDLK_w => ImGuiKey_W,
            SDL_KeyCode_SDLK_x => ImGuiKey_X,
            SDL_KeyCode_SDLK_y => ImGuiKey_Y,
            SDL_KeyCode_SDLK_z => ImGuiKey_Z,
            SDL_KeyCode_SDLK_F1 => ImGuiKey_F1,
            SDL_KeyCode_SDLK_F2 => ImGuiKey_F2,
            SDL_KeyCode_SDLK_F3 => ImGuiKey_F3,
            SDL_KeyCode_SDLK_F4 => ImGuiKey_F4,
            SDL_KeyCode_SDLK_F5 => ImGuiKey_F5,
            SDL_KeyCode_SDLK_F6 => ImGuiKey_F6,
            SDL_KeyCode_SDLK_F7 => ImGuiKey_F7,
            SDL_KeyCode_SDLK_F8 => ImGuiKey_F8,
            SDL_KeyCode_SDLK_F9 => ImGuiKey_F9,
            SDL_KeyCode_SDLK_F10 => ImGuiKey_F10,
            SDL_KeyCode_SDLK_F11 => ImGuiKey_F11,
            SDL_KeyCode_SDLK_F12 => ImGuiKey_F12,
            _ => ImGuiKey_None,
        }
    }

    #[allow(non_upper_case_globals)]
    pub fn handle_event(&mut self, imgui: &mut Context, event: &SDL_Event) -> bool {
        let io = imgui.io_mut();
        unsafe {
            match event.type_ {
                SDL_EventType_SDL_MOUSEWHEEL => {
                    //let wheel = event.wheel;
                    //if wheel.mouseX > 0 {
                    //    io.mouse_wheel_h += 1.0
                    //};
                    //if wheel.mouseX < 0 {
                    //    io.mouse_wheel_h -= 1.0
                    //};
                    //if wheel.mouseY > 0 {
                    //    io.mouse_wheel += 1.0
                    //};
                    //if wheel.mouseY < 0 {
                    //    io.mouse_wheel -= 1.0
                    //};

                    imgui_sys::ImGuiIO_AddMouseWheelEvent(
                        io.raw_mut(),
                        -event.wheel.preciseX,
                        event.wheel.preciseY,
                    );

                    true
                }
                SDL_EventType_SDL_MOUSEBUTTONUP | SDL_EventType_SDL_MOUSEBUTTONDOWN => {
                    let button = event.button;
                    let mut mouse_button = -1;
                    if button.button == SDL_BUTTON_LEFT as u8 {
                        self.mouse_press[0] = true;
                        mouse_button = 0;
                    };
                    if button.button == SDL_BUTTON_RIGHT as u8 {
                        self.mouse_press[1] = true;
                        mouse_button = 1;
                    };
                    if button.button == SDL_BUTTON_MIDDLE as u8 {
                        self.mouse_press[2] = true;
                        mouse_button = 2;
                    };
                    if button.button == SDL_BUTTON_X1 as u8 {
                        self.mouse_press[3] = true;
                        mouse_button = 3;
                    };
                    if button.button == SDL_BUTTON_X2 as u8 {
                        self.mouse_press[4] = true;
                        mouse_button = 4;
                    };

                    imgui_sys::ImGuiIO_AddMouseButtonEvent(
                        io.raw_mut(),
                        mouse_button,
                        event.type_ == SDL_EventType_SDL_MOUSEBUTTONDOWN,
                    );

                    true
                }
                SDL_EventType_SDL_TEXTINPUT => {
                    static mut last_timestamp: u32 = 0;
                    // Hack to prevent double input bug
                    if event.text.timestamp != last_timestamp {
                        imgui_sys::ImGuiIO_AddInputCharactersUTF8(
                            io.raw_mut(),
                            &event.text.text as *const _,
                        );
                        last_timestamp = event.text.timestamp;
                    }
                    true
                }
                SDL_EventType_SDL_KEYDOWN | SDL_EventType_SDL_KEYUP => {
                    let key = Self::keycode_to_imgui_key(event.key.keysym.sym);

                    self.update_modifiers(io, event.key.keysym.mod_ as u32);

                    imgui_sys::ImGuiIO_AddKeyEvent(
                        io.raw_mut(),
                        key,
                        event.type_ == SDL_EventType_SDL_KEYDOWN,
                    );

                    imgui_sys::ImGuiIO_SetKeyEventNativeData(
                        io.raw_mut(),
                        key,
                        event.key.keysym.scancode as i32,
                        event.key.keysym.scancode as i32,
                        0,
                    );

                    true
                }
                SDL_EventType_SDL_MOUSEMOTION => {
                    imgui_sys::ImGuiIO_AddMousePosEvent(
                        io.raw_mut(),
                        event.motion.x as f32,
                        event.motion.y as f32,
                    );

                    true
                }
                _ => false,
            }
        }
    }

    fn update_cursor(&mut self, imgui: &mut Context) {
        let io = imgui.io_mut();

        let imgui_cursor = unsafe { imgui_sys::igGetMouseCursor() };

        if io.mouse_draw_cursor || imgui_cursor == ImGuiMouseCursor_None {
            unsafe { SDL_ShowCursor(SDL_bool_SDL_FALSE as i32) };
        } else {
            let cursor = self.mouse_cursors[imgui_cursor as usize];

            unsafe { SDL_ShowCursor(SDL_bool_SDL_TRUE as i32) };
        }
    }

    pub fn prepare_frame(&mut self, imgui: &mut Context, window: &mut SDL_Window) {
        let io = imgui.io_mut();

        let (mut width, mut height) = (0i32, 0i32);

        unsafe {
            if (SDL_GetWindowFlags(window) & SDL_WindowFlags_SDL_WINDOW_MINIMIZED) != 0 {
                SDL_GetWindowSize(window, &mut width, &mut height);

                let (mut display_width, mut display_height) = (0, 0);

                SDL_GL_GetDrawableSize(window, &mut display_width, &mut display_height);

                io.display_framebuffer_scale[0] = (display_width as f32) / width as f32;
                io.display_framebuffer_scale[1] = (display_height as f32) / height as f32;
            }
        }

        io.display_size[0] = width as f32;
        io.display_size[1] = height as f32;

        let current_time = unsafe { SDL_GetPerformanceCounter() };
        io.delta_time = if self.time > 0 {
            ((current_time - self.time) as f64 / self.frequency as f64) as f32
        } else {
            1.0 / 60.0
        };
        self.time = current_time;

        self.update_mouse(imgui, window);
        self.update_cursor(imgui);
    }
}
