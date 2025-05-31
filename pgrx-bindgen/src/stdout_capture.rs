use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use std::thread;

pub struct StdoutCapture {
    /// File descriptor where the original STDOUT was pointing.
    orig_stdout_fd: RawFd,
    /// The JoinHandle of the background thread that is reading from the pipe.
    reader_handle: thread::JoinHandle<String>,
}

impl StdoutCapture {
    /// Start capturing everything sent to STDOUT.
    ///
    /// After calling `StdoutCapture::start()`, any call to `println!`, `write!`,
    /// etc. that goes to STDOUT will be funneled into an internal pipe.  When you
    /// later call `capture.finish()`, you'll get back a `String` containing
    /// everything that was written.
    pub fn start() -> io::Result<StdoutCapture> {
        // 1) dup the original STDOUT so we can restore it later.
        //
        //    `libc::dup(libc::STDOUT_FILENO)` returns a new fd that points to
        //    wherever STDOUT was pointing.  We must save that, or we'll lose it.
        let orig_stdout_fd = unsafe {
            let dup_fd = libc::dup(libc::STDOUT_FILENO);
            if dup_fd < 0 {
                return Err(io::Error::last_os_error());
            }
            dup_fd
        };

        // 2) make a pipe: fds = [ read_fd, write_fd ]
        let mut fds = [0i32; 2];
        let rc = unsafe { libc::pipe(fds.as_mut_ptr()) };
        if rc < 0 {
            // If pipe() failed, restore nothing; just return the error.
            return Err(io::Error::last_os_error());
        }
        let (read_fd, write_fd) = (fds[0], fds[1]);

        // 3) dup2(write_fd, STDOUT_FILENO) so that STDOUT now writes into the pipe.
        //    After dup2, close(write_fd) so that only STDOUT_FILENO remains to write
        //    into the pipe.  That way, when we later restore STDOUT and drop that fd,
        //    the pipe sees EOF.
        unsafe {
            if libc::dup2(write_fd, libc::STDOUT_FILENO) < 0 {
                return Err(io::Error::last_os_error());
            }
            libc::close(write_fd);
        }

        // 4) Spawn a thread that takes ownership of read_fd and reads until EOF.
        //    The thread will do File::from_raw_fd(read_fd) and read_to_end().
        let reader_handle = thread::spawn(move || {
            // SAFETY: `from_raw_fd(read_fd)` takes ownership; when `file` is dropped,
            // it will close the FD automatically.
            let mut file = unsafe { File::from_raw_fd(read_fd) };
            let mut buf = Vec::new();
            // Read until EOF.  When the write-end is closed, `read_to_end` returns Ok(0) or finishes.
            let _ = file.read_to_end(&mut buf);
            // Convert to String (replace invalid UTF-8 with U+FFFD).
            String::from_utf8_lossy(&buf).into_owned()
        });

        Ok(StdoutCapture { orig_stdout_fd, reader_handle })
    }

    /// Finish capturing and restore the original STDOUT.
    ///
    /// This will:
    /// 1) flush() stdout (so any buffered data makes it into the pipe)
    /// 2) dup2(orig_stdout_fd, STDOUT_FILENO) (which closes the pipe’s write end,
    ///    signaling EOF to the reader thread, and restores STDOUT back to what it was)
    /// 3) close(orig_stdout_fd)
    /// 4) join() the reader thread and return the captured String.
    pub fn finish(self) -> io::Result<String> {
        // 1) flush stdout so that all buffered writes hit the pipe
        io::stdout()
            .flush()
            .map_err(|e| io::Error::new(e.kind(), format!("flush failed: {}", e)))?;

        // 2) dup2(orig_stdout_fd, STDOUT_FILENO) restores STDOUT, closing the
        //    current STDOUT (which was the pipe’s write-end), thus sending EOF
        //    into the pipe.
        unsafe {
            if libc::dup2(self.orig_stdout_fd, libc::STDOUT_FILENO) < 0 {
                return Err(io::Error::last_os_error());
            }
            // 3) close the saved orig_stdout_fd; we don't need it anymore.
            libc::close(self.orig_stdout_fd);
        }

        // 4) join the reader thread—by now, the pipe has seen EOF—and get its String.
        let captured =
            self.reader_handle.join().unwrap_or_else(|_| String::from("<thread panicked>"));
        Ok(captured)
    }
}
