use nogine::{window::WindowCfg, unwrap_res, input::{Input, KeyInput}, utils::rng::RNG, log_info};

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Random number generation. (Press E to log)").init());

    while window.is_running() {
        window.pre_tick(None);

        if Input::key_pressed(KeyInput::E) {
            let random_uint: u32 = RNG::global().gen();
            let random_int: i32 = RNG::global().gen_signed();
            let random_float: f32 = RNG::global().gen_range(1.0..20.0);
            let random_pick = RNG::global().pick(&['A', 'B', 'C', 'D']);

            log_info!("<< RANDOM DATA >>");
            log_info!("Random u32: {random_uint}");
            log_info!("Random i32: {random_int}");
            log_info!("Random f32: {random_float}");
            log_info!("Random pick from ABCD: {random_pick}\n");
        }

        window.post_tick();
    }
}