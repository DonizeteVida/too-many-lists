mod linked_list;

fn main() {
    linked_list::hello_world();

    let mut list = linked_list::LinkedList::<u32>::default();
    list.push(1);
    list.push(2);
    list.push(3);
    println!("{:?}", list);
}
