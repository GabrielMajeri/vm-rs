use super::{Result};

use std::fs::File;
use std::path::Path;
use std::{mem, ptr};

pub struct Handle {
	file: File
}

impl Handle {
	pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
		Self::new_impl(path.as_ref())
	}

	fn new_impl(path: &Path) -> Result<Self> {
		let file = File::create(path)?;

		Ok(Handle { file })
	}

	pub fn receive<R>(&self, code: u32) -> Result<R> {
		self.transmit(code, ())
	}

	pub fn send<S>(&self, code: u32, sent: S) -> Result<()> {
		self.transmit(code, sent)
	}

	pub fn transmit<S, R>(&self, code: u32, sent: S) -> Result<R> {
		use std::os::windows::io::AsRawHandle;
		let device = self.file.as_raw_handle();

		use std::os::raw::c_void;
		use std::os::windows::raw::HANDLE;

		extern "system" {
			/// Used to interact with the HAX driver.
			///
			/// See the [official documentation](https://msdn.microsoft.com/en-us/library/windows/desktop/aa363216(v=vs.85).aspx)
			/// for more information.
			fn DeviceIoControl(
				device: HANDLE,
				ioControlCode: u32,
				inBuffer: *const c_void,
				inBufferSize: u32,
				outBuffer: *mut c_void,
				outBufferSize: u32,
				bytesReturned: &mut u32,
				overlapped: *mut c_void
			) -> u32;
		}

		let sent_size = mem::size_of::<S>() as u32;

		let mut received = unsafe { mem::uninitialized() };
		let received_size = mem::size_of::<R>() as u32;

		let mut returned_bytes = 0;

		let result = unsafe {
			DeviceIoControl(
				device,
				code,
				&sent as *const S as *const c_void,
				sent_size,
				&mut received as *mut R as *mut c_void,
				received_size,
				&mut returned_bytes,
				ptr::null_mut()
			)
		};

		if (result == 0) || (returned_bytes != received_size) {
			Err(::last_os_error())
		} else {
			Ok(received)
		}
	}
}

