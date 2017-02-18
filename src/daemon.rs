use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::process;
use std::sync::Arc;

use libc;

use inventory::*;
use settings::*;
use server::*;
use upstream::*;

pub struct Daemon {
	upstream: Arc <Upstream>,
	server: Arc <Server>,
}

pub fn daemon_start (
	settings: Arc <Settings>,
) -> i32 {

	unsafe {

		let mut pipe_fds: [libc::pid_t; 2] = [0; 2];

		let pipe_result =
			libc::pipe (
				pipe_fds.as_mut_ptr ());

		if pipe_result != 0 {

			panic! (
				"Pipe failed");

		}

		match libc::fork () {

			-1 =>
				panic! (
					"Fork failed"),

			0 =>
				daemon_child (
					settings,
					pipe_fds),

			child_pid =>
				daemon_parent (
					settings,
					pipe_fds,
					child_pid),

		}

	}

}

unsafe fn daemon_parent (
	settings: Arc <Settings>,
	pipe_fds: [libc::pid_t; 2],
	child_pid: libc::pid_t,
) -> i32 {

	let close_result =
		libc::close (
			pipe_fds [1]);

	if close_result != 0 {

		panic! (
			"Failed to close write pipe in parent");

	}

	let mut read_buffer: [u8; 1] = [0; 1];

	let read_result =
		libc::read (
			pipe_fds [0],
			read_buffer.as_mut_ptr ()
				as * mut libc::c_void,
			1);

	if read_result < 1 {

		panic! (
			"Error reading from pipe in parent");

	}

	if read_result == 0 {

		println! (
			"Startup complete, exiting main process");

	}

	read_buffer [0] as i32

}

unsafe fn daemon_child (
	settings: Arc <Settings>,
	pipe_fds: [libc::pid_t; 2],
) -> i32 {

	let close_result =
		libc::close (
			pipe_fds [0]);

	if close_result != 0 {

		panic! (
			"Failed to close read pipe in child");

	}

	let daemon_result = daemon (
		settings.clone (),
	);

	if let Err (error) = daemon_result {

		println! (
			"{}",
			error);

		libc::write (
			pipe_fds [1],
			& [1u8]
				as * const u8
				as * const libc::c_void,
			1);

		process::exit (0);

	}

	let write_result =
		libc::write (
			pipe_fds [1],
			& [0u8]
				as * const u8
				as * const libc::c_void,
			1);

	if write_result != 1 {

		panic! (
			"Failed writing to pipe in child");

	}

	0

}

fn daemon (
	settings: Arc <Settings>,
) -> Result <Daemon, String> {

	write_pid_file (
		settings.clone (),
	) ?;

	let inventory =
		Arc::new (
			Inventory::load (
				settings.clone (),
			) ?
		);

	println! (
		"Loaded project: {}",
		inventory.project ().project_name ());

	for developer in inventory.project ().project_developers () {

		println! (
			"Developer: {} <{}>",
			developer.name (),
			developer.email (),
		);

	}

	let upstream =
		Arc::new (
			Upstream::start (
				settings.clone (),
			) ?
		);

	let server =
		Arc::new (
			Server::start (
				settings.clone (),
				upstream.clone (),
			) ?
		);

	Ok (Daemon {
		upstream: upstream,
		server: server,
	})

}

fn write_pid_file (
	settings: Arc <Settings>,
) -> Result <(), String> {

	let mut pid_file =
		File::create (
			& settings.general.pid_file,
		).map_err (|error|
			format! (
				"Error creating PID file {}: {}",
				settings.general.pid_file.to_string_lossy (),
				error.description ())
		) ?;

	write! (
		pid_file,
		"{}\n",
		unsafe { libc::getpid () },
	).map_err (|error|
		format! (
			"Error writing PID file {}: {}",
			settings.general.pid_file.to_string_lossy (),
			error.description ())
	) ?;

	Ok (())

}

// ex: noet ts=4 filetype=rust
