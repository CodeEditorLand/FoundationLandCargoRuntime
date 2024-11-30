// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use std::sync::{
	atomic::{AtomicBool, AtomicUsize, Ordering},
	Arc,
	RwLock,
};

use llrt_utils::{ctx::CtxExtension, object::ObjectExt, result::ResultExt};
#[cfg(unix)]
use rquickjs::IntoJs;
use rquickjs::{
	prelude::{Opt, Rest, This},
	Class,
	Ctx,
	Exception,
	Function,
	Object,
	Result,
	Undefined,
	Value,
};
#[cfg(unix)]
use tokio::net::UnixListener;
use tokio::{
	net::TcpListener,
	select,
	sync::broadcast::{self, Sender},
};

use super::{get_address_parts, get_hostname, socket::Socket, Listener, NetStream};
use crate::{
	modules::events::{EmitError, Emitter, EventEmitter, EventList},
	stream::{impl_stream_events, SteamEvents},
};

impl_stream_events!(Server);

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct Server<'js> {
	emitter:EventEmitter<'js>,
	address:Value<'js>,
	#[qjs(skip_trace)]
	close_tx:Sender<()>,
	#[qjs(skip_trace)]
	allow_half_open:bool,
	#[qjs(skip_trace)]
	already_listen:Arc<AtomicBool>,
}

impl<'js> Emitter<'js> for Server<'js> {
	fn get_event_list(&self) -> Arc<RwLock<EventList<'js>>> { self.emitter.get_event_list() }
}

#[rquickjs::methods(rename_all = "camelCase")]
impl<'js> Server<'js> {
	#[qjs(constructor)]
	pub fn new(ctx:Ctx<'js>, args:Rest<Value<'js>>) -> Result<Class<'js, Self>> {
		let mut args_iter = args.0.into_iter();

		let mut connection_listener = None;

		let mut allow_half_open = false;

		if let Some(first) = args_iter.next() {
			if let Some(connection_listener_arg) = first.as_function() {
				connection_listener = Some(connection_listener_arg.clone());
			}

			if let Some(opts_arg) = first.as_object() {
				allow_half_open = opts_arg.get_optional("allowHalfOpen")?.unwrap_or_default();
			}
		}

		if let Some(next) = args_iter.next() {
			connection_listener = next.into_function();
		}

		let emitter = EventEmitter::new();

		let (close_tx, _) = broadcast::channel::<()>(1);

		let instance = Class::instance(
			ctx.clone(),
			Self {
				emitter,
				address:Undefined.into_value(ctx.clone()),
				close_tx,
				allow_half_open,
				already_listen:Arc::new(AtomicBool::new(false)),
			},
		)?;

		if let Some(connection_listener) = connection_listener {
			Self::add_event_listener_str(
				This(instance.clone()),
				&ctx,
				"connection",
				connection_listener,
				false,
				false,
			)?;
		}

		Ok(instance)
	}

	pub fn address(&self) -> Value<'js> { self.address.clone() }

	#[allow(unused_assignments)]
	/// TODO add backlog support
	pub fn listen(this:This<Class<'js, Self>>, ctx:Ctx<'js>, args:Rest<Value<'js>>) -> Result<()> {
		let mut args_iter = args.0.into_iter();

		let mut port = None;

		let mut path = None;

		let mut host = None;
		#[allow(unused_variables)] // TODO add backlog support
		let mut backlog = None;

		let mut callback = None;

		let borrow = this.borrow();

		let mut close_rx = borrow.close_tx.subscribe();

		let allow_half_open = borrow.allow_half_open;

		let already_running = borrow.already_listen.clone();

		drop(borrow);

		if already_running.load(Ordering::Relaxed) {
			return Err(Exception::throw_message(&ctx, "ERR_SERVER_ALREADY_LISTEN"));
		}

		if let Some(first) = args_iter.next() {
			if let Some(callback_arg) = first.as_function() {
				callback = Some(callback_arg.clone());
			} else {
				if let Some(port_arg) = first.as_int() {
					if port_arg > 0xFFFF {
						return Err(Exception::throw_range(
							&ctx,
							"port should be between 0 and 65535",
						));
					}

					port = Some(port_arg);
				}

				if let Some(path_arg) = first.as_string() {
					path = Some(path_arg.to_string()?);
				}

				if let Some(opts_arg) = first.as_object() {
					port = opts_arg.get_optional("port")?;

					path = opts_arg.get_optional("path")?;

					host = opts_arg.get_optional("host")?;

					backlog = opts_arg.get_optional("backlog")?;
				}

				let path = first.into_string();

				if let Some(second) = args_iter.next() {
					if let Some(callback_arg) = second.as_function() {
						callback = Some(callback_arg.clone());
					}

					if let Some(host_arg) = second.as_string() {
						host = Some(host_arg.to_string()?);
					}

					if path.is_some() {
						if let Some(backlog_arg) = second.as_int() {
							backlog = Some(backlog_arg);
						}
					}

					if let Some(third) = args_iter.next() {
						if let Some(callback_arg) = third.as_function() {
							callback = Some(callback_arg.clone());
						}

						if port.is_some() {
							if let Some(backlog_arg) = third.as_int() {
								backlog = Some(backlog_arg);

								callback = args_iter.next().and_then(|v| v.into_function());
							}
						}
					}
				}
			}
		}

		if let Some(callback) = callback {
			Self::add_event_listener_str(
				This(this.clone()),
				&ctx,
				"listening",
				callback,
				true,
				true,
			)?;
		}

		let ctx2 = ctx.clone();

		let active_connections = Arc::new(AtomicUsize::new(0));

		if port.is_none() && path.is_none() {
			port = Some(0)
		}

		ctx.spawn_exit(async move {
			already_running.store(true, Ordering::Relaxed);

			let listener = match Self::bind(this.clone(), ctx2.clone(), port, host, path).await {
				Ok(listener) => listener,
				Err(e) => {
					already_running.store(false, Ordering::Relaxed);

					Err::<(), _>(e).emit_error(&ctx2, this.clone())?;

					return Ok(()); // Don't stop the VM if failed to bind
				},
			};

			Self::emit_str(This(this.clone()), &ctx2, "listening", vec![], false)?;

			loop {
				let ctx3 = ctx2.clone();

				let this2 = this.clone();

				select! {
					socket = listener.accept(&ctx3) => {
						Self::handle_socket_connection(
							this2.clone(),
							ctx3.clone(),
							socket,
							active_connections.clone(),
							allow_half_open
						).emit_error(&ctx3, this2)?;
					},
					_ = close_rx.recv() => {
						break;
					}
				}
			}

			Self::emit_str(this, &ctx2, "close", vec![], false)?;

			already_running.store(false, Ordering::Relaxed);

			Ok(())
		})?;

		Ok(())
	}

	fn close(this:This<Class<'js, Self>>, ctx:Ctx<'js>, cb:Opt<Function<'js>>) -> Result<()> {
		if let Some(cb) = cb.0 {
			Self::add_event_listener_str(This(this.clone()), &ctx, "close", cb, true, true)?;
		}

		let _ = this.borrow().close_tx.send(());

		Ok(())
	}
}

