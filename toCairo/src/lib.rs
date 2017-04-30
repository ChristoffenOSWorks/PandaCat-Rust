pub mod cairo {

    pub fn to_cairo(r: i32, g: i32, b: i32) {
        let r0 = r as f32;
        let g0 = g as f32;
        let b0 = b as f32;

        let r1 = r0 / 255.0;
        let g1 = g0 / 255.0;
        let b1 = b0 / 255.0;
    
        let r3 = r1;
        let g3 = g1;
        let b3 = b1;

        println!("cairo_set_source_rgb ({}, {}, {});", r3, g3, b3);
    }

}
