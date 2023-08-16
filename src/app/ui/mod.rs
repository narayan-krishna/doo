mod layout;

use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{
    Block, BorderType, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Wrap,
};
use tui::Frame;

use super::{recent_files::RecentFiles, DooList, Screen};
use layout::*;

use crate::app;

// TODO: should support height, width parameter
/// get the block
fn get_doo_block_from_screen<B: Backend>(
    f: &Frame<B>,
    vertical_orientation: LayoutVertical,
    horizontal_orientation: LayoutHorizontal,
) -> Rect {
    // cut the block into horizontal chunk, return new block to cut
    let horizontal_cut = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(horizontal_orientation.constraints())
        .split(f.size())[horizontal_orientation.block_index()];

    // cut horizontal block vertically, return new block to use
    let vertical_cut = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vertical_orientation.constraints())
        .split(horizontal_cut)[vertical_orientation.block_index()];

    return vertical_cut;
}

fn get_doo_module_chunks(doo_module: Rect) -> Vec<Rect> {
    let doo_modules = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(doo_module);

    return doo_modules;
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut app::App) {

    // working app area
    let doo_module = get_doo_block_from_screen(
        f,
        LayoutVertical::from_str(&app.config.layout.vertical),
        LayoutHorizontal::from_str(&app.config.layout.horizontal),
    );

    let doo_module_chunks = get_doo_module_chunks(doo_module);

    // block around module
    let wrapper_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    f.render_widget(wrapper_block, doo_module_chunks[0]);

    // split main display into todo area and status
    let core_module = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
        .split(doo_module_chunks[0]);

    // render components
    render_status_bar(
        f,
        &app.screen,
        app.doolist.name.clone(),
        &app.doolist.state,
        app.doolist.list.len(),
        core_module[0],
    );

    match app.screen {
        Screen::Help => render_help(f, core_module[1]),
        Screen::DooList => render_doolist(f, &mut app.doolist, core_module[1]),
        Screen::Recents => {
            render_recents(f, &mut app.recent_files, &app.current_path, core_module[1])
        }
    }

    render_input_bar(f, &app.mode, app.input.clone(), doo_module_chunks[1]);
}

fn render_doolist<B: Backend>(f: &mut Frame<B>, doolist: &mut DooList, chunk: Rect) {
    let items: Vec<ListItem> = doolist
        .list
        .iter()
        .map(|s| {
            let completion_marker = match s.complete {
                true => "[X] ",
                false => "[ ] ",
            };

            let mut item = vec![Span::styled(
                completion_marker,
                Style::default().fg(Color::White),
            )];

            item.push(match s.complete {
                true => Span::styled(
                    &s.label,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::CROSSED_OUT),
                ),
                false => Span::styled(&s.label, Style::default().fg(Color::White)),
            });

            return ListItem::new(Spans::from(item));
        })
        .collect();

    let live_draw_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default())
        .start_corner(tui::layout::Corner::TopRight)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Cyan),
        );

    f.render_stateful_widget(live_draw_list, chunk, &mut doolist.state);
}

fn render_help<B: Backend>(f: &mut Frame<B>, chunk: Rect) {
    let help_text = "doo has 4 modes: select, search, insert, and command.\n
        In command mode, you can use the following commands:\n
        \t:q -- quit
        \t:w | :saveas <optional filepath> -- save file (to path)
        \t:wq -- save and quit
        \t:e | :load <optional filepath> -- load file into doo
        \t:rename -- change the file display name
        \t:recent -- load a recent todo
        \t:help -- open this menu
        ";

    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(help_paragraph, chunk);
}

fn render_recents<B: Backend>(
    f: &mut Frame<B>,
    recent_files: &mut RecentFiles,
    current_filepath: &Option<String>,
    chunk: Rect,
) {
    // render the current file as current
    let items: Vec<ListItem> = recent_files
        .queue
        .items
        .iter()
        .map(|s| {
            if let Some(i) = current_filepath {
                if s == i {
                    return ListItem::new(Span::styled(
                        s,
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ));
                }
            }

            return ListItem::new(Span::styled(s, Style::default().fg(Color::Gray)));
        })
        .collect();

    let live_draw_list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default())
        .start_corner(tui::layout::Corner::TopRight)
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::Cyan),
        );

    f.render_stateful_widget(live_draw_list, chunk, &mut recent_files.state);
}

fn render_status_bar<B: Backend>(
    f: &mut Frame<B>,
    screen: &Screen,
    name: Option<String>,
    list_state: &ListState,
    list_length: usize,
    chunk: Rect,
) {
    let status_block = Block::default()
        .title_alignment(Alignment::Left)
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
        .split(chunk);

    let title = Paragraph::new(match screen {
        Screen::DooList => match name {
            Some(n) => n,
            None => "- ':rename <name>' to name list -".to_string(),
        },
        Screen::Help => "HELP (<esc> to exit)".to_string(),
        Screen::Recents => "Recent files (<esc> to exit)".to_string(),
    })
    .style(Style::default())
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

    let widget = Paragraph::new(format!(
        "{}/{}",
        match list_state.selected() {
            Some(i) => format!("{}", i + 1), //TODO: error handlign
            None => "--".to_string(),
        },
        list_length
    ))
    .style(Style::default())
    .alignment(Alignment::Right)
    .wrap(Wrap { trim: true });

    f.render_widget(status_block, chunk);
    f.render_widget(title, status_chunks[0]);
    f.render_widget(widget, status_chunks[1]);
}

fn render_input_bar<B: Backend>(f: &mut Frame<B>, mode: &app::Mode, input: String, chunk: Rect) {
    let mut input_line_elements = vec![];

    if let app::Mode::Command = mode {
        input_line_elements.push(Span::styled(":", Style::default().fg(Color::White)));
    }

    input_line_elements.push(Span::styled(&input, Style::default().fg(Color::White)));

    let input_line = Spans::from(input_line_elements);

    let (input_title, input_title_style) = match mode {
        app::Mode::Select => (" Select ", Style::default().fg(Color::LightMagenta)),
        app::Mode::Search => (" Search ", Style::default().fg(Color::Magenta)),
        app::Mode::Command => (" Command ", Style::default().fg(Color::Red)),
        app::Mode::Input => (" Set task name ", Style::default().fg(Color::Yellow)),
    };

    // impl len for launcher list
    let spans = Spans::from(vec![
        Span::styled(input_title, input_title_style), /*, Span::styled(result_diagnostics(), result_diagnostics_style)*/
    ]);

    let input_block = Paragraph::new(input_line)
        .block(
            Block::default()
                .title(spans)
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .style(Style::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(input_block, chunk);
}
