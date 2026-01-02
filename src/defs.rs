pub const SCREEN_WIDTH: i32 = 800;
pub const SCREEN_HEIGHT: i32 = 600;

#[derive(PartialEq, Clone, Copy)]
pub enum Scene {
    Boot,
    Menu,
    TransitionToDesktop,
    Desktop,
    CombatTransition,
    Combat,
    KernelPanic,
    AyasofyaInside,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Language {
    English,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Front,
    Left,
    Right,
}
