use futures_channel::oneshot;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use rand::prelude::*;
use tokio::sync::mpsc;

use std::collections::{BTreeMap, HashMap};
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

type OneshotSender = [u8; size_of::<oneshot::Sender<()>>()];
type GossipSender = [u8; size_of::<mpsc::UnboundedSender<Vec<u8>>>()];

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct PeerId([u8; 64]);

impl PeerId {
    pub fn random() -> Self {
        let mut arr = [0u8; 64];
        thread_rng().fill(&mut arr[..]);
        Self(arr)
    }
}

pub struct Peer {
    id: PeerId,
    _bytes: [u8; size_of::<bee_protocol::types::peer::Peer>() - size_of::<PeerId>()],
}

impl Peer {
    pub fn random() -> Self {
        let mut id = [0u8; 64];
        let mut bytes = [0u8; size_of::<bee_protocol::types::peer::Peer>() - size_of::<PeerId>()];

        thread_rng().fill(&mut id[..]);
        thread_rng().fill(&mut bytes[..]);

        Self {
            id: PeerId(id),
            _bytes: bytes,
        }
    }

    /// Get the peer's id.
    pub fn id(&self) -> &PeerId {
        &self.id
    }
}

type PeerTuple = (Arc<Peer>, Option<(GossipSender, OneshotSender)>);

struct HashMapPeerManagerInner {
    peers: HashMap<PeerId, PeerTuple>,
    keys: Vec<PeerId>,
}

pub struct HashMapPeerManager {
    inner: RwLock<HashMapPeerManagerInner>,
    counter: AtomicUsize,
}

impl HashMapPeerManager {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(HashMapPeerManagerInner {
                peers: HashMap::new(),
                keys: Vec::new(),
            }),
            counter: 0.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().peers.is_empty()
    }

    pub fn get(&self, id: &PeerId) -> Option<impl std::ops::Deref<Target = PeerTuple> + '_> {
        RwLockReadGuard::try_map(self.inner.read(), |map| map.peers.get(id)).ok()
    }

    pub fn get_mut(&self, id: &PeerId) -> Option<impl std::ops::DerefMut<Target = PeerTuple> + '_> {
        RwLockWriteGuard::try_map(self.inner.write(), |map| map.peers.get_mut(id)).ok()
    }

    pub fn get_all(&self) -> Vec<Arc<Peer>> {
        self.inner
            .read()
            .peers
            .iter()
            .map(|(_, (peer, _))| peer)
            .cloned()
            .collect()
    }

    pub fn add(&self, peer: Arc<Peer>) {
        let mut lock = self.inner.write();
        lock.keys.push(*peer.id());
        lock.peers.insert(*peer.id(), (peer, None));
    }

    pub fn remove(&self, id: &PeerId) -> Option<PeerTuple> {
        let mut lock = self.inner.write();
        lock.keys.retain(|peer_id| peer_id != id);
        lock.peers.remove(id)
    }

    pub fn for_each<F: Fn(&PeerId, &Peer)>(&self, f: F) {
        self.inner
            .read()
            .peers
            .iter()
            .for_each(|(id, (peer, _))| f(id, peer));
    }

    pub fn fair_find(&self, f: impl Fn(&Peer) -> bool) -> Option<PeerId> {
        let guard = self.inner.read();

        for _ in 0..guard.keys.len() {
            let counter = self.counter.fetch_add(1, Ordering::Relaxed);
            let peer_id = &guard.keys[counter % guard.keys.len()];

            if let Some((peer, _)) = guard.peers.get(peer_id) {
                if f(peer.as_ref()) {
                    return Some(*peer_id);
                }
            }
        }

        drop(guard);

        None
    }

    pub fn is_connected(&self, id: &PeerId) -> bool {
        self.inner
            .read()
            .peers
            .get(id)
            .map_or(false, |p| p.1.is_some())
    }

    pub fn connected_peers(&self) -> u8 {
        self.inner
            .read()
            .peers
            .iter()
            .filter(|(_, (_, ctx))| ctx.is_some())
            .count() as u8
    }
}

struct BTreeMapPeerManagerInner {
    peers: BTreeMap<PeerId, PeerTuple>,
    keys: Vec<PeerId>,
}

pub struct BTreeMapPeerManager {
    inner: RwLock<BTreeMapPeerManagerInner>,
    counter: AtomicUsize,
}

