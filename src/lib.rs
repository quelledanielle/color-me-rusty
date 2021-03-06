mod color;

use color::Color;

extern {
    fn setColor(ptr: *const u8);
}

/// Sets the color to a frame of a linear cycle through red, green, blue.
///
/// It takes `frames_per_color` frames to interpolate from red to green,
/// and `frames_per_color` frames to interpolate from green to blue.
///
/// When the ratio `frame` divided by `frames_per_color` is:
/// * `0.0`, then the color is red `[255, 0, 0]`.
/// * `0.5`, then the color is between red and green `[127, 127, 0]`.
/// * `1.0`, then the color is green `[0, 255, 0]`.
/// * `1.5`, then the color is between green and blue `[0, 127, 127]`.
/// * `2.0`, then the color is blue `[0, 0, 255]`.
/// * `2.5`, then the color is between blue and red `[127, 0, 127]`.
/// * `3.0`, then the color is red `[255, 0, 0]`.
/// * ...
///
/// This function is available in JavaScript via the WebAssembly module instance exports.
/// It calls the function imported from JavaScript, setColor, passing a pointer to the
/// offset of the rgb color values in WebAssembly linear memory.
///
/// # Examples
///
///```javascript
/// (async function() {
///   const wasm = await fetch('path/to/color-me-rusty.wasm')
///   const bytes = await wasm.arrayBuffer()
///   const module = await WebAssembly.instantiate(bytes, { env: { setColor }})
///
///   function setColor(valsPtr) {
///     let vals = new Uint8ClampedArray(module.instance.exports.memory.buffer, valsPtr, 3)
///     let rgb = `rgb(${vals[0]}, ${vals[1]}, ${vals[2]})`
///     document.body.style.backgroundColor = rgb
///   }
///
///   // Calls setColor with a pointer to [0, 255, 0],
///   // stored in WebAssembly linear memory
///   module.instance.exports.set_frame_color(100, 100)
/// })()
/// ```
#[no_mangle]
pub extern "C" fn set_frame_color(frame: u32, frames_per_color: u32) {
    let colors: Vec<&Color> = vec![&Color::RED, &Color::GREEN, &Color::BLUE];
    let color: Color = Color::interpolate_linear(&colors, frame as usize, frames_per_color as usize);
    let vals: Vec<u8> = color.values();
    unsafe { setColor(vals.as_ptr()); }
}
