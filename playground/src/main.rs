struct Req {
    data: i32,
}

impl PartialEq for Req {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

fn main() {


}