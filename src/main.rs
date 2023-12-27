#![no_std]
#![no_main]

use libc::{ioctl, winsize, TIOCGWINSZ, usleep};

mod canvas;
use crate::canvas::Canvas;

#[no_mangle]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    // Getting terminal size
    let w: winsize = winsize {
        ws_row: 80,
        ws_col: 24,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    unsafe {
        ioctl(0, TIOCGWINSZ.try_into().unwrap(), &w);
    }

    let sc_w: usize = w.ws_col.into();
    let sc_h: usize = w.ws_row.into();

    // Create screen and make shapes on one
    let mut screen: Canvas = Canvas::new(sc_w, sc_h);
    screen = screen.canvas_to_canvas(Canvas::gosper_gun(), 3, 3);
    screen = screen.canvas_to_canvas(Canvas::rand_canvas(10, 10, 0.2), 30, 15);
    let mut next_screen: Canvas;

    loop {
        screen.print_screen();

        next_screen = Canvas::new(sc_w, sc_h);
        for i in 0..sc_w {
            for j in 0..sc_h {
                next_screen.canvas[i + j * next_screen.dx] = screen.rules(i as i32, j as i32);
            }
        }
        screen = next_screen;

        unsafe {
            usleep(20_000);
        }
    }
}

// For compiler
#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}
