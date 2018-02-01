use futures::{
    Future,
    Stream
};

pub trait SerialWriter<Word> {
    type Error;
    type WritingDoneFuture: Future<Item = usize, Error = Self::Error>;

    fn write(&self, data: Word) -> Self::WritingDoneFuture;
    fn write_all(&self, data: &'static [Word]) -> Self::WritingDoneFuture;

    fn write_sync(&self, data: Word) -> Result<usize, Self::Error>;
    fn write_all_sync(&self, data: &[Word]) -> Result<usize, Self::Error>;
}

pub trait SerialReader<'a, Word> {
    type Error: 'a;
    type ReadIterator: Iterator<Item = Result<Word, Self::Error>>;
    type ReadFuture: Future<Item = Word, Error = Self::Error> + 'a;
    type ReadStream: Stream<Item = Word, Error = Self::Error>;
    type ReadingDoneFuture: Future<Item = &'a mut [u16], Error = Self::Error>;

    fn read(&self) -> Self::ReadFuture;
    fn read_exact(&self, buff: &'static mut [Word]) -> Self::ReadingDoneFuture;
    fn read_stream(&self) -> Self::ReadStream;

    fn read_sync(&self) -> Result<Word, Self::Error>;
    fn read_exact_sync<'b>(&'a self, buff: &'b mut [Word]) -> Result<&'b mut [Word], Self::Error>;
    fn read_iter(&'a self) -> Self::ReadIterator;
}