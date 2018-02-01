pub trait Plotter {
    type Error;
    type Operation;
    type FlushFuture: Future<Item = (), Error = Self::Error>;

    fn set_fill_color(&self);
    fn set_stroke_color(&self);

    fn move_to(&self) -> Self::Operation;

    fn line(&self, x: u16, y: u16) -> Self::Operation;
    fn curve(&self) -> Self::Operation;
    
    fn arc(&self, fill: bool) -> Self::Operation;
    fn rect(&self, fill: bool) -> Self::Operation;
    fn round_rect(&self, fill: bool) -> Self::Operation;
    fn triangle(&self, fill: bool) -> Self::Operation;

    fn draw_bitmap(&self) -> Self::Operation;

    fn flush(&self) -> Self::FlushFuture;
}