#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    match (a.len(), b.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (_, _) if a == b => Comparison::Equal,
        (_, _) if is_sublist(a, b) => Comparison::Sublist,
        (_, _) if is_sublist(b, a) => Comparison::Superlist,
        (_, _) => Comparison::Unequal,
    }
}

fn is_sublist<T: PartialEq>(_sublist: &[T], _list: &[T]) -> bool {
    _list
        .windows(_sublist.len())
        .any(|list_window| list_window == _sublist)
}
