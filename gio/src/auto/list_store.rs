// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ListModel;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;
#[cfg(any(feature = "v2_64", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
use std::mem;

glib::wrapper! {
    pub struct ListStore(Object<ffi::GListStore, ffi::GListStoreClass>) @implements ListModel;

    match fn {
        type_ => || ffi::g_list_store_get_type(),
    }
}

impl ListStore {
    #[doc(alias = "g_list_store_new")]
    pub fn new(item_type: glib::types::Type) -> ListStore {
        unsafe { from_glib_full(ffi::g_list_store_new(item_type.into_glib())) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`ListStore`]
    /// This method returns an instance of [`ListStoreBuilder`] which can be used to create a [`ListStore`].
    pub fn builder() -> ListStoreBuilder {
        ListStoreBuilder::default()
    }

    #[doc(alias = "g_list_store_append")]
    pub fn append<P: IsA<glib::Object>>(&self, item: &P) {
        unsafe {
            ffi::g_list_store_append(self.to_glib_none().0, item.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_64", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
    #[doc(alias = "g_list_store_find")]
    pub fn find<P: IsA<glib::Object>>(&self, item: &P) -> Option<u32> {
        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::g_list_store_find(
                self.to_glib_none().0,
                item.as_ref().to_glib_none().0,
                position.as_mut_ptr(),
            ));
            let position = position.assume_init();
            if ret {
                Some(position)
            } else {
                None
            }
        }
    }

    //#[cfg(any(feature = "v2_64", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
    //#[doc(alias = "g_list_store_find_with_equal_func")]
    //pub fn find_with_equal_func<P: IsA<glib::Object>>(&self, item: &P, equal_func: /*Unimplemented*/FnMut(/*Unimplemented*/Option<Fundamental: Pointer>, /*Unimplemented*/Option<Fundamental: Pointer>) -> bool) -> Option<u32> {
    //    unsafe { TODO: call ffi:g_list_store_find_with_equal_func() }
    //}

    #[doc(alias = "g_list_store_insert")]
    pub fn insert<P: IsA<glib::Object>>(&self, position: u32, item: &P) {
        unsafe {
            ffi::g_list_store_insert(
                self.to_glib_none().0,
                position,
                item.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_list_store_remove")]
    pub fn remove(&self, position: u32) {
        unsafe {
            ffi::g_list_store_remove(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "g_list_store_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::g_list_store_remove_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_list_store_splice")]
    pub fn splice(&self, position: u32, n_removals: u32, additions: &[glib::Object]) {
        let n_additions = additions.len() as u32;
        unsafe {
            ffi::g_list_store_splice(
                self.to_glib_none().0,
                position,
                n_removals,
                additions.to_glib_none().0,
                n_additions,
            );
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`ListStore`].
pub struct ListStoreBuilder {
    item_type: Option<glib::types::Type>,
}

impl ListStoreBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ListStoreBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ListStore`].
    pub fn build(self) -> ListStore {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref item_type) = self.item_type {
            properties.push(("item-type", item_type));
        }
        glib::Object::new::<ListStore>(&properties)
            .expect("Failed to create an instance of ListStore")
    }

    pub fn item_type(mut self, item_type: glib::types::Type) -> Self {
        self.item_type = Some(item_type);
        self
    }
}

impl fmt::Display for ListStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ListStore")
    }
}
