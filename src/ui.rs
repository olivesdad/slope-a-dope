use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Alignment,
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Text},
    widgets::{
        block::title, Axis, Block, BorderType, Borders, Chart, Dataset, Gauge, GraphType, Padding,
        Paragraph, Wrap,
    },
    Frame,
};

use crate::{
    app::{App, CurrentlyEditing, Mode, ScreenID},
    calculator::MeasurementType,
};

pub fn ui(f: &mut Frame, app: &App) {
    // Draw all the things

    // ----- Break the frame into work spaces ------ ////
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
    let mut p1_block = make_block(" Point 1 ");
    let mut p2_block = make_block(" Point 2 ");
    let chart_block = make_block(" Results ");
    let mut sim_block = make_block(" Test function ");
    let footer_block = make_block(" Current Mode ");

    // get inner blocks for P1, P2, and sim
    // [P1]
    let p1_inner = p1_block.inner(p1_area);
    let p1_contents = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(p1_inner);
    let mut p1_v_block = make_block(" p1 Voltage ");
    let mut p1_p_block = make_block(" p1 Physical ");
    // [P2]
    let p2_inner = p2_block.inner(p2_area);
    let p2_contents = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(p2_inner);
    let mut p2_v_block = make_block(" p2 Voltage ");
    let mut p2_p_block = make_block(" p2 Physical ");

    // Get inner blocks for test section
    let sim_inner = sim_block.inner(sim_area);
    let test_values = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(sim_inner);

    // Blocks to render the sim values in
    // will be rendered to the rects in test_values
    let test_v_block = make_block(" Voltage ");
    let test_p_block = make_block(" Pysical ");

    // ------ DYNAMIC RENDERED --------
    // Color blocks for slector
    match app.get_mode() {
        Mode::Select => match app.get_current_screen() {
            ScreenID::P1 => {
                p1_block = p1_block.style(Style::default().fg(Color::LightMagenta));
            }
            ScreenID::P2 => {
                p2_block = p2_block.style(Style::default().fg(Color::LightMagenta));
            }
            ScreenID::Tester => {
                sim_block = sim_block.style(Style::default().fg(Color::LightMagenta));
            }
        },

        // Color the highlighted cell will paint cells in edit mode and persist the outer coloring through editing value mode
        Mode::Edit => match app.get_current_screen() {
            ScreenID::P1 => {
                p1_block = p1_block.style(Style::default().fg(Color::Green));
                // Color volt/phys selector
                if let Some(point_value) = app.get_currently_editing() {
                    match point_value {
                        crate::app::CurrentlyEditing::Voltage => {
                            //currently editing voltage need to color it yellow
                            p1_v_block = p1_v_block.style(Style::default().fg(Color::LightMagenta))
                        }
                        crate::app::CurrentlyEditing::Physical => {
                            //currently editing voltage need to color it yellow
                            p1_p_block = p1_p_block.style(Style::default().fg(Color::LightMagenta))
                        }
                    }
                }
            }
            ScreenID::P2 => {
                p2_block = p2_block.style(Style::default().fg(Color::Green));
                if let Some(point_value) = app.get_currently_editing() {
                    match point_value {
                        crate::app::CurrentlyEditing::Voltage => {
                            //currently editing voltage need to color it yellow
                            p2_v_block = p2_v_block.style(Style::default().fg(Color::LightMagenta))
                        }
                        crate::app::CurrentlyEditing::Physical => {
                            //currently editing voltage need to color it yellow
                            p2_p_block = p2_p_block.style(Style::default().fg(Color::LightMagenta))
                        }
                    }
                }
            }
            ScreenID::Tester => {
                sim_block = sim_block.style(Style::default().fg(Color::Green));
            }
        },
        // Handle coloring and editng values in EditValue mode
        // We need to repaint the currently_editing value cell
        Mode::EditingValue => {
            if let Some(x) = app.get_currently_editing() {
                match app.get_current_screen() {
                    ScreenID::P1 => match x {
                        CurrentlyEditing::Physical => {
                            p1_block = p1_block.style(Style::default().fg(Color::Green));
                            p1_p_block = p1_p_block.style(Style::default().fg(Color::Green));
                        }
                        CurrentlyEditing::Voltage => {
                            p1_block = p1_block.style(Style::default().fg(Color::Green));
                            p1_v_block = p1_v_block.style(Style::default().fg(Color::Green));
                        }
                    },
                    ScreenID::P2 => match x {
                        CurrentlyEditing::Physical => {
                            p2_block = p2_block.style(Style::default().fg(Color::Green));
                            p2_p_block = p2_p_block.style(Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Voltage => {
                            p2_block = p2_block.style(Style::default().fg(Color::Green));
                            p2_v_block = p2_v_block.style(Style::default().fg(Color::Green))
                        }
                    },
                    ScreenID::Tester => {}
                }
            }
        }
        // Color for editingValue
        _ => {}
    }

    let mut test_v_text = make_paragraph("", test_v_block.clone());
    let mut test_p_text = make_paragraph("", test_p_block.clone());


    // Make paragraphs for tester if were holidng a testing value
    if let Some(testing_value) = app.testing_value.as_ref() {
        if let Some(line) = app.line.as_ref() {
            if let Some((_, _)) = line.get_val() {
                if let Ok(calculated_value) = line.get_corresponding_value(&testing_value) {
                    match testing_value {
                        MeasurementType::physical(phys) => {
                            // were using a physicaly input so we need to calc the other one
                             test_p_text = Paragraph::new(format!("{:.4}", phys.clone()))
                                .alignment(Alignment::Center)
                                .block(test_p_block);
                            test_v_text = Paragraph::new(format!("{:.4}", calculated_value))
                                .alignment(Alignment::Center)
                                .block(test_v_block);
                        }
                        MeasurementType::voltage(volt) => {
                             test_v_text = Paragraph::new(format!("{:.4}", volt.clone()))
                                .alignment(Alignment::Center)
                                .block(test_v_block);
                            test_p_text = Paragraph::new(format!("{:.4}", calculated_value))
                                .alignment(Alignment::Center)
                                .block(test_p_block);

                        }
                    }
                }
            }
        }
    }

    // Make paragraphs for [P1] [P2]
    if let Some(points) = app.get_points() {
        // Determine if we should use the temp_point or the stored p1 and p2 values
        // First set the point values
        let mut p1_v_str = format!("{:.4}", points.0.get("v").cloned().unwrap_or(0.0));
        let mut p1_p_str = format!("{:.4}", points.0.get("p").cloned().unwrap_or(0.0));
        let mut p2_v_str = format!("{:.4}", points.1.get("v").cloned().unwrap_or(990.0));
        let mut p2_p_str = format!("{:.4}", points.1.get("p").cloned().unwrap_or(990.0));
        // Then overwrite as needed for editingvalue mode
        if let Mode::EditingValue = app.get_mode() {
            if let Some(x) = app.get_currently_editing() {
                match app.get_current_screen() {
                    ScreenID::P1 => match x {
                        CurrentlyEditing::Physical => {
                            p1_p_str = app.get_temp_point().into();
                        }
                        CurrentlyEditing::Voltage => {
                            p1_v_str = app.get_temp_point().into();
                        }
                    },
                    ScreenID::P2 => match x {
                        CurrentlyEditing::Physical => {
                            p2_p_str = app.get_temp_point().into();
                        }
                        CurrentlyEditing::Voltage => {
                            p2_v_str = app.get_temp_point().into();
                        }
                    },
                    ScreenID::Tester => match x {
                        CurrentlyEditing::Physical => {}
                        CurrentlyEditing::Voltage => {}
                    },
                }
            }
        }
        // make the paragraphs
        // [P1]
        let p1_v_text = make_paragraph(&p1_v_str, p1_v_block);
        let p1_p_text = make_paragraph(&p1_p_str, p1_p_block);

        // [P2]
        let p2_v_text = make_paragraph(&p2_v_str, p2_v_block);
        let p2_p_text = make_paragraph(&p2_p_str, p2_p_block);

        // [TESTER]]
        f.render_widget(test_v_text, test_values[0]);
        f.render_widget(test_p_text, test_values[1]);

        // render
        f.render_widget(p1_block, p1_area);
        f.render_widget(p2_block, p2_area);
        f.render_widget(p1_v_text, p1_contents[0]);
        f.render_widget(p1_p_text, p1_contents[1]);
        f.render_widget(p2_v_text, p2_contents[0]);
        f.render_widget(p2_p_text, p2_contents[1]);
    } else {
        f.render_widget(p1_block, p1_area);
        f.render_widget(p2_block, p2_area);
        f.render_widget(p1_p_block, p1_contents[0]);
        f.render_widget(p1_v_block, p1_contents[1]);
        f.render_widget(p2_p_block, p2_contents[0]);
        f.render_widget(p2_v_block, p2_contents[1]);
    }
    // --------Results Render -----
    // get tester inner block before rendering it
    let temp = chart_block.inner(chart_area);
    let calc_contents = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Max(2), Constraint::Min(10)])
        .split(temp);

    // Render the border to not color everything
    f.render_widget(chart_block, chart_area);

    // Equation def
    f.render_widget(
        Paragraph::new(app.get_line_val()).alignment(Alignment::Center),
        calc_contents[0],
    );

    // ---- STATIC Colors -----

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
        Mode::Select => "Mode: Select Point",
        Mode::Edit => "Mode: Value Selection",
        Mode::Quit => "Bye Bye!",
        Mode::EditingValue => "Editing Value",
    };
    let footer_text = Paragraph::new(s)
        .block(footer_block)
        .alignment(Alignment::Center);

    //  ---- ----- Render things --- ----- -----
    f.render_widget(title_paragrah, title_area);
    f.render_widget(help_block, help_area);
    f.render_widget(sim_block, sim_area);

    f.render_widget(footer_text, footer_area);
}

// ------- Helper Functions -------
pub fn make_block<'a>(s: &'a str) -> Block<'a> {
    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White))
        .border_type(BorderType::Rounded)
        .title(s);
    return block;
}

pub fn make_paragraph<'a>(s: &'a str, b: Block<'a>) -> Paragraph<'a> {
    Paragraph::new(s).block(b).alignment(Alignment::Center)
}
