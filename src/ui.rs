use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Span, Text},
    widgets::{
        block::title, Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, GraphType, Padding,
        Paragraph, Wrap,
    },
    Frame,
};

use crate::app::{App, Mode};

pub fn ui(f: &mut Frame, app: &App) {
    // Draw all the things

    /// ----- Break the frame into work spaces ------ ////
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(3), Constraint::Min(20), Constraint::Max(3)])
        .split(f.size());

    // this rect will be for the workspace to be split later
    let workspace_rect = rows[1];

    //split workspace into columns
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(workspace_rect);

    // Divide the columns into rows
    let right_col = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(6), Constraint::Min(10)])
        .split(cols[1]);

    // divide left col
    let left_col = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(40),
            Constraint::Percentage(40),
        ])
        .split(cols[0]);

    // [[[[[[[[[ Rects that we will render stuf in ]]]]]]]]]
    let title_area = rows[0];
    let chart_area = right_col[1];
    let sim_area = right_col[0];
    let help_area = left_col[0];
    let p1_area = left_col[1];
    let p2_area = left_col[2];
    let footer_area = rows[2];

    // [][][] Make Blocks [][][]

    let title_block = make_block("");
    let help_block = make_block(" help ");
    let mut p1_block = make_block(" p1 ");
    let mut p2_block = make_block(" p2 ");
    let chart_block = make_block(" Plot ");
    let mut sim_block = make_block(" Test function ");
    let footer_block = make_block(" Current Mode ");

    // get inner blocks for P1, P2, and sim
    // [P1]
    let p1_inner = p1_block.inner(p1_area);
    let p1_contents = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(p1_inner);
    let p1_v_block = make_block(" p1 Voltage ");
    let p1_p_block = make_block(" p1 Physical ");
    // [P2]
    let p2_inner = p2_block.inner(p2_area);
    let p2_contents = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ]).split(p2_inner);
    let p2_v_block = make_block(" p2 Voltage ");
    let p2_p_block = make_block(" p2 Physical ");

    // ------ Static contents ------
    // Chart 

    // Title
    let title_paragrah = Paragraph::new(Text::styled(
        "Slope-a-Dope",
        Style::default().fg(Color::Yellow),
    ))
    .block(title_block)
    .alignment(Alignment::Center)
    .add_modifier(Modifier::BOLD);

    // Footer
    let s = match app.get_mode() {
        Mode::Select => {"Mode: Selector"}
        Mode::Edit => {"Mode: Editor"}
        Mode::Quit => {"Bye Bye!"}
    };
    let footer_text = Paragraph::new(s)
        .block(footer_block)
        .alignment(Alignment::Center);

    //  ---- ----- Render things --- ----- -----
    f.render_widget(title_paragrah, title_area);
    f.render_widget(help_block, help_area);
    f.render_widget(p1_block, p1_area);
    f.render_widget(p2_block, p2_area);
    f.render_widget(sim_block, sim_area);
    f.render_widget(chart_block, chart_area);
    f.render_widget(footer_text, footer_area);
    f.render_widget(p1_p_block, p1_contents[0]);
    f.render_widget(p1_v_block, p1_contents[1]);
    f.render_widget(p2_p_block, p2_contents[0]);
    f.render_widget(p2_v_block, p2_contents[1]);
}

// ------- Helper Functions -------
pub fn make_block<'a>(s: &'a str) -> Block<'a> {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(s);
    return block;
}

pub fn set_active(b: Block) -> Block {
    b.style(Style::default().fg(Color::Green))
}
