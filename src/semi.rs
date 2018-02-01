use core::fmt::Write;
use core::fmt::Result;

pub struct SemiHosting { }

pub const STDERR: SemiHosting = SemiHosting {};

impl SemiHosting {
    fn execute(&self, command: u32, message: &[u32; 3]) {
        unsafe {
            asm!(
                "mov r0, $0;
                mov r1, $1;
                bkpt #0xAB"
                : 
                : "r"(command), "r"(message)
                : "r0", "r1"
            );
        }
    }
}

impl Write for SemiHosting {
    fn write_str(&mut self, s: &str) -> Result {
        let message : [u32; 3] = [
            2,               // write to stderr
            s.as_ptr() as u32,
            s.len() as u32
        ];
        self.execute(5, &message);
        Ok(())
    }
}