use std::collections::HashMap;
use crate::settings::Settings;
use std::cell::RefCell;
use std::rc::Rc;
use crate::resource::mesh::Mesh;
use crate::resource::camera::Camera;

pub mod mesh;
pub mod camera;

pub struct AssetManager {
    meshes: HashMap<String, Mesh>,
    active_camera: Camera,
}

impl AssetManager {
    pub fn new(settings: Rc<RefCell<Settings>>) -> Self {
        AssetManager {
            meshes: HashMap::new(),
            active_camera: Camera::new(settings),
        }
    }

    pub fn add_mesh(&mut self, mut mesh: Mesh) {
        if self.meshes.contains_key(mesh.name()) {
            let unique_name = (2..).find_map(|x| {
                let key = mesh.name().to_owned() + "_" + &x.to_string();
                if self.meshes.contains_key(&key) {
                    Some(key)
                } else {
                    None
                }
            }).unwrap();
            mesh.set_name(unique_name.as_str());
            self.meshes.insert(unique_name, mesh);
        } else {
            self.meshes.insert(mesh.name().to_string(), mesh);
        }
    }

    pub fn get_mesh(&self, name: &str) -> Option<&Mesh> {
        self.meshes.get(name)
    }

    pub fn remove_mesh(&mut self, name: &str) {
        self.meshes.remove(name);
    }
}