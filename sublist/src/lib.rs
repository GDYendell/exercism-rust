#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    } else if _second_list.len() > _first_list.len() && is_sublist(_first_list, _second_list) {
        return Comparison::Sublist;
    } else if _first_list.len() > _second_list.len() && is_sublist(_second_list, _first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist<T: PartialEq>(_sublist: &[T], _list: &[T]) -> bool {
    for i in 0..(_list.len() - _sublist.len() + 1) {
        if &_list[i..(i + _sublist.len())] == _sublist {
            return true;
        }
    }

    false
}
