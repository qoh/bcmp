extern crate rand;

use AlgoSpec;
use longest_common_substring;
use longest_common_substrings;
use patch_set;
use unique_strings;
use treematch::SuffixTree;

const ALGO_SPECS_4: &'static [AlgoSpec] = &[
    AlgoSpec::HashMatch(1), AlgoSpec::HashMatch(2), AlgoSpec::HashMatch(3), AlgoSpec::HashMatch(4),
    AlgoSpec::TreeMatch(1), AlgoSpec::TreeMatch(2), AlgoSpec::TreeMatch(3), AlgoSpec::TreeMatch(4),
];

const ALGO_SPECS_8: &'static [AlgoSpec] = &[
    AlgoSpec::HashMatch(1), AlgoSpec::HashMatch(2), AlgoSpec::HashMatch(4), AlgoSpec::HashMatch(8),
    AlgoSpec::TreeMatch(1), AlgoSpec::TreeMatch(2), AlgoSpec::TreeMatch(4), AlgoSpec::TreeMatch(8),
];

#[test]
fn lcs() {
    let a = "abcdefghijklmnopqrstuvwxyz";
    let b = "rstufghijklmnopqvwxyzabcde";
    for algo_spec in ALGO_SPECS_8 {
        let m = longest_common_substring(a.as_bytes(), b.as_bytes(), *algo_spec);
        assert!(m.first_pos  == 5);
        assert!(m.second_pos == 4);
        assert!(m.length     == 12);
    }
}

#[test]
fn lcss() {
    let a = "abcdefghijklmnopqrstuvwxyz";
    let b = "rstufghijklmnopqvwxyzabcde";
    for algo_spec in ALGO_SPECS_4 {
        let ms = longest_common_substrings(a.as_bytes(), b.as_bytes(), *algo_spec, 10);
        assert!(ms.len() == 4);
        assert!(ms[0].first_pos  == 5);
        assert!(ms[0].second_pos == 4);
        assert!(ms[0].length     == 12);
        assert!(ms[1].first_pos  == 0);
        assert!(ms[1].second_pos == 21);
        assert!(ms[1].length     == 5);
        assert!(ms[2].first_pos  == 21);
        assert!(ms[2].second_pos == 16);
        assert!(ms[2].length     == 5);
        assert!(ms[3].first_pos  == 17);
        assert!(ms[3].second_pos == 0);
        assert!(ms[3].length     == 4);
    }
}

#[test]
fn ps1() {
    let a = "abcdefghijqrstuvwxyzfghijklmnopqr";
    let b = "abcdefghijklmnopqrstuvwxyz";
    for algo_spec in ALGO_SPECS_8 {
        let ps = patch_set(a.as_bytes(), b.as_bytes(), *algo_spec);
        assert!(ps.len() == 3);
        assert!(ps[0].first_pos  == 0);
        assert!(ps[0].second_pos == 0);
        assert!(ps[0].length     == 10);
        assert!(ps[1].first_pos  == 25);
        assert!(ps[1].second_pos == 10);
        assert!(ps[1].length     == 8);
        assert!(ps[2].first_pos  == 12);
        assert!(ps[2].second_pos == 18);
        assert!(ps[2].length     == 8);
    }
}

#[test]
fn ps2() {
    let a = "abcdefghijklmnhijklmnopqrstuopqrstuvwxyz";
    let b = "abcdefghijklmnopqrstuvwxyz";

    for algo_spec in ALGO_SPECS_8 {
        let ps = patch_set(a.as_bytes(), b.as_bytes(), *algo_spec);
        assert!(ps.len() == 2);
        assert!(ps[0].first_pos  == 0);
        assert!(ps[0].second_pos == 0);
        assert!(ps[0].length     == 14);
        assert!(ps[1].first_pos  == 28);
        assert!(ps[1].second_pos == 14);
        assert!(ps[1].length     == 12);
    }
}

