use todo::utils::*;

fn get_stub() -> List {
    let mut to_return = List::new();

    to_return.add_task("Done entry");
    to_return.add_task("Done entry 2");

    to_return.add_task("Sample entry");
    to_return.add_task("Sample entry 2");

    to_return
}

#[test]
fn status_dist() {
    let mut list = get_stub();

    list.close_task(0).unwrap();
    list.close_task(1).unwrap();

    let res = list.get_status();

    assert_eq!(res[&Status::Todo], 2);
    assert_eq!(res[&Status::Done], 2);
}
