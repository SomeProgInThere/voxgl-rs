mod voxgl;

fn main() {
    pollster::block_on(voxgl::window::run());
}