#[test]
fn ps3() {
    let a = "abcdefghijklhijklmnhijklmnopqrstuqrstuvwxyz";
    let b = "abcdefghijklmnopqrstuvwxyz";
    for algo_spec in ALGO_SPECS_4 {
        let ps = patch_set(a.as_bytes(), b.as_bytes(), *algo_spec);
        assert!(ps.len() == 3);
        assert!(ps[0].first_pos  == 0);
        assert!(ps[0].second_pos == 0);
        assert!(ps[0].length     == 12);
        assert!(ps[1].first_pos  == 24);
        assert!(ps[1].second_pos == 12);
        assert!(ps[1].length     == 9);
        assert!(ps[2].first_pos  == 38);
        assert!(ps[2].second_pos == 21);
        assert!(ps[2].length     == 5);
    }
}

#[test]
fn us1() {
    let a = "abcdefghijklmnopqrstuvwxyz";
    let b = "abcdef01ghijklmnop3456qrstuvwxyz";
    for algo_spec in ALGO_SPECS_4 {
        let us = unique_strings(a.as_bytes(), b.as_bytes(), *algo_spec);
        assert!(us.len() == 2);
        assert!(us[0].0 == 6);
        assert!(us[0].1 == 8);
        assert!(us[1].0 == 18);
        assert!(us[1].1 == 22);
    }
}

#[test]
fn us2() {
    let a = "abcdefghijklmnopqrstuvwxyz";
    let b = "01234";
    for algo_spec in ALGO_SPECS_4 {
        let us = unique_strings(a.as_bytes(), b.as_bytes(), *algo_spec);
        assert!(us.len() == 1);
        assert!(us[0].0 == 0);
        assert!(us[0].1 == 5);
    }
}

#[test]
fn us3() {
    let a = "abcdefghijklmnopqrstuvwxyz";
    let b = "1234abcd5678";
    for algo_spec in ALGO_SPECS_4 {
        let us = unique_strings(a.as_bytes(), b.as_bytes(), *algo_spec);
        assert!(us.len() == 2);
        assert!(us[0].0 == 0);
        assert!(us[0].1 == 4);
        assert!(us[1].0 == 8);
        assert!(us[1].1 == 12);
    }
}

#[test]
fn us4() {
    let a = "abcdefghijklmnopqrstuvwxyz";
    let b = "abcdefghaxelionijklmnopqrstuvwxyz";
    let us = unique_strings(a.as_bytes(), b.as_bytes(), AlgoSpec::HashMatch(4));
    assert!(us.len() == 1);
    assert!(us[0].0 == 8);
    assert!(us[0].1 == 15);
    let us = unique_strings(a.as_bytes(), b.as_bytes(), AlgoSpec::HashMatch(1));
    assert!(us.len() == 0);
}

#[test]
fn stree() {
    let a = "ABABABC";
    let stree = SuffixTree::new(a.as_bytes());
    println!("{}", stree.to_graphviz(a.as_bytes()));
}

#[test]
fn random_compare() {
    let a : Vec<u8> = (0..1000).map(|_| (rand::random::<u8>() % 2) + b'a').collect();
    let b : Vec<u8> = (0..1000).map(|_| (rand::random::<u8>() % 2) + b'a').collect();
    for mml in [2,4,8].iter() {
        let ms1 = longest_common_substrings(&a, &b, AlgoSpec::HashMatch(*mml), 100);
        let ms2 = longest_common_substrings(&a, &b, AlgoSpec::TreeMatch(*mml), 100);
        assert!(ms1.len() == ms2.len());
        for i in 0..ms1.len() {
            assert!(ms1[i].second_pos == ms2[i].second_pos);
            assert!(ms1[i].length == ms2[i].length);
        }
    }
}

#[test]
fn motif_compare() {
    let mut motif = Vec::<u8>::new();
    for i in 1..20 {
        for _ in 0..i {
            motif.push(0);
        }
        motif.push(i as u8);
    }
    for mml in [2,4,8].iter() {
        let ms1 = longest_common_substrings(&motif, &motif, AlgoSpec::HashMatch(*mml), 100);
        let ms2 = longest_common_substrings(&motif, &motif, AlgoSpec::TreeMatch(*mml), 100);
        assert!(ms1.len() == ms2.len());
        for i in 0..ms1.len() {
            assert!(ms1[i].second_pos == ms2[i].second_pos);
            assert!(ms1[i].length == ms2[i].length);
        }
    }
}
