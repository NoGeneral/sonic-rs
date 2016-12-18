use error::Result;
use model::SonicMessage;
use rux::buf::ByteBuffer;
use rux::handler::Handler;
use rux::handler::mux::{MuxCmd, MuxEvent};
use rux::poll::*;
use std::io;

pub struct SonicHandler<'b, H>
    where H: Handler<In = Result<SonicMessage>, Out = SonicMessage>,
{
    handler: H,
    rbuffer: &'b mut ByteBuffer,
    wbuffer: &'b mut ByteBuffer,
}

impl<'b, H> SonicHandler<'b, H>
    where H: Handler<In = Result<SonicMessage>, Out = SonicMessage>,
{
    pub fn new(
        handler: H,
        rbuffer: &'b mut ByteBuffer,
        wbuffer: &'b mut ByteBuffer
    ) -> SonicHandler<'b, H> {
        SonicHandler {
            handler: handler,
            rbuffer: rbuffer,
            wbuffer: wbuffer,
        }
    }
}

// TODO should read/write until EAGAIN and keep state of readable/writable
impl<'b, H> Handler for SonicHandler<'b, H>
    where H: Handler<In = Result<SonicMessage>, Out = SonicMessage>,
{
    type In = MuxEvent;
    type Out = MuxCmd;

    fn update(&mut self, epfd: EpollFd) {}

    fn ready(&mut self, event: MuxEvent) -> MuxCmd {
        let fd = event.fd;
        let kind = event.kind;

        if kind.contains(EPOLLHUP) {
            return MuxCmd::Close;
        }

        if kind.contains(EPOLLERR) {
            let err = format!("fd={}: EPOLERR", fd);
            error!("{}", err);
            let res = self.handler.ready(Err(err.into()));
            let writer: &mut [u8] = self.wbuffer.mut_slice(4);
            // TODO res.to_writer(&mut writer).unwrap();
        }

        if kind.contains(EPOLLIN) {

        }

        if kind.contains(EPOLLOUT) {

        }

        MuxCmd::Keep
    }
}