impl<'js> Server<'js> {
	async fn bind(
		this:Class<'js, Self>,
		ctx:Ctx<'js>,
		port:Option<i32>,
		host:Option<String>,
		path:Option<String>,
	) -> Result<Listener> {
		let listener = if let Some(port) = port {
			let listener = TcpListener::bind(get_hostname(
				&host.unwrap_or_else(|| String::from("0.0.0.0")),
				port as u16,
			))
			.await
			.or_throw(&ctx)?;

			let address_object = Object::new(ctx.clone())?;

			let (address, port, family) = get_address_parts(&ctx, listener.local_addr())?;

			address_object.set("address", address)?;

			address_object.set("port", port)?;

			address_object.set("family", family)?;

			this.borrow_mut().address = address_object.into_value();

			Listener::Tcp(listener)
		} else if let Some(path) = path {
			#[cfg(unix)]
			{
				let listener:UnixListener = UnixListener::bind(&path).or_throw(&ctx)?;

				this.borrow_mut().address = path.into_js(&ctx)?;

				Listener::Unix(listener)
			}
			#[cfg(not(unix))]
			{
				_ = path;

				return Err(Exception::throw_type(
					&ctx,
					"Unix domain sockets are not supported on this platform",
				));
			}
		} else {
			panic!("unreachable")
		};

		Ok(listener)
	}

	fn handle_socket_connection(
		this:Class<'js, Self>,
		ctx:Ctx<'js>,
		stream_result:Result<NetStream>,
		active_connections:Arc<AtomicUsize>,
		allow_half_open:bool,
	) -> Result<()> {
		let net_stream = stream_result.or_throw(&ctx)?;

		active_connections.fetch_add(1, Ordering::Relaxed);

		ctx.clone().spawn_exit(async move {
			let socket_instance = Socket::new(ctx.clone(), allow_half_open)?;

			let socket_instance2 = socket_instance.clone().as_value().clone();

			Self::emit_str(This(this.clone()), &ctx, "connection", vec![socket_instance2], false)?;

			let had_error = net_stream.process(&socket_instance, &ctx, allow_half_open).await?;

			Socket::emit_close(socket_instance, &ctx, had_error)?;

			active_connections.fetch_sub(1, Ordering::Relaxed);

			Ok(())
		})?;

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use std::time::Duration;

	use rand::Rng;

	use tokio::{
		io::{AsyncReadExt, AsyncWriteExt},
		net::TcpStream,
	};

	use crate::{
		buffer,
		net::NetModule,
		test::{call_test, test_async_with, ModuleEvaluator},
	};

	async fn call_tcp(port:u16) {
		// Connect to server
		tokio::time::sleep(Duration::from_millis(100)).await;

		let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await.unwrap();

		stream.set_nodelay(true).unwrap();

		// Write
		let msg = b"Hello, world!";

		stream.write_all(msg).await.unwrap();

		stream.flush().await.unwrap();

		// Read
		let mut buf = vec![0; 1024];

		let n = stream.read(&mut buf).await.unwrap();

		assert_eq!(&buf[..n], msg);
	}

	#[tokio::test]
	async fn test_server_echo() {
		test_async_with(|ctx| {
			Box::pin(async move {
				buffer::init(&ctx).unwrap();

				ModuleEvaluator::eval_rust::<NetModule>(ctx.clone(), "net").await.unwrap();

				let mut rng = rand::thread_rng();

				let port:u16 = rng.gen_range(49152..=65535);

				let module = ModuleEvaluator::eval_js(
					ctx.clone(),
					"test",
					r#"
                        import { createServer } from 'net';

                        export async function test(port) {
                            const server = createServer(socket => {
                                socket.on('data', data => {
                                    socket.write(data, () => server.close());
                                });
                            });

                            server.listen(port, '127.0.0.1');

                            return new Promise((resolve, reject) => {
                                server.on('close', () => resolve());

                                server.on('error', (err) => reject(err));
                            });
                        }
                    "#,
				)
				.await
				.unwrap();

				tokio::join!(call_test::<(), _>(&ctx, &module, (port,)), call_tcp(port));
			})
		})
		.await;
	}
}
