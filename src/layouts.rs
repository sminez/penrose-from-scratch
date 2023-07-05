//! Configuration of our layouts and custom layout algorithms
use penrose::{
    builtin::layout::{
        messages::{ExpandMain, ShrinkMain},
        transformers::ReflectHorizontal,
        MainAndStack,
    },
    core::layout::{Layout, LayoutStack, Message},
    extensions::layout::Tatami,
    pure::{geometry::Rect, Stack},
    stack, Xid,
};

const DEFAULT_CUTOFF: u32 = 40;

pub fn layouts() -> LayoutStack {
    stack!(
        conditional_on_odd_or_even(),
        Fibonacci::boxed_default(),
        ReflectHorizontal::wrap(Fibonacci::boxed_default()),
        MainAndStack::boxed_default(),
        Tatami::boxed(0.6, 0.1)
    )
}

/// Inspired by the Fibonacci layout available for dwm:
///   https://dwm.suckless.org/patches/fibonacci/
#[derive(Debug, Copy, Clone)]
pub struct Fibonacci {
    cutoff: u32,
    ratio: f32,
    ratio_step: f32,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self {
            cutoff: DEFAULT_CUTOFF,
            ratio: 0.5,
            ratio_step: 0.1,
        }
    }
}

impl Fibonacci {
    /// Create a new [Fibonacci] layout with a specified cutoff for the minimum
    /// dimensions allowed for a client.
    pub fn new(cutoff: u32, ratio: f32, ratio_step: f32) -> Self {
        Fibonacci {
            cutoff,
            ratio,
            ratio_step,
        }
    }

    /// Create a new [Fibonacci] layout as with `new` but returned as a trait
    /// object ready to be added to your [LayoutStack].
    pub fn boxed(cutoff: u32, ratio: f32, ratio_step: f32) -> Box<dyn Layout> {
        Box::new(Fibonacci::new(cutoff, ratio, ratio_step))
    }

    pub fn boxed_default() -> Box<dyn Layout> {
        Box::<Self>::default()
    }
}

impl Layout for Fibonacci {
    fn name(&self) -> String {
        "Fibo".to_string()
    }

    fn boxed_clone(&self) -> Box<dyn Layout> {
        Box::new(*self)
    }

    fn layout(&mut self, s: &Stack<Xid>, r: Rect) -> (Option<Box<dyn Layout>>, Vec<(Xid, Rect)>) {
        let n = s.len();
        let mut positions = Vec::with_capacity(n);
        let (mut r1, mut r2) = r
            .split_at_width_perc(self.ratio)
            .expect("self.ratio is in bounds due to logic in self.handle_message");

        for (i, id) in s.iter().enumerate() {
            let at_cutoff = i == n - 1 || r1.w <= self.cutoff || r1.h <= self.cutoff;
            if at_cutoff {
                if i % 2 == 0 {
                    r1.w += r2.w;
                } else {
                    r1.h += r2.h;
                }
            }

            positions.push((*id, r1));

            if at_cutoff {
                break;
            }

            if i % 2 == 0 {
                (r1, r2) = r2
                    .split_at_height_perc(self.ratio)
                    .expect("self.ratio is in bounds due to logic in self.handle_message");
            } else {
                (r1, r2) = r2
                    .split_at_width_perc(self.ratio)
                    .expect("self.ratio is in bounds due to logic in self.handle_message");
            }
        }

        (None, positions)
    }

    fn handle_message(&mut self, m: &Message) -> Option<Box<dyn Layout>> {
        if let Some(&ExpandMain) = m.downcast_ref() {
            self.ratio += self.ratio_step;
            if self.ratio > 1.0 {
                self.ratio = 1.0;
            }
        } else if let Some(&ShrinkMain) = m.downcast_ref() {
            self.ratio -= self.ratio_step;
            if self.ratio < 0.0 {
                self.ratio = 0.0;
            }
        };

        None
    }
}

fn conditional_on_odd_or_even() -> Box<dyn Layout> {
    Box::new(Conditional {
        left: Fibonacci::boxed_default(),
        right: MainAndStack::boxed_default(),
        should_use_left: |s, _| s.len() % 2 == 0,
        left_if_active: true,
    })
}

#[derive(Debug)]
struct Conditional {
    left: Box<dyn Layout>,
    right: Box<dyn Layout>,
    should_use_left: fn(&Stack<Xid>, Rect) -> bool,
    left_if_active: bool,
}

impl Layout for Conditional {
    fn name(&self) -> String {
        if self.left_if_active {
            format!("Cond<{}>", self.left.name())
        } else {
            format!("Cond<{}>", self.right.name())
        }
    }

    fn boxed_clone(&self) -> Box<dyn Layout> {
        Box::new(Self {
            left: self.left.boxed_clone(),
            right: self.right.boxed_clone(),
            should_use_left: self.should_use_left,
            left_if_active: self.left_if_active,
        })
    }

    fn layout(&mut self, s: &Stack<Xid>, r: Rect) -> (Option<Box<dyn Layout>>, Vec<(Xid, Rect)>) {
        self.left_if_active = (self.should_use_left)(s, r);
        if self.left_if_active {
            self.left.layout(s, r)
        } else {
            self.right.layout(s, r)
        }
    }

    fn handle_message(&mut self, m: &Message) -> Option<Box<dyn Layout>> {
        if self.left_if_active {
            self.left.handle_message(m)
        } else {
            self.right.handle_message(m)
        }
    }
}
