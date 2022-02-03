use std::sync::Arc;

use playground::{HashMapPeerManager, Peer, PeerId, VecPeerManager, BTreeMapPeerManager};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("get");

    for size in [1, 5, 10] {
        for kind in ["Vec", "HashMap", "BTreeMap"] {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{:02}", kind, size)),
                &(kind, size),
                |b, &(kind, size)| {
                    macro_rules! expand {
                        ($peer_manager:ident) => {
                            for _ in 0..size {
                                let peer = Peer::random();
                                $peer_manager.add(Arc::new(peer));
                            }

                            b.iter(|| {
                                let id = PeerId::random();
                                $peer_manager.get(black_box(&id))
                            })
                        };
                    }

                    match kind {
                        "Vec" => {
                            let peer_manager = VecPeerManager::new();
                            expand!(peer_manager);
                        }
                        "HashMap" => {
                            let peer_manager = HashMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        "BTreeMap" => {
                            let peer_manager = BTreeMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        _ => unreachable!(),
                    }
                },
            );
        }
    }

    group.finish();

    let mut group = c.benchmark_group("get_all");

    for size in [1, 5, 10] {
        for kind in ["Vec", "HashMap", "BTreeMap"] {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{:02}", kind, size)),
                &(kind, size),
                |b, &(kind, size)| {
                    macro_rules! expand {
                        ($peer_manager:ident) => {
                            for _ in 0..size {
                                let peer = Peer::random();
                                $peer_manager.add(Arc::new(peer));
                            }

                            b.iter(|| $peer_manager.get_all())
                        };
                    }

                    match kind {
                        "Vec" => {
                            let peer_manager = VecPeerManager::new();
                            expand!(peer_manager);
                        }
                        "HashMap" => {
                            let peer_manager = HashMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        "BTreeMap" => {
                            let peer_manager = BTreeMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        _ => unreachable!(),
                    }
                },
            );
        }
    }

    group.finish();

    let mut group = c.benchmark_group("fair_find");

    for size in [1, 5, 10] {
        for kind in ["Vec", "HashMap", "BTreeMap"] {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{:02}", kind, size)),
                &(kind, size),
                |b, &(kind, size)| {
                    macro_rules! expand {
                        ($peer_manager:ident) => {
                            for _ in 0..size {
                                let peer = Peer::random();
                                $peer_manager.add(Arc::new(peer));
                            }

                            b.iter(|| $peer_manager.fair_find(|_| false))
                        };
                    }

                    match kind {
                        "Vec" => {
                            let peer_manager = VecPeerManager::new();
                            expand!(peer_manager);
                        }
                        "HashMap" => {
                            let peer_manager = HashMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        "BTreeMap" => {
                            let peer_manager = BTreeMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        _ => unreachable!(),
                    }
                },
            );
        }
    }

    group.finish();

    let mut group = c.benchmark_group("add");

    for size in [1, 5, 10] {
        for kind in ["Vec", "HashMap", "BTreeMap"] {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{:02}", kind, size)),
                &(kind, size),
                |b, &(kind, size)| {
                    macro_rules! expand {
                        ($peer_manager:ident) => {
                            b.iter(|| {
                                let $peer_manager = VecPeerManager::new();
                                for _ in 0..size {
                                    let peer = Peer::random();
                                    $peer_manager.add(Arc::new(peer));
                                }
                            })
                        };
                    }

                    match kind {
                        "Vec" => {
                            expand!(peer_manager);
                        }
                        "HashMap" => {
                            expand!(peer_manager);
                        }
                        "BTreeMap" => {
                            expand!(peer_manager);
                        }
                        _ => unreachable!(),
                    }
                },
            );
        }
    }

    group.finish();

    let mut group = c.benchmark_group("remove");

    for size in [1, 5, 10] {
        for kind in ["Vec", "HashMap", "BTreeMap"] {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{:02}", kind, size)),
                &(kind, size),
                |b, &(kind, size)| {
                    macro_rules! expand {
                        ($peer_manager:ident) => {
                            for _ in 0..size {
                                let peer = Peer::random();
                                $peer_manager.add(Arc::new(peer));
                            }

                            b.iter(|| $peer_manager.remove(&PeerId::random()))
                        };
                    }

                    match kind {
                        "Vec" => {
                            let peer_manager = VecPeerManager::new();
                            expand!(peer_manager);
                        }
                        "HashMap" => {
                            let peer_manager = HashMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        "BTreeMap" => {
                            let peer_manager = BTreeMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        _ => unreachable!(),
                    }
                },
            );
        }
    }

    group.finish();

    let mut group = c.benchmark_group("for_each");

    for size in [1, 5, 10] {
        for kind in ["Vec", "HashMap", "BTreeMap"] {
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{}/{:02}", kind, size)),
                &(kind, size),
                |b, &(kind, size)| {
                    macro_rules! expand {
                        ($peer_manager:ident) => {
                            for _ in 0..size {
                                let peer = Peer::random();
                                $peer_manager.add(Arc::new(peer));
                            }

                            b.iter(|| $peer_manager.for_each(|_, _| ()))
                        };
                    }

                    match kind {
                        "Vec" => {
                            let peer_manager = VecPeerManager::new();
                            expand!(peer_manager);
                        }
                        "HashMap" => {
                            let peer_manager = HashMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        "BTreeMap" => {
                            let peer_manager = BTreeMapPeerManager::new();
                            expand!(peer_manager);
                        }
                        _ => unreachable!(),
                    }
                },
            );
        }
    }

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
