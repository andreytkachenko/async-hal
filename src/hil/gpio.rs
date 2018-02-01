use futures::Stream;

pub trait Exti<'p> {
    type EdgeType;
    type Stream: Stream<Item = Self::EdgeType, Error = ()>;

    fn trigger_on_rising(&self);
    fn trigger_on_falling(&self);
    fn trigger_on_both(&self);
    fn stream(&'p self) -> Self::Stream;
}

pub trait ExtiPin<'p> {
    type Exti: Exti<'p>;
    fn exti(&'p self) -> Option<Self::Exti>;
}

pub trait BasicPin {
    fn make_input(&self);
    fn make_output(&self);
    fn set_high(&self);
    fn set_low(&self);
    fn toggle(&self);
    fn read(&self) -> bool;
}

pub trait AdvancedPin {
    type OutputSpeed;

    fn make_pull_up(&self);
    fn make_pull_down(&self);
    fn make_pull_none(&self);
    fn set_output_speed(&self, speed: Self::OutputSpeed);
    fn make_open_drain_output(&self);
    fn make_push_pull_output(&self);
}