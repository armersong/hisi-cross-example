pub trait IoHandler: std::io::Write + std::io::Read {

}

pub struct IoTest {

}

impl std::io::Write for IoTest {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Ok(0)
    }

    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}

impl std::io::Read for IoTest {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Ok(0)
    }
}

impl IoHandler for IoTest {
}