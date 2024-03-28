use nix::{
    sys::socket::{bind, connect, listen, socket, AddressFamily, sockaddr, SockFlag, SockType},
    unistd::{close, read, write},
};
use anyhow::{Error, anyhow, Result};
use crate::error::{ErrorType, RuntimeError};
pub struct IpcChannel {
    fd: i32,
    sock_path: String,
    _client: Option<i32>,
}

impl IpcChannel {
    pub fn new(path: &String) -> Result<IpcChannel> {
        let socket_raw_fd = socket(
            AddressFamily::Unix,
            SockType::SeqPacket,
            SockFlag::SOCK_CLOEXEC,
            None,
        )
        .map_err(|_| Err(anyhow!(RuntimeError {
            msg: "unable to create IPC socket".to_string(),
            err_type: ErrorType::Runtime,
        })))?;

        let sockaddr = sockaddr::new_unix(Path::new(path)).map_err(|_| Error {
            msg: "unable to create unix socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;

        bind(socket_raw_fd, &sockaddr).map_err(|_| Error {
            msg: "unable to bind IPC socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;

        listen(socket_raw_fd, 10).map_err(|_| Error {
            msg: "unable to listen IPC socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;
        Ok(IpcChannel {
            fd: socket_raw_fd,
            sock_path: path.clone(),
            _client: None,
        })
    }

    pub fn connect(path: &String) -> Result<IpcChannel> {
        let socket_raw_fd = socket(
            AddressFamily::Unix,
            SockType::SeqPacket,
            SockFlag::SOCK_CLOEXEC,
            None,
        )
        .map_err(|_| Error {
            msg: "unable to create IPC socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;

        let sockaddr = SockAddr::new_unix(Path::new(path)).map_err(|_| Error {
            msg: "unable to create unix socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;

        connect(socket_raw_fd, &sockaddr).map_err(|_| Error {
            msg: "unable to connect to unix socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;

        Ok(IpcChannel {
            fd: socket_raw_fd,
            sock_path: path.clone(),
            _client: None,
        })
    }

    pub fn accept(&mut self) -> Result<()> {
        let child_socket_fd = nix::sys::socket::accept(self.fd).map_err(|_| Error {
            msg: "unable to accept incoming socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;

        self._client = Some(child_socket_fd);
        Ok(())
    }

    pub fn send(&self, msg: &str) -> Result<()> {
        let fd = match self._client {
            Some(fd) => fd,
            None => self.fd,
        };

        write(fd, msg.as_bytes()).map_err(|err| Error {
            msg: format!("unable to write to unix socket {}", err),
            err_type: ErrorType::Runtime,
        })?;

        Ok(())
    }

    pub fn recv(&self) -> Result<String> {
        let fd = match self._client {
            Some(fd) => fd,
            None => self.fd,
        };
        let mut buf = [0; 1024];
        let num = read(fd, &mut buf).unwrap();

        match std::str::from_utf8(&buf[0..num]) {
            Ok(str) => Ok(str.trim().to_string()),
            Err(_) => Err(Error {
                msg: "error while converting byte to string {}".to_string(),
                err_type: ErrorType::Runtime,
            }),
        }
    }

    pub fn close(&self) -> Result<()> {
        close(self.fd).map_err(|_| Error {
            msg: "error closing socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;

        std::fs::remove_file(&self.sock_path).map_err(|_| Error {
            msg: "error removing socket".to_string(),
            err_type: ErrorType::Runtime,
        })?;

        Ok(())
    }
}