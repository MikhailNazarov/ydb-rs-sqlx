#[derive(Default)]
pub struct YdbQueryResult {}

impl Extend<YdbQueryResult> for YdbQueryResult {
    fn extend<T: IntoIterator<Item = YdbQueryResult>>(&mut self, iter: T) {
        todo!()
    }
}
