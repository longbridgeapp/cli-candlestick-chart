use crate::{
    candle_set::CandleSet, chart::Candle, chart_renderer::ChartRenderer, info_bar::InfoBar,
    volume_pane::VolumePane, y_axis::YAxis,
};
use terminal_size::terminal_size;

#[derive(Debug, Clone)]
pub struct ChartData {
    pub main_candle_set: CandleSet,
    pub visible_candle_set: CandleSet,
    pub canvas_size: (u16, u16),
    pub height: i64,
}

impl ChartData {
    pub fn new(candles: Vec<Candle>) -> ChartData {
        let (terminal_width, terminal_height) = terminal_size().unwrap();
        ChartData::new_with_canvas_size(candles, (terminal_width.0, terminal_height.0))
    }

    pub fn new_with_canvas_size(candles: Vec<Candle>, canvas_size: (u16, u16)) -> ChartData {
        let (w, h) = canvas_size;

        let mut chart_data = ChartData {
            main_candle_set: CandleSet::new(candles),
            visible_candle_set: CandleSet::new(Vec::new()),
            canvas_size: (w, h),
            height: h as i64,
        };

        chart_data.compute_visible_candles();
        chart_data
    }

    pub fn compute_height(&mut self, info_bar: &InfoBar, volume_pane: &VolumePane) {
        let info_bar_height = if info_bar.enabled { InfoBar::HEIGHT } else { 0 };

        let volume_pane_height = if volume_pane.enabled {
            volume_pane.height
        } else {
            0
        };

        self.height = self.canvas_size.1 as i64
            - ChartRenderer::MARGIN_TOP
            - info_bar_height
            - volume_pane_height;
    }

    pub fn compute_visible_candles(&mut self) {
        let term_width = self.canvas_size.0 as usize as i64;
        let nb_candles = self.main_candle_set.candles.len();

        let nb_visible_candles = term_width - YAxis::WIDTH;

        self.visible_candle_set.set_candles(
            self.main_candle_set
                .candles
                .iter()
                .skip((nb_candles as i64 - nb_visible_candles as i64).max(0) as usize)
                .cloned()
                .collect::<Vec<Candle>>(),
        );
    }
}
