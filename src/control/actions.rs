#[derive(Clone, Copy)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Skill(usize),
    None
}
