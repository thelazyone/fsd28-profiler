use yew::prelude::*;
use fsd28_lib::models::{
    profile::Profile,
    characteristics::Characteristics,
    action::Action,
    damage_chart::DamageChart,
    damage_chart::Color,
};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use wasm_bindgen::JsCast;

#[derive(Properties, PartialEq)]
pub struct CardGeneratorProps {
    pub profile: Profile,
}

pub struct CardGenerator {
    canvas: NodeRef,
    ctx: Option<CanvasRenderingContext2d>,
}

// Constants for card dimensions and layout
const CARD_WIDTH: f64 = 750.0; // 2.5 inches at 300 DPI
const CARD_HEIGHT: f64 = 1050.0; // 3.5 inches at 300 DPI
const MARGIN: f64 = 30.0;
const TITLE_SIZE: f64 = 60.0;
const SUBTITLE_SIZE: f64 = 28.0;
const TEXT_SIZE: f64 = 24.0;
const ACTION_TITLE_SIZE: f64 = 36.0;
const ACTION_DESCRIPTION_SIZE: f64 = 30.0;
const ACTION_COUNTER_SIZE: f64 = 36.0;
const ACTION_COST_SIZE: f64 = 50.0;
const STAT_GRID_ROWS: usize = 2;
const STAT_GRID_COLS: usize = 3;
const DICE_BOX_SIZE_MM: f64 = 10.0; // 10mm
const DICE_BOX_SIZE: f64 = DICE_BOX_SIZE_MM * 11.81; // Convert mm to pixels at 300 DPI
const DICE_BOX_BORDER_WIDTH: f64 = 8.0;
const DICE_BOX_CORNER_RADIUS: f64 = 10.0;
const LINE_HEIGHT: f64 = 1.2; // Line height multiplier

impl CardGenerator {
    pub fn new() -> Self {
        Self {
            canvas: NodeRef::default(),
            ctx: None,
        }
    }

    pub fn initialize_canvas(&mut self) {
        if let Some(canvas) = self.canvas.cast::<HtmlCanvasElement>() {
            canvas.set_width(CARD_WIDTH as u32);
            canvas.set_height(CARD_HEIGHT as u32);
            self.ctx = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().ok();
        }
    }

    pub fn generate_card(&self, profile: &Profile) {
        if let Some(ctx) = &self.ctx {
            // Clear canvas
            ctx.clear_rect(0.0, 0.0, CARD_WIDTH, CARD_HEIGHT);

            // Draw card background
            ctx.set_fill_style(&"white".into());
            ctx.fill_rect(0.0, 0.0, CARD_WIDTH, CARD_HEIGHT);

            // Draw title and subtitle
            self.draw_title(ctx, &profile.name, &profile.description);

            // Draw stats grid
            self.draw_stats_grid(ctx, &profile.characteristics);

            // Draw actions
            self.draw_actions(ctx, &profile.actions);

            // Draw special abilities
            self.draw_special_abilities(ctx, &profile.special_abilities);

            // Draw damage chart
            self.draw_damage_chart(ctx, &profile.damage_chart);
        }
    }

    fn draw_title(&self, ctx: &CanvasRenderingContext2d, title: &str, subtitle: &str) {
        // Set up the font
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", TITLE_SIZE));
        ctx.set_fill_style(&"black".into());
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");

        // Calculate title position - moved down a bit
        let title_x = CARD_WIDTH / 2.0;
        let title_y = MARGIN + 20.0; // Added 20px padding from top

        // Draw title with word wrapping and all caps
        let uppercase_title = title.to_uppercase();
        self.draw_wrapped_text(ctx, &uppercase_title, title_x, title_y, CARD_WIDTH - 2.0 * MARGIN, TITLE_SIZE);

        // Draw subtitle - moved up and added small caps
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", SUBTITLE_SIZE));
        ctx.set_text_align("center");
        let subtitle_y = title_y + TITLE_SIZE * LINE_HEIGHT - 10.0; // Moved up by 10px
        let small_caps_subtitle = subtitle.to_uppercase();
        self.draw_wrapped_text(ctx, &small_caps_subtitle, title_x, subtitle_y, CARD_WIDTH - 2.0 * MARGIN, SUBTITLE_SIZE);
    }

