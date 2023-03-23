use std::marker::PhantomData;

use crate::component::Component;

pub struct With<C> {
    _marker: PhantomData<C>
}

pub struct Without<C> {
    _marker: PhantomData<C>
}

pub trait FilterCollection {
    fn filter() -> bool;
}

impl FilterCollection for () {
    fn filter() -> bool {
        // Don't filter anything 
        true
    }
}

impl<F> FilterCollection for With<F> where F: Component {
    fn filter() -> bool {
        println!("Filtering with {}", std::any::type_name::<Self>());
        true
    }
}

impl<F> FilterCollection for Without<F> where F: Component {
    fn filter() -> bool {
        true
    }
}