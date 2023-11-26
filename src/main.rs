mod app;
mod calculator;
mod ui;
use app::App;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    //setup terminal
    enable_raw_mode()?;

    //use to log to stderr
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;


    // elements
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    //create app and run it
    let mut app = App::new();

    let _res: Result<bool, io::Error> = run_app(&mut terminal, &mut app);

    // clean up
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    // create channels
    // Draw loop
    loop {
        // render terminal
        terminal.draw(|f| ui::ui(f, app))?;
        //thread::sleep(time::Duration::from_millis(5000));

        // update app state (waits for keypress)
        if let Err(_) = app.update_state() {
            break;
        }

        // If keypress changed it to quit then break
        match app.get_mode() {
            app::Mode::Quit => break,
            _ => {}
        }
    }

    Ok(true)
}
