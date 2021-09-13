use crate::frame::Frame;
use crate::frame::Drawable;
use std::time::Duration;
use rusty_time::timer::Timer;

pub struct Shot{
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

impl Shot{
    pub fn new(x: usize, y: usize) -> Self {
        Self{
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(50),
        }
    }
    pub fn update(&mut self, delta: Duration){
        self.timer.update(delta);
        if self.timer.ready && !self.exploding{
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explode(&mut self){
        self.exploding = true;
        self.timer = Timer::from_millis(250);
    }
    pub fn dead(&self) -> bool{
        // if exploded and timer has run out or we've reached the top of the screen
        (self.exploding && self.timer.ready) || (self.y == 0) // tail expression
    }
}

impl Drawable for Shot{
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = if self.exploding {"*"} else {"|"};
    }
}