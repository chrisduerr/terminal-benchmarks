#![feature(test)]
extern crate test;
use test::Bencher;
fn main() {}

const BUFFER_CHARACTER_WIDTH: usize = 185;

#[bench]
fn big_changes(b: &mut Bencher) {
    let right_chars = vec!['>' as u8; BUFFER_CHARACTER_WIDTH];
    let left_chars = vec!['<' as u8; BUFFER_CHARACTER_WIDTH];
    let right_string = String::from_utf8_lossy(&right_chars);
    let left_string = String::from_utf8_lossy(&left_chars);
    b.iter(|| {
        for i in 0..1_000 {
            if i % 2 == 0 {
                println!("{}", right_string);
            } else {
                println!("{}", left_string);
            }
        }
    });
}

#[bench]
fn small_changes(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1_000 {
            if i % 2 == 0 {
                println!(">");
            } else {
                println!("<");
            }
        }
    });
}

#[bench]
fn cursor_changes(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1_000 {
            if i % 3 == 0 {
                println!("\u{1b}[1 q");
            } else if i % 3 == 1 {
                println!("\u{1b}[3 q");
            } else {
                println!("\u{1b}[5 q");
            }
        }
    });
    println!("\u{1b}[0 q");
}

#[bench]
fn color_changes(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..1_000 {
            if i % 2 == 0 {
                println!("\u{1b}[0;41;34mTEST\u{1b}[0;00m");
            } else {
                println!("\u{1b}[0;46;35mTEST\u{1b}[0;00m");
            }
        }
    });
}

#[bench]
fn all_colors(b: &mut Bencher) {
    let bg_iter: Vec<u8> = (40..47).chain(100..107).collect();
    let fg_iter: Vec<u8> = (30..37).chain(90..97).collect();
    b.iter(|| {
        for i in 0..10 {
            for bg in &bg_iter {
                for fg in &fg_iter {
                    println!("\u{1b}[0;{};{}mTEST\u{1b}[0;00m", fg, bg);
                }
            }
        }
    });
}

#[bench]
fn small_unicode(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..u16::max_value() {
            let vec = vec![i];
            println!("{}", String::from_utf16_lossy(&vec));
        }
    });
}

#[bench]
fn big_unicode(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..u16::max_value() {
            let chars = vec![i; BUFFER_CHARACTER_WIDTH];
            println!("{}", String::from_utf16_lossy(&chars));
        }
    });
}
