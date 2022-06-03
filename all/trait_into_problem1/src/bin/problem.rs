// xxx : for test

fn main()
{
}

pub trait IdentityInterface
{
}

pub trait EditableInterface
{
  type Id : IdentityInterface;

  /// Iterate output nodes of the node.
  fn node_extend_out_nodes< Id, Iter >
  (
    &mut self,
    node_id : Id,
    out_nodes_iter : Iter,
  )
  where
    Id : Into< Self::Id >,
    Iter : IntoIterator< Item = Id >,
  ;

  /// Iterate output nodes of the node.
  fn node_extend_out_node< Id >
  (
    &mut self,
    node_id : Id,
    out_node_id : Id,
  )
  where
    Id : Into< Self::Id >,
    core::iter::Once< Id > : Clone,
  {
    let out_node_id : Self::Id = out_node_id.into();
    self.node_extend_out_nodes( node_id, core::iter::once( out_node_id ) );
    //
    // throws excpetion:
    // ```
    //    |
    // 26 |   fn node_extend_out_node< Id >
    //    |                            -- this type parameter
    // ...
    // 37 |     self.node_extend_out_nodes( node_id, core::iter::once( out_node_id ) );
    //    |          ^^^^^^^^^^^^^^^^^^^^^ expected type parameter `Id`, found associated type
    //    |
    //    = note: expected type parameter `Id`
    //              found associated type `<Self as EditableInterface>::Id`
    //    = note: you might be missing a type parameter or trait bound
    //```
    //
    // but variant bellow works
    //
    // self.node_extend_out_nodes( node_id, core::iter::once( out_node_id ) );
  }

}
