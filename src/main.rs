use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const DIGITS: [[&str; 5]; 10] = [
    [" ██████ ", "██    ██", "██    ██", "██    ██", " ██████ "],
    ["    ██  ", "  ████  ", "    ██  ", "    ██  ", "  ██████"],
    [" ██████ ", "██    ██", "   ████ ", " ██     ", "████████"],
    ["████████", "      ██", "  ████  ", "      ██", "████████"],
    ["██    ██", "██    ██", "████████", "      ██", "      ██"],
    ["████████", "██      ", "███████ ", "      ██", "███████ "],
    [" ██████ ", "██      ", "███████ ", "██    ██", " ██████ "],
    ["████████", "      ██", "    ██  ", "   ██   ", "   ██   "],
    [" ██████ ", "██    ██", " ██████ ", "██    ██", " ██████ "],
    [" ██████ ", "██    ██", " ███████", "      ██", " ██████ "],
];

const COLON: [&str; 5] = [
    "        ",
    "   ██   ",
    "        ",
    "   ██   ",
    "        ",
];

const ROWS: usize = 5;
const REFLECT_ROWS: usize = 3;

#[derive(Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn dim(self, factor: f64) -> Color {
        Color {
            r: (self.r as f64 * factor) as u8,
            g: (self.g as f64 * factor) as u8,
            b: (self.b as f64 * factor) as u8,
        }
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn gradient_color(stops: &[(f64, Color)], t: f64) -> Color {
    let t = t.clamp(0.0, 1.0);
    for w in stops.windows(2) {
        if t >= w[0].0 && t <= w[1].0 {
            let local = (t - w[0].0) / (w[1].0 - w[0].0);
            return Color {
                r: lerp(w[0].1.r as f64, w[1].1.r as f64, local) as u8,
                g: lerp(w[0].1.g as f64, w[1].1.g as f64, local) as u8,
                b: lerp(w[0].1.b as f64, w[1].1.b as f64, local) as u8,
            };
        }
    }
    stops.last().unwrap().1
}

#[derive(Clone)]
struct Cell {
    text: String,
    color: Color,
}

struct DigitGrid {
    cells: Vec<Vec<Cell>>,
}

fn build_digit_grid(time_str: &str, stops: &[(f64, Color)]) -> DigitGrid {
    let mut cells = vec![vec![]; ROWS];
    let total_width = 70.0;

    for (i, ch) in time_str.chars().enumerate() {
        let d = (ch as u8 - b'0') as usize;

        for row in 0..ROWS {
            let x0 = cells[row].len() as f64;
            if !cells[row].is_empty() {
                cells[row].push(Cell { text: " ".into(), color: Color { r: 0, g: 0, b: 0 } });
            }
            for (cx, c) in DIGITS[d][row].chars().enumerate() {
                let char_pos = (x0 + 1.0 + cx as f64) / total_width;
                cells[row].push(Cell { text: c.to_string(), color: gradient_color(stops, char_pos) });
            }
        }

        if i == 1 || i == 3 {
            for row in 0..ROWS {
                let x0 = cells[row].len() as f64;
                if !cells[row].is_empty() {
                    cells[row].push(Cell { text: " ".into(), color: Color { r: 0, g: 0, b: 0 } });
                }
                for (cx, c) in COLON[row].chars().enumerate() {
                    let char_pos = (x0 + 1.0 + cx as f64) / total_width;
                    cells[row].push(Cell { text: c.to_string(), color: gradient_color(stops, char_pos) });
                }
            }
        }
    }

    DigitGrid { cells }
}

fn render_line(cells: &[Cell]) -> String {
    let mut out = String::new();
    for cell in cells {
        out.push_str(&format!("\x1b[38;2;{};{};{}m{}", cell.color.r, cell.color.g, cell.color.b, cell.text));
    }
    out.push_str("\x1b[0m");
    out
}

