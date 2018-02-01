pub trait Color: IntoARGB + FromARGB;

pub trait FrameBufferLayer {
    type Error;
    type PixelFormat: PixelFormat;
    type Color: Color;
    type OperationFuture: Future<Item = (), Error = Self::Error>;

    fn set_pixel_format(&self, pixel: Self::PixelFormat);
    fn get_pixel_format(&self) -> Self::PixelFormat;

    fn put_pixel(&mut self, x: u16, y: u16, pixel: Self::Color);
    fn get_pixel(&self, x: u16, y: u16) -> Self::Color;

    fn get_width(&self) -> u16;
    fn get_height(&self) -> u16;

    fn fill(&mut self, x: u16, y: u16, width: u16, height: u16, color: Self::Color) -> Self::OperationFuture;
    fn clear(&mut self, color: Self::Color) -> Self::OperationFuture;

    fn draw_bitmap(&mut self) -> Self::OperationFuture;
    fn draw_pixels(&mut self) -> Self::OperationFuture;
}

pub trait FrameBuffer {
    type Error;
    type Layer: FrameBufferLayer;
    type OperationFuture: Future<Item = (), Error = Self::Error>;

    fn layer(&self, layer: u16) -> &mut Self::Layer;
    fn layer_count(&self) -> u16;
    fn set_layer_props(&self, layer: u16);
    fn bitblt(&self, layer_from: u16, x: u16, y: u16, w: u16, h: u16, layer_to: u16, x: u16, y: u16) -> Self::OperationFuture;
}