

use sdl2_bindings::{
    SDL_Event, SDL_EventType_SDL_KEYDOWN, SDL_EventType_SDL_KEYUP,
    SDL_EventType_SDL_MOUSEBUTTONDOWN, SDL_EventType_SDL_MOUSEMOTION, SDL_EventType_SDL_MOUSEWHEEL,
    SDL_EventType_SDL_TEXTINPUT, SDL_GetMouseState, SDL_GetWindowFlags,
    SDL_Scancode_SDL_SCANCODE_A, SDL_Scancode_SDL_SCANCODE_BACKSPACE, SDL_Scancode_SDL_SCANCODE_C,
    SDL_Scancode_SDL_SCANCODE_DELETE, SDL_Scancode_SDL_SCANCODE_DOWN,
    SDL_Scancode_SDL_SCANCODE_END, SDL_Scancode_SDL_SCANCODE_ESCAPE,
    SDL_Scancode_SDL_SCANCODE_HOME, SDL_Scancode_SDL_SCANCODE_KP_ENTER,
    SDL_Scancode_SDL_SCANCODE_LEFT, SDL_Scancode_SDL_SCANCODE_PAGEDOWN,
    SDL_Scancode_SDL_SCANCODE_PAGEUP, SDL_Scancode_SDL_SCANCODE_RIGHT,
    SDL_Scancode_SDL_SCANCODE_SPACE, SDL_Scancode_SDL_SCANCODE_TAB, SDL_Scancode_SDL_SCANCODE_UP,
    SDL_Scancode_SDL_SCANCODE_V, SDL_Scancode_SDL_SCANCODE_X, SDL_Scancode_SDL_SCANCODE_Y,
    SDL_Scancode_SDL_SCANCODE_Z, SDL_WarpMouseInWindow, SDL_Window,
    SDL_WindowFlags_SDL_WINDOW_INPUT_FOCUS, SDL_BUTTON_LEFT, SDL_BUTTON_MIDDLE, SDL_BUTTON_RIGHT,
    SDL_BUTTON_X1, SDL_BUTTON_X2,
};

use imgui::{internal::RawCast, Context, Key};

struct ImGuiSDL2<'a> {
    mouse_press: [bool; 5],
    window: &'a mut SDL_Window,
}

impl<'a> ImGuiSDL2<'a> {
    pub fn new(imgui: &mut Context, window: &'a mut SDL_Window) -> Self {
        imgui.io_mut().key_map[Key::Tab as usize] = SDL_Scancode_SDL_SCANCODE_TAB;
        imgui.io_mut().key_map[Key::LeftArrow as usize] = SDL_Scancode_SDL_SCANCODE_LEFT;
        imgui.io_mut().key_map[Key::RightArrow as usize] = SDL_Scancode_SDL_SCANCODE_RIGHT;
        imgui.io_mut().key_map[Key::UpArrow as usize] = SDL_Scancode_SDL_SCANCODE_UP;
        imgui.io_mut().key_map[Key::DownArrow as usize] = SDL_Scancode_SDL_SCANCODE_DOWN;
        imgui.io_mut().key_map[Key::PageUp as usize] = SDL_Scancode_SDL_SCANCODE_PAGEUP;
        imgui.io_mut().key_map[Key::PageDown as usize] = SDL_Scancode_SDL_SCANCODE_PAGEDOWN;
        imgui.io_mut().key_map[Key::Home as usize] = SDL_Scancode_SDL_SCANCODE_HOME;
        imgui.io_mut().key_map[Key::End as usize] = SDL_Scancode_SDL_SCANCODE_END;
        imgui.io_mut().key_map[Key::Delete as usize] = SDL_Scancode_SDL_SCANCODE_DELETE;
        imgui.io_mut().key_map[Key::Backspace as usize] = SDL_Scancode_SDL_SCANCODE_BACKSPACE;
        imgui.io_mut().key_map[Key::Enter as usize] = SDL_Scancode_SDL_SCANCODE_KP_ENTER;
        imgui.io_mut().key_map[Key::Escape as usize] = SDL_Scancode_SDL_SCANCODE_ESCAPE;
        imgui.io_mut().key_map[Key::Space as usize] = SDL_Scancode_SDL_SCANCODE_SPACE;
        imgui.io_mut().key_map[Key::A as usize] = SDL_Scancode_SDL_SCANCODE_A;
        imgui.io_mut().key_map[Key::C as usize] = SDL_Scancode_SDL_SCANCODE_C;
        imgui.io_mut().key_map[Key::V as usize] = SDL_Scancode_SDL_SCANCODE_V;
        imgui.io_mut().key_map[Key::X as usize] = SDL_Scancode_SDL_SCANCODE_X;
        imgui.io_mut().key_map[Key::Y as usize] = SDL_Scancode_SDL_SCANCODE_Y;
        imgui.io_mut().key_map[Key::Z as usize] = SDL_Scancode_SDL_SCANCODE_Z;

        Self {
            mouse_press: [false; 5],
            window,
        }
    }

    pub unsafe fn prepare_frame(&mut self, imgui: &mut Context) {
        let io = imgui.io_mut();

        if io.want_set_mouse_pos {
            SDL_WarpMouseInWindow(self.window, io.mouse_pos[0] as i32, io.mouse_pos[1] as i32);
        } else {
            io.mouse_pos[0] = -f32::MAX;
            io.mouse_pos[1] = -f32::MAX;
        }

        let (mx, my) = (0i32, 0i32);
        let mouse_buttons =
            SDL_GetMouseState(&mx as *const _ as *mut i32, &my as *const _ as *mut i32);
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

        //if SDL_GetWindowFlags(window as *mut SDL_Window) & SDL_WindowFlags_SDL_WINDOW_INPUT_FOCUS == 1 {
        io.mouse_pos[0] = mx as f32;
        io.mouse_pos[1] = my as f32;
        //}
    }

    pub unsafe fn handle_event(&mut self, imgui: &mut Context, event: &SDL_Event) -> bool {
        match event.type_ {
            SDL_EventType_SDL_MOUSEWHEEL => {
                let wheel = event.wheel;
                if wheel.mouseX > 0 {
                    imgui.io_mut().mouse_wheel_h += 1.0
                };
                if wheel.mouseX < 0 {
                    imgui.io_mut().mouse_wheel_h -= 1.0
                };
                if wheel.mouseY > 0 {
                    imgui.io_mut().mouse_wheel += 1.0
                };
                if wheel.mouseY < 0 {
                    imgui.io_mut().mouse_wheel -= 1.0
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
                println!("Hi");
                true
            }
            SDL_EventType_SDL_TEXTINPUT => {
                unsafe {
                    imgui_sys::ImGuiIO_AddInputCharactersUTF8(
                        imgui.io_mut().raw_mut(),
                        &event.text.text as *const _,
                    );
                }
                true
            }
            SDL_EventType_SDL_KEYDOWN | SDL_EventType_SDL_KEYUP => {
                let key = event.key.keysym.scancode;

                //TODO: Implement.

                true
            }
            SDL_EventType_SDL_MOUSEMOTION => true,
            _ => false,
        }
    }
}