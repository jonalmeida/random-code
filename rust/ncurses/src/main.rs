extern crate ncurses;

use ncurses::*;
/*
pub use std::char;

fn main() {
    initscr();
    raw();

    keypad(stdscr, true);
    noecho();

    printw("Enter a character: ");

    let ch = getch();
    if ch == KEY_F1
    {
        attron(A_BOLD() | A_BLINK());
        printw("\nF1");
        attroff(A_BOLD() | A_BLINK());
        printw(" pressed");
    } else  {
        printw("\nKey pressed: ");
        attron(A_BOLD() | A_BLINK());
        //let this_shiz = format!("{}\n", char::from_u32(ch as u32).unwrap()).as_slice();
        printw("Something else pressed");
        attroff(A_BOLD() | A_BLINK());
    }

    refresh();

    getch();
    endwin();
}
*/

use std::io;
use std::fs::File;
use std::os;
use std::path::Path;


fn open_file() -> File
{
    let args = os::args();
    if args.len() != 2
    {
        println!("Usage:\n\t{} <rust -file>", args[0]);
        println!("Usage:\n\t{} examples/ex_3.rs", args[0]);
        panic!("Exiting");
    }

    let reader = File::open(&Path::new(args[1].to_string()));
    reader.ok().expect("Unable to open file");
}

fn prompt()
{
    printw("<-Press Any Key->");
    getch();
}

fn main()
{
    let mut reader = open_file();

    initscr();
    keypad(stdscr, true);
    noecho();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr, &mut max_y, &mut max_x);

    while !reader.eof()
    {
        let ch = reader.read_byte();
        if ch.is_err()
        {
            break;
        }
        let ch = ch.unwrap();

        let mut cur_x = 0;
        let mut cur_y = 0;
        getyx(stdscr, &mut cur_y, &mut cur_x);

        if cur_y == (max_y - 1)
        {
            prompt();

            clear();
            mv(0,0);
        }
        else
        {
            addch(ch as chtype);
        }
    }

    mv(max_y - 1, 0);
    prompt();
    endwin();

}
