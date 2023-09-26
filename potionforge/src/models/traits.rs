pub trait GetName {
    fn name(&self) -> &'static str;
}

pub trait GetByKey<K, V> {
    fn get_by_key(&self, key: &K) -> &V;
}

pub trait ToHumanReadable {
    fn to_human(&self) -> String;
}
