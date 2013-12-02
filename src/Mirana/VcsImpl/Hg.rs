use Traits::Vcs;
use VcsCmd::Hg::{hgPull, hgPush, hgRebase, hgList};

pub struct Hg;

impl Vcs for Hg {
    fn list(&self) {
        hgList();
    }
    fn pull(&self, branch: &str) {
        hgPull(branch);
    }
    fn push(&self, _: &str) {
        hgPush();
    }
    fn rebase(&self, branch: &str, _: &Option<~str>, maybe_upstream: &Option<~str>) {
        hgRebase(branch, maybe_upstream);
    }
}