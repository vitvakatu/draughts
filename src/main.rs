extern crate three;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

struct Piece {
    sprite: three::Sprite,
    position: [i32; 2],
    color: Color,
}

fn create_desk(window: &mut three::Window) -> Vec<three::Sprite> {
    let white_cell = window.factory.load_texture("data/sprites/white_cell.png");
    let black_cell = window.factory.load_texture("data/sprites/black_cell.png");
    let mut sprites = Vec::new();
    for i in 0..8 {
        for j in 0..8 {
            let cell = if (i + j) % 2 == 0 {
                white_cell.clone()
            } else {
                black_cell.clone()
            };
            let mut sprite = window.factory.sprite(three::material::Sprite { map: cell });
            sprite.set_scale(1.0 / 8.0);
            sprite.set_parent(&window.scene);
            sprite.set_position([-1.0 + 1.0 / 8.0 + i as f32 * 0.25, 1.0 - 1.0 / 8.0 - j as f32 * 0.25, 0.0]);
            sprites.push(sprite);
        }
    }
    sprites
}

fn place_pieces(window: &mut three::Window) -> Vec<Piece> {
    let white_piece = window.factory.load_texture("data/sprites/white_piece.png");
    let black_piece = window.factory.load_texture("data/sprites/black_piece.png");
    let mut pieces = Vec::new();
    let mut color = Color::White;
    for i in 0..8 {
        for j in 0..8 {
            if (i + j) % 2 == 0 {
                continue;
            }
            let cell = if j >= 5 {
                color = Color::White;
                white_piece.clone()
            } else if j <= 2 {
                color = Color::Black;
                black_piece.clone()
            } else {
                continue;
            };
            let mut sprite = window.factory.sprite(three::material::Sprite { map: cell });
            sprite.set_scale(1.0 / 8.5);
            sprite.set_parent(&window.scene);
            sprite.set_position([
                -1.0 + 1.0 / 8.0 + i as f32 * 0.25,
                1.0 - 1.0 / 8.0 - j as f32 * 0.25,
                0.0]);
            let piece = Piece {
                sprite,
                color,
                position: [0; 2],
            };
            pieces.push(piece);
        }
    }
    pieces
}

fn in_borders(scene: &three::Scene, aspect: f32, piece: &mut Piece, pos: [f32; 2]) -> bool {
    let info = piece.sprite.sync(scene);
    let mut piece_pos: [f32; 3] = info.world_transform.position.into();
    piece_pos[0] /= aspect;
    let piece_size: f32 = info.world_transform.scale;
    pos[0] < piece_pos[0] + piece_size &&
        pos[0] > piece_pos[0] - piece_size &&
        pos[1] > piece_pos[1] - piece_size &&
        pos[1] < piece_pos[1] + piece_size
}

fn main() {
    let mut window = three::Window::builder("Draughts").dimensions(640, 480).build();
    let camera = window.factory.orthographic_camera([0.0, 0.0], 1.0, -10.0 .. 10.0);
    window.scene.background = three::Background::Color(0xFFFFFF);
    let _desk = create_desk(&mut window);
    let mut pieces = place_pieces(&mut window);

    let hl = window.factory.load_texture("data/sprites/highlighting.png");
    let mut hl_sprite = window.factory.sprite(three::material::Sprite { map: hl });
    hl_sprite.set_parent(&window.scene);
    hl_sprite.set_scale(1.0 / 8.5);
    hl_sprite.set_visible(false);

    while window.update() {
        let mut highlight = false;
        let mut hightlight_position: [f32; 3] = [0.0; 3];
        for piece in &mut pieces {
            let pos: [f32; 2] = window.input.mouse_pos_ndc().into();
            if in_borders(&window.scene, window.renderer.get_aspect(), piece, pos) {
                highlight = true;
                hightlight_position = piece.sprite.sync(&window.scene).world_transform.position.into();
            }
        }
        if highlight {
            hl_sprite.set_visible(true);
            hl_sprite.set_position(hightlight_position);
        } else {
            hl_sprite.set_visible(false);
        }
        window.render(&camera);
    }
}
