use crate::test_utils::{load_snappy_ssz, load_yaml, Config};
use ethereum_consensus::state_transition::{Context, Result, Validation};
use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
pub struct FinalityTestCase<S, B> {
    pre: S,
    post: Option<S>,
    blocks: Vec<B>,
    config: Config,
}

#[derive(Deserialize)]
struct FinalityMeta {
    blocks_count: usize,
}

impl<S, B> FinalityTestCase<S, B>
where
    S: fmt::Debug + ssz_rs::Deserialize + PartialEq<S>,
    B: fmt::Debug + ssz_rs::Deserialize + PartialEq<B>,
{
    pub fn from(test_case_path: &str) -> Self {
        let path = test_case_path.to_string() + "/pre.ssz_snappy";
        let pre: S = load_snappy_ssz(&path).unwrap();

        let path = test_case_path.to_string() + "/post.ssz_snappy";
        let post = load_snappy_ssz::<S>(&path);

        let path = test_case_path.to_string() + "/meta.yaml";
        let meta: FinalityMeta = load_yaml(&path);
        let blocks_count = meta.blocks_count;

        let mut blocks = vec![];
        for i in 0..blocks_count {
            let path = format!("{test_case_path}/blocks_{i}.ssz_snappy");
            let block: B = load_snappy_ssz(&path).unwrap();
            blocks.push(block);
        }

        let config =
            if test_case_path.contains("minimal") { Config::Minimal } else { Config::Mainnet };

        Self { pre, post, blocks, config }
    }

    pub fn execute<F>(&mut self, f: F)
    where
        F: FnOnce(&mut S, &mut [B], Validation, &Context) -> Result<()>,
    {
        let context = match self.config {
            Config::Minimal => Context::for_minimal(),
            Config::Mainnet => Context::for_mainnet(),
        };
        let result = f(&mut self.pre, &mut self.blocks, Validation::Enabled, &context);
        if let Some(post) = self.post.as_ref() {
            assert_eq!(&self.pre, post);
        } else {
            assert!(result.is_err())
        }
    }
}
