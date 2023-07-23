use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
    time::Duration,
};

use egui::{Color32, Sense, Slider, Stroke};

use day9::{get_directions, Direction, Position};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "AoC 2022 - Day9 part1",
        options,
        //Box::new(|_cc| Box::new(MyApp::<2>::init())),
        Box::new(|_cc| Box::new(MyApp::<10>::init())),
    )
    .unwrap();
}

struct MyApp<const N: usize> {
    directions: Box<dyn Iterator<Item = Direction>>,
    last_direction: Option<Direction>,
    knots: [Position; N],
    tail_positions: HashSet<Position>,
    // Display settings.
    paused: bool,
    speed: u8,
    show_sidebar: bool,
}

impl<const N: usize> MyApp<N> {
    fn init() -> Self {
        let input = std::fs::File::open("./src/input.txt").expect("correct input file");
        let reader = BufReader::new(input);
        Self::init_from(reader)
    }

    #[inline(always)]
    fn init_from(input: impl BufRead + 'static) -> Self {
        assert!(N > 1, "invalid N");
        let directions = get_directions(input);

        Self {
            directions: Box::new(directions),
            last_direction: None,
            knots: [Position::default(); N],
            tail_positions: HashSet::from([Position::default()]),
            paused: true,
            speed: 1,
            show_sidebar: true,
        }
    }

    fn step(&mut self) -> Option<()> {
        // Update head.
        self.last_direction = self.directions.next();
        let direction = self.last_direction?;
        self.knots[0] = self.knots[0].move_to(direction);
        let mut previous_knot_position = self.knots[0];
        // Update in between knots.
        for i in 1..N - 1 {
            let knot_pos = self.knots[i];
            let diff = previous_knot_position - knot_pos;
            self.knots[i] = knot_pos.move_delta(diff);
            previous_knot_position = self.knots[i];
        }
        // Update tail.
        let tail_pos = self.knots[N - 1];
        let diff = previous_knot_position - tail_pos;
        self.knots[N - 1] = tail_pos.move_delta(diff);
        self.tail_positions.insert(self.knots[N - 1]);
        Some(())
    }
}

impl<const N: usize> eframe::App for MyApp<N> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.style_mut().spacing.interact_size.y *= 1.4;
                ui.style_mut()
                    .text_styles
                    .get_mut(&egui::TextStyle::Button)
                    .unwrap()
                    .size *= 1.4;

                if ui.button("Reset").clicked() {
                    *self = Self::init();
                }
                if ui.button("Step").clicked() {
                    self.step();
                }

                let paused = self.paused;
                ui.toggle_value(&mut self.paused, if paused { "▶" } else { "⏸" });

                ui.toggle_value(&mut self.show_sidebar, "Sidebar");
            });

            ui.horizontal(|ui| {
                ui.label("Speed: ");
                ui.add(Slider::new(&mut self.speed, 1..=20).prefix("x"));
            });
        });

        if !self.paused {
            for _ in 0..self.speed {
                self.step();
            }
            ctx.request_repaint_after(Duration::from_millis(25));
        }

        if self.show_sidebar {
            egui::SidePanel::right("side_panel").show(ctx, |ui| {
                ui.label(format!("{} places visited", self.tail_positions.len()));
                egui::ScrollArea::new([false, true]).show(ui, |ui| {
                    if let Some(dir) = self.last_direction {
                        let arrow = match dir {
                            Direction::Up => "⬆",
                            Direction::Down => "⬇",
                            Direction::Right => "➡",
                            Direction::Left => "⬅",
                        };
                        ui.label(arrow);
                    }
                })
            });
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut painter_size = ui.available_size_before_wrap();
            if !painter_size.is_finite() {
                painter_size = egui::vec2(500.0, 500.0);
            }

            const SIDE: f32 = 5.0;

            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            let center = res.rect.center().to_vec2();

            let to_panel_pos = |pos: Position| {
                (egui::vec2(pos.x() as f32 * SIDE, pos.y() as f32 * SIDE * -1.0) + center).to_pos2()
            };

            let half_width = (painter_size.x / SIDE).floor() as i16;
            let half_height = (painter_size.y / SIDE).floor() as i16;

            for x in -half_width..half_width {
                for y in -half_height..half_height {
                    let dot = Position::new(x, y);
                    let color = if dot.x() == 0 && dot.y() == 0 {
                        Color32::WHITE
                    } else if self.tail_positions.contains(&dot) {
                        Color32::DARK_RED
                    } else {
                        continue;
                    };

                    let dot_pos = to_panel_pos(dot);
                    painter.circle_stroke(dot_pos, 1.0, Stroke::new(2.0, color));
                }
            }

            // paint the head
            let head_pos = to_panel_pos(self.knots[0]);
            painter.circle_stroke(head_pos, 2.0, Stroke::new(2.0, Color32::GREEN));

            for w in self.knots[0..N - 1].windows(2) {
                // paint the in between knot
                let knot_pos = to_panel_pos(w[1]);
                painter.circle_stroke(knot_pos, 2.0, Stroke::new(2.0, Color32::LIGHT_GRAY));
            }

            // paint the tail
            let tail_pos = to_panel_pos(self.knots[1]);
            painter.circle_stroke(tail_pos, 2.0, Stroke::new(2.0, Color32::YELLOW))
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part1() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut app = MyApp::<2>::init_from(reader);
        while app.step().is_some() {}
        assert_eq!(app.tail_positions.len(), 13);
    }

    #[test]
    fn test_part2() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut app = MyApp::<10>::init_from(reader);
        while app.step().is_some() {}
        assert_eq!(app.tail_positions.len(), 1);

        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let reader = BufReader::new(input.as_bytes());
        let mut app = MyApp::<10>::init_from(reader);
        while app.step().is_some() {}
        assert_eq!(app.tail_positions.len(), 36);
    }
}
