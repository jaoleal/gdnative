use sys;
use get_api;
use Variant;
use VariantArray;
use GodotString;
use GodotType;
use std::fmt;

/// A `Dictionary` of `Variant` key-value pairs.
pub struct Dictionary(pub(crate) sys::godot_dictionary);

impl Dictionary {
    /// Creates an empty `Dictionary`.
    pub fn new() -> Self { Dictionary::default() }

    /// Returns `true` if the `Dictionary` contains no elements.
    pub fn is_empty(&self) -> bool {
        unsafe {
            (get_api().godot_dictionary_empty)(&self.0)
        }
    }

    /// Returns the number of elements in the `Dictionary`.
    pub fn len(&self) -> i32 {
        unsafe {
            (get_api().godot_dictionary_size)(&self.0)
        }
    }

    /// Clears the `Dictionary`, removing all key-value pairs.
    pub fn clear(&mut self) {
        unsafe {
            (get_api().godot_dictionary_clear)(&mut self.0)
        }
    }

    /// Returns true if the `Dictionary` contains the specified key.
    pub fn contains(&self, key: &Variant) -> bool {
        unsafe {
            (get_api().godot_dictionary_has)(&self.0, &key.0)
        }
    }

    /// Returns true if the `Dictionary` has all of the keys in the given array.
    pub fn contains_all(&self, keys: &VariantArray) -> bool {
        unsafe {
            (get_api().godot_dictionary_has_all)(&self.0, &keys.0)
        }
    }

    /// Erase a key-value pair in the `Dictionary` by the specified key.
    pub fn erase(&mut self, key: &Variant) {
        unsafe {
            (get_api().godot_dictionary_erase)(&mut self.0, &key.0)
        }
    }

    /// Returns a copy of the value corresponding to the key.
    pub fn get(&self, key: &Variant) -> Variant {
        unsafe {
            Variant((get_api().godot_dictionary_get)(&self.0, &key.0))
        }
    }

    /// Sets a value to the element corresponding to the key.
    pub fn set(&mut self, key: &Variant, val: &Variant) {
        unsafe {
            (get_api().godot_dictionary_set)(&mut self.0, &key.0, &val.0)
        }
    }

    /// Returns a reference to the value corresponding to the key.
    pub fn get_ref(&self, key: &Variant) -> &Variant {
        unsafe {
            Variant::cast_ref((get_api().godot_dictionary_operator_index_const)(&self.0, &key.0))
        }
    }

    /// Returns a mutable reference to the value corresponding to the key.
    pub fn get_mut_ref(&mut self, key: &Variant) -> &mut Variant {
        unsafe {
            Variant::cast_mut_ref((get_api().godot_dictionary_operator_index)(&mut self.0, &key.0))
        }
    }

    /// Returns a GodotString of the `Dictionary`.
    pub fn to_json(&self) -> GodotString {
        unsafe {
            GodotString((get_api().godot_dictionary_to_json)(&self.0))
        }
    }

    /// Returns an array of the keys in the `Dictionary`.
    pub fn keys(&self) -> VariantArray {
        unsafe {
            VariantArray((get_api().godot_dictionary_keys)(&self.0))
        }
    }

    /// Returns an array of the values in the `Dictionary`.
    pub fn values(&self) -> VariantArray {
        unsafe {
            VariantArray((get_api().godot_dictionary_values)(&self.0))
        }
    }

    pub fn get_next(&self, key: &Variant) -> &Variant {
        unsafe {
            Variant::cast_ref((get_api().godot_dictionary_next)(&self.0, &key.0))
        }
    }

    /// Return a hashed i32 value representing the dictionary's contents.
    pub fn hash(&self) -> i32 {
        unsafe {
            (get_api().godot_dictionary_hash)(&self.0)
        }
    }
}

impl_basic_traits!(
    for Dictionary as godot_dictionary {
        Drop => godot_dictionary_destroy;
        Clone => godot_dictionary_new_copy;
        Default => godot_dictionary_new;
        Eq => godot_dictionary_operator_equal;
    }
);

impl GodotType for Dictionary {
    fn to_variant(&self) -> Variant { Variant::from_dictionary(self) }
    fn from_variant(variant: &Variant) -> Option<Self> { variant.try_to_dictionary() }
}

impl fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.to_json().to_string().fmt(f)
    }
}

godot_test!(test_dictionary {
    use VariantType;
    let foo = Variant::from_str("foo");
    let bar = Variant::from_str("bar");
    let nope = Variant::from_str("nope");

    let mut dict = Dictionary::new();

    dict.set(&foo, &Variant::from_i64(42));
    dict.set(&bar, &Variant::from_i64(1337));

    assert!(dict.contains(&foo));
    assert!(dict.contains(&bar));
    assert!(!dict.contains(&nope));

    let variant = Variant::from_dictionary(&dict);
    assert!(variant.get_type() == VariantType::Dictionary);

    let dict_clone = dict.clone();
    assert!(dict == dict_clone);
    assert!(dict_clone.contains(&foo));
    assert!(dict_clone.contains(&bar));

    if let Some(dic_variant) = variant.try_to_dictionary() {
        assert!(dic_variant == dict);
    } else {
        panic!("variant should be a Dictionary");
    }
});
