#![allow(dead_code)]
use serde::Serialize;

pub trait TreeNode: Serialize + Clone {
    fn id(&self) -> String;
    fn parent_id(&self) -> Option<String>;
}

#[derive(Serialize, Debug, Clone)]
pub struct Tree<T>
where
    T: TreeNode,
{
    #[serde(flatten)]
    data: T,
    children: Vec<Tree<T>>,
}

impl<T> Tree<T>
where
    T: TreeNode,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            children: vec![],
        }
    }
    pub fn build(data: Vec<T>, root_id: Option<String>) -> Vec<Tree<T>> {
        Self::build_tree(data.into_iter().map(|d| Tree::new(d)).collect(), root_id)
    }
    fn build_tree(data: Vec<Tree<T>>, parent_id: Option<String>) -> Vec<Tree<T>> {
        let mut tree = Vec::new();
        for mut item in data.clone() {
            if item.data.parent_id() == parent_id {
                item.children = Self::build_tree(data.clone(), Some(item.data.id()));
                tree.push(item.clone());
            }
        }
        tree
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize, Debug, Clone)]
    struct TestNode {
        id: String,
        parent_id: Option<String>,
    }

    impl TreeNode for TestNode {
        fn id(&self) -> String {
            self.id.clone()
        }

        fn parent_id(&self) -> Option<String> {
            self.parent_id.clone()
        }
    }

    #[test]
    fn test_build_tree_happy_path() {
        // 创建测试数据
        let nodes = vec![
            TestNode {
                id: "1".to_string(),
                parent_id: None,
            },
            TestNode {
                id: "2".to_string(),
                parent_id: Some("1".to_string()),
            },
            TestNode {
                id: "3".to_string(),
                parent_id: Some("1".to_string()),
            },
            TestNode {
                id: "4".to_string(),
                parent_id: Some("2".to_string()),
            },
        ];

        // 构建树
        let tree = Tree::build(nodes, None);

        // 验证树的结构
        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].data.id(), "1".to_string());
        assert_eq!(tree[0].children.len(), 2);
        assert_eq!(tree[0].children[0].data.id(), "2".to_string());
        assert_eq!(tree[0].children[0].children.len(), 1);
        assert_eq!(tree[0].children[0].children[0].data.id(), "4".to_string());
        assert_eq!(tree[0].children[1].data.id(), "3".to_string());
        assert_eq!(tree[0].children[1].children.len(), 0);
    }

    #[test]
    fn test_build_tree_empty_input() {
        // 创建空数据
        let nodes: Vec<TestNode> = vec![];

        // 构建树
        let tree = Tree::build(nodes, None);

        // 验证树为空
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_build_tree_no_root() {
        // 创建没有根节点的数据
        let nodes = vec![
            TestNode {
                id: "1".to_string(),
                parent_id: Some("0".to_string()),
            },
            TestNode {
                id: "2".to_string(),
                parent_id: Some("1".to_string()),
            },
        ];

        // 构建树
        let tree = Tree::build(nodes, None);

        // 验证树为空
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_build_tree_multiple_roots() {
        // 创建多个根节点的数据
        let nodes = vec![
            TestNode {
                id: "1".to_string(),
                parent_id: None,
            },
            TestNode {
                id: "2".to_string(),
                parent_id: None,
            },
        ];

        // 构建树
        let tree = Tree::build(nodes, None);

        // 验证树的结构
        assert_eq!(tree.len(), 2);
        assert_eq!(tree[0].data.id(), "1".to_string());
        assert_eq!(tree[0].children.len(), 0);
        assert_eq!(tree[1].data.id(), "2".to_string());
        assert_eq!(tree[1].children.len(), 0);
    }
}
