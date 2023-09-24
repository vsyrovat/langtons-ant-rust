use std::{
    fs::{create_dir_all, File},
    io::BufWriter,
    path::Path,
};

mod game;

use game::{Ant, Board, Game};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let board = Board::new();
    let ant = Ant::new();
    let mut game = Game::new(board, ant);
    game.play(100000);

    create_dir_all("out")?;
    let path = Path::new("out/image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 1024, 1024);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::One);
    let mut writer = encoder.write_header().unwrap();

    // let data = Box::new([255, 1]);
    let data = game.board.as_png_data();
    writer.write_image_data(data.as_ref()).unwrap();

    println!(
        "The game finished in {} iterations with {} black cells.",
        game.age, game.black_count
    );
    println!("See out/image.png for the path image.");

    Ok(())
}
