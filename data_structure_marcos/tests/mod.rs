use data_structure_marcos::*;
use data_structures::*;

#[test]
fn test_list_macro() {
    let _list = single_list!([1, 2, 3]);
}

#[test]
fn test_tree_macro() {
    let _tree = binary_tree!([1, null, 3]);
}

#[test]
fn test_box_list_macro() {
    use data_structures::BoxListNode as ListNode;
    let _list = box_list!([1, 2, 3]);
}
