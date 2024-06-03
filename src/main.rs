use image::{ImageBuffer, Rgb};
use rand::Rng;
use array2d;

const SQUAREX: u32 = 8;
const SQUAREY: u32 = 8;

const SQUARE_SIZE: u8 = 5;
const OFFSET: u8 = SQUARE_SIZE - 1;

const BRANCHES: u32 = 1;
const BRANCH_LENGTH: u32 = 1;

const COLOR_WHITE: Rgb<u8> = image::Rgb([240, 240, 240]);
const COLOR_BACKGROUND: Rgb<u8> = image::Rgb([0, 0, 0]);

fn main() 
{
    let imgx: u32 = 800;
    let imgy: u32 = 600;

    let mut imgbuf: ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(imgx,imgy);

    // fill background with color
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = COLOR_BACKGROUND;
    }

    // create two dimensionl array of information about each square
    let mut square_vec: Vec<Vec<Square>> = Vec::new();

    for n in 0..SQUAREX
    {
        let mut inner_vec: Vec<Square> = Vec::new();
        for m in 0..SQUAREY
        {
            inner_vec.push(Square
                {
                    x: n,
                    y: m,
                    visible: false,
                    up: false,
                    down: false,
                    left: false,
                    right: false
                });
        }
        print!(" ");
        square_vec.push(inner_vec);
    }

    // labyrinth
    for n in 0..BRANCHES
    {
        let rng_x: f64 = rand::thread_rng().gen();
        let rng_y: f64 = rand::thread_rng().gen();

        let starting_point_x = (rng_x * SQUAREX as f64) as usize;
        let starting_point_y = (rng_y * SQUAREY as f64) as usize;

        let mut vec = square_vec.get_mut(starting_point_x).unwrap();
        let mut sqr = vec.get_mut(starting_point_y).unwrap();
        

        sqr.visible = true;

        for m in 0..BRANCH_LENGTH
        {
            let rng: f64 = rand::thread_rng().gen();

            if rng < 0.25 && sqr.x > 0
            {
                sqr.up = true;
                
                vec = square_vec.get_mut((sqr.x - 1) as usize).unwrap();
                sqr = vec.get_mut(starting_point_y).unwrap();
                
            }
            else if rng < 0.5 && sqr.x < SQUAREX - 1
            {
                sqr.down = true;
            }
            else if rng < 0.75 && sqr.y > 0
            {
                sqr.left = true;
            }
            else if sqr.y < SQUAREY - 1
            {
                sqr.right = true;
            }
        }
    }

    // draws the ones that are visible
    for n in square_vec 
    {
        for m in n
        {
            if m.visible
            {
                draw_square(&m, &mut imgbuf);
            }
        }        
    }

    //saves file
    imgbuf.save("images/image.png").unwrap();
}

/// takes x and y values of the location of the square
fn draw_square(sqr: & Square, _imgbuf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>)
{
    //find coordinate
    let base_x: u32 = sqr.x * (SQUARE_SIZE + 1) as u32;
    let base_y: u32 = sqr.y * (SQUARE_SIZE + 1) as u32;

    // draws a square
    for n in 0..5
    {
        _imgbuf.put_pixel(base_x, base_y + n, COLOR_WHITE);
        _imgbuf.put_pixel(base_x + OFFSET as u32, base_y + n, COLOR_WHITE);
        
        _imgbuf.put_pixel(base_x + n, base_y, COLOR_WHITE);
        _imgbuf.put_pixel(base_x + n, base_y + OFFSET as u32, COLOR_WHITE);
    }

    // remove walls
    if sqr.up 
    {
        _imgbuf.put_pixel(base_x + (OFFSET / 2) as u32, base_y, COLOR_BACKGROUND);
    }
    if sqr.down 
    {
        _imgbuf.put_pixel(base_x + (OFFSET / 2) as u32, base_y + OFFSET as u32, COLOR_BACKGROUND);
    }
    if sqr.left 
    {
        _imgbuf.put_pixel(base_x, base_y + (OFFSET / 2) as u32, COLOR_BACKGROUND);
    }
    if sqr.right
    {
        _imgbuf.put_pixel(base_x + 4, base_y + (OFFSET / 2) as u32, COLOR_BACKGROUND);
    }

}

struct Square
{
    x: u32,
    y: u32,
    visible: bool,
    up: bool,
    down: bool,
    left: bool,
    right: bool
}