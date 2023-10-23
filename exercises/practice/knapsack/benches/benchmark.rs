#![feature(test)]
extern crate knapsack;
extern crate test;

use knapsack::Item;
use test::Bencher;

#[bench]
fn bench(b: &mut Bencher) {
    b.iter(|| knapsack::maximum_value(MAX_WEIGHT, &ITEMS));
}

const MAX_WEIGHT: u32 = 750;
const ITEMS: [Item; 15] = [
    Item {
        weight: 70,
        value: 135,
    },
    Item {
        weight: 73,
        value: 139,
    },
    Item {
        weight: 77,
        value: 149,
    },
    Item {
        weight: 80,
        value: 150,
    },
    Item {
        weight: 82,
        value: 156,
    },
    Item {
        weight: 87,
        value: 163,
    },
    Item {
        weight: 90,
        value: 173,
    },
    Item {
        weight: 94,
        value: 184,
    },
    Item {
        weight: 98,
        value: 192,
    },
    Item {
        weight: 106,
        value: 201,
    },
    Item {
        weight: 110,
        value: 210,
    },
    Item {
        weight: 113,
        value: 214,
    },
    Item {
        weight: 115,
        value: 221,
    },
    Item {
        weight: 118,
        value: 229,
    },
    Item {
        weight: 120,
        value: 240,
    },
];
