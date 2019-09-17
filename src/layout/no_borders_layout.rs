extern crate wtftw;

use self::wtftw::layout::Layout;
use layout::with_borders_layout::WithBordersLayout;

pub struct NoBordersLayout;

impl NoBordersLayout {
    pub fn new(layout: Box<dyn Layout>) -> Box<dyn Layout> {
        WithBordersLayout::new(0, layout)
    }
}
