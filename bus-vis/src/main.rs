use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, GraphType, Dataset, Chart, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
    symbols,
    style::Style,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    use tui::widgets::*;
    use tui::text::*;
    use tui::style::*;
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let datasets = vec![
    Dataset::default()
        .name("data1")
        .marker(symbols::Marker::Dot)
        .graph_type(GraphType::Scatter)
        .style(Style::default().fg(Color::Cyan))
        .data(&[(0.0, 5.0), (1.0, 6.0), (1.5, 6.434)]),
    Dataset::default()
        .name("data2")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Magenta))
        .data(&[(4.0, 5.0), (5.0, 8.0), (7.66, 13.5)]),
    ];

    let x = Chart::new(datasets)
        .block(Block::default().title("Chart"))
        .x_axis(Axis::default()
            .title(Span::styled("X Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()))
        .y_axis(Axis::default()
            .title(Span::styled("Y Axis", Style::default().fg(Color::Red)))
            .style(Style::default().fg(Color::White))
            .bounds([0.0, 10.0])
            .labels(["0.0", "5.0", "10.0"].iter().cloned().map(Span::from).collect()));

    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(x, size);
    })?;
    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}