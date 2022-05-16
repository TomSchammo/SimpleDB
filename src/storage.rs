use std::mem::MaybeUninit;

const PAGE_SIZE: usize = 4096;

#[cfg(not(feature = "dyn_table"))]
const MAX_PAGES: usize = 255;

struct Page {
    data: [MaybeUninit<u8>; PAGE_SIZE],
}

#[cfg(not(feature = "dyn_table"))]
struct ConstTable {
    num_of_rows: u32,
    pages: [Option<Page>; MAX_PAGES],
}

#[cfg(feature = "dyn_table")]
struct DynTable {
    num_of_rows: u32,
    pages: Vec<Page>,
}
