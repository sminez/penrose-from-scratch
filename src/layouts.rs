//! Configuration of our layouts and custom layout algorithms
use penrose::{
    builtin::layout::{
        messages::{ExpandMain, ShrinkMain},
        transformers::ReflectHorizontal,
        CenteredMain, MainAndStack,
    },
    core::layout::{Layout, LayoutStack, Message},
    extensions::layout::Tatami,
    pure::{geometry::Rect, Stack},
    stack, Xid,
};

const DEFAULT_CUTOFF: u32 = 40;

pub fn layouts() -> LayoutStack {
    stack!(
        flex_main(),
        odd_even(),
        Fibonacci::boxed_default(),
        ReflectHorizontal::wrap(Fibonacci::boxed_default()),
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

/// Switch between using one or two stack areas for additional windows depending on
/// whether there is sufficent screen space to ensure that things don't feel crowded.
///
/// > This was added after Ep08 where we implemented the [Conditional] struct and is
/// > the motivating behaviour for writing it in the first place!
fn flex_main() -> Box<dyn Layout> {
    Conditional::boxed(
        "FlexMain",
        MainAndStack::default(),
        CenteredMain::default(),
        |_, r| r.w <= 1400,
    )
}

/// A slightly silly layout where the [Layout] used flips between the [Fibonacci]
/// layout and [MainAndStack] depending on whether or not there are an even number
/// of clients in the workspace.
///
/// > This is only really here to easily demonstrate this thing working
fn odd_even() -> Box<dyn Layout> {
    Conditional::boxed(
        "Odd/Even",
        Fibonacci::default(),
        MainAndStack::default(),
        |s, _| s.len() % 2 == 0,
    )
}

/// Conditionally run one of two layouts based on a predicate function.
///
/// This struct implements [Layout] by selecting between the two provided layouts using
/// a predicate function. By default the left layout will be used, switching to the right
/// when the predicate returns false. Examples of predicate functions that might be useful are:
///   - When the screen size being laid out is smaller than a given threshold
///   - When there are more than a given number of clients that need to be laid out
///   - Based on the absolute position of the screen being laid out.
#[derive(Debug)]
pub struct Conditional {
    name: String,
    left: Box<dyn Layout>,
    right: Box<dyn Layout>,
    should_use_left: fn(&Stack<Xid>, Rect) -> bool,
    left_is_active: bool,
}

impl Conditional {
    /// Construct a new [Conditional] layout, selecting from one of two layouts based on
    /// a predicate function.
    pub fn new<L: Layout + 'static, R: Layout + 'static>(
        name: impl Into<String>,
        left: L,
        right: R,
        should_use_left: fn(&Stack<Xid>, Rect) -> bool,
    ) -> Self {
        Self {
            name: name.into(),
            left: Box::new(left),
            right: Box::new(right),
            should_use_left,
            left_is_active: true,
        }
    }

    /// Create a new [Conditional] layout as with `new` but returned as a trait
    /// object ready to be added to your layout stack in config.
    pub fn boxed<L: Layout + 'static, R: Layout + 'static>(
        name: impl Into<String>,
        left: L,
        right: R,
        should_use_left: fn(&Stack<Xid>, Rect) -> bool,
    ) -> Box<dyn Layout> {
        Box::new(Self::new(name, left, right, should_use_left))
    }
}

impl Layout for Conditional {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn boxed_clone(&self) -> Box<dyn Layout> {
        Box::new(Self {
            name: self.name.clone(),
            left: self.left.boxed_clone(),
            right: self.right.boxed_clone(),
            should_use_left: self.should_use_left,
            left_is_active: self.left_is_active,
        })
    }

    fn layout(&mut self, s: &Stack<Xid>, r: Rect) -> (Option<Box<dyn Layout>>, Vec<(Xid, Rect)>) {
        self.left_is_active = (self.should_use_left)(s, r);
        if self.left_is_active {
            self.left.layout(s, r)
        } else {
            self.right.layout(s, r)
        }
    }

    fn handle_message(&mut self, m: &Message) -> Option<Box<dyn Layout>> {
        if self.left_is_active {
            self.left.handle_message(m)
        } else {
            self.right.handle_message(m)
        }
    }
}
