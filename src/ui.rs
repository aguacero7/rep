use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Direction as LayoutDirection, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line as TxtLine, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::game::Game;

pub fn draw_ui(f: &mut Frame, game: &Game, elapsed_secs: u64) {
    let area = f.area();

    let chunks = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);

    let title = Span::styled(
        " Reptil en Peligro ",
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    );
    let sep = Span::styled(" ─ ", Style::default().fg(Color::DarkGray));
    let score = Span::styled(
        format!("{} pts", game.score),
        Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
    );
    let secs = Span::styled(
        format!("{} s", elapsed_secs),
        Style::default().fg(Color::Yellow),
    );

    let line = TxtLine::from(vec![title, sep.clone(), score, sep, secs]);

    let banner = Paragraph::new(line)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_type(BorderType::Double),
        )
        .alignment(ratatui::layout::Alignment::Center);

    f.render_widget(banner, chunks[0]);

    // ---------- Board ----------
    let board = Board { game };
    f.render_widget(board, chunks[1]);
}

struct Board<'a> {
    game: &'a Game,
}

impl Widget for Board<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Cadre
        let block = Block::default().borders(Borders::ALL).title("Board");
        let inner = block.inner(area);
        block.render(area, buf);

        if inner.width < 2 || inner.height == 0 {
            return;
        }

        let world_w = self.game.world.width as u16;
        let world_h = self.game.world.height as u16;

        let vis_w_cells = inner.width / 2;
        let vis_h_cells = inner.height;

        let vis_w_cells = vis_w_cells.min(world_w);
        let vis_h_cells = vis_h_cells.min(world_h);

        let snake = &self.game.snake;
        let head = snake.head();
        let food = self.game.food;

        // Styles
        let style_empty = Style::default();
        let style_snake = Style::default().fg(Color::Green);
        let style_head = Style::default()
            .fg(Color::LightGreen)
            .add_modifier(Modifier::BOLD);
        let style_food = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);

        for vy in 0..vis_h_cells {
            for vx in 0..vis_w_cells {
                let mx = vx as i32;
                let my = vy as i32;

                let col = inner.x + vx * 2;
                let row = inner.y + vy;

                if head.x == mx && head.y == my {
                    buf.set_string(col, row, "▓▓", style_head);
                    continue;
                }

                if snake.body.iter().any(|c| c.x == mx && c.y == my) {
                    buf.set_string(col, row, "██", style_snake);
                    continue;
                }

                if food.x == mx && food.y == my {
                    buf.set_string(col, row, "██", style_food);
                    continue;
                }

                buf.set_string(col, row, "  ", style_empty);
            }
        }
    }
}
