use std::{
    fs,
    thread,
    sync::{Arc, Mutex}
};
use serde::{
    Serialize,
    Deserialize
};

#[derive(Serialize, Deserialize)]
pub struct Tileset {
    pub name: String,
    pub tiles: Vec<Vec<u8>>,
    pub tile_size: u32
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub tileset: usize,
    pub tile_index: usize
}

#[derive(Default, Serialize, Deserialize)]
pub struct Tilemap {
    pub tilesets: Vec<Tileset>,
    pub tiles: Vec<Vec<Tile>>
}

impl Tileset {
    pub fn new(name: String, directory: String, count: u32, tile_size: u32) -> Self {
        let tiles = Arc::new(Mutex::new(Vec::new()));
        tiles.lock()
            .unwrap()
            .resize(count as usize, Vec::new());
        let directory = Arc::new(directory);
        let mut thread_handles = Vec::new();

        for i in 0..count {
            let tiles = tiles.clone();
            let directory = directory.clone();

            thread_handles.push(thread::spawn(move || {
                let decoder = png::Decoder::new(
                    fs::File::open(format!("{}/{}.png", directory, i))
                        .unwrap()
                );
                let mut reader = decoder.read_info()
                    .unwrap();
                let mut buf = vec![0; reader.output_buffer_size()];

                let info = reader.next_frame(&mut buf)
                    .unwrap();
                let bytes = &buf[..info.buffer_size()];

                tiles.lock()
                    .unwrap()[i as usize] = Vec::from(bytes);
            }));
        }

        for handle in thread_handles {
            handle.join()
                .unwrap();
        }

        Tileset {
            name,
            tiles: Arc::try_unwrap(tiles)
                .unwrap()
                .into_inner()
                .unwrap(),
            tile_size
        }
    }
}
