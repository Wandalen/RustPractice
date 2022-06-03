
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
  fn node_extend_out_nodes< Id1, Id2, Iter >
  (
    &mut self,
    node_id : Id1,
    out_nodes_iter : Iter,
  )
  where
    Id1 : Into< Self::Id >,
    Id2 : Into< Self::Id >,
    Iter : IntoIterator< Item = Id2 >,
  ;

  /// Iterate output nodes of the node.
  fn node_extend_out_node< Id1, Id2 >
  (
    &mut self,
    node_id : Id1,
    out_node_id : Id2,
  )
  where
    Id1 : Into< Self::Id >,
    Id2 : Into< Self::Id >,
    Id2 : Clone,
  {
    let out_node_id : Self::Id = out_node_id.into();
    self.node_extend_out_nodes( node_id, core::iter::once( out_node_id ) );
  }

}
