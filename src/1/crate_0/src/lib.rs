pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("警告: 割り当ての75％以上を使用してしまいました！");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("切迫した警告: 割り当ての90%以上を使用してしまいました！");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("エラー: 割り当てを超えています！");
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
