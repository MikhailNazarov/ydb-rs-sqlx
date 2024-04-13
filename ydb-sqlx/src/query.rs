#[derive(Default)]
pub struct YdbQueryResult {
    pub(super) rows_affected: u64,
}

impl YdbQueryResult {
    pub fn rows_affected(&self) -> u64 {
        self.rows_affected
    }
}

impl Extend<YdbQueryResult> for YdbQueryResult {
    fn extend<T: IntoIterator<Item = YdbQueryResult>>(&mut self, iter: T) {
        for elem in iter {
            self.rows_affected += elem.rows_affected;
        }
    }
}

// #[cfg(feature = "any")]
// impl From<YdbQueryResult> for crate::any::AnyQueryResult {
//     fn from(done: YdbQueryResult) -> Self {
//         crate::any::AnyQueryResult {
//             rows_affected: done.rows_affected,
//             last_insert_id: None,
//         }
//     }
// }
