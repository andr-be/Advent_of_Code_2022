
use std::{sync::{RwLock, Arc, Weak}, fmt::Display, ops::{Deref, DerefMut}};

/// Advent of Code 2022 - Day 7 - Parts 1 & 2
/// This code is going to serve as my first implementation of a non-binary tree.
/// 
/// A non-binary tree is a structure built from nodes that are linked to each other by pointers.
/// 1) these nodes will have a value that is their name as a string
/// 2) these nodes have a value (in this instance, this value will be the sum of files in the dir)
/// 3) they can also have children nodes, that they 'own' i.e. they go out of scope with the parent
/// 4) this implies that nodes can have parents, that will not go out of scope with the child
///
/// My approach is going to be to build the tree structure based on an input parse of the string
/// each time 'dir' is referenced I will add a new child node
/// each time a file is listed I will add its value to the current node value
/// each time a cd command is given, I will traverse the tree to the parent node

pub struct NodeData<T>
where
    T: Display,
{
    value: T,
    parent: Parent<T>,
    children: Children<T>,
}

type NodeDataRef<T> = Arc<NodeData<T>>;
type WeakNodeNodeRef<T> = Weak<NodeData<T>>;

/// Parent relationship is one of non-ownership
/// This is not a `RwLock<NodeDataRef<T>` which would cause a memory leak
type Parent<T> = RwLock<WeakNodeNodeRef<T>>;

/// Children relationship is one of ownership
type Children<T> = RwLock<Vec<Child<T>>>;
type Child<T> = NodeDataRef<T>;

pub struct Node<T: Display> {
    arc_ref: NodeDataRef<T>,
}

impl<T> Deref for Node<T>
where 
    T: Display,
{
    type Target = NodeData<T>;

    fn deref(&self) -> &Self::Target {
        &self.arc_ref
    }
}

impl<T> DerefMut for Node<T>
where   
    T: Display,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.arc_ref
    }
}

impl<T> Node<T>
where
    T: Display,
{
    pub fn new(value: T) -> Node<T> {
        let new_node = NodeData {
            value,
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
        };
        let arc_ref = Arc::new(new_node);
        Node { arc_ref }
    }

    pub fn get_copy_of_internal_arc(self: &Self) -> NodeDataRef<T> {
        Arc::clone(&self.arc_ref)
    }

    pub fn create_and_add_child(
        self: &Self,
        value: T,
    ) -> NodeDataRef<T> {
        let new_child = Node::new(value);
        self.add_child_and_update_its_parent(&new_child);
        new_child.get_copy_of_internal_arc()
    }
    
    /// WRITE LOCKS USED
    pub fn add_child_and_update_its_parent(
        self: &Self,
        child: &Node<T>,
    ) {
        {
            let mut my_children = self.arc_ref.children.write().unwrap();
            my_children.push(child.get_copy_of_internal_arc());
        } // `my_children` guard dropped

        {
            let mut childs_parent = child.arc_ref.parent.write().unwrap();
            *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
        } // `my_parent` guard dropped
    }

    pub fn has_parent(self: &Self) -> bool {
        self.get_parent().is_some()
    }

    /// READ LOCK USED
    pub fn get_parent(self: &Self) -> Option<NodeDataRef<T>> {
        let my_parent_weak = self.arc_ref.parent.read().unwrap();
        if let Some(my_parent_arc_ref) = my_parent_weak.upgrade() {
            Some(my_parent_arc_ref)
        } else {
            None
        }
    }
}

struct Directory {
    name: String,
    size: usize,
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let size = &self.size;
        write!(f, "{}: {}", name, size)
    }
}

fn main() {
    let mut dir_1 = Directory { name: "/".to_string(), size: 0,};
    let mut dir_2 = Directory { name: "home".to_string(), size: 0};

    let mut node_1 = Node::new(dir_1);
    let mut node_2 = Node::new(dir_2);
    node_1.add_child_and_update_its_parent(&node_2);

    node_1.value.size += 100;

    
}


