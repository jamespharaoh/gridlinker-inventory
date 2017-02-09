use std::io::Error as IoError;
use std::io::Read;
use std::io::Write;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use hyper::error::Error as HyperError;
use hyper::net::HttpStream as HyperHttpStream;
use hyper::net::NetworkStream as HyperNetworkStream;
use hyper::net::SslClient as HyperSslClient;

use openssl::ssl::SslStream;
use openssl::ssl::SslConnector;

#[ derive (Clone) ]
pub struct OpensslStream {
	ssl_stream: Arc <Mutex <SslStream <HyperHttpStream>>>,
}

impl Read for OpensslStream {

	fn read (
		& mut self,
		buffer: & mut [u8],
	) -> Result <usize, IoError> {

		let mut ssl_stream =
			self.ssl_stream.lock ().unwrap ();

		ssl_stream.read (
			buffer,
		)

	}

}

impl Write for OpensslStream {

	fn write (
		& mut self,
		buffer: & [u8],
	) -> Result <usize, IoError> {

		let mut ssl_stream =
			self.ssl_stream.lock ().unwrap ();

		ssl_stream.write (
			buffer,
		)

	}

	fn flush (
		& mut self,
	) -> Result <(), IoError> {

		let mut ssl_stream =
			self.ssl_stream.lock ().unwrap ();

		ssl_stream.flush ()

	}

}

impl HyperNetworkStream for OpensslStream {

	fn peer_addr (
		& mut self,
	) -> Result <SocketAddr, IoError> {

		let mut ssl_stream =
			self.ssl_stream.lock ().unwrap ();

		ssl_stream.get_mut ().peer_addr ()

	}

	fn set_read_timeout (
		& self,
		value: Option <Duration>,
	) -> Result <(), IoError> {

		let ssl_stream =
			self.ssl_stream.lock ().unwrap ();

		ssl_stream.get_ref ().set_read_timeout (
			value,
		)

	}

	fn set_write_timeout (
		& self,
		value: Option <Duration>,
	) -> Result <(), IoError> {

		let ssl_stream =
			self.ssl_stream.lock ().unwrap ();

		ssl_stream.get_ref ().set_write_timeout (
			value,
		)

	}

}

pub struct OpensslClient {
	ssl_connector: SslConnector,
}

impl OpensslClient {

	pub fn new (
		ssl_connector: SslConnector,
	) -> OpensslClient {

		OpensslClient {
			ssl_connector: ssl_connector,
		}

	}

}

impl HyperSslClient for OpensslClient {

	type Stream = OpensslStream;

	fn wrap_client (
		& self,
		stream: HyperHttpStream,
		host: & str,
	) -> Result <OpensslStream, HyperError> {

		let ssl_stream =
			Arc::new (Mutex::new (
				self.ssl_connector.connect (
					host,
					stream,
				).map_err (|error|
					HyperError::Ssl (Box::new (error))
				) ?
			));

		Ok (OpensslStream {
			ssl_stream: ssl_stream,
		})

	}

}

// ex: noet ts=4 filetype=rust
