pub trait Messenger {
    fn send(&mut self, msg: &str);
}
pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a mut T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
    where T: Messenger {
    pub fn new(messenger: &mut T, max: usize) -> LimitTracker<T> {
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
    use super::*;
    struct MockMessenger {
        sent_messages: Vec<String>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&mut self, message: &str) { // error[E0053]: method `send` has an incompatible type for trait
//        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mut mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}

