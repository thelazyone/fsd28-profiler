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
const CARD_WIDTH: f64 = 240.0; // Reduced from 2.5 inches
const CARD_HEIGHT: f64 = 336.0; // Reduced from 3.5 inches
const MARGIN: f64 = 10.0;
const TITLE_SIZE: f64 = 20.0; // Slightly reduced
const SUBTITLE_SIZE: f64 = 16.0; // Slightly reduced
const TEXT_SIZE: f64 = 12.0; // Slightly reduced
const STAT_GRID_ROWS: usize = 2;
const STAT_GRID_COLS: usize = 3;
const DICE_SIZE: f64 = 24.0; // Reduced from 8mm

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
        ctx.set_font(&format!("{}px Arial", TITLE_SIZE));
        ctx.set_fill_style(&"black".into());
        ctx.fill_text(title, MARGIN, MARGIN + TITLE_SIZE).unwrap();

        ctx.set_font(&format!("{}px Arial", SUBTITLE_SIZE));
        ctx.fill_text(subtitle, MARGIN, MARGIN + TITLE_SIZE + SUBTITLE_SIZE).unwrap();
    }

    fn draw_stats_grid(&self, ctx: &CanvasRenderingContext2d, stats: &Characteristics) {
        let grid_start_y = MARGIN + TITLE_SIZE + SUBTITLE_SIZE + MARGIN;
        let cell_width = (CARD_WIDTH - 2.0 * MARGIN) / STAT_GRID_COLS as f64;
        let cell_height = 40.0;

        // Draw grid lines
        ctx.set_stroke_style(&"black".into());
        ctx.set_line_width(1.0);

        // Draw stats
        ctx.set_font(&format!("{}px Arial", TEXT_SIZE));
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
                let x = MARGIN + j as f64 * cell_width;
                let y = grid_start_y + i as f64 * cell_height;
                
                // Draw stat name
                ctx.fill_text(stat[0], x + 5.0, y + 20.0).unwrap();
                // Draw stat value
                ctx.fill_text(stat[1], x + 5.0, y + 35.0).unwrap();
            }
        }
    }

    fn draw_actions(&self, ctx: &CanvasRenderingContext2d, actions: &Vec<Action>) {
        let actions_start_y = MARGIN + TITLE_SIZE + SUBTITLE_SIZE + MARGIN + STAT_GRID_ROWS as f64 * 40.0 + MARGIN;
        let action_height = 60.0;

        for (i, action) in actions.iter().enumerate() {
            let y = actions_start_y + i as f64 * action_height;
            self.draw_action(ctx, action, y);
        }
    }

    fn draw_action(&self, ctx: &CanvasRenderingContext2d, action: &Action, y: f64) {
        // Draw S1, S2, S3 labels
        ctx.set_font(&format!("{}px Arial", TEXT_SIZE));
        ctx.fill_text("S1", MARGIN, y + 20.0).unwrap();
        ctx.fill_text("S2", MARGIN, y + 40.0).unwrap();
        ctx.fill_text("S3", MARGIN, y + 60.0).unwrap();

        // Draw dice boxes
        let mut x = MARGIN + 30.0;
        for cost in &action.cost.goon {
            self.draw_dice_box(ctx, x, y, cost);
            x += DICE_SIZE + 5.0;
        }

        // Draw action name and text
        ctx.fill_text(&action.name, x, y + 20.0).unwrap();
        ctx.fill_text(&action.text, x, y + 40.0).unwrap();
    }

    fn draw_dice_box(&self, ctx: &CanvasRenderingContext2d, x: f64, y: f64, range: &(u32, u32)) {
        ctx.set_stroke_style(&"black".into());
        ctx.set_line_width(1.0);
        ctx.stroke_rect(x, y, DICE_SIZE, DICE_SIZE);

        let text = if range.0 == range.1 {
            range.0.to_string()
        } else {
            format!("{}-{}", range.0, range.1)
        };
        ctx.fill_text(&text, x + DICE_SIZE/2.0 - 5.0, y + DICE_SIZE/2.0 + 5.0).unwrap();
    }

    fn draw_special_abilities(&self, ctx: &CanvasRenderingContext2d, abilities: &Vec<String>) {
        let abilities_start_y = MARGIN + TITLE_SIZE + SUBTITLE_SIZE + MARGIN + STAT_GRID_ROWS as f64 * 40.0 + MARGIN + 3.0 * 60.0 + MARGIN;
        
        ctx.set_font(&format!("{}px Arial", TEXT_SIZE));
        ctx.fill_text("Special Abilities:", MARGIN, abilities_start_y).unwrap();
        
        let abilities_text = abilities.join(", ");
        ctx.fill_text(&abilities_text, MARGIN, abilities_start_y + 20.0).unwrap();
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