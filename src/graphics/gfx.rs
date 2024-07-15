use crate::math::{Rect, vec2};

use super::{pipeline::RenderTexture, BlendingMode, RenderStats, Graphics};


/// Upscales the `source` texture into the `target` texture only doing integer scaling.
pub fn integer_scaling(target: &mut RenderTexture, source: &RenderTexture, stats: &mut RenderStats) {
    let src_res = source.res();
    let dst_res = target.res();
    
    let scaling = integer_scaling::get_scaling(src_res, dst_res);
    let rect = integer_scaling::get_screen_rect(src_res, dst_res, scaling);

    target.combine_ext(source, BlendingMode::AlphaMix, rect, Rect::IDENT, stats);
}

/// Scales the provided screen position to get a screen position relative to the source texture instead of the window.<br>
/// If the mouse position lies outside the source texture, the function will return `Err(result)`, so the result can still be used.
pub fn integer_scaling_mouse_pos(screen_pos: vec2, src_res: (u32, u32), dst_res: (u32, u32)) -> Result<vec2, vec2> {
    let scaling = integer_scaling::get_scaling(src_res, dst_res);
    let rect = integer_scaling::get_screen_rect(src_res, dst_res, scaling);

    let x = inverse_lerp(rect.l() as f32, rect.r() as f32, screen_pos.0);
    let y = inverse_lerp(rect.d() as f32, rect.u() as f32, screen_pos.1);

    let pixel_pos = vec2(x * src_res.0 as f32, y * src_res.1 as f32);
    if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
        return Err(pixel_pos);
    } else {
        return Ok(pixel_pos);
    }
}

pub fn screen_to_world_pos(screen_pos: vec2, screen_res: (u32, u32)) -> vec2 {
    let screen_pos = vec2(screen_pos.0, screen_res.1 as f32 - screen_pos.1); // Invert y-axis
    let pos_clip = screen_pos.inv_scale(vec2(screen_res.0 as f32, screen_res.1 as f32)) * 2.0 - vec2::ONE;

    let cam_data = Graphics::get_cam_data();
    
    return pos_clip.scale(cam_data.half_size) + cam_data.pos;
}

mod integer_scaling {
    use crate::graphics::pipeline::ScreenRect;

    pub fn get_scaling(src_res: (u32, u32), dst_res: (u32, u32)) -> u32 {
        let x_scaling = (dst_res.0 / src_res.0).max(1);
        let y_scaling = (dst_res.1 / src_res.1).max(1);
    
        return u32::min(x_scaling, y_scaling);
    }

    pub fn get_screen_rect(src_res: (u32, u32), dst_res: (u32, u32), scaling: u32) -> ScreenRect {
        let size_px = ((src_res.0 * scaling) as i32, (src_res.1 * scaling) as i32);
        let offset_px = ((dst_res.0 as i32 - size_px.0 as i32) / 2, (dst_res.1 as i32 - size_px.1 as i32) / 2);
        return ScreenRect::new(offset_px, size_px);
    }

}

fn inverse_lerp(a: f32, b: f32, c: f32) -> f32 {
    return (c - a) / (b - a);
}