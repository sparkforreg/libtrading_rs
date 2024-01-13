use ::libc;
extern "C" {
    pub type _GHashTable;
    pub type _GTree;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn g_list_free(list: *mut GList);
    fn g_list_insert_sorted(
        list: *mut GList,
        data: gpointer,
        func: GCompareFunc,
    ) -> *mut GList;
    fn g_list_remove(list: *mut GList, data: gconstpointer) -> *mut GList;
    fn g_hash_table_new_full(
        hash_func: GHashFunc,
        key_equal_func: GEqualFunc,
        key_destroy_func: GDestroyNotify,
        value_destroy_func: GDestroyNotify,
    ) -> *mut GHashTable;
    fn g_hash_table_destroy(hash_table: *mut GHashTable);
    fn g_hash_table_insert(
        hash_table: *mut GHashTable,
        key: gpointer,
        value: gpointer,
    ) -> gboolean;
    fn g_hash_table_remove(hash_table: *mut GHashTable, key: gconstpointer) -> gboolean;
    fn g_hash_table_lookup(hash_table: *mut GHashTable, key: gconstpointer) -> gpointer;
    fn g_direct_hash(v: gconstpointer) -> guint;
    fn g_direct_equal(v1: gconstpointer, v2: gconstpointer) -> gboolean;
    fn g_tree_new(key_compare_func: GCompareFunc) -> *mut GTree;
    fn g_tree_destroy(tree: *mut GTree);
    fn g_tree_insert(tree: *mut GTree, key: gpointer, value: gpointer);
    fn g_tree_remove(tree: *mut GTree, key: gconstpointer) -> gboolean;
}
pub type gint = libc::c_int;
pub type gboolean = gint;
pub type gulong = libc::c_ulong;
pub type guint = libc::c_uint;
pub type gpointer = *mut libc::c_void;
pub type gconstpointer = *const libc::c_void;
pub type GCompareFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
>;
pub type GEqualFunc = Option::<
    unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
