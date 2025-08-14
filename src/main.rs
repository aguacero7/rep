mod event;
mod game;
mod ui;
const HEADER_ROWS: u16 = 3;

use anyhow::Result;
use crossterm::{
    event::KeyCode,
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
        size as term_size,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    time::{Duration, Instant},
};

use event::{Event, EventLoop};
use game::{Dir, Game};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tw, th) = term_size()?; // colonnes, lignes
    let (world_w, world_h) = world_from_terminal(tw, th);

    let tick_rate = Duration::from_millis(100);
    let events = EventLoop::new(tick_rate);

    let mut game = Game::new(world_w as i32, world_h as i32, 0xDEADBEEF);
    let start = Instant::now();

    'outer: loop {
        match events.recv()? {
            Event::Tick => {
                game.update();
                if game.game_over {
                    break 'outer;
                }
            }
            Event::Key(KeyCode::Char('q')) | Event::Key(KeyCode::Esc) => break 'outer,
            Event::Key(KeyCode::Up) => game.change_dir(Dir::Up),
            Event::Key(KeyCode::Down) => game.change_dir(Dir::Down),
            Event::Key(KeyCode::Left) => game.change_dir(Dir::Left),
            Event::Key(KeyCode::Right) => game.change_dir(Dir::Right),
            Event::Resize(w, h) => {
                let (nw, nh) = world_from_terminal(w, h);

                game.world.width = nw as i32;
                game.world.height = nh as i32;

                // Clamp snake segments
                for seg in game.snake.body.iter_mut() {
                    seg.x = seg.x.clamp(0, game.world.width - 1);
                    seg.y = seg.y.clamp(0, game.world.height - 1);
                }
                // Clamp fruit
                game.food.x = game.food.x.clamp(0, game.world.width - 1);
                game.food.y = game.food.y.clamp(0, game.world.height - 1);
            }
            Event::Key(_) => {}
        }

        terminal.draw(|f| {
            let elapsed = start.elapsed().as_secs();
            ui::draw_ui(f, &game, elapsed);
        })?;
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn world_from_terminal(term_cols: u16, term_rows: u16) -> (u16, u16) {
    let inner_cols = term_cols.saturating_sub(2);

    let inner_rows = term_rows.saturating_sub(HEADER_ROWS).saturating_sub(2);

    let cells_w = (inner_cols / 2).max(1);
    let cells_h = inner_rows.max(1);

    (cells_w, cells_h)
}
