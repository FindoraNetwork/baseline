use alloc::{boxed::Box, collections::BTreeMap, vec::Vec};

use crate::{prelude::ModuleDefined, types::BlockHeight};

pub struct Version<C, Tx, OTx> {
    pub modules:
        Vec<Box<dyn ModuleDefined<Context = C, Transaction = Tx, OriginTransaction = OTx>>>,
    pub height: BTreeMap<BlockHeight, usize>,
}
