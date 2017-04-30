mod tests {

    fn to_rgb(r: f32, g: f32, b: f32) {
        let r0: f32 = r * 255.0;
        let g0: f32 = g * 255.0;
        let b0: f32 = b * 255.0;

        let r1: f32 = r0.round();
        let g1: f32 = g0.round();
        let b1: f32 = b0.round();

        println!("Original RGB values before Cairo: ({}, {}, {})", r1, g1, b1);
    }

}
