// use recursion::{map_layer::MapLayer, recursive_tree::{RecursiveTree, StackMarker}};

// pub enum ValueF<A> {
//     Int { n: i64 },
//     Fun { arg: String, body: A },
// }

// impl<A, B> MapLayer<B> for ValueF<A> {
//     type To = ValueF<B>;
//     type Unwrapped = A;

//     fn map_layer<F: FnMut(Self::Unwrapped) -> B>(self, mut f: F) -> Self::To {
//         match self {
//             ValueF::Int { n } => ValueF::Int { n },
//             ValueF::Fun { arg, body } => ValueF::Fun { arg, body: f(body) },
//         }
//     }
// }

// pub type Value = RecursiveTree<ValueF<StackMarker>, StackMarker>;

#[derive(Debug)]
pub enum Value {
    Int { n: i64 },
    Fun { arg: String, body: Box<Value> },
}
