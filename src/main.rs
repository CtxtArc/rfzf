use clap::Parser;
use crossterm::event::{self, KeyCode, KeyEventKind, KeyModifiers};
use ignore::WalkBuilder;
use nucleo::{Config, Nucleo};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::prelude::Stylize;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

#[derive(Parser)]
#[command(author, version, about = "Blazingly fast fuzzy finder")]
struct Args {
    #[arg(default_value = ".")]
    path: String,
    #[arg(short = 'H', long)]
    hidden: bool,
    #[arg(short, long)]
    preview: bool,
    #[arg(short, long)]
    relative: bool,
    #[arg(short = 's', long)]
    sensitive: bool,
}

#[derive(Clone)]
struct AppTheme {
    name: &'static str,
    bg: Color,
    fg: Color,
    header: Color,
    selection_bg: Color,
    selection_fg: Color,
    cursor: Color,
    spinner: Color,
    footer_bg: Color,
    matched_count: Color,
    syntect_theme: &'static str,
}

impl AppTheme {
    fn nord() -> Self {
        Self {
            name: "Nord",
            bg: Color::Rgb(46, 52, 64),
            fg: Color::Rgb(229, 233, 240),
            header: Color::Rgb(136, 192, 208),
            selection_bg: Color::Rgb(76, 86, 106),
            selection_fg: Color::Rgb(143, 188, 187),
            cursor: Color::Rgb(136, 192, 208),
            spinner: Color::Yellow,
            footer_bg: Color::Rgb(59, 66, 82),
            matched_count: Color::Rgb(76, 86, 106),
            syntect_theme: "base16-ocean.dark",
        }
    }
    fn dracula() -> Self {
        Self {
            name: "Dracula",
            bg: Color::Rgb(40, 42, 54),
            fg: Color::Rgb(248, 248, 242),
            header: Color::Rgb(189, 147, 249),
            selection_bg: Color::Rgb(68, 71, 90),
            selection_fg: Color::Rgb(80, 250, 123),
            cursor: Color::Rgb(255, 121, 198),
            spinner: Color::Rgb(241, 250, 140),
            footer_bg: Color::Rgb(33, 34, 44),
            matched_count: Color::Rgb(98, 114, 164),
            syntect_theme: "base16-mocha.dark",
        }
    }
    fn catppuccin() -> Self {
        Self {
            name: "Catppuccin",
            bg: Color::Rgb(30, 30, 46),
            fg: Color::Rgb(205, 214, 244),
            header: Color::Rgb(137, 180, 250),
            selection_bg: Color::Rgb(49, 50, 68),
            selection_fg: Color::Rgb(245, 194, 231),
            cursor: Color::Rgb(166, 227, 161),
            spinner: Color::Rgb(250, 179, 135),
            footer_bg: Color::Rgb(17, 17, 27),
            matched_count: Color::Rgb(108, 112, 134),
            syntect_theme: "base16-ocean.dark",
        }
    }
}

