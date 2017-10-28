#[macro_use]
extern crate euler;
extern crate ndarray;
#[macro_use]
extern crate three;

const WIDTH: usize = 8;
const HEIGHT: usize = 8;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Side {
    Player,
    Enemy,
}

#[derive(Clone, Debug)]
struct Piece {
    sprite: three::Sprite,
    position: euler::Vec2,
    color: Color,
    hovered: bool,
    selected: bool,
    side: Side,
}

impl Piece {
    fn new(
        factory: &mut three::Factory,
        color: Color,
    ) -> Self {
        let (sprite_path, side) = match color {
            Color::White => ("data/sprites/white_piece.png", Side::Player),
            Color::Black => ("data/sprites/black_piece.png", Side::Enemy),
        };
        Self {
            sprite: load_sprite(factory, sprite_path),
            position: vec2!(0, 0),
            color,
            hovered: false,
            selected: false,
            side,
        }
    }
}

#[derive(Clone, Debug)]
struct Cell {
    sprite: three::Sprite,
}

impl Cell {
    fn new(
        factory: &mut three::Factory,
        color: Color,
    ) -> Self {
        let sprite_path = match color {
            Color::White => "data/sprites/white_cell.png",
            Color::Black => "data/sprites/black_cell.png",
        };
        Self {
            sprite: load_sprite(factory, sprite_path),
        }
    }
}

three_object_wrapper!(Piece::sprite, Cell::sprite);

#[inline]
fn load_sprite(
    factory: &mut three::Factory,
    path: &str,
) -> three::Sprite {
    let texture = factory.load_texture(path);
    factory.sprite(three::material::Sprite { map: texture })
}

struct Board {
    cells: ndarray::Array2<Cell>,
    pieces: ndarray::Array2<Option<Piece>>,
}

#[inline]
fn to_world(
    x: usize,
    y: usize,
) -> (f32, f32) {
    let x_world = -1.0 + x as f32 * 2.0 / WIDTH as f32;
    let y_world = 1.0 - y as f32 * 2.0 / HEIGHT as f32;
    (x_world, y_world)
}

impl Board {
    fn new(
        factory: &mut three::Factory,
        scene: &three::Scene,
    ) -> Self {
        let mut cells = ndarray::Array2::from_shape_fn((WIDTH, HEIGHT), |(i, j)| {
            let color = if (i + j) % 2 == 0 {
                Color::White
            } else {
                Color::Black
            };
            let mut cell = Cell::new(factory, color);
            let size = 1.0 / HEIGHT as f32;
            cell.set_scale(size);
            let (mut x, mut y) = to_world(i, j);
            x += size;
            y -= size;
            cell.set_position(vec3!(x, y, 0.0));
            cell.set_parent(&scene);
            cell
        });
        let mut pieces = ndarray::Array2::from_shape_fn((WIDTH, HEIGHT), |(i, j)| {
            let color = if j < 3 {
                Color::Black
            } else if j >= 5 {
                Color::White
            } else {
                return None;
            };
            if (i + j) % 2 == 0 {
                return None;
            }
            let mut piece = Piece::new(factory, color);
            let size = 1.0 / HEIGHT as f32;
            piece.set_scale(size * 0.9);
            let (mut x, mut y) = to_world(i, j);
            x += size;
            y -= size;
            piece.set_position(vec3!(x, y, 0.0));
            piece.set_parent(&scene);
            Some(piece)
        });
        Self { cells, pieces }
    }
}

fn in_borders(
    scene: &three::Scene,
    aspect: f32,
    piece: &mut Piece,
    pos: euler::Vec2,
) -> bool {
    let info = piece.sprite.sync(scene);
    let mut piece_pos: euler::Vec3 = info.world_transform.position.into();
    piece_pos.x /= aspect;
    let piece_size: f32 = info.world_transform.scale;
    pos.x < piece_pos.x + piece_size && pos.x > piece_pos.x - piece_size && pos.y > piece_pos.y - piece_size && pos.y < piece_pos.y + piece_size
}

fn main() {
    let mut window = three::Window::builder("Draughts")
        .dimensions(640, 480)
        .build();
    let camera = window
        .factory
        .orthographic_camera([0.0, 0.0], 1.0, -10.0 .. 10.0);
    window.scene.background = three::Background::Color(0xFFFFFF);
    let mut board = Board::new(&mut window.factory, &window.scene);

    let mut hover_sprite = load_sprite(&mut window.factory, "data/sprites/highlighting.png");
    let mut select_sprite = load_sprite(&mut window.factory, "data/sprites/selection.png");

    while window.update() {
        let aspect = window.renderer.get_aspect();

        hover_sprite.set_visible(false);
        select_sprite.set_visible(false);

        // reset state
        for mut piece in &mut board.pieces {
            if let Some(ref mut piece) = *piece {
                piece.hovered = false;
                if window.input.hit(three::MOUSE_LEFT) {
                    piece.selected = false;
                }
            }
        }

        // update state
        let mouse_pos: euler::Vec2 = window.input.mouse_pos_ndc().into();
        for mut piece in &mut board.pieces {
            if let Some(ref mut piece) = *piece {
                if in_borders(&window.scene, aspect, piece, mouse_pos) && piece.side == Side::Player {
                    piece.hovered = true;
                    if window.input.hit(three::MOUSE_LEFT) {
                        piece.selected = true;
                    }
                }
            }
        }

        // render state
        for piece in &board.pieces {
            if let Some(ref piece) = *piece {
                if piece.hovered && !piece.selected {
                    hover_sprite.set_parent(piece);
                    hover_sprite.set_visible(true);
                } else if piece.selected {
                    select_sprite.set_parent(piece);
                    select_sprite.set_visible(true);
                }
            }
        }

        window.render(&camera);
    }
}
