use bst::Bst;

#[test]
fn empty_height() {
    let bst: Bst<i32> = Bst::default();
    assert_eq!(bst.height(), 0);
}

#[test]
fn height() {
    let mut bst: Bst<i32> = Bst::default();
    bst.insert(1);
    bst.insert(2);
    bst.insert(3);
    bst.insert(4);

    assert_eq!(bst.height(), 3);

    let mut bst2: Bst<i32> = Bst::default();
    bst2.insert(2);
    bst2.insert(1);
    bst2.insert(3);

    assert_eq!(bst2.height(), 1);
}

#[test]
fn leaf() {
    let mut bst: Bst<i32> = Bst::default();
    bst.insert(5);

    assert!(bst.root.as_ref().unwrap().is_leaf());

    bst.insert(6);

    assert!(!bst.root.as_ref().unwrap().is_leaf());
    assert!(bst.root.as_ref().unwrap().right.as_ref().unwrap().is_leaf());
}

#[test]
fn distance() {
    let mut tree: Bst<i32> = Bst::default();
    tree.insert(2);
    tree.insert(1);
    tree.insert(5);
    tree.insert(4);
    tree.insert(3);

    assert_eq!(tree.distance_to(1), 1);
    assert_eq!(tree.distance_to(2), 0);
    assert_eq!(tree.distance_to(3), 3);
    assert_eq!(tree.distance_to(4), 2);
    assert_eq!(tree.distance_to(5), 1);
}