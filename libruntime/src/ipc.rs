use crate::{
    error::{ErrorType, RuntimeError},
    fs,
};
use anyhow::Result;
use nix::{
    sys::socket::{
        bind, connect, listen, socket, AddressFamily, Backlog, SockFlag, SockType, UnixAddr,
    },
    unistd::{close, read, write},
};
use std::{
    os::fd::{AsRawFd, BorrowedFd, OwnedFd},
    path::Path,
};
pub struct IpcChannel {
    fd: OwnedFd,
    sock_path: String,
    _client: Option<i32>,
}

impl IpcChannel {
    pub fn new(path: &String) -> Result<IpcChannel, RuntimeError> {
        let socket_fd = socket(
            AddressFamily::Unix,
            SockType::SeqPacket,
            SockFlag::SOCK_CLOEXEC,
            None,
        )
        .map_err(|_| RuntimeError {
            message: "unable to create IPC socket".to_string(),
            error_type: ErrorType::Runtime,
        })?;

        let sockaddr = UnixAddr::new(Path::new(&fs::abs_path(path).expect("IPC path is None")))
            .map_err(|_| RuntimeError {
                message: "unable to create unix socket".to_string(),
                error_type: ErrorType::Runtime,
            })?;

        bind(socket_fd.as_raw_fd(), &sockaddr).map_err(|_| RuntimeError {
            message: "unable to bind IPC socket".to_string(),
            error_type: ErrorType::Runtime,
        })?;

        listen(
            &socket_fd,
            Backlog::new(10).map_err(|_| RuntimeError {
                message: "invalid backlog size".to_string(),
                error_type: ErrorType::Runtime,
            })?,
        )
        .map_err(|_| RuntimeError {
            message: "unable to listen IPC socket".to_string(),
            error_type: ErrorType::Runtime,
        })?;
        Ok(IpcChannel {
            fd: socket_fd,
            sock_path: path.clone(),
            _client: None,
        })
    }

    pub fn connect(path: &String) -> Result<IpcChannel, RuntimeError> {
        let socket_fd = socket(
            AddressFamily::Unix,
            SockType::SeqPacket,
            SockFlag::SOCK_CLOEXEC,
            None,
        )
        .map_err(|_| RuntimeError {
            message: "unable to create IPC socket".to_string(),
            error_type: ErrorType::Runtime,
        })?;

        let sockaddr = UnixAddr::new(Path::new(path)).map_err(|_| RuntimeError {
            message: "unable to create unix socket".to_string(),
            error_type: ErrorType::Runtime,
        })?;

        connect(socket_fd.as_raw_fd(), &sockaddr).map_err(|_| RuntimeError {
            message: "unable to connect to unix socket".to_string(),
            error_type: ErrorType::Runtime,
        })?;

        Ok(IpcChannel {
            fd: socket_fd,
            sock_path: path.clone(),
            _client: None,
        })
    }

    pub fn accept(&mut self) -> Result<()> {
        let child_socket_fd =
            nix::sys::socket::accept(self.fd.as_raw_fd()).map_err(|_| RuntimeError {
                message: "unable to accept incoming socket".to_string(),
                error_type: ErrorType::Runtime,
            })?;

        self._client = Some(child_socket_fd);
        Ok(())
    }

    pub fn send(&self, message: &str) -> Result<()> {
        let fd = match self._client {
            Some(fd) => fd,
            None => self.fd.as_raw_fd(),
        };

        write(unsafe { BorrowedFd::borrow_raw(fd) }, message.as_bytes()).map_err(|err| {
            RuntimeError {
                message: format!("unable to write to unix socket {}", err),
                error_type: ErrorType::Runtime,
            }
        })?;

        Ok(())
    }

    pub fn recv(&self) -> Result<String, RuntimeError> {
        let fd = match self._client {
            Some(fd) => fd,
            None => self.fd.as_raw_fd(),
        };
        let mut buf = [0; 1024];
        let num = read(fd, &mut buf).unwrap();

        match std::str::from_utf8(&buf[0..num]) {
            Ok(str) => Ok(str.trim().to_string()),
            Err(_) => Err(RuntimeError {
                message: "error while converting byte to string {}".to_string(),
                error_type: ErrorType::Runtime,
            }),
        }
    }

    pub fn close(&self) -> Result<()> {
        close(self.fd.as_raw_fd()).map_err(|_| RuntimeError {
            message: "error closing socket".to_string(),
            error_type: ErrorType::Runtime,
        })?;

        std::fs::remove_file(&self.sock_path).map_err(|_| RuntimeError {
            message: "error removing socket".to_string(),
            error_type: ErrorType::Runtime,
        })?;

        Ok(())
    }
}
