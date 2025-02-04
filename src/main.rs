use crossterm::{
    cursor::{Hide, Show},
    event::{poll, read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType::Purge, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use rand::Rng;
use std::{
    io::{stdout, Write},
    time::{Duration, Instant},
};

const PIECEDATA: [((i32, i32), (i32, i32), (i32, i32), (i32, i32)); 28] = [
    ((-1, 0), (0, 0), (1, 0), (2, 0)),   // I-0
    ((-1, 0), (0, 0), (1, 0), (0, 1)),   // T-0
    ((-1, 0), (0, 0), (0, -1), (1, -1)), // S-0
    ((0, 0), (1, 0), (0, 1), (1, 1)),    // O-0
    ((0, 0), (0, -1), (-1, 1), (0, 1)),  // J-0
    ((0, -1), (0, 0), (0, 1), (1, 1)),   // L-0
    ((-1, 0), (0, 0), (0, 1), (1, 1)),   // Z-0
    ((0, -1), (0, 0), (0, 1), (0, 2)),   // I-1
    ((-1, 0), (0, 0), (0, 1), (0, -1)),  // T-1
    ((1, 1), (1, 0), (0, 0), (0, -1)),   // S-1
    ((0, 0), (1, 0), (0, 1), (1, 1)),    // O-1
    ((0, 0), (-1, 0), (-1, -1), (1, 0)), // J-1
    ((0, 0), (-1, 0), (-1, 1), (1, 0)),  // L-1
    ((0, 1), (0, 0), (1, 0), (1, -1)),   // Z-1
    ((-1, 0), (0, 0), (1, 0), (2, 0)),   // I-2
    ((-1, 0), (0, 0), (1, 0), (0, -1)),  // T-2
    ((-1, 1), (0, 1), (0, 0), (1, 0)),   // S-2
    ((0, 0), (1, 0), (0, 1), (1, 1)),    // O-2
    ((0, 0), (0, 1), (0, -1), (1, -1)),  // J-2
    ((0, 0), (0, 1), (0, -1), (-1, -1)), // L-2
    ((-1, 0), (0, 0), (0, 1), (1, 1)),   // Z-2
    ((0, -1), (0, 0), (0, 1), (0, 2)),   // I-3
    ((1, 0), (0, 0), (0, 1), (0, -1)),   // T-3
    ((0, 1), (0, 0), (-1, 0), (-1, -1)), // S-3
    ((0, 0), (1, 0), (0, 1), (1, 1)),    // O-3
    ((0, 0), (-1, 0), (1, 0), (1, 1)),   // J-3
    ((0, 0), (1, -1), (-1, 0), (1, 0)),  // L-3
    ((0, 1), (0, 0), (1, 0), (1, -1)),   // Z-3
];
const HPIECEDATA: [((i32, i32), (i32, i32), (i32, i32), (i32, i32)); 7] = [
    ((-4, 0), (-2, 0), (0, 0), (2, 0)),  // I-0
    ((-3, 0), (-1, 0), (1, 0), (-1, 1)), // T-0
    ((-3, 1), (-1, 1), (-1, 0), (1, 0)), // S-0
    ((-2, 0), (0, 0), (-2, 1), (0, 1)),  // O-0
    ((0, 0), (0, -1), (-2, 1), (0, 1)),  // J-0
    ((-2, -1), (-2, 0), (-2, 1), (0, 1)),// L-0
    ((-3, 0), (-1, 0), (-1, 1), (1, 1)), // Z-0
];

fn render(
    tsize: (i32, i32),
    pieces: &Vec<(i32, i32, usize)>,
    cdata: (i32, i32, usize, usize),
    next: usize,
    score: i32,
    linecount: i32,
    hold: usize,
) {
    let mut cxraydata = cdata.clone();
    'b: loop {
        for px in [
            PIECEDATA[cdata.2 + cdata.3 * 7].0,
            PIECEDATA[cdata.2 + cdata.3 * 7].1,
            PIECEDATA[cdata.2 + cdata.3 * 7].2,
            PIECEDATA[cdata.2 + cdata.3 * 7].3,
        ] {
            if px.1 + cxraydata.1 >= 17
                || pieces.contains(&(px.0 + cdata.0, px.1 + cxraydata.1 + 1, 0))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cxraydata.1 + 1, 1))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cxraydata.1 + 1, 2))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cxraydata.1 + 1, 3))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cxraydata.1 + 1, 4))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cxraydata.1 + 1, 5))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cxraydata.1 + 1, 6))
            {
                break 'b;
            }
        }
        cxraydata = (cdata.0, cxraydata.1 + 1, cdata.2, cdata.3);
    }
    execute!(stdout(), Clear(Purge)).unwrap();
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1mTetrominal\x1b[0m",
        tsize.1 / 2 - 12,
        tsize.0 / 2 - 5
    );

    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255m╭. . . . . . . . . . ╮\x1b[0m",
        tsize.1 / 2 - 10,
        tsize.0 / 2 - 11,
    );
    for i in 0..18 {
        print!(
            "\x1b[{};{}H\x1b[38;2;255;255;255m│{}│\x1b[0m",
            tsize.1 / 2 - 9 + i,
            tsize.0 / 2 - 11,
            ". ".repeat(10)
        );
    }
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255m╰{}╯\x1b[0m",
        tsize.1 / 2 + 9,
        tsize.0 / 2 - 11,
        "──".repeat(10)
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╭─┤Level├─╮\x1b[0m",
        tsize.1 / 2 - 12,
        tsize.0 / 2 + 20
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│   {:03}   │\x1b[0m",
        tsize.1 / 2 - 11,
        tsize.0 / 2 + 20,
        (linecount as f32 / 10.0).floor() as usize + 1
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╰─────────╯\x1b[0m",
        tsize.1 / 2 - 10,
        tsize.0 / 2 + 20
    );

    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╭─┤Score├─╮\x1b[0m",
        tsize.1 / 2 - 8,
        tsize.0 / 2 + 20
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│ {:07} │\x1b[0m",
        tsize.1 / 2 - 7,
        tsize.0 / 2 + 20,
        score,
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╰─────────╯\x1b[0m",
        tsize.1 / 2 - 6,
        tsize.0 / 2 + 20
    );

    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╭─┤Lines├─╮\x1b[0m",
        tsize.1 / 2 - 4,
        tsize.0 / 2 + 20
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│  {:05}  │\x1b[0m",
        tsize.1 / 2 - 3,
        tsize.0 / 2 + 20,
        linecount,
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╰─────────╯\x1b[0m",
        tsize.1 / 2 - 2,
        tsize.0 / 2 + 20
    );

    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╭─┤Next├─╮\x1b[0m",
        tsize.1 / 2,
        tsize.0 / 2 + 20
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│. . . . │\x1b[0m",
        tsize.1 / 2 + 1,
        tsize.0 / 2 + 20
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│. . . . │\x1b[0m",
        tsize.1 / 2 + 2,
        tsize.0 / 2 + 20
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│. . . . │\x1b[0m",
        tsize.1 / 2 + 3,
        tsize.0 / 2 + 20
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│. . . . │\x1b[0m",
        tsize.1 / 2 + 4,
        tsize.0 / 2 + 20
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╰────────╯\x1b[0m",
        tsize.1 / 2 + 5,
        tsize.0 / 2 + 20
    );

    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╭─┤Hold├─╮\x1b[0m",
        tsize.1 / 2 - 7,
        tsize.0 / 2 - 30
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│. . . . │\x1b[0m",
        tsize.1 / 2 - 6,
        tsize.0 / 2 - 30
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│. . . . │\x1b[0m",
        tsize.1 / 2 - 5,
        tsize.0 / 2 - 30
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│. . . . │\x1b[0m",
        tsize.1 / 2 - 4,
        tsize.0 / 2 - 30
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m│. . . . │\x1b[0m",
        tsize.1 / 2 - 3,
        tsize.0 / 2 - 30
    );
    print!(
        "\x1b[{};{}H\x1b[38;2;255;255;255;1m╰────────╯\x1b[0m",
        tsize.1 / 2 - 2,
        tsize.0 / 2 - 30
    );

    let colours = [
        "\x1b[38;2;50;250;255m", // Cyan   (I)
        "\x1b[38;2;255;60;255m", // Purple (T)
        "\x1b[38;2;80;255;80m",  // Green  (S)
        "\x1b[38;2;255;255;61m", // Yellow (O)
        "\x1b[38;2;60;60;255m",  // Blue   (J)
        "\x1b[38;2;255;150;61m", // Orange (L)
        "\x1b[38;2;255;60;60m",  // Red    (Z)
    ];

    print!(
        "\x1b[{};{}H{}██\x1b[0m",
        tsize.1 as i32 / 2 + 2 + HPIECEDATA[next].0 .1,
        tsize.0 as i32 / 2 + 25 + HPIECEDATA[next].0 .0,
        colours[next]
    );
    print!(
        "\x1b[{};{}H{}██\x1b[0m",
        tsize.1 as i32 / 2 + 2 + HPIECEDATA[next].1 .1,
        tsize.0 as i32 / 2 + 25 + HPIECEDATA[next].1 .0,
        colours[next]
    );
    print!(
        "\x1b[{};{}H{}██\x1b[0m",
        tsize.1 as i32 / 2 + 2 + HPIECEDATA[next].2 .1,
        tsize.0 as i32 / 2 + 25 + HPIECEDATA[next].2 .0,
        colours[next]
    );
    print!(
        "\x1b[{};{}H{}██\x1b[0m",
        tsize.1 as i32 / 2 + 2 + HPIECEDATA[next].3 .1,
        tsize.0 as i32 / 2 + 25 + HPIECEDATA[next].3 .0,
        colours[next]
    );

    if hold != 7 {
        print!(
            "\x1b[{};{}H{}██\x1b[0m",
            tsize.1 as i32 / 2 - 5 + HPIECEDATA[hold].0 .1,
            tsize.0 as i32 / 2 - 25 + HPIECEDATA[hold].0 .0,
            colours[hold]
        );
        print!(
            "\x1b[{};{}H{}██\x1b[0m",
            tsize.1 as i32 / 2 - 5 + HPIECEDATA[hold].1 .1,
            tsize.0 as i32 / 2 - 25 + HPIECEDATA[hold].1 .0,
            colours[hold]
        );
        print!(
            "\x1b[{};{}H{}██\x1b[0m",
            tsize.1 as i32 / 2 - 5 + HPIECEDATA[hold].2 .1,
            tsize.0 as i32 / 2 - 25 + HPIECEDATA[hold].2 .0,
            colours[hold]
        );
        print!(
            "\x1b[{};{}H{}██\x1b[0m",
            tsize.1 as i32 / 2 - 5 + HPIECEDATA[hold].3 .1,
            tsize.0 as i32 / 2 - 25 + HPIECEDATA[hold].3 .0,
            colours[hold]
        );
    }

    for p in pieces {
        print!(
            "\x1b[{};{}H{}██\x1b[0m",
            tsize.1 / 2 - 9 + p.1,
            tsize.0 / 2 - 10 + p.0 * 2,
            colours[p.2]
        );
    }

    print!(
        "\x1b[{};{}H{}::\x1b[0m",
        tsize.1 as i32 / 2 - 9 + cxraydata.1 + PIECEDATA[cxraydata.2 + cxraydata.3 * 7].0 .1,
        tsize.0 as i32 / 2 - 10 + (cxraydata.0 + PIECEDATA[cxraydata.2 + cxraydata.3 * 7].0 .0) * 2,
        colours[cdata.2]
    );
    print!(
        "\x1b[{};{}H{}::\x1b[0m",
        tsize.1 as i32 / 2 - 9 + cxraydata.1 + PIECEDATA[cxraydata.2 + cxraydata.3 * 7].1 .1,
        tsize.0 as i32 / 2 - 10 + (cxraydata.0 + PIECEDATA[cxraydata.2 + cxraydata.3 * 7].1 .0) * 2,
        colours[cdata.2]
    );
    print!(
        "\x1b[{};{}H{}::\x1b[0m",
        tsize.1 as i32 / 2 - 9 + cxraydata.1 + PIECEDATA[cxraydata.2 + cxraydata.3 * 7].2 .1,
        tsize.0 as i32 / 2 - 10 + (cxraydata.0 + PIECEDATA[cxraydata.2 + cxraydata.3 * 7].2 .0) * 2,
        colours[cdata.2]
    );
    print!(
        "\x1b[{};{}H{}::\x1b[0m",
        tsize.1 as i32 / 2 - 9 + cxraydata.1 + PIECEDATA[cxraydata.2 + cxraydata.3 * 7].3 .1,
        tsize.0 as i32 / 2 - 10 + (cxraydata.0 + PIECEDATA[cxraydata.2 + cxraydata.3 * 7].3 .0) * 2,
        colours[cdata.2]
    );

    print!(
        "\x1b[{};{}H{}██\x1b[0m",
        tsize.1 as i32 / 2 - 9 + cdata.1 + PIECEDATA[cdata.2 + cdata.3 * 7].0 .1,
        tsize.0 as i32 / 2 - 10 + (cdata.0 + PIECEDATA[cdata.2 + cdata.3 * 7].0 .0) * 2,
        colours[cdata.2]
    );
    print!(
        "\x1b[{};{}H{}██\x1b[0m",
        tsize.1 as i32 / 2 - 9 + cdata.1 + PIECEDATA[cdata.2 + cdata.3 * 7].1 .1,
        tsize.0 as i32 / 2 - 10 + (cdata.0 + PIECEDATA[cdata.2 + cdata.3 * 7].1 .0) * 2,
        colours[cdata.2]
    );
    print!(
        "\x1b[{};{}H{}██\x1b[0m",
        tsize.1 as i32 / 2 - 9 + cdata.1 + PIECEDATA[cdata.2 + cdata.3 * 7].2 .1,
        tsize.0 as i32 / 2 - 10 + (cdata.0 + PIECEDATA[cdata.2 + cdata.3 * 7].2 .0) * 2,
        colours[cdata.2]
    );
    print!(
        "\x1b[{};{}H{}██\x1b[0m",
        tsize.1 as i32 / 2 - 9 + cdata.1 + PIECEDATA[cdata.2 + cdata.3 * 7].3 .1,
        tsize.0 as i32 / 2 - 10 + (cdata.0 + PIECEDATA[cdata.2 + cdata.3 * 7].3 .0) * 2,
        colours[cdata.2]
    );

    stdout().flush().unwrap();
}

fn clearlines(
    pieces: &mut Vec<(i32, i32, usize)>,
    score: &mut i32,
    linecount: &mut i32,
    cdata: (i32, i32, usize, usize),
    next: usize,
    tsize: (i32, i32),
    hold: usize,
) {
    let mut lines = vec![vec![]; 18];
    for piece in pieces.clone() {
        lines[piece.1 as usize].push(piece);
    }
    for i in 0..18 {
        lines[i].sort_by(|a, b| a.0.cmp(&b.0));
    }
    let lastlcount = *linecount;

    for col in 0..5 {
        let mut hasslept = false;
        for (idx, line) in lines.iter().enumerate() {
            if line.len() >= 10 {
                pieces.remove(pieces.iter().position(|r| r == &line[4 - col]).unwrap());
                pieces.remove(pieces.iter().position(|r| r == &line[5 + col]).unwrap());
                if !hasslept {
                    std::thread::sleep(Duration::from_millis(100));
                    hasslept = true;
                }
                if col == 4 {
                    *linecount += 1;
                    for i in 0..pieces.len() {
                        if pieces[i].1 < idx as i32 {
                            pieces[i] = (pieces[i].0, pieces[i].1 + 1, pieces[i].2);
                        }
                    }
                }
                render(tsize, &*pieces, cdata, next, *score, *linecount, hold);
            }
        }
    }
    let scoretrack = *linecount - lastlcount;
    *score += match scoretrack {
        1 => 100 * (*linecount as f32 / 10.0 + 1.0).floor() as i32,
        2 => 300 * (*linecount as f32 / 10.0 + 1.0).floor() as i32,
        3 => 500 * (*linecount as f32 / 10.0 + 1.0).floor() as i32,
        4 => 800 * (*linecount as f32 / 10.0 + 1.0).floor() as i32,
        _ => 0,
    };
}

fn gameover(
    pieces: &mut Vec<(i32, i32, usize)>,
    cdata: &mut (i32, i32, usize, usize),
    next: &mut usize,
    linecount: &mut i32,
    score: &mut i32,
    hold: &mut usize,
    tsize: (i32, i32),
) {
    pieces.sort_by(|a, b| (a.0 + a.1 * 10).cmp(&(b.0 + b.1 * 10)));
    for _ in 0..pieces.len() {
        pieces.pop();
        std::thread::sleep(Duration::from_millis(50));
        render(tsize, pieces, *cdata, *next, *score, *linecount, *hold);
    }

    execute!(stdout(), Clear(Purge)).unwrap();

    print!(
        "\x1b[{};{}H\x1b[1mGame Over",
        tsize.1 / 2 - 1,
        tsize.0 / 2 - 5
    );
    print!(
        "\x1b[{};{}H'q' to quit, 'r' to restart",
        tsize.1 / 2,
        tsize.0 / 2 - 13
    );
    stdout().flush().unwrap();

    loop {
        if poll(Duration::from_millis(1)).unwrap() {
            let read = read().unwrap();

            if let Event::Key(KeyEvent {
                code: c,
                modifiers: _,
                kind: _,
                state: _,
            }) = read
            {
                match c {
                    KeyCode::Char('q') => {
                        disable_raw_mode().unwrap();
                        execute!(stdout(), Show, LeaveAlternateScreen).unwrap();
                        std::process::exit(0);
                    }
                    KeyCode::Char('r') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    *score = 0;
    *hold = 7;
    *pieces = vec![];
    *next = rand::rng().random_range(0..7);
    *cdata = (4, 0, rand::rng().random_range(0..7), 0);
}

fn main() {
    let mut stdout = stdout();
    let tsize = (size().unwrap().0 as i32, size().unwrap().1 as i32);
    let mut pieces = vec![];
    let mut cdata = (4, 0, rand::rng().random_range(0..7), 0);
    let mut next = rand::rng().random_range(0..7);
    let mut now = Instant::now();
    let mut bottomframe;
    let mut linecount = 0;
    let mut score = 0;
    let mut hold = 7;
    let speeds = [
        48, 43, 38, 33, 28, 23, 18, 13, 8, 6, 5, 5, 5, 4, 4, 4, 3, 3, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2,
        1,
    ];
    enable_raw_mode().unwrap();
    execute!(stdout, EnterAlternateScreen, Clear(Purge), Hide).unwrap();

    render(tsize, &pieces, cdata, next, score, linecount, hold);

    loop {
        let mut tempb = false;
        for px in [
            PIECEDATA[cdata.2 + cdata.3 * 7].0,
            PIECEDATA[cdata.2 + cdata.3 * 7].1,
            PIECEDATA[cdata.2 + cdata.3 * 7].2,
            PIECEDATA[cdata.2 + cdata.3 * 7].3,
        ] {
            if px.1 + cdata.1 >= 17
                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 0))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 1))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 2))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 3))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 4))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 5))
                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 6))
            {
                tempb = true;
                break;
            }
        }
        bottomframe = tempb;
        let _ = tempb;
        if poll(Duration::from_millis(1)).unwrap() {
            let read = read().unwrap();

            if let Event::Key(KeyEvent {
                code: c,
                modifiers: _,
                kind: _,
                state: _,
            }) = read
            {
                match c {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => {
                        let oldcdata = cdata.clone();
                        cdata = (cdata.0, cdata.1, cdata.2, (cdata.3 + 1) % 4);
                        for px in [
                            PIECEDATA[cdata.2 + cdata.3 * 7].0,
                            PIECEDATA[cdata.2 + cdata.3 * 7].1,
                            PIECEDATA[cdata.2 + cdata.3 * 7].2,
                            PIECEDATA[cdata.2 + cdata.3 * 7].3,
                        ] {
                            if px.0 + cdata.0 >= 10
                                || px.0 + cdata.0 <= -1
                                || px.1 + cdata.1 >= 18
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 0))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 1))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 2))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 3))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 4))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 5))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 6))
                            {
                                cdata = oldcdata;
                                break;
                            }
                        }
                        render(tsize, &pieces, cdata, next, score, linecount, hold);
                    }
                    KeyCode::Right | KeyCode::Char('d') => {
                        cdata = (cdata.0 + 1, cdata.1, cdata.2, cdata.3);
                        for px in [
                            PIECEDATA[cdata.2 + cdata.3 * 7].0,
                            PIECEDATA[cdata.2 + cdata.3 * 7].1,
                            PIECEDATA[cdata.2 + cdata.3 * 7].2,
                            PIECEDATA[cdata.2 + cdata.3 * 7].3,
                        ] {
                            if px.0 + cdata.0 >= 10
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 0))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 1))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 2))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 3))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 4))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 5))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 6))
                            {
                                cdata = (cdata.0 - 1, cdata.1, cdata.2, cdata.3);
                                break;
                            }
                        }
                        render(tsize, &pieces, cdata, next, score, linecount, hold);
                    }
                    KeyCode::Left | KeyCode::Char('a') => {
                        cdata = (cdata.0 - 1, cdata.1, cdata.2, cdata.3);
                        for px in [
                            PIECEDATA[cdata.2 + cdata.3 * 7].0,
                            PIECEDATA[cdata.2 + cdata.3 * 7].1,
                            PIECEDATA[cdata.2 + cdata.3 * 7].2,
                            PIECEDATA[cdata.2 + cdata.3 * 7].3,
                        ] {
                            if px.0 + cdata.0 <= -1
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 0))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 1))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 2))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 3))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 4))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 5))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 6))
                            {
                                cdata = (cdata.0 + 1, cdata.1, cdata.2, cdata.3);
                                break;
                            }
                        }
                        render(tsize, &pieces, cdata, next, score, linecount, hold);
                    }
                    KeyCode::Down | KeyCode::Char('s') => {
                        if !bottomframe {
                            cdata = (cdata.0, cdata.1 + 1, cdata.2, cdata.3);
                            for px in [
                                PIECEDATA[cdata.2 + cdata.3 * 7].0,
                                PIECEDATA[cdata.2 + cdata.3 * 7].1,
                                PIECEDATA[cdata.2 + cdata.3 * 7].2,
                                PIECEDATA[cdata.2 + cdata.3 * 7].3,
                            ] {
                                if px.1 + cdata.1 >= 17
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 0))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 1))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 2))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 3))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 4))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 5))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 6))
                                {
                                    bottomframe = true;
                                    break;
                                }
                            }
                            render(tsize, &pieces, cdata, next, score, linecount, hold);
                        } else {
                            for px in [
                                PIECEDATA[cdata.2 + cdata.3 * 7].0,
                                PIECEDATA[cdata.2 + cdata.3 * 7].1,
                                PIECEDATA[cdata.2 + cdata.3 * 7].2,
                                PIECEDATA[cdata.2 + cdata.3 * 7].3,
                            ] {
                                pieces.push((cdata.0 + px.0, cdata.1 + px.1, cdata.2));
                            }
                            cdata = (4, 0, next, 0);
                            next = rand::rng().random_range(0..7);
                            for px in [
                                PIECEDATA[cdata.2 + cdata.3 * 7].0,
                                PIECEDATA[cdata.2 + cdata.3 * 7].1,
                                PIECEDATA[cdata.2 + cdata.3 * 7].2,
                                PIECEDATA[cdata.2 + cdata.3 * 7].3,
                            ] {
                                if pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 0))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 1))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 2))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 3))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 4))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 5))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 6))
                                {
                                    gameover(
                                        &mut pieces,
                                        &mut cdata,
                                        &mut next,
                                        &mut linecount,
                                        &mut score,
                                        &mut hold,
                                        tsize,
                                    );
                                }
                            }

                            clearlines(
                                &mut pieces,
                                &mut score,
                                &mut linecount,
                                cdata,
                                next,
                                tsize,
                                hold,
                            );

                            bottomframe = false;
                            render(tsize, &pieces, cdata, next, score, linecount, hold);
                        }
                    }
                    KeyCode::Up | KeyCode::Char('w') => {
                        while !bottomframe {
                            cdata = (cdata.0, cdata.1 + 1, cdata.2, cdata.3);
                            for px in [
                                PIECEDATA[cdata.2 + cdata.3 * 7].0,
                                PIECEDATA[cdata.2 + cdata.3 * 7].1,
                                PIECEDATA[cdata.2 + cdata.3 * 7].2,
                                PIECEDATA[cdata.2 + cdata.3 * 7].3,
                            ] {
                                if px.1 + cdata.1 >= 17
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 0))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 1))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 2))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 3))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 4))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 5))
                                    || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 6))
                                {
                                    bottomframe = true;
                                    break;
                                }
                            }
                            render(tsize, &pieces, cdata, next, score, linecount, hold);
                        }
                        for px in [
                            PIECEDATA[cdata.2 + cdata.3 * 7].0,
                            PIECEDATA[cdata.2 + cdata.3 * 7].1,
                            PIECEDATA[cdata.2 + cdata.3 * 7].2,
                            PIECEDATA[cdata.2 + cdata.3 * 7].3,
                        ] {
                            pieces.push((cdata.0 + px.0, cdata.1 + px.1, cdata.2));
                        }
                        cdata = (4, 0, next, 0);
                        next = rand::rng().random_range(0..7);
                        for px in [
                            PIECEDATA[cdata.2 + cdata.3 * 7].0,
                            PIECEDATA[cdata.2 + cdata.3 * 7].1,
                            PIECEDATA[cdata.2 + cdata.3 * 7].2,
                            PIECEDATA[cdata.2 + cdata.3 * 7].3,
                        ] {
                            if pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 0))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 1))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 2))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 3))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 4))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 5))
                                || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 6))
                            {
                                gameover(
                                    &mut pieces,
                                    &mut cdata,
                                    &mut next,
                                    &mut linecount,
                                    &mut score,
                                    &mut hold,
                                    tsize,
                                );
                            }
                        }

                        clearlines(
                            &mut pieces,
                            &mut score,
                            &mut linecount,
                            cdata,
                            next,
                            tsize,
                            hold,
                        );

                        bottomframe = false;
                        render(tsize, &pieces, cdata, next, score, linecount, hold);
                    }
                    KeyCode::Char('h') => {
                        let lastp = cdata.2;
                        if hold == 7 {
                            cdata = (4, 0, next, 0);
                        } else {
                            cdata = (4, 0, hold, 0);
                        }
                        hold = lastp;
                        render(tsize, &pieces, cdata, next, score, linecount, hold);
                    }
                    _ => {}
                }
            }
        }
        if now.elapsed().as_millis()
            >= speeds[(linecount as f32 / 10.0).floor() as usize] * 1000 / 60
        {
            if bottomframe {
                for px in [
                    PIECEDATA[cdata.2 + cdata.3 * 7].0,
                    PIECEDATA[cdata.2 + cdata.3 * 7].1,
                    PIECEDATA[cdata.2 + cdata.3 * 7].2,
                    PIECEDATA[cdata.2 + cdata.3 * 7].3,
                ] {
                    pieces.push((cdata.0 + px.0, cdata.1 + px.1, cdata.2));
                }
                cdata = (4, 0, next, 0);
                next = rand::rng().random_range(0..7);
                for px in [
                    PIECEDATA[cdata.2 + cdata.3 * 7].0,
                    PIECEDATA[cdata.2 + cdata.3 * 7].1,
                    PIECEDATA[cdata.2 + cdata.3 * 7].2,
                    PIECEDATA[cdata.2 + cdata.3 * 7].3,
                ] {
                    if pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 0))
                        || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 1))
                        || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 2))
                        || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 3))
                        || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 4))
                        || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 5))
                        || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1, 6))
                    {
                        gameover(
                            &mut pieces,
                            &mut cdata,
                            &mut next,
                            &mut linecount,
                            &mut score,
                            &mut hold,
                            tsize,
                        );
                    }
                }

                clearlines(
                    &mut pieces,
                    &mut score,
                    &mut linecount,
                    cdata,
                    next,
                    tsize,
                    hold,
                );

                // bottomframe = false;
            } 
            else {
                cdata = (cdata.0, cdata.1 + 1, cdata.2, cdata.3);
                // for px in [
                //     PIECEDATA[cdata.2 + cdata.3 * 7].0,
                //     PIECEDATA[cdata.2 + cdata.3 * 7].1,
                //     PIECEDATA[cdata.2 + cdata.3 * 7].2,
                //     PIECEDATA[cdata.2 + cdata.3 * 7].3,
                // ] {
                //     if px.1 + cdata.1 >= 17
                //         || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 0))
                //         || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 1))
                //         || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 2))
                //         || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 3))
                //         || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 4))
                //         || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 5))
                //         || pieces.contains(&(px.0 + cdata.0, px.1 + cdata.1 + 1, 6))
                //     {
                //         bottomframe = true;
                //         break;
                //     }
                // }
            }
            render(tsize, &pieces, cdata, next, score, linecount, hold);
            now = Instant::now();
        }
    }

    disable_raw_mode().unwrap();
    execute!(stdout, LeaveAlternateScreen, Show).unwrap();
}
