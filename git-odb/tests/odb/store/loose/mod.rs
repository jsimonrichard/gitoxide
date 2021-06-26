use git_object::{bstr::ByteSlice, immutable, Sign, Time};

pub mod backend;

fn signature(time: u32) -> imgit_actor::Signature<'static> {
    imgit_actor::Signature {
        name: b"Sebastian Thiel".as_bstr(),
        email: b"byronimo@gmail.com".as_bstr(),
        time: Time {
            time,
            offset: 7200,
            sign: Sign::Plus,
        },
    }
}