    fn draw_wrapped_text(&self, ctx: &CanvasRenderingContext2d, text: &str, x: f64, y: f64, max_width: f64, font_size: f64) {
        // Fixed-width wrapping based on character count
        let chars_per_line = (max_width / (font_size * 0.6)) as usize; // Approximate characters per line
        let mut current_y = y;
        
        // Split text into words
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();
        
        for word in words {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else {
                let test_line = format!("{} {}", current_line, word);
                if test_line.len() <= chars_per_line {
                    current_line = test_line;
                } else {
                    // Draw current line and start new one
                    ctx.fill_text(&current_line, x, current_y).unwrap();
                    current_y += font_size * LINE_HEIGHT;
                    current_line = word.to_string();
                }
            }
        }
        
        // Draw the last line if there is one
        if !current_line.is_empty() {
            ctx.fill_text(&current_line, x, current_y).unwrap();
        }
    }

    fn draw_stats_grid(&self, ctx: &CanvasRenderingContext2d, stats: &Characteristics) {
        let grid_start_y = MARGIN + TITLE_SIZE * LINE_HEIGHT + SUBTITLE_SIZE * LINE_HEIGHT + 20.0;
        let cell_width = (CARD_WIDTH - 2.0 * MARGIN) / STAT_GRID_COLS as f64;
        let cell_height = 80.0;

        // Draw grid lines
        ctx.set_stroke_style(&"black".into());
        ctx.set_line_width(1.0);

        // Draw stats
        let stats_text = [
            ["Cmd", &stats.stat_cmd.to_string()],
            ["Def", &stats.stat_def.to_string()],
            ["Save", &stats.stat_save.display()],
            ["Move", &stats.stat_move.to_string()],
            ["Shoot", &stats.stat_shoot.display()],
            ["Melee", &stats.stat_melee.display()],
        ];

        for (i, row) in stats_text.chunks(STAT_GRID_COLS).enumerate() {
            for (j, stat) in row.iter().enumerate() {
                let x = MARGIN + j as f64 * cell_width + cell_width / 2.0;
                let y = grid_start_y + i as f64 * cell_height;
                
                // Draw stat name
                ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", TEXT_SIZE));
                ctx.fill_text(stat[0], x, y + 20.0).unwrap();
                
                // Draw stat value with larger font
                ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", TEXT_SIZE * 2.0));
                ctx.fill_text(stat[1], x, y + 50.0).unwrap();
            }
        }
    }

    fn draw_actions(&self, ctx: &CanvasRenderingContext2d, actions: &Vec<Action>) {
        let actions_start_y = MARGIN + TITLE_SIZE * LINE_HEIGHT + SUBTITLE_SIZE * LINE_HEIGHT + 20.0 + 
                              STAT_GRID_ROWS as f64 * 80.0 + MARGIN;
        let action_height = 120.0; // Increased to accommodate larger fonts

        for (i, action) in actions.iter().enumerate() {
            let y = actions_start_y + i as f64 * action_height;
            
            // Draw the S1, S2, S3 label for this action
            ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_COUNTER_SIZE));
            ctx.set_text_align("left");
            let label = format!("S{}", i + 1);
            ctx.fill_text(&label, MARGIN, y + 20.0).unwrap();
            
