use libc::SIGTRAP;
use libc::WIFSTOPPED;
use nix::errno::errno;
use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use nix::Error;
use std::io;
use std::os::unix::process::CommandExt;
use std::process::Child;
use std::process::Command;

use crate::dwarf_data::DwarfData;

pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

pub struct Inferior {
    child: Child,
}

impl Drop for Inferior {
    fn drop(&mut self) {
        if let Ok(_) = self.child.kill() {
            println!("Killing running inferior (pid {})", self.pid());
            let _ = self.wait(None);
        }
    }
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>) -> Option<Inferior> {
        let mut cmd = Command::new(target);
        unsafe {
            cmd.pre_exec(child_traceme);
        }
        let inferior = Inferior {
            child: cmd.args(args).spawn().ok()?,
        };
        if let Ok(Status::Stopped(_, _)) = inferior.wait(Some(WaitPidFlag::WSTOPPED)) {
            Some(inferior)
        } else {
            None
        }
    }

    pub fn print_backtrace(&self, debug_data: &DwarfData) -> Result<(), nix::Error> {
        let regs = ptrace::getregs(self.pid())?;

        let print_function_name_and_line = |addr: usize| {
            if let Some(function) = debug_data.get_function_from_addr(addr) {
                print!("{} ", function);
            }
            if let Some(line) = debug_data.get_line_from_addr(addr) {
                print!("{} ", line);
            }
        };

        let mut index = 0;
        print!("{}: ", index);
        index += 1;
        print_function_name_and_line(regs.rip as usize);

        println!("rip: {:#x}", regs.rip);

        // try to get all backtrace (cdecl)
        let mut bp = regs.rbp;
        loop {
            match ptrace::read(self.pid(), bp as ptrace::AddressType) {
                Ok(new_bp) => {
                    match ptrace::read(self.pid(), (bp + 8) as ptrace::AddressType) {
                        Ok(addr) => {
                            print!("{}: ", index);
                            print_function_name_and_line(addr as usize);
                            println!("rip: {:#x}", addr as usize);
                            index += 1;
                        }
                        Err(_) => break,
                    }
                    bp = new_bp as u64;
                }
                Err(_) => {
                    // end the bt
                    break;
                }
            }
        }

        Ok(())
    }

    pub fn continues(&self) -> Result<Status, nix::Error> {
        ptrace::cont(self.pid(), None)?;
        self.wait(None)
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }
}
