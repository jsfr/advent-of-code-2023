pub trait Solution {
    fn compute_1(&self, input: &str) -> anyhow::Result<String>;
    fn compute_2(&self, input: &str) -> anyhow::Result<String>;
}
