/*
    fn next(&mut self) -> Option<Self::Item>; // Devuelve el próximo elemento (core del trait)

    // Métodos provistos por default:

    fn size_hint(&self) -> (usize, Option<usize>) { ... }          // Estima cuántos elementos quedan
    fn count(self) -> usize { ... }                                // Cuenta cuántos elementos tiene
    fn last(self) -> Option<Self::Item> { ... }                    // Devuelve el último elemento
    fn nth(&mut self, n: usize) -> Option<Self::Item> { ... }      // Devuelve el n-ésimo elemento
    fn step_by(self, step: usize) -> StepBy<Self> { ... }          // Itera de a saltos
    fn chain<U>(self, other: U) -> Chain<Self, U::IntoIter> { ... } // Une dos iteradores
    fn zip<U>(self, other: U) -> Zip<Self, U::IntoIter> { ... }    // Une dos iteradores en tuplas
    fn map<B, F>(self, f: F) -> Map<Self, F> { ... }                // Transforma cada ítem
    fn for_each<F>(self, f: F) { ... }                              // Ejecuta una función por ítem
    fn filter<P>(self, predicate: P) -> Filter<Self, P> { ... }     // Filtra los ítems
    fn filter_map<B, F>(self, f: F) -> FilterMap<Self, F> { ... }   // Filtra y transforma a la vez
    fn enumerate(self) -> Enumerate<Self> { ... }                  // Agrega índice a cada ítem
    fn peekable(self) -> Peekable<Self> { ... }                    // Permite mirar sin consumir
    fn skip(self, n: usize) -> Skip<Self> { ... }                  // Salta los primeros `n` ítems
    fn take(self, n: usize) -> Take<Self> { ... }                  // Toma los primeros `n` ítems
    fn scan<B, F>(self, initial: B, f: F) -> Scan<Self, B, F> { ... } // Acumulador con estado
    fn flat_map<U, F>(self, f: F) -> FlatMap<Self, F, U> { ... }   // Aplana y mapea
    fn flatten(self) -> Flatten<Self> { ... }                      // Aplana estructuras anidadas
    fn fuse(self) -> Fuse<Self> { ... }                            // Después de None, siempre None
    fn inspect<F>(self, f: F) -> Inspect<Self, F> { ... }          // Espía cada ítem (debug/log)
    fn by_ref(&mut self) -> &mut Self { ... }                      // Permite seguir usando el iterador
    fn collect<B>(self) -> B where B: FromIterator<Self::Item> { ... } // Convierte a Vec, HashMap, etc.
    fn partition<B, F>(self, f: F) -> (B, B) { ... }               // Divide en dos colecciones por predicado
    fn all<F>(self, f: F) -> bool { ... }                          // `true` si todos cumplen
    fn any<F>(self, f: F) -> bool { ... }                          // `true` si alguno cumple
    fn find<F>(&mut self, predicate: F) -> Option<Self::Item> { ... } // Devuelve el primero que cumple
    fn find_map<B, F>(self, f: F) -> Option<B> { ... }             // Mapea y encuentra a la vez
    fn position<F>(self, predicate: F) -> Option<usize> { ... }    // Devuelve índice del primero que cumple
    fn rposition<F>(self, predicate: F) -> Option<usize>           // Igual, pero de atrás para adelante
        where Self: DoubleEndedIterator { ... }
    fn max(self) -> Option<Self::Item> where Self::Item: Ord { ... }      // Máximo (requiere `Ord`)
    fn min(self) -> Option<Self::Item> where Self::Item: Ord { ... }      // Mínimo
    fn max_by<F>(self, compare: F) -> Option<Self::Item> { ... }          // Máximo con función personalizada
    fn min_by<F>(self, compare: F) -> Option<Self::Item> { ... }          // Mínimo personalizado
    fn max_by_key<B, F>(self, f: F) -> Option<Self::Item> { ... }         // Máximo según clave
    fn min_by_key<B, F>(self, f: F) -> Option<Self::Item> { ... }         // Mínimo según clave
    fn sum<S>(self) -> S where S: Sum<Self::Item> { ... }                 // Suma todos los ítems
    fn product<P>(self) -> P where P: Product<Self::Item> { ... }         // Multiplica todos los ítems
    fn cmp<I>(self, other: I) -> Ordering                                 // Compara dos iteradores
        where I: IntoIterator<Item = Self::Item>, Self::Item: Ord { ... }
    fn eq<I>(self, other: I) -> bool                                      // Igualdad entre iteradores
        where I: IntoIterator<Item = Self::Item>, Self::Item: PartialEq { ... }
    fn ne<I>(self, other: I) -> bool                                      // Distintos
        where I: IntoIterator<Item = Self::Item>, Self::Item: PartialEq { ... }
    fn lt<I>(self, other: I) -> bool                                      // Menor
        where I: IntoIterator<Item = Self::Item>, Self::Item: PartialOrd { ... }
    fn le<I>(self, other: I) -> bool                                      // Menor o igual
        where I: IntoIterator<Item = Self::Item>, Self::Item: PartialOrd { ... }
    fn gt<I>(self, other: I) -> bool                                      // Mayor
        where I: IntoIterator<Item = Self::Item>, Self::Item: PartialOrd { ... }
    fn ge<I>(self, other: I) -> bool                                      // Mayor o igual
        where I: IntoIterator<Item = Self::Item>, Self::Item: PartialOrd { ... }
    fn is_sorted(self) -> bool where Self::Item: PartialOrd { ... }       // ¿Está ordenado?
    fn is_sorted_by<F>(self, compare: F) -> bool { ... }                  // ¿Ordenado con función?
    fn is_sorted_by_key<F, K>(self, f: F) -> bool { ... }                 // ¿Ordenado por clave?
    fn cloned<'a, T>(self) -> Cloned<Self>                                // Clona cada ítem si son `&T`
        where T: 'a + Clone, Self: Iterator<Item = &'a T> { ... }
    fn copied<'a, T>(self) -> Copied<Self>                                // Copia cada ítem si son `&Copy`
        where T: 'a + Copy, Self: Iterator<Item = &'a T> { ... }
    fn cycle(self) -> Cycle<Self> where Self: Clone { ... }               // Repite el iterador infinitamente
    fn sum_by_key<B, F>(self, f: F) -> B { ... } // nightly                // Suma aplicando una clave (solo nightly)
*/
