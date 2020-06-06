
pub struct PositiveInteger(u64);

pub trait Weight {}

pub trait Cost {}

impl Weight for PositiveInteger {}
impl Cost for PositiveInteger {}


