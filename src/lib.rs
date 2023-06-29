#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![feature(let_chains)]

pub extern crate sdl2_bindings;

use imgui::{internal::RawCast, Context, Io};
use imgui_sys::*;
use sdl2_bindings::*;

pub struct ImGuiSDL2 {
    frequency: u64,
    time: u64,
    last_cursor: Option<*const SDL_Cursor>,
    clipboard_data: Option<*mut cty::c_char>,
    mouse_cursors: Vec<*mut SDL_Cursor>,
}

impl ImGuiSDL2 {
    pub fn new(imgui: &mut Context) -> Self {
        let io = imgui.io_mut();

        io.backend_flags |= imgui::BackendFlags::HAS_MOUSE_CURSORS;
        io.backend_flags |= imgui::BackendFlags::HAS_SET_MOUSE_POS;

        let mut mouse_cursors: Vec<*mut SDL_Cursor> = Vec::with_capacity(9);

        unsafe {
            mouse_cursors.set_len(9);

            mouse_cursors[ImGuiMouseCursor_Arrow as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_ARROW);
            mouse_cursors[ImGuiMouseCursor_TextInput as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_IBEAM);
            mouse_cursors[ImGuiMouseCursor_ResizeAll as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEALL);
            mouse_cursors[ImGuiMouseCursor_ResizeNS as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENS);
            mouse_cursors[ImGuiMouseCursor_ResizeEW as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZEWE);
            mouse_cursors[ImGuiMouseCursor_ResizeNESW as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENESW);
            mouse_cursors[ImGuiMouseCursor_ResizeNWSE as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_SIZENWSE);
            mouse_cursors[ImGuiMouseCursor_Hand as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_HAND);
            mouse_cursors[ImGuiMouseCursor_NotAllowed as usize] =
                SDL_CreateSystemCursor(SDL_SystemCursor_SDL_SYSTEM_CURSOR_NO);
        }

        let result = Self {
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
        }

        result
    }

    unsafe fn get_backend_data() -> Option<*mut Self> {
        match imgui_sys::igGetCurrentContext() as usize {
            0 => None,
            _ => Some((*imgui_sys::igGetIO()).BackendPlatformUserData as *mut Self),
        }
    }

    unsafe extern "C" fn set_clipboard_text(_: *mut cty::c_void, text: *const cty::c_char) {
        SDL_SetClipboardText(text);
    }

    unsafe extern "C" fn set_platform_ime_data(
        _: *mut imgui_sys::ImGuiViewport,
        data: *mut ImGuiPlatformImeData,
    ) {
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
        unsafe { SDL_GetMouseState(&mut mx, &mut my) };

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
                    imgui_sys::ImGuiIO_AddMouseWheelEvent(
                        io.raw_mut(),
                        -event.wheel.x as f32,
                        event.wheel.y as f32,
                    );

                    true
                }
                SDL_EventType_SDL_MOUSEBUTTONUP | SDL_EventType_SDL_MOUSEBUTTONDOWN => {
                    let mouse_button = match event.button.button as u32 {
                        SDL_BUTTON_LEFT => 0,
                        SDL_BUTTON_RIGHT => 1,
                        SDL_BUTTON_MIDDLE => 2,
                        SDL_BUTTON_X1 => 3,
                        SDL_BUTTON_X2 => 4,
                        _ => return false,
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
                    // Hack to prevent double input bug, no idea what causes it but it occurs during testing
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
            let expected_cursor = match self.mouse_cursors[imgui_cursor as usize] as u64 {
                0 => self.mouse_cursors[ImGuiMouseCursor_Arrow as usize],
                _ => self.mouse_cursors[imgui_cursor as usize],
            };

            if let Some(last_cursor) = self.last_cursor && last_cursor != expected_cursor {
                unsafe { SDL_SetCursor(expected_cursor) };
                self.last_cursor = Some(expected_cursor);
            }

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

                io.display_size[0] = width as f32;
                io.display_size[1] = height as f32;
            }
        }

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
