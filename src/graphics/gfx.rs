use super::{pipeline::RenderTexture, BlendingMode, RenderStats};

/// Upscales the `source` texture into the `target` texture only doing integer scaling.
pub fn integer_scaling(target: &mut RenderTexture, source: &RenderTexture, stats: &mut RenderStats) {
    let src_res = source.res();
    let dst_res = target.res();
    
    let scaling = integer_scaling::get_scaling(src_res, dst_res);

    let size_px = (src_res.0 * scaling, src_res.1 * scaling);
    let offset_px = ((dst_res.0 as i32 - size_px.0 as i32) / 2, (dst_res.1 as i32 - size_px.1 as i32) / 2);
    let rect = (offset_px.0, offset_px.1, size_px.0, size_px.1);

    target.combine_ext(source, BlendingMode::AlphaMix, rect, stats);
}

mod integer_scaling {
    pub fn get_scaling(src_res: (u32, u32), dst_res: (u32, u32)) -> u32 {
        let x_scaling = (dst_res.0 / src_res.0).max(1);
        let y_scaling = (dst_res.1 / src_res.1).max(1);
    
        return u32::min(x_scaling, y_scaling);
    }
}