impl BTreeMapPeerManager {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(BTreeMapPeerManagerInner {
                peers: BTreeMap::new(),
                keys: Vec::new(),
            }),
            counter: 0.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().peers.is_empty()
    }

    pub fn get(&self, id: &PeerId) -> Option<impl std::ops::Deref<Target = PeerTuple> + '_> {
        RwLockReadGuard::try_map(self.inner.read(), |map| map.peers.get(id)).ok()
    }

    pub fn get_mut(&self, id: &PeerId) -> Option<impl std::ops::DerefMut<Target = PeerTuple> + '_> {
        RwLockWriteGuard::try_map(self.inner.write(), |map| map.peers.get_mut(id)).ok()
    }

    pub fn get_all(&self) -> Vec<Arc<Peer>> {
        self.inner
            .read()
            .peers
            .iter()
            .map(|(_, (peer, _))| peer)
            .cloned()
            .collect()
    }

    pub fn add(&self, peer: Arc<Peer>) {
        let mut lock = self.inner.write();
        lock.keys.push(*peer.id());
        lock.peers.insert(*peer.id(), (peer, None));
    }

    pub fn remove(&self, id: &PeerId) -> Option<PeerTuple> {
        let mut lock = self.inner.write();
        lock.keys.retain(|peer_id| peer_id != id);
        lock.peers.remove(id)
    }

    pub fn for_each<F: Fn(&PeerId, &Peer)>(&self, f: F) {
        self.inner
            .read()
            .peers
            .iter()
            .for_each(|(id, (peer, _))| f(id, peer));
    }

    pub fn fair_find(&self, f: impl Fn(&Peer) -> bool) -> Option<PeerId> {
        let guard = self.inner.read();

        for _ in 0..guard.keys.len() {
            let counter = self.counter.fetch_add(1, Ordering::Relaxed);
            let peer_id = &guard.keys[counter % guard.keys.len()];

            if let Some((peer, _)) = guard.peers.get(peer_id) {
                if f(peer.as_ref()) {
                    return Some(*peer_id);
                }
            }
        }

        drop(guard);

        None
    }

    pub fn is_connected(&self, id: &PeerId) -> bool {
        self.inner
            .read()
            .peers
            .get(id)
            .map_or(false, |p| p.1.is_some())
    }

    pub fn connected_peers(&self) -> u8 {
        self.inner
            .read()
            .peers
            .iter()
            .filter(|(_, (_, ctx))| ctx.is_some())
            .count() as u8
    }
}

struct VecPeerManagerInner {
    peers: Vec<(PeerId, PeerTuple)>,
}

impl VecPeerManagerInner {
    fn get(&self, id: &PeerId) -> Option<&PeerTuple> {
        self.peers
            .binary_search_by_key(id, |(id, _)| *id)
            .ok()
            .map(|i| &self.peers[i].1)
    }

    fn get_mut(&mut self, id: &PeerId) -> Option<&mut PeerTuple> {
        self.peers
            .binary_search_by_key(id, |(id, _)| *id)
            .ok()
            .map(|i| &mut self.peers[i].1)
    }

    fn insert(&mut self, id: PeerId, peer: PeerTuple) {
        match self.peers.binary_search_by_key(&id, |(id, _)| *id) {
            Ok(i) => self.peers[i] = (id, peer),
            Err(i) => self.peers.insert(i, (id, peer)),
        }
    }

    fn remove(&mut self, id: &PeerId) -> Option<PeerTuple> {
        if let Ok(i) = self.peers.binary_search_by_key(id, |(id, _)| *id) {
            Some(self.peers.remove(i).1)
        } else {
            None
        }
    }
}

pub struct VecPeerManager {
    inner: RwLock<VecPeerManagerInner>,
    counter: AtomicUsize,
}

impl VecPeerManager {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(VecPeerManagerInner { peers: Vec::new() }),
            counter: 0.into(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.read().peers.is_empty()
    }

    pub fn get(&self, id: &PeerId) -> Option<impl std::ops::Deref<Target = PeerTuple> + '_> {
        RwLockReadGuard::try_map(self.inner.read(), |map| map.get(id)).ok()
    }

    pub fn get_mut(&self, id: &PeerId) -> Option<impl std::ops::DerefMut<Target = PeerTuple> + '_> {
        RwLockWriteGuard::try_map(self.inner.write(), |map| map.get_mut(id)).ok()
    }

    pub fn get_all(&self) -> Vec<Arc<Peer>> {
        self.inner
            .read()
            .peers
            .iter()
            .map(|(_, (peer, _))| peer)
            .cloned()
            .collect()
    }

    pub fn add(&self, peer: Arc<Peer>) {
        let mut lock = self.inner.write();
        lock.insert(*peer.id(), (peer, None));
    }

    pub fn remove(&self, id: &PeerId) -> Option<PeerTuple> {
        let mut lock = self.inner.write();
        lock.remove(id)
    }

    pub fn for_each<F: Fn(&PeerId, &Peer)>(&self, f: F) {
        self.inner
            .read()
            .peers
            .iter()
            .for_each(|(id, (peer, _))| f(id, peer));
    }

    pub fn fair_find(&self, f: impl Fn(&Peer) -> bool) -> Option<PeerId> {
        let guard = self.inner.read();

        for _ in 0..guard.peers.len() {
            let counter = self.counter.fetch_add(1, Ordering::Relaxed);
            let (peer_id, (peer, _)) = &guard.peers[counter % guard.peers.len()];

            if f(peer.as_ref()) {
                return Some(*peer_id);
            }
        }

        drop(guard);

        None
    }

    pub fn is_connected(&self, id: &PeerId) -> bool {
        self.inner.read().get(id).map_or(false, |p| p.1.is_some())
    }

    pub fn connected_peers(&self) -> u8 {
        self.inner
            .read()
            .peers
            .iter()
            .filter(|(_, (_, ctx))| ctx.is_some())
            .count() as u8
    }
}
