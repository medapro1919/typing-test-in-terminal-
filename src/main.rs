
 // Code by Jacob Wheeler
// RUST TYPE - A terminal-based typing test.

use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io;
use std::time::Instant;

// UI colour palette
const ORANGE: Color = Color::Rgb(215, 125, 65);
const LIGHT_ORANGE: Color = Color::Rgb(230, 155, 95);
const DIM_ORANGE: Color = Color::Rgb(170, 105, 60);

// Centre the application
fn centered_area(area: Rect) -> Rect {
    let width = area.width.min(90);
    let height = area.height.min(19);

    Rect::new(
        (area.width - width) / 2,
        (area.height - height) / 2,
        width,
        height,
    )
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Load words
    let words: Vec<String> = std::fs::read_to_string("words.txt")?
        .lines()
        .map(|line| line.trim().to_lowercase())
        .filter(|line| !line.is_empty())
        .collect();

    if words.is_empty() {
        println!("Error: words.txt contains no words.");
        return Ok(());
    }

    // Ask for test length
    println!();
    println!("RUST TYPE");
    println!("Enter how many words you would like.");
    println!("Enter 0 for free typing.");
    print!("> ");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let no_of_words: usize = match input.trim().parse() {
        Ok(number) => number,
        Err(_) => {
            println!("Please enter a valid number.");
            return Ok(());
        }
    };

    // Generate random words
    let mut rng = rand::thread_rng();

    let prompt_words: Vec<String> = (0..no_of_words)
        .map(|_| words[rng.gen_range(0..words.len())].clone())
        .collect();

    let prompt = prompt_words.join(" ");

    // Set up terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut user_input = String::new();
    let mut start: Option<Instant> = None;
    let mut finished = false;

    enable_raw_mode()?;
    terminal.hide_cursor()?;

    // Main application loop
    let app_result = (|| -> Result<(), Box<dyn std::error::Error>> {

        loop {
            terminal.draw(|frame| {

                let area = centered_area(frame.area());

                // Main layout
                let layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(2),
                        Constraint::Length(1),
                        Constraint::Length(2),
                        Constraint::Length(6),
                        Constraint::Length(4),
                        Constraint::Length(2),
                        Constraint::Length(1),
                    ])
                    .split(area);

                // Main heading - normal terminal font
                let title = Paragraph::new("RUST TYPE")
                    .alignment(Alignment::Center)
                    .style(
                        Style::default()
                            .fg(ORANGE)
                            .add_modifier(Modifier::BOLD),
                    );

                frame.render_widget(title, layout[0]);

                // Subtitle
                let subtitle = Paragraph::new("A TERMINAL TYPING TEST")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(DIM_ORANGE));

                frame.render_widget(subtitle, layout[1]);

                // Spacer / test instructions
                let instructions = Paragraph::new(
                    if no_of_words == 0 {
                        "FREE TYPING MODE"
                    } else {
                        "TYPE THE FOLLOWING WORDS"
                    }
                )
                .alignment(Alignment::Center)
                .style(
                    Style::default()
                        .fg(LIGHT_ORANGE)
                        .add_modifier(Modifier::BOLD),
                );

                frame.render_widget(instructions, layout[2]);

                // Prompt
                let prompt_block = Block::default()
                    .title(" TYPING TEST ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(DIM_ORANGE));

                let prompt_lines = if no_of_words == 0 {
                    vec![Line::from(Span::styled(
                        "Type whatever you like!",
                        Style::default().fg(ORANGE),
                    ))]
                } else {
                    vec![Line::from(
                        prompt
                            .split_whitespace()
                            .map(|word| {
                                Span::styled(
                                    format!("{} ", word),
                                    Style::default().fg(ORANGE),
                                )
                            })
                            .collect::<Vec<Span>>()
                    )]
                };

                let prompt_widget = Paragraph::new(prompt_lines)
                    .block(prompt_block)
                    .wrap(Wrap { trim: true });

                frame.render_widget(prompt_widget, layout[3]);

                // User input
                let typing_text = if finished {
                    "TEST COMPLETED".to_string()
                } else {
                    format!("> {}", user_input)
                };

                let typing_widget = Paragraph::new(typing_text)
                    .style(Style::default().fg(LIGHT_ORANGE))
                    .block(
                        Block::default()
                            .title(" YOUR INPUT ")
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(DIM_ORANGE)),
                    )
                    .wrap(Wrap { trim: false });

                frame.render_widget(typing_widget, layout[4]);

                // Calculate live statistics
                let elapsed = start
                    .map(|time| time.elapsed().as_secs_f64())
                    .unwrap_or(0.0);

                let typed_words = user_input.split_whitespace().count();

                let wpm = if elapsed > 0.0 {
                    typed_words as f64 / elapsed * 60.0
                } else {
                    0.0
                };

                let correct_words = prompt
                    .split_whitespace()
                    .zip(user_input.split_whitespace())
                    .filter(|(expected, typed)| expected == typed)
                    .count();

                let accuracy = if typed_words > 0 && no_of_words > 0 {
                    correct_words as f64 / typed_words as f64 * 100.0
                } else {
                    100.0
                };

                // Statistics
                let stats = format!(
                    "WPM: {:.1}     ACCURACY: {:.1}%     TIME: {:.1}s",
                    wpm,
                    accuracy,
                    elapsed
                );

                let stats_widget = Paragraph::new(stats)
                    .alignment(Alignment::Center)
                    .style(
                        Style::default()
                            .fg(ORANGE)
                            .add_modifier(Modifier::BOLD),
                    );

                frame.render_widget(stats_widget, layout[5]);

                // Footer
                let footer = Paragraph::new(
                    "ESC: QUIT     BACKSPACE: DELETE     ENTER: FINISH"
                )
                .alignment(Alignment::Center)
                .style(Style::default().fg(DIM_ORANGE));

                frame.render_widget(footer, layout[6]);

            })?;

            if finished {
                break;
            }

            // Keyboard input
            if let Event::Key(key_event) = event::read()? {

                if key_event.kind == KeyEventKind::Release {
                    continue;
                }

                match key_event.code {

                    KeyCode::Char(c) => {
                        if start.is_none() {
                            start = Some(Instant::now());
                        }

                        user_input.push(c);
                    }

                    KeyCode::Backspace => {
                        user_input.pop();
                    }

                    KeyCode::Enter => {
                        finished = true;
                    }

                    KeyCode::Esc => {
                        finished = true;
                    }

                    _ => {}
                }

                // Only finish when the entire prompt is correct
                if no_of_words > 0
                    && user_input.trim_end() == prompt
                {
                    finished = true;
                }
            }
        }

        Ok(())
    })();

    // Restore terminal
    disable_raw_mode()?;
    terminal.show_cursor()?;

    app_result?;

    // Final results
    let elapsed = start
        .map(|time| time.elapsed().as_secs_f64())
        .unwrap_or(0.0);

    let typed_words = user_input.split_whitespace().count();

    let wpm = if elapsed > 0.0 {
        typed_words as f64 / elapsed * 60.0
    } else {
        0.0
    };

    let correct_words = prompt
        .split_whitespace()
        .zip(user_input.split_whitespace())
        .filter(|(expected, typed)| expected == typed)
        .count();

    let accuracy = if typed_words > 0 && no_of_words > 0 {
        correct_words as f64 / typed_words as f64 * 100.0
    } else {
        0.0
    };

    println!();
    println!("========== RUST TYPE ==========");
    println!("TIME: {:.2}s", elapsed);
    println!("WPM: {:.2}", wpm);

    if no_of_words > 0 {
        println!("CORRECT WORDS: {}/{}", correct_words, no_of_words);
        println!("ACCURACY: {:.2}%", accuracy);
    }

    println!("===============================");

    Ok(())
}
