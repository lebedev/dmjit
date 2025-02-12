use std::ptr::null_mut;

use auxtools::raw_types::funcs::{append_to_list, create_list, dec_ref_count, remove_from_list};
use auxtools::raw_types::lists::{AssociativeListEntry, List, ListId};
use auxtools::raw_types::values::{Value, ValueData, ValueTag};
use crate::pads::{byond_imports, find_by_call, find_by_reference};
use libc::{c_void, free};

byond_imports!(
    var GLOB_LIST_ARRAY: *mut *mut List
        = find_by_reference!(
            unix    => "a1 >?? ?? ?? ?? 8b 04 90 85 c0 74 a8 83 40 ?? ?? 8b 97 ?? ?? ?? ?? e9 74 f9 ff ff",
            windows => "8b 35 >?? ?? ?? ?? 8b 45 10 90 80 f9 06 0f 84 04 03 00 00 80"
        );
    fn UNSET_ASSOC_LIST_PTR: extern "cdecl" fn(*mut *mut AssociativeListEntry, Value)
        = find_by_call!(
            unix    => "e8 >?? ?? ?? ?? 39 5d d0 72 e0 8b 7d c8 8b 5d c4 8b 75 d0",
            windows => "e8 >?? ?? ?? ?? 83 c4 0c 4b 75 ec 8b 7d f4 8b 5d ec 8b 75 f8 8b 45 fc"
        );
    fn ASSOC_LIST_SET_PTR: extern "cdecl" fn(u32, Value, Value)
        = find_by_call!(
            unix    => "8b 55 14 89 44 24 0c 89 7c 24 04 89 54 24 10 8b 55 c8 89 54 24 08 e8 >?? ?? ?? ??",
            windows => "e8 >?? ?? ?? ?? 8b 45 fc 83 c4 14 8b 55 08 83 c0 04"
        );
    fn ASSOC_FIND_NODE_BY_KEY_PTR: extern "cdecl" fn(*mut AssociativeListEntry, Value) -> *mut AssociativeListEntry
        = find_by_call!(
            unix    => "e8 >?? ?? ?? ?? 85 c0 89 c3 0f 84 b8 fe ff ff 8b 40 08 8b 53 0c",
            windows => "e8 >?? ?? ?? ?? 83 c4 0c 85 c0 75 03 5f 5d c3 56 50 57"
        );
    fn COPY_LIST_LIKE_PTR: extern "cdecl" fn(Value, u32, i32) -> Value
        = find_by_call!(
            unix    => "89 54 24 10 89 4c 24 0c 89 55 a0 89 4d a4 89 74 24 08 89 04 24 89 5c 24 04 e8 >?? ?? ?? ?? 8b 75 e0 8b 7d e4 89 75 c0 89 7d c4",
            windows => "e8 >?? ?? ?? ?? 6a 00 6a 01 ff 75 14 8b f0 8b fa ff 75 10"
        );
    fn LIST_ENSURE_CAPACITY_PTR: extern "cdecl" fn(*mut List, u32)
        = find_by_call!(
            unix    => "8b 40 0c 89 3c 24 83 c0 01 89 44 24 04 e8 >?? ?? ?? ?? 8b 47 0c 85 c0",
            windows => "e8 >?? ?? ?? ?? 8b 45 e0 33 c9 8b 55 e8 83 c4 08 39 30 0f 42 d9 33 f6 39 77 14 0f"
        );
);

pub fn init() {
    init_byond_imports();
}

pub fn get_glob_list() -> *mut *mut *mut List {
    unsafe {
        return GLOB_LIST_ARRAY.origin();
    }
}

pub fn unset_assoc_list(assoc_part: *mut *mut AssociativeListEntry, value: Value) {
    unsafe {
        UNSET_ASSOC_LIST_PTR(assoc_part, value)
    }
}

pub fn get_list(list: Value) -> *mut List {
    unsafe {
        return *(GLOB_LIST_ARRAY.add(list.data.list.0 as usize));
    }
}

pub fn list_ensure_capacity(list: Value, size: u32) {
    unsafe {
        LIST_ENSURE_CAPACITY_PTR(get_list(list), size);
    }
}

pub fn list_copy(list: Value) -> Value {
    unsafe {
        return COPY_LIST_LIKE_PTR(list, 0, 0);
    }
}

pub fn list_append(list: Value, value: Value) {
    unsafe {
        append_to_list(list, value);
    }
}

pub fn list_remove(list: Value, value: Value) {
    unsafe {
        remove_from_list(list, value);
    }
}

pub fn create_new_list(capacity: u32) -> u32 {
    let mut id = ListId(0);
    unsafe {
        create_list(&mut id, capacity);
    }
    return id.0;
}

pub fn list_associative_get(list: Value, index: Value) -> Value {
    unsafe {
        let node = ASSOC_FIND_NODE_BY_KEY_PTR((*get_list(list)).assoc_part, index);
        if node == null_mut() {
            return Value { tag: ValueTag::Null, data: ValueData { id: 0 } };
        }
        return (*node).value;
    }
}

pub fn list_associative_set(list: Value, index: Value, value: Value) {
    unsafe {
        ASSOC_LIST_SET_PTR(list.data.list.0, index, value)
    };
}
