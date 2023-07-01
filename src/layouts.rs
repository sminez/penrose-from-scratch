//! Configuration of our layouts and custom layout algorithms
use penrose::{
    builtin::layout::MainAndStack,
    core::layout::{Layout, LayoutStack, Message},
    pure::{geometry::Rect, Stack},
    stack, Xid,
};

const DEFAULT_CUTOFF: u32 = 40;

pub fn layouts() -> LayoutStack {
    stack!(
        Fibonacci::boxed(DEFAULT_CUTOFF),
        Box::<MainAndStack>::default() as Box<dyn Layout>
    )
}

/// Inspired by the Fibonacci layout available for dwm:
///   https://dwm.suckless.org/patches/fibonacci/
#[derive(Debug, Copy, Clone)]
pub struct Fibonacci {
    cutoff: u32,
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self {
            cutoff: DEFAULT_CUTOFF,
        }
    }
}

impl Fibonacci {
    /// Create a new [Fibonacci] layout with a specified cutoff for the minimum
    /// dimensions allowed for a client.
    pub fn new(cutoff: u32) -> Self {
        Fibonacci { cutoff }
    }

    /// Create a new [Fibonacci] layout as with `new` but returned as a trait
    /// object ready to be added to your [LayoutStack].
    pub fn boxed(cutoff: u32) -> Box<dyn Layout> {
        Box::new(Fibonacci::new(cutoff))
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
        let (mut r1, mut r2) = r.split_at_mid_width();

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
                (r1, r2) = r2.split_at_mid_height();
            } else {
                (r1, r2) = r2.split_at_mid_width();
            }
        }

        (None, positions)
    }

    fn handle_message(&mut self, _: &Message) -> Option<Box<dyn Layout>> {
        // TODO: handle some of the standard messages that make sense for this layout
        None
    }
}