pub struct App {
    exit: bool,
    matcher: Nucleo<String>,
    list_state: ListState,
    input: String,
    last_input_time: Instant,
    needs_reparse: bool,
    show_preview: bool,
    preview_scroll: u16,
    previous_input: String,
    frame_count: usize,
    last_selected_index: Option<usize>,
    preview_cache: Vec<Line<'static>>,
    ps: SyntaxSet,
    ts: ThemeSet,
    theme: AppTheme,
    show_theme_menu: bool,
    theme_list_state: ListState,
    relative: bool,
    base_path: PathBuf,
    case_sensitive: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let base_path = std::fs::canonicalize(&args.path).unwrap_or_else(|_| PathBuf::from(&args.path));
    let matcher = Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 1);
    let injector = matcher.injector();
    let root = args.path.clone();

    std::thread::spawn(move || {
        let logical_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let mut builder = WalkBuilder::new(root);
        builder
            .hidden(!args.hidden)
            .git_ignore(true)
            .threads(std::cmp::min(8, logical_cores));
        builder.build_parallel().run(|| {
            let local_injector = injector.clone();
            let mut batch = Vec::with_capacity(5000);
            Box::new(move |result| {
                if let Ok(entry) = result {
                    if entry.file_type().map(|f| f.is_file()).unwrap_or(false) {
                        if let Ok(abs_path) = std::fs::canonicalize(entry.path()) {
                            batch.push(abs_path.to_string_lossy().into_owned());
                        } else {
                            batch.push(entry.path().to_string_lossy().into_owned());
                        }

                        if batch.len() >= 5000 {
                            for path in batch.drain(..) {
                                local_injector.push(path, |s, dst| {
                                    dst[0] = s.as_str().into();
                                });
                            }
                        }
                    }
                }
                ignore::WalkState::Continue
            })
        });
    });

    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        matcher,
        list_state: ListState::default(),
        input: String::new(),
        last_input_time: Instant::now(),
        needs_reparse: false,
        show_preview: false,
        preview_scroll: 0,
        previous_input: String::new(),
        frame_count: 0,
        last_selected_index: None,
        preview_cache: Vec::new(),
        ps: SyntaxSet::load_defaults_newlines(),
        ts: ThemeSet::load_defaults(),
        theme: AppTheme::catppuccin(),
        show_theme_menu: false,
        theme_list_state: ListState::default(),
        relative: args.relative,
        base_path,
        case_sensitive: args.sensitive,
    };

    let result = app.run(&mut terminal);
    ratatui::restore();
    result
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        let mut last_draw = Instant::now();
        while !self.exit {
            let status = self.matcher.tick(0);
            let mut user_active = false;

            if event::poll(Duration::from_millis(1))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_input(key);
                        user_active = true;
                    }
                }
            }

            if self.needs_reparse && self.last_input_time.elapsed() > Duration::from_millis(10) {
                self.update_search();
                self.needs_reparse = false;
                user_active = true;
            }

            let now = Instant::now();
            let elapsed = now.duration_since(last_draw);
            let should_draw = if user_active {
                true
            } else if status.running || status.changed {
                elapsed > Duration::from_millis(33)
            } else {
                elapsed > Duration::from_millis(200)
            };

            if should_draw {
                terminal.draw(|frame| self.draw(frame))?;
                last_draw = now;
            }
            std::thread::sleep(Duration::from_millis(1));
        }
        Ok(())
    }

    fn handle_input(&mut self, key: event::KeyEvent) {
        let ctrl_only = key.modifiers == KeyModifiers::CONTROL;

        if self.show_theme_menu {
            match key.code {
                KeyCode::Esc => self.show_theme_menu = false,
                KeyCode::Char('t') if ctrl_only => self.show_theme_menu = false,
                KeyCode::Up => {
                    let i = self.theme_list_state.selected().unwrap_or(0);
                    self.theme_list_state.select(Some(i.saturating_sub(1)));
                }
                KeyCode::Down => {
                    let i = self.theme_list_state.selected().unwrap_or(0);
                    if i < 2 {
                        self.theme_list_state.select(Some(i + 1));
                    }
                }
                KeyCode::Enter => {
                    let i = self.theme_list_state.selected().unwrap_or(0);
                    self.theme = match i {
                        0 => AppTheme::nord(),
                        1 => AppTheme::dracula(),
                        _ => AppTheme::catppuccin(),
                    };
                    self.last_selected_index = None;
                    self.show_theme_menu = false;
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Esc => self.exit = true,
            KeyCode::Char('t') if ctrl_only => {
                self.show_theme_menu = true;
                self.theme_list_state.select(Some(0));
            }
            KeyCode::Char('p') if ctrl_only => {
                self.show_preview = !self.show_preview;
                self.preview_scroll = 0;
            }
            KeyCode::Char('u') if ctrl_only => {
                self.input.clear();
                self.needs_reparse = true;
                self.last_input_time = Instant::now();
            }
            KeyCode::Char('r') if ctrl_only => {
                self.relative = !self.relative;
            }
            KeyCode::Char('s') if ctrl_only => {
                self.case_sensitive = !self.case_sensitive;
                self.update_search();
            }
            KeyCode::Enter => {
                let snapshot = self.matcher.snapshot();
                if let Some(i) = self.list_state.selected() {
                    if let Some(item) = snapshot.get_matched_item(i as u32) {
                        let res = item.data.clone();
                        let _ = ratatui::restore();
                        println!("{}", res);
                        std::process::exit(0);
                    }
                }
            }
            KeyCode::Up => {
                if self.show_preview && self.preview_scroll > 0 {
                    self.preview_scroll = self.preview_scroll.saturating_sub(1);
                } else {
                    let i = self.list_state.selected().unwrap_or(0);
                    self.list_state.select(Some(i.saturating_sub(1)));
                }
            }
            KeyCode::Down => {
                if self.show_preview {
                    self.preview_scroll = self.preview_scroll.saturating_add(1);
                } else {
                    let i = self.list_state.selected().unwrap_or(0);
                    let count = self.matcher.snapshot().matched_item_count() as usize;
                    if i < count.saturating_sub(1) {
                        self.list_state.select(Some(i + 1));
                    }
                }
            }
            KeyCode::Char(c) => {
                self.input.push(c);
                self.needs_reparse = true;
                self.last_input_time = Instant::now();
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.needs_reparse = true;
                self.last_input_time = Instant::now();
            }
            _ => {}
        }
    }

    fn update_search(&mut self) {
        let can_append = self.input.starts_with(&self.previous_input);
        let case_matching = if self.case_sensitive {
            nucleo::pattern::CaseMatching::Respect
        } else {
            nucleo::pattern::CaseMatching::Ignore
        };
        self.matcher.pattern.reparse(
            0,
            &self.input,
            case_matching,
            nucleo::pattern::Normalization::Smart,
            can_append,
        );
        self.previous_input = self.input.clone();
        self.last_selected_index = None;
        self.list_state.select(Some(0));
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.frame_count = self.frame_count.wrapping_add(1);
        let t = self.theme.clone();
        let area = frame.area();
        let status = self.matcher.tick(0);
        let snapshot = self.matcher.snapshot();
        let matched = snapshot.matched_item_count();

        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

        let spinner =
            ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"][(self.frame_count / 2) % 10];
        let loading = if status.running {
            Span::styled(
                format!(" {}", spinner),
                Style::default().fg(t.spinner).bold(),
            )
        } else {
            Span::raw("  ")
        };

        frame.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled("> ", Style::default().fg(t.header).bold()),
                Span::raw(&self.input).fg(t.fg),
                loading,
                Span::styled(
                    format!(" ({}/{})", matched, snapshot.item_count()),
                    Style::default().fg(t.matched_count),
                ),
            ])),
            chunks[0],
        );

        let items = snapshot
            .matched_items(0..matched.min(chunks[1].height as u32))
            .map(|item| {
                let full_path = item.data.as_str();
                let (icon, color) = self.get_icon_info(full_path);

                let path_to_show = if self.relative {
                    std::path::Path::new(full_path)
                        .strip_prefix(&self.base_path)
                        .map(|p| p.to_string_lossy().into_owned())
                        .unwrap_or_else(|_| full_path.to_string())
                } else {
                    full_path.to_string()
                };

                let line = match path_to_show.rfind('/') {
                    Some(idx) => {
                        let dir = path_to_show[..idx + 1].to_string();
                        let file = path_to_show[idx + 1..].to_string();
                        Line::from(vec![
                            Span::styled(format!("{} ", icon), Style::default().fg(color)),
                            Span::styled(dir, Style::default().fg(t.matched_count)),
                            Span::styled(file, Style::default().fg(t.fg).bold()),
                        ])
                    }
                    None => Line::from(vec![
                        Span::styled(format!("{} ", icon), Style::default().fg(color)),
                        Span::styled(path_to_show, Style::default().fg(t.fg).bold()),
                    ]),
                };

                ListItem::new(line)
            });

        let list = List::new(items)
            .highlight_style(
                Style::default()
                    .bg(t.selection_bg)
                    .fg(t.selection_fg)
                    .bold(),
            )
            .highlight_symbol(Span::styled("  ", Style::default().fg(t.cursor)));
        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);

        let footer = Line::from(vec![
            Span::styled(
                " ESC ",
                Style::default()
                    .bg(Color::Rgb(191, 97, 106))
                    .fg(Color::Black)
                    .bold(),
            ),
            Span::raw(" Quit  ").fg(t.fg),
            Span::styled(
                " Ctrl-T ",
                Style::default().bg(t.header).fg(Color::Black).bold(),
            ),
            Span::raw(" Themes  ").fg(t.fg),
            Span::styled(
                " Ctrl-P ",
                Style::default()
                    .bg(Color::Rgb(235, 203, 139))
                    .fg(Color::Black)
                    .bold(),
            ),
            Span::raw(" Preview  ").fg(t.fg),
            Span::styled(
                " Ctrl-U ",
                Style::default()
                    .bg(Color::Rgb(180, 142, 173))
                    .fg(Color::Black)
                    .bold(),
            ),
            Span::raw(" Clear  ").fg(t.fg),
            Span::styled(
                " Ctrl-R ",
                Style::default()
                    .bg(if self.relative {
                        Color::Green
                    } else {
                        Color::DarkGray
                    })
                    .fg(Color::Black)
                    .bold(),
            ),
            Span::raw(if self.relative {
                " Rel ON "
            } else {
                " Rel OFF "
            })
            .fg(t.fg),
            Span::styled(
                " Ctrl-S ",
                Style::default()
                    .bg(if self.case_sensitive {
                        Color::Red
                    } else {
                        Color::DarkGray
                    })
                    .fg(Color::Black)
                    .bold(),
            ),
            Span::raw(if self.case_sensitive {
                " Case SEN "
            } else {
                " Case INS "
            })
            .fg(t.fg),
        ]);
        frame.render_widget(Paragraph::new(footer).bg(t.footer_bg), chunks[2]);

        if self.show_preview {
            if let Some(i) = self.list_state.selected() {
                if let Some(item) = snapshot.get_matched_item(i as u32) {
                    if Some(i) != self.last_selected_index {
                        self.preview_cache = self.read_file_preview_colored(item.data);
                        self.last_selected_index = Some(i);
                    }
                    let p_area = self.centered_rect(85, 80, area);
                    let block = Block::default()
                        .title(format!(" Preview: {} ", item.data))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(t.header))
                        .bg(t.bg);
                    frame.render_widget(Clear, p_area);
                    frame.render_widget(
                        Paragraph::new(self.preview_cache.clone())
                            .block(block)
                            .scroll((self.preview_scroll, 0))
                            .wrap(Wrap { trim: false }),
                        p_area,
                    );
                }
            }
        }

        if self.show_theme_menu {
            let t_area = self.centered_rect(30, 30, area);
            frame.render_widget(Clear, t_area);
            let themes = vec![
                ListItem::new("Nord"),
                ListItem::new("Dracula"),
                ListItem::new("Catppuccin"),
            ];
            let t_list = List::new(themes)
                .block(
                    Block::default()
                        .title(format!(" Themes [{}] ", t.name))
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(t.header)),
                )
                .highlight_style(Style::default().bg(t.selection_bg).fg(t.selection_fg))
                .highlight_symbol("> ");
            frame.render_stateful_widget(t_list, t_area, &mut self.theme_list_state);
        }
    }

    fn read_file_preview_colored(&self, path: &str) -> Vec<Line<'static>> {
        let syntax = self
            .ps
            .find_syntax_for_file(path)
            .unwrap_or(None)
            .unwrap_or_else(|| self.ps.find_syntax_plain_text());
        let mut h = HighlightLines::new(syntax, &self.ts.themes[self.theme.syntect_theme]);
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return vec![Line::from(" (Error) ".red())],
        };
        let mut lines = Vec::new();
        for res in BufReader::new(file)
            .lines()
            .take(300)
            .filter_map(Result::ok)
        {
            let line_nl = format!("{}\n", res);
            if let Ok(ranges) = h.highlight_line(&line_nl, &self.ps) {
                let spans = ranges
                    .into_iter()
                    .map(|(s, text)| {
                        let fg = s.foreground;
                        Span::styled(
                            text.trim_end_matches('\n').to_string(),
                            Style::default().fg(Color::Rgb(fg.r, fg.g, fg.b)),
                        )
                    })
                    .collect::<Vec<_>>();
                lines.push(Line::from(spans));
            }
        }
        lines
    }

    fn get_icon_info(&self, path: &str) -> (&'static str, Color) {
        let ext = path.rsplit('.').next().unwrap_or("");
        match ext.to_lowercase().as_str() {
            "rs" => ("", Color::Rgb(222, 165, 132)),
            "py" => ("", Color::Rgb(53, 114, 165)),
            "go" => ("", Color::Rgb(0, 173, 216)),
            "js" | "ts" => ("", Color::Rgb(241, 224, 90)),
            "md" => ("", Color::Rgb(136, 192, 208)),
            "toml" | "yaml" | "yml" => ("", Color::Rgb(180, 142, 173)),
            _ => ("󰈔", Color::Rgb(143, 188, 187)),
        }
    }

    fn centered_rect(&self, px: u16, py: u16, r: Rect) -> Rect {
        let v = Layout::vertical([
            Constraint::Percentage((100 - py) / 2),
            Constraint::Percentage(py),
            Constraint::Percentage((100 - py) / 2),
        ])
        .split(r);
        Layout::horizontal([
            Constraint::Percentage((100 - px) / 2),
            Constraint::Percentage(px),
            Constraint::Percentage((100 - px) / 2),
        ])
        .split(v[1])[1]
    }
}
