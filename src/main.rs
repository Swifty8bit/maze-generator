use image::{ImageBuffer, Rgb};
use rand::Rng;

// Variables to change

const SQUAREX: u32 = 30;
const SQUAREY: u32 = 30;

const BRANCHES: u32 = 10;
const BRANCH_LENGTH: u32 = 100;

// Variables to not change

const SQUARE_SIZE: u8 = 5;
const OFFSET: u8 = SQUARE_SIZE - 1;

const COLOR_WHITE: Rgb<u8> = image::Rgb([240, 240, 240]);
const COLOR_BACKGROUND: Rgb<u8> = image::Rgb([0, 0, 0]);

fn main() 
{
    let imgx: u32 = 180;
    let imgy: u32 = 180;

    let mut imgbuf: ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(imgx,imgy);

    // fill background with color
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = COLOR_BACKGROUND;
    }

    // create two dimensionl array of information about each square
    let tempsq = Square
    {
        x: 0,
        y: 0,
        visible: false,
        up: false,
        down: false,
        left: false,
        right: false
    };

    let mut square_arr: [[Square; SQUAREY as usize]; SQUAREX as usize] = 
    [[tempsq; SQUAREY as usize]; SQUAREX as usize];

    for n in 0..SQUAREX
    {
        for m in 0..SQUAREY
        {
            square_arr[n as usize][m as usize] = Square
                {
                    x: n,
                    y: m,
                    visible: false,
                    up: false,
                    down: false,
                    left: false,
                    right: false
                };
        }
    }

    // labyrinth
    for n in 0..BRANCHES
    {
        let rng_x: f64 = rand::thread_rng().gen();
        let rng_y: f64 = rand::thread_rng().gen();

        let starting_point_x = (rng_x * SQUAREX as f64) as usize;
        let starting_point_y = (rng_y * SQUAREY as f64) as usize;
        
        let mut sqr = &mut square_arr[starting_point_x][starting_point_y];

        sqr.visible = true;

        for m in 0..BRANCH_LENGTH
        {
            let rng: f64 = rand::thread_rng().gen();

            if rng < 0.25 && sqr.x > 0
            {
                sqr.up = true;
                sqr = &mut square_arr[(sqr.x - 1) as usize][sqr.y as usize];
                sqr.down = true;
            }
            else if rng < 0.5 && sqr.x < SQUAREX - 1
            {
                sqr.down = true;
                sqr = &mut square_arr[(sqr.x + 1) as usize][sqr.y as usize];
                sqr.up = true;
            }
            else if rng < 0.75 && sqr.y > 0
            {
                sqr.left = true;
                sqr = &mut square_arr[sqr.x as usize][(sqr.y - 1)as usize];
                sqr.right = true;
            }
            else if sqr.y < SQUAREY - 1
            {
                sqr.right = true;
                sqr = &mut square_arr[sqr.x as usize][(sqr.y + 1)as usize];
                sqr.left = true;
            }

            sqr.visible = true;
        }
    }

    // draws the ones that are visible
    for n in square_arr.iter_mut() 
    {
        for m in n
        {
            if m.visible
            {
                draw_square(&m, &mut imgbuf);
            }
            // print!("{} ", m.visible);
        }
        // println!("");
    }

    //saves file
    imgbuf.save("images/image.png").unwrap();
}

/// takes x and y values of the location of the square
fn draw_square(sqr: & Square, _imgbuf: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>)
{
    //find coordinate
    let base_x: u32 = sqr.y * (SQUARE_SIZE + 1) as u32;
    let base_y: u32 = sqr.x * (SQUARE_SIZE + 1) as u32;

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

#[derive(Clone, Copy)]
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