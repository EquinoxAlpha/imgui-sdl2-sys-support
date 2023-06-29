# imgui-sdl2-sys-support
Simple SDL2 input handling support for imgui-rs using only C bindings

## Why?
I was working on a project that involved hooking some SDL2 functions inside another program. For that reason, I only had access to their C representation (which makes using the sdl2 crate impossible), and other crates that provide this support all use the sdl2 crate. So I rewrote the original dear imgui SDL2 support in Rust.

However, the sdl2_sys bindings were missing a lot of definitions to make them suitable for this, so I had to generate my own with bindgen. The bindings can be found in the src/bindings directory.

## Usage
Initialize the ImGuiSDL2 struct:
```let mut platform = ImGuiSDL2::new(&mut imgui)```

Then, before calling imgui.frame() to begin a frame, call this, where sdl_window is a reference to SDL_Window.
```platform.prepare_frame(&mut imgui, &mut sdl_window);```

Now you will also need to capture events somewhere, such as in a SDL_PollEvent hook.
Call this in your hook, where "event" is a SDL_Event reference:
```platform.handle_event(&mut imgui, event)```

## Notes

Clipboard support is missing. I'll add it later.
SDL cursor changing is not supported. I'll maybe add it later.