use draco_rs::{Encoder, Mesh, PointCloud};
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Create a simple mesh (or load one from a file)
    let mut mesh = Mesh::new();

    // Add vertices to the mesh
    mesh.add_vertex([0.0, 0.0, 0.0]);
    mesh.add_vertex([1.0, 0.0, 0.0]);
    mesh.add_vertex([0.0, 1.0, 0.0]);

    // Add a face (triangle) to the mesh
    mesh.add_face([0, 1, 2]);

    // Create an encoder
    let mut encoder = Encoder::new();

    // Set encoding options (e.g., compression level)
    encoder.set_compression_level(10);

    // Encode the mesh
    let encoded_data = encoder
        .encode_mesh_to_buffer(&mesh)
        .expect("Failed to encode mesh");

    // Write the encoded data to a file
    let mut file = File::create("output.drc")?;
    file.write_all(&encoded_data)?;

    println!("Mesh successfully encoded and saved to 'output.drc'");

    Ok(())
}
