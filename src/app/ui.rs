use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{
    Block, BorderType, Borders, Clear, Gauge, List, ListItem, ListState, Paragraph, Wrap,
};
use tui::Frame;

use super::doolist::DooList;

use crate::app;

#[derive(Debug)]
enum LayoutHorizontal {
    Full,
    Right,
    Center,
    Left,
}

impl LayoutHorizontal {
    fn constraints(&self) -> [Constraint; 3] {
        match self {
            LayoutHorizontal::Full => [
                Constraint::Percentage(100),
                Constraint::Percentage(0),
                Constraint::Percentage(0),
            ],
            LayoutHorizontal::Right => [
                Constraint::Percentage(60),
                Constraint::Percentage(40),
                Constraint::Percentage(0),
            ],
            LayoutHorizontal::Center => [
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(0),
            ],
            LayoutHorizontal::Left => [
                Constraint::Percentage(40),
                Constraint::Percentage(60),
                Constraint::Percentage(0),
            ],
        }
    }

    fn block_index(&self) -> usize {
        match self {
            LayoutHorizontal::Full => 0,
            LayoutHorizontal::Right => 1,
            LayoutHorizontal::Center => 1,
            LayoutHorizontal::Left => 0,
        }
    }
}

#[derive(Debug)]
enum LayoutVertical {
    Full,
    Top,
    Middle,
    Bottom,
}

impl LayoutVertical {
    fn constraints(&self) -> [Constraint; 3] {
        match self {
            LayoutVertical::Full => [
                Constraint::Percentage(100),
                Constraint::Percentage(0),
                Constraint::Percentage(0),
            ],
            LayoutVertical::Top => [
                Constraint::Percentage(60),
                Constraint::Percentage(40),
                Constraint::Percentage(0),
            ],
            LayoutVertical::Middle => [
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ],
            LayoutVertical::Bottom => [
                Constraint::Percentage(40),
                Constraint::Percentage(60),
                Constraint::Percentage(0),
            ],
        }
    }

    fn block_index(&self) -> usize {
        match self {
            LayoutVertical::Full => 0,
            LayoutVertical::Top => 0,
            LayoutVertical::Middle => 1,
            LayoutVertical::Bottom => 1,
        }
    }
}

// TODO: should support height, width parameter
fn get_doo_block_from_screen<B: Backend>(
    f: &Frame<B>,
    vertical_orientation: Option<LayoutVertical>,
    horizontal_orientation: Option<LayoutHorizontal>,
) -> Rect {
    // assign default directions
    let horizontal_orientation = match horizontal_orientation {
        Some(orientation) => orientation,
        None => LayoutHorizontal::Left,
    };

    // assign default directions
    let vertical_orientation = match vertical_orientation {
        Some(orientation) => orientation,
        None => LayoutVertical::Bottom,
    };

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

fn get_doo_module<B: Backend>(
    f: &Frame<B>,
    vertical_orientation: Option<LayoutVertical>,
    horizontal_orientation: Option<LayoutHorizontal>,
) -> Vec<Rect> {
    let doo_block = get_doo_block_from_screen(f, vertical_orientation, horizontal_orientation);

    let doo_modules = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(doo_block);

    return doo_modules;
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut app::App) {
    // working app area
    let doo_modules = get_doo_module(
        f,
        Some(LayoutVertical::Middle),
        Some(LayoutHorizontal::Center),
    );

    // split main display into todo area and status
    let core_module = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(15), Constraint::Percentage(85)].as_ref())
        .split(doo_modules[0]);

    let wrapper_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    f.render_widget(wrapper_block, doo_modules[0]);
    render_status_bar(f, app.current_path.clone(), core_module[0]);
    render_list(f, &mut app.doolist, core_module[1]);
    render_input_bar(f, &app.mode, app.input.clone(), doo_modules[1]);
}

fn render_list<B: Backend>(f: &mut Frame<B>, doolist: &mut DooList, chunk: Rect) {
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
                .add_modifier(Modifier::UNDERLINED)
                .fg(Color::Cyan),
        );

    f.render_stateful_widget(live_draw_list, chunk, &mut doolist.state);
}

fn render_status_bar<B: Backend>(f: &mut Frame<B>, current_path: Option<String>, chunk: Rect) {
    let status_block = Block::default()
        .title_alignment(Alignment::Left)
        .borders(Borders::BOTTOM)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));

    let status_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
        .split(chunk);

    let filename = Paragraph::new(match current_path {
        Some(path) => path,
        None => "-- new file --".to_string(),
    })
    .style(Style::default())
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

    let widget = Paragraph::new("0/5")
        .style(Style::default())
        .alignment(Alignment::Right)
        .wrap(Wrap { trim: true });

    f.render_widget(status_block, chunk);
    f.render_widget(filename, status_chunks[0]);
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

    // let result_diagnostics_style = Style::default().fg(Color::Magenta);

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