            self.draw_action(ctx, action, y);
        }
    }

    fn draw_action(&self, ctx: &CanvasRenderingContext2d, action: &Action, y: f64) {
        // Draw dice boxes and costs
        let mut x = MARGIN + 60.0;
        for cost in &action.cost.goon {
            self.draw_dice_box(ctx, x, y, cost);
            x += DICE_BOX_SIZE + 10.0;
        }

        // Draw action name
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_TITLE_SIZE));
        ctx.set_text_align("left");
        let text_x = x + 20.0;
        ctx.fill_text(&action.name, text_x, y + 20.0).unwrap();

        // Draw action description
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_DESCRIPTION_SIZE));
        self.draw_wrapped_text(ctx, &action.text, text_x, y + 60.0, CARD_WIDTH - text_x - MARGIN, ACTION_DESCRIPTION_SIZE);
    }

    fn draw_dice_box(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, range: &(u32, u32)) {
        ctx.set_stroke_style(&"black".into());
        ctx.set_line_width(DICE_BOX_BORDER_WIDTH);

        // Draw rounded rectangle
        ctx.begin_path();
        ctx.move_to(x + DICE_BOX_CORNER_RADIUS, y);
        ctx.line_to(x + DICE_BOX_SIZE - DICE_BOX_CORNER_RADIUS, y);
        ctx.quadratic_curve_to(x + DICE_BOX_SIZE, y, x + DICE_BOX_SIZE, y + DICE_BOX_CORNER_RADIUS);
        ctx.line_to(x + DICE_BOX_SIZE, y + DICE_BOX_SIZE - DICE_BOX_CORNER_RADIUS);
        ctx.quadratic_curve_to(x + DICE_BOX_SIZE, y + DICE_BOX_SIZE, x + DICE_BOX_SIZE - DICE_BOX_CORNER_RADIUS, y + DICE_BOX_SIZE);
        ctx.line_to(x + DICE_BOX_CORNER_RADIUS, y + DICE_BOX_SIZE);
        ctx.quadratic_curve_to(x, y + DICE_BOX_SIZE, x, y + DICE_BOX_SIZE - DICE_BOX_CORNER_RADIUS);
        ctx.line_to(x, y + DICE_BOX_CORNER_RADIUS);
        ctx.quadratic_curve_to(x, y, x + DICE_BOX_CORNER_RADIUS, y);
        ctx.close_path();
        ctx.stroke();

        let text = if range.0 == range.1 {
            range.0.to_string()
        } else {
            format!("{}-{}", range.0, range.1)
        };

        // Center the cost number in the box
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_COST_SIZE));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        ctx.fill_text(&text, x + DICE_BOX_SIZE/2.0, y + DICE_BOX_SIZE/2.0).unwrap();
    }

    fn draw_special_abilities(&self, ctx: &CanvasRenderingContext2d, abilities: &Vec<String>) {
        let abilities_start_y = MARGIN + TITLE_SIZE * LINE_HEIGHT + SUBTITLE_SIZE * LINE_HEIGHT + 20.0 + 
                              STAT_GRID_ROWS as f64 * 80.0 + MARGIN + 3.0 * 100.0 + MARGIN;
        
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", TEXT_SIZE));
        ctx.set_text_align("left");
        ctx.fill_text("Special Abilities:", MARGIN, abilities_start_y).unwrap();
        
        let abilities_text = abilities.join(", ");
        self.draw_wrapped_text(ctx, &abilities_text, MARGIN, abilities_start_y + 30.0, 
                             CARD_WIDTH - 2.0 * MARGIN, TEXT_SIZE);
    }

    fn draw_damage_chart(&self, ctx: &CanvasRenderingContext2d, chart: &DamageChart) {
        let chart_start_y = CARD_HEIGHT - 60.0;
        let column_width = (CARD_WIDTH - 2.0 * MARGIN) / 6.0;

        // Draw numbers 1-6
        ctx.set_font(&format!("{}px Arial", TEXT_SIZE));
        for i in 1..=6 {
            let x = MARGIN + (i - 1) as f64 * column_width + column_width/2.0 - 5.0;
            ctx.fill_text(&i.to_string(), x, chart_start_y).unwrap();
        }

        // Draw colored intervals
        let mut current_x = MARGIN;
        for (span, color, text) in &chart.intervals {
            let width = *span as f64 * column_width;
            self.draw_damage_interval(ctx, current_x, chart_start_y + 20.0, width, color, text);
            current_x += width;
        }
    }

    fn draw_damage_interval(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, color: &Color, text: &str) {
        let color_str = match color {
            Color::Red => "#FF0000",
            Color::Yellow => "#FFFF00",
            Color::Green => "#00FF00",
        };
        ctx.set_fill_style(&color_str.into());
        ctx.fill_rect(x, y, width, 30.0);

        ctx.set_fill_style(&"black".into());
        ctx.fill_text(text, x + width/2.0 - 10.0, y + 20.0).unwrap();
    }
}

impl Component for CardGenerator {
    type Message = ();
    type Properties = CardGeneratorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self::new()
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        self.initialize_canvas();
        self.generate_card(&ctx.props().profile);
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="card-container">
                <canvas ref={self.canvas.clone()} />
            </div>
        }
    }
} 