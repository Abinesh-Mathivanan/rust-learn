use std::{thread, time::Duration};

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    let heart = vec![
        "   ****     ****   ",
        " **    ** **    ** ",
        "**       **       **",
        "**               **",
        " **             ** ",
        "  **           **  ",
        "    **       **    ",
        "      **   **      ",
        "        ***        ",
        "",
        "    I LOVE YOU!    ",
        "  ❤️  ❤️  ❤️  ❤️  ❤️   "
    ];

    loop {
        for i in 0..3 {
            clear_screen();
            for line in &heart {
                println!("{}{}", " ".repeat(i * 2), line);
            }
            thread::sleep(Duration::from_millis(300));
        }
        
        for i in (0..3).rev() {
            clear_screen();
            for line in &heart {
                println!("{}{}", " ".repeat(i * 2), line);
            }
            thread::sleep(Duration::from_millis(300));
        }
    }
}