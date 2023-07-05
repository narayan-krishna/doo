use tui::layout::Constraint;

#[derive(Debug)]
pub enum LayoutHorizontal {
    Full,
    Right,
    Center,
    Left,
}

impl LayoutHorizontal {
    pub fn constraints(&self) -> [Constraint; 3] {
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

    pub fn block_index(&self) -> usize {
        match self {
            LayoutHorizontal::Full => 0,
            LayoutHorizontal::Right => 1,
            LayoutHorizontal::Center => 1,
            LayoutHorizontal::Left => 0,
        }
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "full" => LayoutHorizontal::Full,
            "right" => LayoutHorizontal::Right,
            "left" => LayoutHorizontal::Left,
            _ => LayoutHorizontal::Center,
        }
    }
}

#[derive(Debug)] 
pub enum LayoutVertical {
    Full,
    Top,
    Center,
    Bottom,
}

impl LayoutVertical {
    pub fn constraints(&self) -> [Constraint; 3] {
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
            LayoutVertical::Center => [
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

    pub fn block_index(&self) -> usize {
        match self {
            LayoutVertical::Full => 0,
            LayoutVertical::Top => 0,
            LayoutVertical::Center => 1,
            LayoutVertical::Bottom => 1,
        }
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "full" => LayoutVertical::Full,
            "top" => LayoutVertical::Top,
            "bottom" => LayoutVertical::Bottom,
            _ => LayoutVertical::Center,
        }
    }
}

