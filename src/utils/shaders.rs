pub struct Shaders {
    pub vertex_shader: String,
    pub fragment_shader: String,
}

fn vertex() -> Vec<String> {
    vec![include_str!("../shaders/default.vert").to_string()]
}

fn fragment() -> Vec<String> {
    vec![include_str!("../shaders/default.frag").to_string()]
}

pub fn load_shaders() -> Shaders {
    Shaders {
        vertex_shader: vertex().join("\n"),
        fragment_shader: fragment().join("\n"),
    }
}
