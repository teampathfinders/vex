use std::marker::PhantomData;

use crate::{component::{Component, ComponentCollection}, filter::FilterCollection};

pub struct Req<C, F = ()>
where
    C: ComponentCollection,
    F: FilterCollection,
{
    _marker: PhantomData<(C, F)>
}

impl<'a, C, F> IntoIterator for &'a Req<C, F>
where
    C: ComponentCollection,
    F: FilterCollection,
{
    type IntoIter = ReqIter<'a, C, F>;
    type Item = C;

    fn into_iter(self) -> Self::IntoIter {
        ReqIter {
            req: self
        }
    }   
}

pub struct ReqIter<'a, C, F>
where
    C: ComponentCollection,
    F: FilterCollection
{
    req: &'a Req<C, F>
}

impl<C, F> Iterator for ReqIter<'_, C, F> 
where 
    C: ComponentCollection,
    F: FilterCollection
{
    type Item = C;

    fn next(&mut self) -> Option<Self::Item> {
        todo!();
    }   
}