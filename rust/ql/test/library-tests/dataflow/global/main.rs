fn source(i: i64) -> i64 {
    1000 + i
}

fn sink(s: i64) {
    println!("{}", s);
}

// -----------------------------------------------------------------------------
// Data flow in, out, and through functions.

fn get_data(n: i64) -> i64 {
    source(n)
}

fn data_out_of_call() {
    let a = get_data(7);
    sink(a); // $ hasValueFlow=n
}

fn data_in(n: i64) {
    sink(n + 7); // $ hasValueFlow
}

fn data_in_to_call() {
    let a = source(3);
    data_in(a);
}

fn pass_through(i: i64) -> i64 {
    i
}

fn data_through_call() {
    let a = source(1);
    let b = pass_through(a);
    sink(b); // $ hasValueFlow=1
}

// -----------------------------------------------------------------------------
// Data flow in, out, and through method.

struct MyFlag {
    flag: bool,
}

impl MyFlag {
    fn data_in(&self, n: i64) {
        sink(n); // $ hasValueFlow=1
    }
    fn get_data(&self) -> i64 {
        if self.flag {
            0
        } else {
            source(2)
        }
    }
    fn data_through(&self, n: i64) -> i64 {
        if self.flag {
            0
        } else {
            n
        }
    }
}

fn data_out_of_method() {
    let mn = MyFlag { flag: true };
    let a = mn.get_data();
    sink(a);
}

fn data_in_to_method_call() {
    let mn = MyFlag { flag: true };
    let a = source(1);
    mn.data_in(a)
}

fn data_through_method() {
    let mn = MyFlag { flag: true };
    let a = source(4);
    mn.data_through(a);
    sink(a); // $ hasValueFlow=4
}

fn main() {
    data_out_of_call();
    data_in_to_call();
    data_through_call();

    data_out_of_method();
    data_in_to_method_call();
    data_through_method();
}
