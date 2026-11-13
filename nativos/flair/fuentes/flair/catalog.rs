pub const ID: &str = "flair";
pub const VERSION: &str = "1.0.1";
pub const DESCRIPTION: &str = "A cosmetics catalog: animated capes, hats, wings, shoulder pets and auras, and your own. Only you see them.";
pub const HEADER: f32 = 24.0;
pub const SIDEBAR: f32 = 92.0;
pub const TAB_H: f32 = 20.0;
pub const SIDE_W: f32 = 140.0;
pub const CARD_W: f32 = 62.0;
pub const CARD_H: f32 = 74.0;
pub const GAP: f32 = 6.0;
pub const PAD: f32 = 8.0;
pub const ROW_H: f32 = 16.0;
pub const WEARING_TITLE: f32 = 16.0;
pub const CLOSE: f32 = 16.0;
pub const SCROLL_STEP: f32 = 32.0;
const MARGIN: f32 = 12.0;
const MAX_W: f32 = 620.0;
const MAX_H: f32 = 340.0;
const SLOTS: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hit {
    Close,
    HideArmor,
    Tab(usize),
    Card(usize),
    Remove(usize),
    FirstPerson(usize),
    Theme(usize),
    Preview,
    Panel,
    Outside,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Layout {
    pub panel: [f32; 4],
    pub content: [f32; 4],
    pub side: [f32; 4],
    pub preview: Option<[f32; 4]>,
    pub wearing: [f32; 4],
    pub columns: usize,
    grid_x: f32,
}

pub fn inside(rect: [f32; 4], p: [f32; 2]) -> bool {
    p[0] >= rect[0] && p[0] < rect[0] + rect[2] && p[1] >= rect[1] && p[1] < rect[1] + rect[3]
}

pub fn clip(rect: [f32; 4], to: [f32; 4]) -> Option<[f32; 4]> {
    let (x0, y0) = (rect[0].max(to[0]), rect[1].max(to[1]));
    let (x1, y1) = ((rect[0] + rect[2]).min(to[0] + to[2]), (rect[1] + rect[3]).min(to[1] + to[3]));
    (x1 > x0 && y1 > y0).then_some([x0, y0, x1 - x0, y1 - y0])
}

impl Layout {
    pub fn new(width: f32, height: f32, preview: bool) -> Self {
        let (w, h) = ((width - 2.0 * MARGIN).min(MAX_W), (height - 2.0 * MARGIN).min(MAX_H));
        let (x, y) = (((width - w) / 2.0).floor(), ((height - h) / 2.0).floor());
        let side = [x + w - SIDE_W, y + HEADER, SIDE_W, h - HEADER];
        let wearing_h = WEARING_TITLE + SLOTS as f32 * ROW_H + PAD;
        let wearing = [side[0], side[1] + side[3] - wearing_h, SIDE_W, wearing_h];
        let content = [x + SIDEBAR + PAD, y + HEADER + PAD, w - SIDEBAR - SIDE_W - 2.0 * PAD, h - HEADER - 2.0 * PAD];
        let columns = (((content[2] + GAP) / (CARD_W + GAP)).floor() as usize).max(1);
        let grid_w = columns as f32 * (CARD_W + GAP) - GAP;
        Layout {
            panel: [x, y, w, h],
            content,
            side,
            preview: preview.then_some([side[0], side[1], SIDE_W, side[3] - wearing_h]),
            wearing,
            columns,
            grid_x: content[0] + ((content[2] - grid_w) / 2.0).floor(),
        }
    }

    pub fn header(&self) -> [f32; 4] {
        [self.panel[0], self.panel[1], self.panel[2], HEADER]
    }

    pub fn sidebar(&self) -> [f32; 4] {
        [self.panel[0], self.panel[1] + HEADER, SIDEBAR, self.panel[3] - HEADER]
    }

    pub fn close(&self) -> [f32; 4] {
        [self.panel[0] + self.panel[2] - CLOSE - 4.0, self.panel[1] + (HEADER - CLOSE) / 2.0, CLOSE, CLOSE]
    }

    pub fn tab(&self, i: usize) -> [f32; 4] {
        [self.panel[0] + 4.0, self.panel[1] + HEADER + 6.0 + i as f32 * (TAB_H + 2.0), SIDEBAR - 8.0, TAB_H]
    }

    pub fn hide_armor(&self) -> [f32; 4] {
        [self.panel[0] + 4.0, self.panel[1] + self.panel[3] - TAB_H - 6.0, SIDEBAR - 8.0, TAB_H]
    }

    pub fn card(&self, index: usize, scroll: f32) -> [f32; 4] {
        let (col, row) = ((index % self.columns) as f32, (index / self.columns) as f32);
        [self.grid_x + col * (CARD_W + GAP), self.content[1] + row * (CARD_H + GAP) - scroll, CARD_W, CARD_H]
    }

    pub fn max_scroll(&self, cards: usize) -> f32 {
        let rows = cards.div_ceil(self.columns) as f32;
        (rows * (CARD_H + GAP) - GAP - self.content[3]).max(0.0)
    }

    pub fn visible(&self, cards: usize, scroll: f32) -> std::ops::Range<usize> {
        let step = CARD_H + GAP;
        let first = (scroll / step).floor().max(0.0) as usize * self.columns;
        let last = (((scroll + self.content[3]) / step).floor() as usize + 1) * self.columns;
        first.min(cards)..last.min(cards)
    }

    pub fn wearing_row(&self, k: usize) -> [f32; 4] {
        [self.wearing[0] + 4.0, self.wearing[1] + WEARING_TITLE + k as f32 * ROW_H, SIDE_W - 8.0, ROW_H - 2.0]
    }

    pub fn remove(&self, k: usize) -> [f32; 4] {
        let r = self.wearing_row(k);
        [r[0] + r[2] - 13.0, r[1] + 1.0, 12.0, 12.0]
    }

    pub fn first_person(&self, k: usize) -> [f32; 4] {
        let r = self.wearing_row(k);
        [r[0] + r[2] - 33.0, r[1] + 1.0, 18.0, 12.0]
    }

    pub fn theme(&self, k: usize) -> [f32; 4] {
        let r = self.wearing_row(k);
        [r[0] + r[2] - 49.0, r[1] + 1.0, 12.0, 12.0]
    }

    pub fn hit(&self, p: [f32; 2], tabs: usize, cards: usize, scroll: f32, worn: &[(usize, bool, bool)]) -> Hit {
        if !inside(self.panel, p) {
            return Hit::Outside;
        }
        if inside(self.close(), p) {
            return Hit::Close;
        }
        if inside(self.hide_armor(), p) {
            return Hit::HideArmor;
        }
        if let Some(i) = (0..tabs).find(|i| inside(self.tab(*i), p)) {
            return Hit::Tab(i);
        }
        for (k, (slot, toggles, themed)) in worn.iter().enumerate() {
            if inside(self.remove(k), p) {
                return Hit::Remove(*slot);
            }
            if *toggles && inside(self.first_person(k), p) {
                return Hit::FirstPerson(*slot);
            }
            if *themed && inside(self.theme(k), p) {
                return Hit::Theme(*slot);
            }
        }
        if self.preview.is_some_and(|r| inside(r, p)) {
            return Hit::Preview;
        }
        if !inside(self.content, p) {
            return Hit::Panel;
        }
        self.visible(cards, scroll).find(|i| inside(self.card(*i, scroll), p)).map_or(Hit::Panel, Hit::Card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn center(rect: [f32; 4]) -> [f32; 2] {
        [rect[0] + rect[2] / 2.0, rect[1] + rect[3] / 2.0]
    }

    #[test]
    fn everything_stays_inside_the_panel() {
        for (w, h) in [(427.0, 240.0), (480.0, 270.0), (640.0, 360.0), (960.0, 540.0)] {
            let layout = Layout::new(w, h, true);
            let [x, y, pw, ph] = layout.panel;
            assert!(x >= 0.0 && y >= 0.0 && x + pw <= w && y + ph <= h);
            assert!(layout.card(layout.columns - 1, 0.0)[0] + CARD_W <= layout.content[0] + layout.content[2] + 0.5, "{w}x{h}");
            assert!(layout.content[0] + layout.content[2] <= layout.side[0]);
            assert!(layout.tab(4)[1] + TAB_H <= layout.hide_armor()[1]);
            assert!(layout.preview.unwrap()[3] > 60.0, "{w}x{h}");
            assert!(layout.wearing_row(4)[1] + ROW_H <= y + ph + 0.5);
        }
    }

    #[test]
    fn a_partial_row_shows_and_scrolling_reaches_the_end() {
        let layout = Layout::new(640.0, 360.0, true);
        let cards = layout.columns * 6;
        let shown = layout.visible(cards, 0.0);
        let last = layout.card(shown.end - 1, 0.0);
        assert!(last[1] < layout.content[1] + layout.content[3] && last[1] + CARD_H > layout.content[1] + layout.content[3]);
        let max = layout.max_scroll(cards);
        let bottom = layout.card(cards - 1, max);
        assert!((bottom[1] + CARD_H - (layout.content[1] + layout.content[3])).abs() < 0.01);
        assert_eq!(layout.max_scroll(1), 0.0);
        assert_eq!(clip([0.0, 0.0, 10.0, 10.0], [5.0, 5.0, 10.0, 10.0]), Some([5.0, 5.0, 5.0, 5.0]));
        assert_eq!(clip([0.0, 0.0, 1.0, 1.0], [5.0, 5.0, 1.0, 1.0]), None);
    }

    #[test]
    fn hits_find_every_control() {
        let layout = Layout::new(640.0, 360.0, true);
        let worn = [(1, false, false), (4, true, true)];
        assert_eq!(layout.hit(center(layout.card(0, 0.0)), 5, 20, 0.0, &worn), Hit::Card(0));
        assert_eq!(layout.hit(center(layout.card(layout.columns, 30.0)), 5, 20, 30.0, &worn), Hit::Card(layout.columns));
        assert_eq!(layout.hit(center(layout.tab(2)), 5, 20, 0.0, &worn), Hit::Tab(2));
        assert_eq!(layout.hit(center(layout.close()), 5, 20, 0.0, &worn), Hit::Close);
        assert_eq!(layout.hit(center(layout.hide_armor()), 5, 20, 0.0, &worn), Hit::HideArmor);
        assert_eq!(layout.hit(center(layout.remove(0)), 5, 20, 0.0, &worn), Hit::Remove(1));
        assert_eq!(layout.hit(center(layout.first_person(1)), 5, 20, 0.0, &worn), Hit::FirstPerson(4));
        assert_eq!(layout.hit(center(layout.first_person(0)), 5, 20, 0.0, &worn), Hit::Panel);
        assert_eq!(layout.hit(center(layout.theme(1)), 5, 20, 0.0, &worn), Hit::Theme(4));
        assert_eq!(layout.hit(center(layout.preview.unwrap()), 5, 20, 0.0, &worn), Hit::Preview);
        assert_eq!(layout.hit([1.0, 1.0], 5, 20, 0.0, &worn), Hit::Outside);
    }
}
