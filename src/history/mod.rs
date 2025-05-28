use std::collections::VecDeque;

pub mod packet;
pub mod traffic;

const DEFAULT_HISTORY_SIZE: usize = 60;

struct History<V> {
    store: VecDeque<V>,
    max_size: usize,
}

impl<V> History<V> {
    pub fn new(max_size: usize) -> Self {
        Self {
            store: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn add(&mut self, value: V) {
        if self.store.len() >= self.max_size {
            self.store.pop_front();
        }

        self.store.push_back(value);
    }

    pub fn values(&self) -> &VecDeque<V> {
        &self.store
    }
}

enum Direction {
    Download,
    Upload,
}

#[macro_export]
macro_rules! define_directional_history {
    (
        $name:ident,
        $value:ty,
        $size:expr,
        {
            $( $field:ident : $field_ty:ty = $field_init:expr, )*
        },
        {
            $( $extra_fn:item )*
        }
    ) => {
        pub struct $name {
            history: History<$value>,
            direction: Direction,
            $( $field: $field_ty, )*
        }

        impl $name {
            fn new(direction: Direction) -> Self {
                Self {
                    history: History::new($size),
                    direction,
                    $( $field: $field_init, )*
                }
            }

            pub fn download() -> Self {
                Self::new(Direction::Download)
            }

            pub fn upload() -> Self {
                Self::new(Direction::Upload)
            }

            $( $extra_fn )*
        }
    };
}
