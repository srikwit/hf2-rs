use crate::command::{rx, xmit, Command, Commander, Error, NoResponse};
use scroll::{ctx::TryIntoCtx, Pwrite, LE};

///Dual of READ WORDS, with the same constraints. No Result.
pub struct WriteWords {
    pub target_address: u32,
    pub num_words: u32,
    pub words: Vec<u32>,
}

impl<'a> TryIntoCtx<::scroll::Endian> for &'a WriteWords {
    type Error = ::scroll::Error;

    fn try_into_ctx(
        self,
        dst: &mut [u8],
        ctx: ::scroll::Endian,
    ) -> ::scroll::export::result::Result<usize, Self::Error> {
        let mut offset = 0;

        dst.gwrite_with(self.target_address, &mut offset, ctx)?;
        dst.gwrite_with(self.num_words, &mut offset, ctx)?;

        for i in &self.words {
            dst.gwrite_with(i, &mut offset, ctx)?;
        }

        Ok(offset)
    }
}

impl<'a> Commander<'a, NoResponse> for WriteWords {
    const ID: u32 = 0x0009;

    fn send(&self, d: &hidapi::HidDevice) -> Result<NoResponse, Error> {
        let mut data = vec![0_u8; self.words.len() * 4 + 8];
        let _ = self.try_into_ctx(&mut data, LE)?;

        let command = Command::new(Self::ID, 0, data);

        xmit(command, d)?;

        let _ = rx(d)?;

        Ok(NoResponse {})
    }
}
