use crate::symbol::Symbol;
use crate::utils;
use std::fmt;
use std::iter::Flatten;
use std::slice::Iter as SliceIter;

const SYMBOL_TABLE_MAX_LOAD_FACTOR: f64 = 0.75;

type SymbolTuple = (Symbol, usize);
type SymbolSlot = Option<SymbolTuple>;

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
    pub fn insert(&mut self, symbol: Symbol) -> usize {
        if self.should_grow() {
            self.grow();
        }

        let mut i = (symbol.hash_code() as usize) % self.capacity();

        while let Some((old_symbol, old_symbol_index)) = self.values[i].as_ref() {
            if old_symbol == &symbol {
                return *old_symbol_index;
            } else {
                i = (i + 1) % self.capacity();
            }
        }

        self.len += 1;
        self.values[i] = Some((symbol, self.len));
        self.len
    }

    #[inline]
    pub fn iter(&self) -> SymbolTableIter {
        SymbolTableIter(self.values.iter().flatten())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn load_factor(&self) -> f64 {
        self.len as f64 / self.capacity() as f64
    }

    fn should_grow(&self) -> bool {
        if self.values.len() == 0 {
            true
        } else {
            (self.len + 1) as f64 / self.capacity() as f64 > SYMBOL_TABLE_MAX_LOAD_FACTOR
        }
    }

    fn grow(&mut self) {
        let next_cap =
            if self.capacity() == 0 { 5 } else { utils::next_prime(self.capacity() * 2) };

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
    use crate::symbol::Const;

    #[test]
    fn test_symbol_table() {
        let mut symbol_table = SymbolTable::default();
        assert!(symbol_table.is_empty());

        let i1 = symbol_table.insert(Symbol::Const(Const::I32(1)));
        assert!(symbol_table.len() == 1);

        let i2 = symbol_table.insert(Symbol::Const(Const::I32(1)));
        assert!(symbol_table.len() == 1);
        assert_eq!(i1, i2);

        let i3 = symbol_table.insert(Symbol::Ident("counter".to_string()));
        assert!(symbol_table.len() == 2);
        assert_ne!(i1, i3);
    }
}
