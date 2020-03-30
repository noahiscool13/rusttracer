
pub(super) struct KDTreeNode {
    bounding_box: BoundingBox,

    left: Box<KDTreeNode>,
    right: Box<KDTreeNode>,

}