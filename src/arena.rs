use std::collections::HashMap;
use std::ops::{Index, IndexMut};

use crate::models::LimitOrder;

#[derive(Debug)]
pub struct OrderArena {
    orders: Vec<LimitOrder>,
    free: Vec<usize>,
    order_map: HashMap<String, usize>,
}

impl OrderArena {
    pub fn new(capacity: usize) -> Self {
        let mut list = Self {
            orders: Vec::with_capacity(capacity),
            free: Vec::with_capacity(capacity),
            order_map: HashMap::with_capacity(capacity),
        };

        // Preallocate
        for i in 0..capacity {
            list.orders.push(LimitOrder {
                id: String::from("0"),
                price: 0.0,
                qty: 0.0,
            });
            list.free.push(i);
        }
        list
    }

    pub fn get(&self, id: String) -> Option<(f64, usize)> {
        self.order_map.get(&id).map(|i| (self.orders[*i].price, *i))
    }

    #[cfg(test)]
    pub fn get_full(&self, id: String) -> Option<(f64, f64, usize)> {
        self.order_map
            .get(&id)
            .map(|i| (self.orders[*i].price, self.orders[*i].qty, *i))
    }

    pub fn insert(&mut self, id: String, price: f64, qty: f64) -> usize {
        match self.free.pop() {
            None => {
                self.orders.push(LimitOrder { id: id.clone(), price, qty });
                let index = self.orders.len() - 1;
                self.order_map.insert(id.clone( ), index);
                index
            }
            Some(index) => {
                let ord = &mut self.orders[index];
                ord.id = id.clone();
                ord.qty = qty;
                ord.price = price;
                self.order_map.insert(id.clone(), index);
                index
            }
        }
    }

    pub fn delete(&mut self, id: &String) -> bool {
        if let Some(idx) = self.order_map.remove(id) {
            if let Some(mut ord) = self.orders.get_mut(idx) {
                self.free.push(idx);
                ord.qty = 0.0;
                return true;
            }
        }
        false
    }
}

impl Index<usize> for OrderArena {
    type Output = LimitOrder;

    #[inline]
    fn index(&self, index: usize) -> &LimitOrder {
        &self.orders[index]
    }
}

impl IndexMut<usize> for OrderArena {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut LimitOrder {
        &mut self.orders[index]
    }
}

// #[cfg(test)]
// mod test {
//     use super::OrderArena;

//     #[test]
//     fn growing_arena() {
//         // All the integer casting below is necessary because we are using the
//         // indices to compute the prices. It's a contrived example and the size
//         // casts do not result in overflows.
//         //
//         // This test also addresses a bug that only occurred after all the
//         // pre-allocated limit orders were used. The new limit orders would be
//         // created with a swapped quantity and price, which unfortunately have
//         // the same type (u64) and the compiler could not catch that bug.
//         for capacity in 0_u64..30 {
//             let mut arena = OrderArena::new(capacity as usize);
//             for i in 0_u64..capacity {
//                 arena.insert(i as u128, i * 100 + i, 2 * i);
//             }
//             for i in 0_u64..capacity {
//                 assert_eq!(
//                     arena.get_full(i as u128),
//                     Some((i * 100 + i, 2 * i, (capacity - i) as usize - 1))
//                 );
//             }
//             for i in capacity..2 * capacity {
//                 assert_eq!(arena.get_full(i as u128), None);
//             }
//             for i in capacity..2 * capacity {
//                 arena.insert(i as u128, i * 100 + i, 2 * i);
//             }
//             for i in 0..capacity {
//                 assert_eq!(
//                     arena.get_full(i as u128),
//                     Some((i * 100 + i, 2 * i, (capacity - i) as usize - 1))
//                 );
//             }
//             for i in capacity..2 * capacity {
//                 assert_eq!(
//                     arena.get_full(i as u128),
//                     Some((i * 100 + i, 2 * i, i as usize,))
//                 );
//             }
//         }
//     }
// }
