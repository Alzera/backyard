use bumpalo::{ Bump, boxed::Box, collections::Vec };

pub trait CloneIn<'arena>: Sized {
  type Cloned: ?Sized;

  fn clone_in(&self, arena: &'arena Bump) -> Self::Cloned;
}

impl<'arena, T, C: 'arena> CloneIn<'arena> for Box<'_, T> where T: CloneIn<'arena, Cloned = C> {
  type Cloned = Box<'arena, C>;

  fn clone_in(&self, allocator: &'arena Bump) -> Self::Cloned {
    Box::new_in(self.as_ref().clone_in(allocator), allocator)
  }
}

impl<'arena, T, C: 'arena> CloneIn<'arena> for Vec<'_, T> where T: CloneIn<'arena, Cloned = C> {
  type Cloned = Vec<'arena, C>;

  fn clone_in(&self, allocator: &'arena Bump) -> Self::Cloned {
    Vec::from_iter_in(
      self.iter().map(|it| it.clone_in(allocator)),
      allocator
    )
  }
}