>;
pub type GDestroyNotify = Option::<unsafe extern "C" fn(gpointer) -> ()>;
pub type GHashFunc = Option::<unsafe extern "C" fn(gconstpointer) -> guint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _GList {
    pub data: gpointer,
    pub next: *mut GList,
    pub prev: *mut GList,
}
pub type GList = _GList;
pub type GHashTable = _GHashTable;
pub type GTree = _GTree;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ob_level {
    pub seq_num: libc::c_ulong,
    pub price: libc::c_ulong,
    pub size: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ob_order {
    pub seq_num: libc::c_ulong,
    pub price: libc::c_ulong,
    pub size: libc::c_ulong,
    pub buy: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct order_book {
    pub ghbids: *mut GHashTable,
    pub ghasks: *mut GHashTable,
    pub gtbids: *mut GTree,
    pub gtasks: *mut GTree,
    pub glbids: *mut GList,
    pub glasks: *mut GList,
}
#[inline]
unsafe extern "C" fn g_level_compare(
    mut pa: gconstpointer,
    mut pb: gconstpointer,
) -> gint {
    let mut la: *const ob_level = pa as *const ob_level;
    let mut lb: *const ob_level = pb as *const ob_level;
    let mut ret: gint = 0 as libc::c_int;
    if (*la).price < (*lb).price {
        ret = -(1 as libc::c_int);
    } else if (*la).price > (*lb).price {
        ret = 1 as libc::c_int;
    }
    return ret;
}
#[inline]
unsafe extern "C" fn g_uint_compare(
    mut pa: gconstpointer,
    mut pb: gconstpointer,
) -> gint {
    let mut a: libc::c_uint = pa as gulong as guint;
    let mut b: libc::c_uint = pb as gulong as guint;
    let mut ret: gint = 0 as libc::c_int;
    if a < b {
        ret = -(1 as libc::c_int);
    } else if a > b {
        ret = 1 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ob_init(mut ob: *mut order_book) -> libc::c_int {
    if !ob.is_null() {
        (*ob)
            .ghbids = g_hash_table_new_full(
            Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
            Some(
                g_direct_equal
                    as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
            ),
            None,
            Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        if !((*ob).ghbids).is_null() {
            (*ob)
                .ghasks = g_hash_table_new_full(
                Some(g_direct_hash as unsafe extern "C" fn(gconstpointer) -> guint),
                Some(
                    g_direct_equal
                        as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gboolean,
                ),
                None,
                Some(free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
            );
            if !((*ob).ghasks).is_null() {
                (*ob)
                    .gtasks = g_tree_new(
                    Some(
                        g_uint_compare
                            as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
                    ),
                );
                if !((*ob).gtasks).is_null() {
                    (*ob)
                        .gtbids = g_tree_new(
                        Some(
                            g_uint_compare
                                as unsafe extern "C" fn(
                                    gconstpointer,
                                    gconstpointer,
                                ) -> gint,
                        ),
                    );
                    if !((*ob).gtbids).is_null() {
                        (*ob).glasks = 0 as *mut GList;
                        (*ob).glbids = 0 as *mut GList;
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    ob_fini(ob);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ob_fini(mut ob: *mut order_book) {
    if ob.is_null() {
        return;
    }
    if !((*ob).ghbids).is_null() {
        g_hash_table_destroy((*ob).ghbids);
    }
    if !((*ob).ghasks).is_null() {
        g_hash_table_destroy((*ob).ghasks);
    }
    if !((*ob).gtbids).is_null() {
        g_tree_destroy((*ob).gtbids);
    }
    if !((*ob).gtasks).is_null() {
        g_tree_destroy((*ob).gtasks);
    }
    if !((*ob).glbids).is_null() {
        g_list_free((*ob).glbids);
    }
    if !((*ob).glasks).is_null() {
        g_list_free((*ob).glasks);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ob_clear(mut ob: *mut order_book) -> libc::c_int {
    ob_fini(ob);
    return ob_init(ob);
}
#[no_mangle]
pub unsafe extern "C" fn ob_level_lookup(
    mut ob: *mut order_book,
    mut order: *mut ob_order,
) -> *mut ob_level {
    let mut ghtable: *mut GHashTable = 0 as *mut GHashTable;
    if (*order).buy {
        ghtable = (*ob).ghbids;
    } else {
        ghtable = (*ob).ghasks;
    }
    return g_hash_table_lookup(ghtable, (*order).price as gpointer as gconstpointer)
        as *mut ob_level;
}
#[no_mangle]
pub unsafe extern "C" fn ob_level_modify(
    mut ob: *mut order_book,
    mut order: *mut ob_order,
) -> libc::c_int {
    let mut current_block: u64;
    let mut level: *mut ob_level = 0 as *mut ob_level;
    let mut ghtable: *mut GHashTable = 0 as *mut GHashTable;
    let mut glist: *mut *mut GList = 0 as *mut *mut GList;
    let mut gtree: *mut GTree = 0 as *mut GTree;
    if !ob.is_null() {
        if (*order).buy {
            ghtable = (*ob).ghbids;
            glist = &mut (*ob).glbids;
            gtree = (*ob).gtbids;
        } else {
            ghtable = (*ob).ghasks;
            glist = &mut (*ob).glasks;
            gtree = (*ob).gtasks;
        }
        level = g_hash_table_lookup(ghtable, (*order).price as gpointer as gconstpointer)
            as *mut ob_level;
        if level.is_null() {
            level = calloc(
                1 as libc::c_int as libc::c_ulong,
                ::core::mem::size_of::<ob_level>() as libc::c_ulong,
            ) as *mut ob_level;
            if level.is_null() {
                current_block = 1884538341147204295;
            } else {
                (*level).seq_num = (*order).seq_num;
                (*level).price = (*order).price;
                (*level).size = (*order).size;
                g_hash_table_insert(
                    ghtable,
                    (*level).price as gpointer,
                    level as gpointer,
                );
                *glist = g_list_insert_sorted(
                    *glist,
                    level as gpointer,
                    Some(
                        g_level_compare
                            as unsafe extern "C" fn(gconstpointer, gconstpointer) -> gint,
                    ),
                );
                g_tree_insert(gtree, (*level).price as gpointer, level as gpointer);
                current_block = 2370887241019905314;
            }
        } else if (*level).seq_num >= (*order).seq_num {
            current_block = 1884538341147204295;
        } else {
            (*level).seq_num = (*order).seq_num;
            (*level).size = (*order).size;
            current_block = 2370887241019905314;
        }
        match current_block {
            1884538341147204295 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ob_level_delete(
    mut ob: *mut order_book,
    mut order: *mut ob_order,
) -> libc::c_int {
    let mut current_block: u64;
    let mut level: *mut ob_level = 0 as *mut ob_level;
    let mut ghtable: *mut GHashTable = 0 as *mut GHashTable;
    let mut glist: *mut *mut GList = 0 as *mut *mut GList;
    let mut gtree: *mut GTree = 0 as *mut GTree;
    if !ob.is_null() {
        if (*order).buy {
            ghtable = (*ob).ghbids;
            glist = &mut (*ob).glbids;
            gtree = (*ob).gtbids;
        } else {
            ghtable = (*ob).ghasks;
            glist = &mut (*ob).glasks;
            gtree = (*ob).gtasks;
        }
        level = g_hash_table_lookup(ghtable, (*order).price as gpointer as gconstpointer)
            as *mut ob_level;
        if !level.is_null() {
            if g_hash_table_remove(ghtable, (*order).price as gpointer as gconstpointer)
                == 0
            {
                current_block = 7599097671867897590;
            } else if g_tree_remove(gtree, (*order).price as gpointer as gconstpointer)
                == 0
            {
                current_block = 7599097671867897590;
            } else {
                *glist = g_list_remove(*glist, level as gconstpointer);
                current_block = 12800627514080957624;
            }
        } else {
            current_block = 12800627514080957624;
        }
        match current_block {
            7599097671867897590 => {}
            _ => return 0 as libc::c_int,
        }
    }
    return -(1 as libc::c_int);
}
