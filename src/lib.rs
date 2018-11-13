mod renderer;
mod settings;

use gfx_hal::Backend;

use self::renderer::{Renderer};
use self::settings::Settings;

/// A struct representing the top level of this library.
/// It provides access to all the subsystems that can be used.
pub struct Smml<B: Backend> {
    renderer: Renderer<B>,
    //resource_manager: AssetManager,
    //input_system: InputSystem,
    settings: Settings,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
