use sdl2_bindings::{
    SDL_Event, SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_KEYUP,
    SDL_EventType_SDL_MOUSEBUTTONDOWN, SDL_EventType_SDL_MOUSEMOTION, SDL_EventType_SDL_MOUSEWHEEL,
    SDL_EventType_SDL_TEXTINPUT, SDL_GL_GetDrawableSize, SDL_GetModState, SDL_GetMouseState,
    SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency, SDL_GetWindowFlags, SDL_GetWindowSize,
    SDL_Keymod_KMOD_ALT, SDL_Keymod_KMOD_CTRL, SDL_Keymod_KMOD_GUI, SDL_Keymod_KMOD_SHIFT,
    SDL_Scancode_SDL_SCANCODE_A, SDL_Scancode_SDL_SCANCODE_BACKSPACE, SDL_Scancode_SDL_SCANCODE_C,
    SDL_Scancode_SDL_SCANCODE_DELETE, SDL_Scancode_SDL_SCANCODE_DOWN,
    SDL_Scancode_SDL_SCANCODE_END, SDL_Scancode_SDL_SCANCODE_ESCAPE,
    SDL_Scancode_SDL_SCANCODE_HOME, SDL_Scancode_SDL_SCANCODE_KP_ENTER,
    SDL_Scancode_SDL_SCANCODE_LEFT, SDL_Scancode_SDL_SCANCODE_PAGEDOWN,
    SDL_Scancode_SDL_SCANCODE_PAGEUP, SDL_Scancode_SDL_SCANCODE_RIGHT,
    SDL_Scancode_SDL_SCANCODE_SPACE, SDL_Scancode_SDL_SCANCODE_TAB, SDL_Scancode_SDL_SCANCODE_UP,
    SDL_Scancode_SDL_SCANCODE_V, SDL_Scancode_SDL_SCANCODE_X, SDL_Scancode_SDL_SCANCODE_Y,
    SDL_Scancode_SDL_SCANCODE_Z, SDL_WarpMouseInWindow, SDL_Window,
    SDL_WindowFlags_SDL_WINDOW_INPUT_FOCUS, SDL_WindowFlags_SDL_WINDOW_MINIMIZED,
    SDL_BUTTON_LEFT, SDL_BUTTON_MIDDLE, SDL_BUTTON_RIGHT, SDL_BUTTON_X1, SDL_BUTTON_X2,
};

pub extern crate sdl2_bindings;

use imgui::{internal::RawCast, Context, Key};

pub struct ImGuiSDL2<> {
    mouse_press: [bool; 5],
    frequency: u64,
    time: u64,
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

        Self {
            mouse_press: [false; 5],
            frequency: unsafe { SDL_GetPerformanceFrequency() },
            time: unsafe { SDL_GetPerformanceCounter() },
        }
    }

    fn update_mouse(&mut self, imgui: &mut Context, window: &mut SDL_Window) {
        let io = imgui.io_mut();

        if io.want_set_mouse_pos {
            unsafe {
                SDL_WarpMouseInWindow(window, io.mouse_pos[0] as i32, io.mouse_pos[1] as i32)
            }
        } else {
            io.mouse_pos[0] = -f32::MAX;
            io.mouse_pos[1] = -f32::MAX;
        }

        let (mut mx, mut my) = (0i32, 0i32);
        let mouse_buttons =
            unsafe { SDL_GetMouseState(&mut mx, &mut my) };
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

    #[allow(non_upper_case_globals)]
    pub fn handle_event(&mut self, imgui: &mut Context, event: &SDL_Event) -> bool {
        let io = imgui.io_mut();
        unsafe {
            match event.type_ {
                SDL_EventType_SDL_MOUSEWHEEL => {
                    let wheel = event.wheel;
                    if wheel.mouseX > 0 {
                        io.mouse_wheel_h += 1.0
                    };
                    if wheel.mouseX < 0 {
                        io.mouse_wheel_h -= 1.0
                    };
                    if wheel.mouseY > 0 {
                        io.mouse_wheel += 1.0
                    };
                    if wheel.mouseY < 0 {
                        io.mouse_wheel -= 1.0
                    };
                    true
                }
                SDL_EventType_SDL_MOUSEBUTTONDOWN => {
                    let button = event.button;
                    if button.button == SDL_BUTTON_LEFT as u8 {
                        self.mouse_press[0] = true
                    };
                    if button.button == SDL_BUTTON_RIGHT as u8 {
                        self.mouse_press[1] = true
                    };
                    if button.button == SDL_BUTTON_MIDDLE as u8 {
                        self.mouse_press[2] = true
                    };
                    if button.button == SDL_BUTTON_X1 as u8 {
                        self.mouse_press[3] = true
                    };
                    if button.button == SDL_BUTTON_X2 as u8 {
                        self.mouse_press[4] = true
                    };
                    
                    true
                }
                SDL_EventType_SDL_TEXTINPUT => {
                    imgui_sys::ImGuiIO_AddInputCharactersUTF8(
                        imgui.io_mut().raw_mut(),
                        &event.text.text as *const _,
                    );
                    true
                }
                SDL_EventType_SDL_KEYDOWN | SDL_EventType_SDL_KEYUP => {
                    let key = event.key.keysym.scancode as usize;
                    let mod_state = SDL_GetModState();

                    io.keys_down[key] = event.type_ == SDL_EventType_SDL_KEYDOWN;
                    io.key_shift = mod_state & SDL_Keymod_KMOD_SHIFT != 0;
                    io.key_ctrl = mod_state & SDL_Keymod_KMOD_CTRL != 0;
                    io.key_alt = mod_state & SDL_Keymod_KMOD_ALT != 0;
                    io.key_super = mod_state & SDL_Keymod_KMOD_GUI != 0;

                    true
                }
                SDL_EventType_SDL_MOUSEMOTION => true,
                _ => false,
            }
        }
    }

    pub fn prepare_frame(&mut self, imgui: &mut Context, window: &mut SDL_Window) {
        let io = imgui.io_mut();

        let (mut width, mut height) = (0i32, 0i32);

        unsafe {
            if (SDL_GetWindowFlags(window) & SDL_WindowFlags_SDL_WINDOW_MINIMIZED) != 0 {
                SDL_GetWindowSize(window, &mut width, &mut height);

                let (mut display_width, mut display_height) = (0, 0);

                SDL_GL_GetDrawableSize(
                    window,
                    &mut display_width,
                    &mut display_height,
                );

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
    }
}
