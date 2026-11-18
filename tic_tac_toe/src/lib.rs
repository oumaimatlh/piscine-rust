pub fn tic_tac_toe(table: [[char; 3]; 3]) -> String {
    if diagonals('O', table) || vertical('O', table) || horizontal('O', table) {
        String::from("player O won")
    }else if diagonals('X', table) || vertical('X', table) || horizontal('X', table) {
        String::from("player X won")
    }else {
        String::from("tie")
    }

}

pub fn diagonals(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][1] == player && table[2][2] == player {
        return true;
    }
    if table[0][2] == player &&  table[1][1] == player && table[2][0] == player {
        return true;
    }
    false
}

pub fn horizontal(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[0][1] == player && table[0][2] == player {
        return true;
    }

    if table[1][0] == player && table[1][1] == player && table[1][2] == player {
        return true;
    }

    if table[2][0] == player && table[2][1] == player && table[2][2] == player {
        return true;
    }
    false
}

pub fn vertical(player: char, table: [[char; 3]; 3]) -> bool {
    if table[0][0] == player && table[1][0] == player && table[2][0] == player {
        return true;
    }

    if table[0][1] == player && table[1][1] == player && table[2][1] == player {
        return true;
    }

    if table[0][2] == player && table[1][2] == player && table[2][2] == player {
        return true;
    }

    false
}