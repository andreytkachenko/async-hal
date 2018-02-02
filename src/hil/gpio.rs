use futures::Stream;

pub trait Exti: Stream<Error = ()> {
    type EdgeType;
    type Item = Self::EdgeType;

    fn trigger_on_rising(&self);
    fn trigger_on_falling(&self);
    fn trigger_on_both(&self);
    fn trigger_off(&self);
}

pub trait ExtiPin<'p> {
    type Exti: Exti;

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

    fn make_pulled_up(&self);
    fn make_pulled_down(&self);
    fn make_pulled_none(&self);
    fn set_output_speed(&self, speed: Self::OutputSpeed);
    fn make_output_open_drain(&self);
    fn make_output_push_pull(&self);
}