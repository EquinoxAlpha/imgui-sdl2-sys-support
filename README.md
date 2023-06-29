# imgui-sdl2-sys-support
SDL2 input handling support for imgui-rs using only C bindings

## Why?
I was working on a project that involved hooking some SDL2 functions inside another program. For that reason, I only had access to their C representation (which makes using the sdl2 crate impossible), and other crates that provide this support all use the sdl2 crate. So I rewrote the original dear imgui SDL2 support in Rust.

However, the sdl2_sys bindings were missing a lot of definitions to make them suitable for this, so I had to generate my own with bindgen. You can find the bindings used for this in the src/bindings directory. 

## Usage
Initialize the ImGuiSDL2 struct:
```let mut platform = ImGuiSDL2::new(&mut imgui, &mut sdl_window)```

sdl_window can be a reference (you'll need to do some pointer casting if you're using this with hooking) to a normal SDL_Window struct, just as it would be in C/C++.

Then, before calling imgui.frame() to begin a frame, call
```platform.prepare_frame(&mut imgui);```

Now you will also need to capture events somewhere, such as in a SDL_PollEvent hook.
Call this in your hook, where "event" is a SDL_Event reference:
```platform.handle_event(&mut imgui, event)```

## Notes

Clipboard support is missing. I'll add it later.