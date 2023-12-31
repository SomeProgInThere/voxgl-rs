use std::{collections::HashMap, fs};

use super::{world::voxel::VoxelId, texture::Texture};

pub struct Textures {
    texture_map: HashMap<VoxelId, Texture>
}

impl Textures {
    pub fn init(
        textures: HashMap<VoxelId, &str>, device: &wgpu::Device, queue: &wgpu::Queue
    ) -> anyhow::Result<Self> {
    
        let mut texture_map = HashMap::<VoxelId, Texture>::with_capacity(textures.capacity());
        for (id, path) in textures.into_iter() {
            let mut resource_path = std::env::current_dir()?;
            resource_path.push("resources");
            resource_path.push(path);
            
            println!("loading texture: {:?}", resource_path);
            
            let buffer = fs::read(resource_path).unwrap();
            let texture = Texture::from_image(device, queue, buffer.as_slice(), Some(path))?;
            texture_map.insert(id, texture);
        }

        println!("loaded {} textures.", texture_map.len());
        Ok(Textures { texture_map })
    }

    pub fn get_texture_ref(&self, id: VoxelId) -> anyhow::Result<&Texture> {
        Ok(self.texture_map.get(&id).unwrap())
    }
}