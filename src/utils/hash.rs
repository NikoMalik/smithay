pub(crate) type FastBuildHasher = probemap::AutoBuildHasher;
pub(crate) type HashMap<K, V> = probemap::HashMap<K, V>;
pub(crate) type IndexMap<K, V> = indexmap::IndexMap<K, V, FastBuildHasher>;
