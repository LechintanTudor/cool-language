use crate::symbols::Symbol;
use crate::utils;
use std::fmt;
use std::iter::Flatten;
use std::slice::Iter as SliceIter;

/// Max load factor before the [SymbolTable] resizes.
const SYMBOL_TABLE_MAX_LOAD_FACTOR: f64 = 0.75;

/// [Symbol] and associated code.
type SymbolTuple = (Symbol, usize);

/// Slot for storing a [Symbol].
type SymbolSlot = Option<SymbolTuple>;

/// Hashmap-based symbol table.
#[derive(Default)]
pub struct SymbolTable {
    values: Vec<SymbolSlot>,
    len: usize,
}

impl fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_map();
        debug.entries(self.iter().map(|(ref s, ref i)| (s, i)));
        debug.finish()
    }
}

impl SymbolTable {
    /// Inserts a symbol into the table if it doesn't exist already.
    /// Returns the code associated to the symbol.
    pub fn insert(&mut self, symbol: Symbol) -> usize {
        if self.should_grow() {
            self.grow();
        }

        let mut i = (symbol.hash_code() as usize) % self.slots();

        while let Some((old_symbol, old_symbol_index)) = self.values[i].as_ref() {
            if old_symbol == &symbol {
                return *old_symbol_index;
            } else {
                i = (i + 1) % self.slots();
            }
        }

        self.len += 1;
        self.values[i] = Some((symbol, self.len));
        self.len
    }

    /// Returns the code associated to the symbol, if it exists.
    pub fn get(&self, symbol: &Symbol) -> Option<usize> {
        let mut i = (symbol.hash_code() as usize) % self.slots();

        while let Some((old_symbol, index)) = self.values[i].as_ref() {
            if old_symbol == symbol {
                return Some(*index);
            } else {
                i = (i + 1) % self.slots();
            }
        }

        None
    }

    /// Returns whether the table contains a symbol.
    #[inline]
    pub fn contains(&self, symbol: &Symbol) -> bool {
        self.get(symbol).is_some()
    }

    /// Returns an iterator over all symbols in the table and their associated codes.
    #[inline]
    pub fn iter(&self) -> SymbolTableIter {
        SymbolTableIter(self.values.iter().flatten())
    }

    /// Returns the symbol table as a sorted vector.
    pub fn to_sorted_vec(&self) -> Vec<(&Symbol, usize)> {
        let mut symbols = self.iter().map(|(symbol, id)| (symbol, *id)).collect::<Vec<_>>();
        symbols.sort_by_key(|(_, id)| *id);
        symbols
    }

    /// Returns the number of symbols in the table.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns whether the table is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of symbol slots available.
    #[inline]
    fn slots(&self) -> usize {
        self.values.len()
    }

    /// Returns the load factor of the hash table.
    #[inline]
    fn load_factor(&self) -> f64 {
        self.len as f64 / self.slots() as f64
    }

    /// Returns whether the table should grow before inserting a new elements.
    fn should_grow(&self) -> bool {
        if self.values.len() == 0 {
            true
        } else {
            (self.len + 1) as f64 / self.slots() as f64 > SYMBOL_TABLE_MAX_LOAD_FACTOR
        }
    }

    /// Grows the hash table.
    fn grow(&mut self) {
        let next_cap = if self.slots() == 0 { 5 } else { utils::next_prime(self.slots() * 2) };

        let mut next_values = vec![SymbolSlot::None; next_cap];

        for (symbol, symbol_index) in self.values.drain(..).flatten() {
            let mut i = symbol.hash_code() as usize % next_values.len();

            while next_values[i].is_some() {
                i = (i + 1) % next_values.len();
            }

            next_values[i] = Some((symbol, symbol_index));
        }

        self.values = next_values;
    }
}

/// Iterator over all symbols in the table and their associated codes.
pub struct SymbolTableIter<'a>(Flatten<SliceIter<'a, SymbolSlot>>);

impl<'a> Iterator for SymbolTableIter<'a> {
    type Item = &'a SymbolTuple;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbols::Const;

    #[test]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::default();
        assert!(symbol_table.is_empty());

        let s1 = Symbol::Const(Const::I32(1));
        let s2 = Symbol::Ident("counter".to_string());

        // Add a new symbol
        let c1 = symbol_table.insert(s1.clone());
        assert!(symbol_table.len() == 1);
        assert!(symbol_table.contains(&s1));

        // Add an existing symbol
        assert_eq!(symbol_table.insert(s1.clone()), c1);
        assert_eq!(symbol_table.len(), 1);
        assert!(symbol_table.contains(&s1));

        // Add a new symbol
        let c2 = symbol_table.insert(s2.clone());
        assert_ne!(c1, c2);
        assert_eq!(symbol_table.len(), 2);
        assert!(symbol_table.contains(&s1));
        assert!(symbol_table.contains(&s2));
    }
}