fn render_line_dim(cells: &[Cell], row_from_bottom: usize) -> String {
    let factor = 1.0 - (row_from_bottom as f64 * 0.3);
    let factor = factor.max(0.1);
    let mut out = String::new();
    for cell in cells {
        let dc = cell.color.dim(factor);
        out.push_str(&format!("\x1b[38;2;{};{};{}m{}", dc.r, dc.g, dc.b, cell.text));
    }
    out.push_str("\x1b[0m");
    out
}

fn parse_hex(hex: &str) -> Option<Color> {
    let hex = hex.trim().trim_start_matches('#');
    if hex.len() != 6 { return None; }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(Color { r, g, b })
}

fn load_colors_from_cava() -> Option<Vec<(f64, Color)>> {
    let path = dirs().join(".config/cava/themes/your-theme");
    let content = std::fs::read_to_string(&path).ok()?;

    let mut colors: Vec<Color> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        for n in 1..=8 {
            if line.starts_with(&format!("gradient_color_{}", n)) {
                if let Some(c) = extract_hex_from_line(line) {
                    if n > colors.len() {
                        colors.resize(n, Color { r: 0, g: 0, b: 0 });
                    }
                    colors[n - 1] = c;
                }
            }
        }
    }

    if colors.is_empty() {
        return None;
    }
    if colors.len() == 1 {
        return Some(vec![(0.0, colors[0]), (1.0, colors[0])]);
    }

    let last = colors.len() - 1;
    Some(colors.iter().enumerate().map(|(i, c)| (i as f64 / last as f64, *c)).collect())
}

fn extract_hex_from_line(line: &str) -> Option<Color> {
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '#' {
            let mut hex = String::new();
            while let Some(&next) = chars.peek() {
                if next.is_ascii_hexdigit() {
                    hex.push(next);
                    chars.next();
                } else {
                    break;
                }
            }
            if hex.len() == 6 {
                return parse_hex(&hex);
            }
        }
    }
    None
}

fn default_stops() -> Vec<(f64, Color)> {
    vec![
        (0.00, Color { r: 125, g: 207, b: 255 }),
        (0.25, Color { r: 122, g: 162, b: 247 }),
        (0.50, Color { r: 187, g: 154, b: 247 }),
        (0.75, Color { r: 247, g: 118, b: 142 }),
        (1.00, Color { r: 255, g: 158, b: 100 }),
    ]
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    print!("\x1b[?1049h\x1b[?25l\x1b[2J");
    stdout.flush()?;

    let stops = load_colors_from_cava().unwrap_or_else(default_stops);

    loop {
        let (h, m, s) = get_time();
        let time_str = format!("{:02}{:02}{:02}", h, m, s);
        let grid = build_digit_grid(&time_str, &stops);

        print!("\x1b[1;1H\x1b[2J\x1b[?25l");

        println!();
        for row in 0..ROWS {
            println!("  {}  ", render_line(&grid.cells[row]));
        }

        // mirror separator
        let sep = stops.first().map(|c| c.1).unwrap_or(Color { r: 97, g: 93, b: 148 });
        println!("\x1b[38;2;{};{};{}m  ─────────────────────────────────────────────  \x1b[0m",
            (sep.r as u32 * 7 / 10) as u8,
            (sep.g as u32 * 7 / 10) as u8,
            (sep.b as u32 * 7 / 10) as u8,
        );

        for r in 0..REFLECT_ROWS {
            let src_row = ROWS - 1 - (r % ROWS);
            let cells = &grid.cells[src_row];
            println!("  {}  ", render_line_dim(cells, r + 1));
        }

        stdout.flush()?;
        thread::sleep(Duration::from_millis(500));
    }
}

fn get_time() -> (u32, u32, u32) {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let beijing = secs + 8 * 3600;
    let s = (beijing % 60) as u32;
    let m = ((beijing / 60) % 60) as u32;
    let h = ((beijing / 3600) % 24) as u32;
    (h, m, s)
}

fn dirs() -> std::path::PathBuf {
    std::env::var("HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
}
