use yew::prelude::*;
use fsd28_lib::models::{
    profile::Profile,
    characteristics::Characteristics,
    action::Action,
    damage_chart::DamageChart,
    damage_chart::Color,
};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, console};
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

            // Get the final profile with modifiers applied
            let final_profile = profile.get_final_profile();

            // Draw card background
            ctx.set_fill_style(&"white".into());
            ctx.fill_rect(0.0, 0.0, CARD_WIDTH, CARD_HEIGHT);

            // Draw title and subtitle
            self.draw_title(ctx, &final_profile.name, &final_profile.description);

            // Draw stats grid
            self.draw_stats_grid(ctx, &final_profile.characteristics);

            // Draw actions
            self.draw_actions(ctx, &final_profile.actions);

            // Draw special abilities
            self.draw_special_abilities(ctx, &final_profile.special_abilities);

            // Draw damage chart
            self.draw_damage_chart(ctx, &final_profile.damage_chart);
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
        let chars_per_line = (max_width / (font_size * 0.6)) as usize;
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
        let mut current_y = actions_start_y;

        for (i, action) in actions.iter().enumerate() {
            // Calculate the height needed for this action
            let action_height = self.calculate_action_height(ctx, action);
            
            // Draw the S1, S2, S3 label centered vertically and rotated
            ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", TEXT_SIZE));
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let label = format!("S{}", i + 1);
            
            // Save context, rotate, draw text, restore
            ctx.save();
            ctx.translate(MARGIN + 15.0, current_y + action_height/2.0).unwrap();
            ctx.rotate(-std::f64::consts::PI / 2.0).unwrap();
            ctx.fill_text(&label, 0.0, 0.0).unwrap();
            ctx.restore();
            
            // Draw the action at the calculated position
            self.draw_action(ctx, action, current_y);
            
            // Move to the next action position
            current_y += action_height + MARGIN;
        }
    }

    fn calculate_action_height(&self, ctx: &CanvasRenderingContext2d, action: &Action) -> f64 {
        // Calculate text height
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_TITLE_SIZE));
        let title_height = ACTION_TITLE_SIZE * LINE_HEIGHT;
        
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_DESCRIPTION_SIZE));
        let description_height = self.calculate_wrapped_text_height(ctx, &action.text, 
            CARD_WIDTH - (MARGIN + 60.0 + action.cost.goon.len() as f64 * (DICE_BOX_SIZE + 10.0) + 20.0 + MARGIN), 
            ACTION_DESCRIPTION_SIZE);

        // Return the maximum of box height and text height, plus some padding
        f64::max(DICE_BOX_SIZE, title_height + description_height) + 20.0
    }

    fn calculate_wrapped_text_height(&self, ctx: &CanvasRenderingContext2d, text: &str, max_width: f64, font_size: f64) -> f64 {
        let chars_per_line = (max_width / (font_size * 0.6)) as usize;
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_line = String::new();
        let mut line_count = 1;
        
        for word in words {
            if current_line.is_empty() {
                current_line = word.to_string();
            } else {
                let test_line = format!("{} {}", current_line, word);
                if test_line.len() <= chars_per_line {
                    current_line = test_line;
                } else {
                    line_count += 1;
                    current_line = word.to_string();
                }
            }
        }
        
        line_count as f64 * font_size * LINE_HEIGHT
    }

    fn draw_action(&self, ctx: &CanvasRenderingContext2d, action: &Action, y: f64) {
        let action_height = self.calculate_action_height(ctx, action);
        let vertical_center = y + action_height/2.0;
        
        // Draw cost boxes or FREE
        let mut x_cursor = MARGIN + 30.0;
        if action.cost.goon.is_empty() {
            // Draw FREE in a box
            self.draw_dice_box(ctx, x_cursor, vertical_center - DICE_BOX_SIZE/2.0, &(0, 0));
            x_cursor += DICE_BOX_SIZE + 10.0;
        } else {
            for cost in &action.cost.goon {
                self.draw_dice_box(ctx, x_cursor, vertical_center - DICE_BOX_SIZE/2.0, cost);
                x_cursor += DICE_BOX_SIZE + 10.0;
            }
        }

        // Calculate total text height for proper vertical centering
        let text_width = if action.slot {
            // If there's a prepared box, leave space for it. No extra margins needed.
            CARD_WIDTH - MARGIN - DICE_BOX_SIZE - x_cursor
        } else {
            // If no prepared box, use full width
            CARD_WIDTH - MARGIN - x_cursor 
        };
        
        // Calculate text block height
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_TITLE_SIZE));
        let title_height = ACTION_TITLE_SIZE * LINE_HEIGHT;
        
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_DESCRIPTION_SIZE));
        let description_height = self.calculate_wrapped_text_height(ctx, &action.text, text_width, ACTION_DESCRIPTION_SIZE);
        
        let total_text_height = title_height + description_height;
        let text_start_y = vertical_center - total_text_height/2.0;

        // Draw title
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_TITLE_SIZE));
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        ctx.fill_text(&action.name, x_cursor, text_start_y).unwrap();

        // Draw description
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_DESCRIPTION_SIZE));
        self.draw_wrapped_text(ctx, &action.text, x_cursor, 
            text_start_y + title_height, 
            text_width, 
            ACTION_DESCRIPTION_SIZE);

        // Draw prepared box if needed
        if action.slot {
            self.draw_prepared_box(ctx, CARD_WIDTH - MARGIN - DICE_BOX_SIZE, vertical_center - DICE_BOX_SIZE/2.0);
        }
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

        let text = if range.0 == 0 && range.1 == 0 {
            "FREE".to_string()
        } else if range.0 == range.1 {
            range.0.to_string()
        } else {
            format!("{}-{}", range.0, range.1)
        };

        // Center the cost number in the box
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_COST_SIZE));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        
        if range.0 == 0 && range.1 == 0 {
            // Save the current context state
            ctx.save();
            // Move to the center of the box
            ctx.translate(x + DICE_BOX_SIZE/2.0, y + DICE_BOX_SIZE/2.0).unwrap();
            // Rotate 45 degrees counter-clockwise
            ctx.rotate(-std::f64::consts::PI / 4.0).unwrap();
            // Draw the text at the new origin
            ctx.fill_text(&text, 0.0, 0.0).unwrap();
            // Restore the context state
            ctx.restore();
        } else {
            ctx.fill_text(&text, x + DICE_BOX_SIZE/2.0, y + DICE_BOX_SIZE/2.0).unwrap();
        }
    }

    fn draw_prepared_box(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64) {
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
    }

    fn draw_special_abilities(&self, ctx: &CanvasRenderingContext2d, abilities: &Vec<String>) {
        if abilities.is_empty() {
            return;
        }

        // Position above the damage profile numbers
        let abilities_start_y = CARD_HEIGHT - 60.0 - 20.0;
        
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", ACTION_TITLE_SIZE));
        ctx.set_text_align("center"); // Center the text
        
        let abilities_text = abilities.join(", ");
        let text_height = self.calculate_wrapped_text_height(ctx, &abilities_text, 
            CARD_WIDTH - 2.0 * MARGIN, ACTION_TITLE_SIZE);
        
        // Draw the text aligned to the bottom and centered
        self.draw_wrapped_text(ctx, &abilities_text, CARD_WIDTH/2.0, 
            abilities_start_y - text_height,
            CARD_WIDTH - 2.0 * MARGIN, 
            ACTION_TITLE_SIZE);
    }

    fn draw_damage_chart(&self, ctx: &CanvasRenderingContext2d, chart: &DamageChart) {
        let chart_start_y = CARD_HEIGHT - 60.0;
        let column_width = (CARD_WIDTH - 2.0 * MARGIN) / 6.0;
        const SLOT_PADDING: f64 = 5.0; // Padding between slots
        const BAR_HEIGHT: f64 = 80.0; // Increased from 30.0 to 40.0

        // Draw numbers 1-6
        ctx.set_font(&format!("bold {}px Arial", TEXT_SIZE));
        for i in 1..=6 {
            let x = MARGIN + (i - 1) as f64 * column_width + column_width/2.0 - 5.0;
            ctx.fill_text(&i.to_string(), x, chart_start_y - 20.0).unwrap(); // Moved numbers up by 20px
        }

        // Draw colored intervals
        let mut current_x = MARGIN;
        for (span, color, text) in &chart.intervals {
            let width = *span as f64 * column_width;
            self.draw_damage_interval(ctx, current_x, chart_start_y, width, color, text, SLOT_PADDING, BAR_HEIGHT);
            current_x += width;
        }
    }

    fn draw_damage_interval(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, width: f64, color: &Color, text: &str, padding: f64, height: f64) {
        let color_str = match color {
            Color::Red => "#951c07",
            Color::Yellow => "#ab7a1e",
            Color::Green => "#5a7e26",
        };
        const EFFECT_SIZE: f64 = 42.0; // Increased from 30.0 to 40.0
        
        // Draw the colored rectangle with padding
        ctx.set_fill_style(&color_str.into());
        ctx.fill_rect(x + padding, y + padding, width - 2.0 * padding, 200.0);

        // Draw the text centered both horizontally and vertically
        ctx.set_fill_style(&"white".into());
        ctx.set_font(&format!("bold {}px 'Trebuchet MS', sans-serif", EFFECT_SIZE));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        ctx.fill_text(&text.to_uppercase(), x + width/2.0, y + height/2.0 - 10.).unwrap();
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