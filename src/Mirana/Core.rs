use Model  ::{ Repository, Gentoo, MakeCfg
            , pull, push, rebase};

// Modules:
use Misc                ::{toVCS, toTrait};
use VcsCmd::Gentoo      ::{gentooFullUpdate};

// Stars
use Traits::Vcs;

use std::os::{change_dir, self_exe_path};
use extra::time;

fn make(mk: ~str) {
    ;
}

pub fn runSync(repo: Repository, location: Path, typeFilter: Option<~str>, ncore: uint) {
    let loc = &location;
    let nowt = time::now_utc();
    let nowt_str = nowt.rfc3339();
    if loc.exists() {
        change_dir(loc);
        for r in repo.remotes.iter().filter(
            |&r| match typeFilter {
                Some(ref rt) => r.t == toVCS(rt.to_owned()),
                None => true
            }) {
            match r.t {
            Gentoo => gentooFullUpdate(repo.loc, ncore),
            _ => {  for b in r.branches.iter() {
                        println!(" [{:s}]  branch: {:s}", nowt_str, *b);
                        match toTrait(r.t) {
                            Some(vcs) => {
                                for action in repo.actions.iter() {
                                    match *action {
                                        pull    => vcs.pull(*b),
                                        push    => vcs.push(*b),
                                        rebase  => vcs.rebase(*b
                                                            , &r.master
                                                            , &r.upstream),
                                        _       => fail!("Non implemented yet")
                                    }
                                }
                            }, None => fail!("Non trait implementation found")
                        }
                        match repo.make {
                            Some(ref mk) => { make(mk.to_owned());
                            }, None => println(" [No make]")
                        }
                    }
                }
            }
        }
        change_dir(&self_exe_path().unwrap());
    } else {
        println!(" -> {:s} does not exist", repo.loc);
    }
}
