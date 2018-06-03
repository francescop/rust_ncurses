extern crate ncurses;

use std::char;
use ncurses::*;

fn main() {	
	initscr();			/* Start curses mode 		*/
	raw();			/* Line buffering disabled, Pass on
					 * everty thing to me 		*/
	keypad(stdscr(), true);		/* I need that nifty F1 	*/
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

	let height = 3;
	let width = 10;
	let mut starty = (max_y - height) / 2;	/* Calculating for a center placement */
	let mut startx = (max_x - width) / 2;	/* of the window		*/
	printw("Press F1 or q to exit");
	refresh();

	let mut my_win = create_newwin(height, width, startx, starty);

    let mut ch = getch();

	while ch != 0x71 && ch != KEY_F(1){	

        update_stats((max_x - width), (max_y - height), startx, starty, ch);

        match ch {
            KEY_LEFT => {
                if startx > 0 { startx -= 1; }
            },
            KEY_RIGHT => {
                if startx < (max_x - width) { startx += 1; }
            },
            KEY_UP => {
                if starty > 1 { starty -= 1; }
            },
            KEY_DOWN => {
                if starty < (max_y - height) { starty += 1; }
            },
            _ => { }
        }

        destroy_win(my_win);
        my_win = create_newwin(height, width, startx, starty);
        ch = getch();
	}

	endwin();			/* End curses mode		  */
}

fn create_newwin(height: i32, width: i32, startx: i32, starty: i32) -> WINDOW {	
	let local_win = newwin(height, width, starty, startx);
	box_(local_win, 0, 0);		/* 0, 0 gives default characters 
					 * for the vertical and horizontal
					 * lines			*/
	wrefresh(local_win);		/* Show that box 		*/

	local_win
}

fn destroy_win(win: WINDOW) {	
	/* box(local_win, ' ', ' '); : This won't produce the desired
	 * result of erasing the window. It will leave it's four corners 
	 * and so an ugly remnant of window. 
	 */
    let ch = ' ' as chtype;
	wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
	/* The parameters taken are 
	 * 1. win: the window on which to operate
	 * 2. ls: character to be used for the left side of the window 
	 * 3. rs: character to be used for the right side of the window 
	 * 4. ts: character to be used for the top side of the window 
	 * 5. bs: character to be used for the bottom side of the window 
	 * 6. tl: character to be used for the top left corner of the window 
	 * 7. tr: character to be used for the top right corner of the window 
	 * 8. bl: character to be used for the bottom left corner of the window 
	 * 9. br: character to be used for the bottom right corner of the window
	 */
	wrefresh(win);
	delwin(win);
}

fn update_stats(startx: i32, starty: i32, game_win_x: i32, game_win_y: i32, ch: i32){
    //let coord = format!("max_x {:?} \n max_y {:?} \n start_x {:?} \n start_y {:?}\n", max_x, max_y, startx, starty);
    let coord = format!("x: {:?}\n y: {:?}\n ch: {}", game_win_x, game_win_y, ch);
	let mut win = create_newwin(10, 20, startx, starty);
    wprintw(win, &coord);

	wrefresh(win);
}
