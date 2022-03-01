use cgmath::Vector3;

pub fn vert_to_u8(vecs: Vec<Vector3<f32>>) -> &'static [u8] {
    let return_data: &[u8];
    unsafe {
        let mut data: Vec<f32> = Vec::new();
        for vec in vecs {
            data.push(vec.x);
            data.push(vec.y);
            data.push(vec.z);
        }

        return_data = core::slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * core::mem::size_of::<f64>()
        );
        // self.amount = data.len() as u32
        &return_data
    }
}

