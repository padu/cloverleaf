use libnice_sys::{nice_candidate_free, nice_candidate_new, NiceCandidate};
use std::ptr::NonNull;

use crate::{candidate_type::CandidateType, transport::Transport};

pub(crate) struct IceCandidate {
    inner: NonNull<NiceCandidate>,
    foundation: u8,
    component: u8,
    transport: Transport,
    priority: u32,
    ip: String,
    port: u16,
    typ: CandidateType,
}

impl IceCandidate {
    pub(crate) fn new(
        foundation: u8,
        component: u8,
        transport: Transport,
        priority: u32,
        ip: String,
        port: u16,
        typ: CandidateType,
    ) -> Result<Self, String> {
        let inner;
        unsafe {
            inner = NonNull::new(nice_candidate_new()).ok_or("candidate creation failed")?;
        }
        Ok(Self {
            inner,
            foundation,
            component,
            transport,
            priority,
            ip,
            port,
            typ,
        })
    }
}

impl Drop for IceCandidate {
    fn drop(&mut self) {
        unsafe {
            nice_candidate_free(self.inner.as_ptr() as *mut _);
        }
    }
}