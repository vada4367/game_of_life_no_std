#![allow(dead_code)]

extern crate libc;
use libc::{c_char, c_int, c_uint, c_void, free, malloc, time};

/*
 * If you system is linux, change "openbsd" to "linux"
 */
#[cfg(target_os = "linux")]
#[link(name = "c")]

extern "C" {
    fn srand(seed: c_uint);
    fn rand() -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

// For "Vec" and "Box"

extern crate alloc;
use core::alloc::{GlobalAlloc, Layout};

#[derive(Default)]
pub struct Allocator;
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size() as usize) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr as *mut c_void);
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

use alloc::boxed::Box;
use alloc::vec;


#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Life,
    Death,
}

#[derive(Clone)]
pub struct Canvas {
    pub canvas: Box<[Cell]>,
    pub dx: usize,
    pub dy: usize,
}

impl Canvas {
    pub fn rules(&self, i: i32, j: i32) -> Cell {
        let mut nb: i32 = 0;

        // Counting neightbours
        for ii in 0..3i32 {
            for jj in 0..3i32 {
                let mut x = i + ii - 1;
                let mut y = j + jj - 1;

                if x < 0 {
                    x = self.dx as i32 - 1;
                }
                if x >= self.dx as i32 {
                    x = 0;
                }
                if y < 0 {
                    y = self.dy as i32 - 1;
                }
                if y >= self.dy as i32 {
                    y = 0;
                }

                match self.canvas[(x + y * self.dx as i32) as usize] {
                    Cell::Life => nb += 1,
                    Cell::Death => {}
                }
            }
        }

        // Rules of conway's game
        let i_and_j: usize = (i + j * self.dx as i32) as usize;

        if self.canvas[i_and_j] == Cell::Life {
            nb -= 1;
        }
        if self.canvas[i_and_j] == Cell::Death && nb == 3 {
            return Cell::Life;
        }
        if self.canvas[i_and_j] == Cell::Life && (nb == 2 || nb == 3) {
            return Cell::Life;
        }

        Cell::Death
    }

    pub fn new(_dx: usize, _dy: usize) -> Self {
        Self {
            canvas: vec![Cell::Death; _dx * _dy].into_boxed_slice(),
            dx: _dx,
            dy: _dy,
        }
    }

    pub fn print_screen(&self) {
        unsafe {
            printf("\x1b[H\0".as_ptr() as *const i8);
        }

        for i in 0..self.dx * self.dy {
            if i % self.dx == 0 {
                unsafe {
                    printf("\n\0".as_ptr() as *const i8);
                }
            }
            match self.canvas[i] {
                Cell::Life => unsafe {
                    printf("#\0".as_ptr() as *const i8);
                },
                Cell::Death => unsafe {
                    printf(" \0".as_ptr() as *const i8);
                },
            }
        }
    }

    pub fn gosper_gun() -> Self {
        /*
         * 36 and 9 is not magic numbers,
         * gosper gun have size 36x9
         */

        let sc_w = 36;
        let sc_h = 9;
        let mut canvas = Self::new(sc_w, sc_h);

        canvas.canvas[0 + 4 * canvas.dx] = Cell::Life;
        canvas.canvas[0 + 5 * canvas.dx] = Cell::Life;
        canvas.canvas[1 + 4 * sc_w] = Cell::Life;
        canvas.canvas[1 + 5 * sc_w] = Cell::Life;
        canvas.canvas[10 + 4 * sc_w] = Cell::Life;
        canvas.canvas[10 + 5 * sc_w] = Cell::Life;
        canvas.canvas[10 + 6 * sc_w] = Cell::Life;
        canvas.canvas[11 + 3 * sc_w] = Cell::Life;
        canvas.canvas[11 + 7 * sc_w] = Cell::Life;
        canvas.canvas[12 + 2 * sc_w] = Cell::Life;
        canvas.canvas[12 + 8 * sc_w] = Cell::Life;
        canvas.canvas[13 + 2 * sc_w] = Cell::Life;
        canvas.canvas[13 + 8 * sc_w] = Cell::Life;
        canvas.canvas[14 + 5 * sc_w] = Cell::Life;
        canvas.canvas[15 + 3 * sc_w] = Cell::Life;
        canvas.canvas[15 + 7 * sc_w] = Cell::Life;
        canvas.canvas[16 + 4 * sc_w] = Cell::Life;
        canvas.canvas[16 + 5 * sc_w] = Cell::Life;
        canvas.canvas[16 + 6 * sc_w] = Cell::Life;
        canvas.canvas[17 + 5 * sc_w] = Cell::Life;
        canvas.canvas[20 + 2 * sc_w] = Cell::Life;
        canvas.canvas[20 + 3 * sc_w] = Cell::Life;
        canvas.canvas[20 + 4 * sc_w] = Cell::Life;
        canvas.canvas[21 + 2 * sc_w] = Cell::Life;
        canvas.canvas[21 + 3 * sc_w] = Cell::Life;
        canvas.canvas[21 + 4 * sc_w] = Cell::Life;
        canvas.canvas[22 + 1 * sc_w] = Cell::Life;
        canvas.canvas[22 + 5 * sc_w] = Cell::Life;
        canvas.canvas[24 + 0 * sc_w] = Cell::Life;
        canvas.canvas[24 + 1 * sc_w] = Cell::Life;
        canvas.canvas[24 + 5 * sc_w] = Cell::Life;
        canvas.canvas[24 + 6 * sc_w] = Cell::Life;
        canvas.canvas[34 + 2 * sc_w] = Cell::Life;
        canvas.canvas[34 + 3 * sc_w] = Cell::Life;
        canvas.canvas[35 + 3 * sc_w] = Cell::Life;
        canvas.canvas[35 + 2 * sc_w] = Cell::Life;

        canvas
    }

    pub fn rand_canvas(dx: usize, dy: usize, chance: f32) -> Self {
        let mut canvas = Self::new(dx, dy);
        unsafe {
            srand(time(core::ptr::null_mut()).try_into().unwrap());
        }

        for i in 0..canvas.dx {
            for j in 0..canvas.dy {
                if unsafe { rand() } % (1. / chance) as i32 == 0 {
                    canvas.canvas[i + j * canvas.dx] = Cell::Life;
                }
            }
        }

        canvas
    }

    pub fn canvas_to_canvas(mut self, canvas: Self, x: usize, y: usize) -> Self {
        for i in x..x + canvas.dx {
            for j in y..y + canvas.dy {
                self.canvas[i + j * self.dx] = canvas.canvas[(i - x) + (j - y) * canvas.dx];
            }
        }

        self
    }
}

// I did not release it to Canvas

/*
fn xmirror_canvas(canvas: Box<[Cell]>, dx: usize, dy: usize) -> Box<[Cell]> {
    let mut result_canvas = free_canvas(dx, dy);

    for i in 0..dx {
        for j in 0..dy {
            result_canvas[i + j * dx] = canvas[dx - i - 1 + j * dx];
        }
    }

    result_canvas
}

fn ymirror_canvas(canvas: Box<[Cell]>, dx: usize, dy: usize) -> Box<[Cell]> {
    let mut result_canvas = free_canvas(dx, dy);

    for i in 0..dx {
        for j in 0..dy {
            result_canvas[i + j * dx] = canvas[i + (dy - j - 1) * dx];
        }
    }

    result_canvas
}
*/

