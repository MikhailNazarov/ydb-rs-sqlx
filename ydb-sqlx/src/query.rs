#[derive(Default)]
pub struct YdbQueryResult {
    pub(crate) rows_affected: u64,
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
