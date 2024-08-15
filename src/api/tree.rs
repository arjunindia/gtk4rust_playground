use std::cell::RefCell;
use std::rc::Rc;

use gtk::gdk_pixbuf::Pixbuf;
use gtk::gio::{Cancellable, MemoryInputStream};
use gtk::glib::Bytes;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Image, Label, Orientation};
// Define an enum to represent different types of leaf nodes.
#[derive(Debug)]
enum LeafNode {
    Text(String),
    Heading(String),
    Image(String), // Assuming the string is a URL or path to the image.
}

// Define an enum to represent container nodes and their types.
#[derive(Debug)]
enum ContainerNode {
    Vertical(Vec<Rc<RefCell<Node>>>),
    Horizontal(Vec<Rc<RefCell<Node>>>),
}

// Define an enum to represent all types of nodes (both leaf and container nodes).
#[derive(Debug)]
pub enum Node {
    Leaf(LeafNode),
    Container(ContainerNode),
}

// Implement methods for appending children to container nodes.

impl ContainerNode {
    pub fn children(&self) -> &[Rc<RefCell<Node>>] {
        match self {
            ContainerNode::Vertical(children) | ContainerNode::Horizontal(children) => children,
        }
    }
}

impl Node {
    pub fn append_child(&mut self, child: Rc<RefCell<Node>>) {
        match self {
            Node::Container(ContainerNode::Vertical(children))
            | Node::Container(ContainerNode::Horizontal(children)) => {
                children.push(child);
            }
            _ => panic!("Cannot append children to a leaf node."),
        }
    }
}

// Helper function to create a new leaf node.
pub fn create_leaf_node(content: &str, kind: &str) -> Rc<RefCell<Node>> {
    let leaf_node = match kind {
        "text" => LeafNode::Text(content.to_string()),
        "heading" => LeafNode::Heading(content.to_string()),
        "image" => LeafNode::Image(content.to_string()),
        _ => panic!("Unknown leaf node type."),
    };
    Rc::new(RefCell::new(Node::Leaf(leaf_node)))
}

// Helper function to create a new container node.
pub fn create_container_node(kind: &str) -> Rc<RefCell<Node>> {
    let container_node = match kind {
        "vertical" => ContainerNode::Vertical(Vec::new()),
        "horizontal" => ContainerNode::Horizontal(Vec::new()),
        _ => panic!("Unknown container node type."),
    };
    Rc::new(RefCell::new(Node::Container(container_node)))
}

// Convert the custom node tree to GTK4 widgets.
pub fn create_gtk_widget_from_node(node: Rc<RefCell<Node>>) -> gtk::Widget {
    let node = node.borrow();

    match &*node {
        Node::Leaf(leaf_node) => match leaf_node {
            LeafNode::Text(text) => {
                let label = Label::new(Some(text));
                label.upcast()
            }
            LeafNode::Heading(heading) => {
                let label = Label::new(Some(heading));
                label.set_css_classes(&["heading"]); // Apply CSS class for styling if needed
                label.upcast()
            }
            LeafNode::Image(image_path) => {
                let result = reqwest::blocking::get(image_path).unwrap();
                let bytes = result.bytes().unwrap().to_vec();
                let bytes = Bytes::from(&bytes.to_vec());
                let stream = MemoryInputStream::from_bytes(&bytes);
                let pixbuf = Pixbuf::from_stream(&stream, Cancellable::NONE).unwrap();
                let image = Image::from_pixbuf(Some(&pixbuf));
                image.set_width_request(400);
                image.set_height_request(300);
                image.upcast()
            }
        },
        Node::Container(container_node) => {
            let container = match container_node {
                ContainerNode::Vertical(_) => Box::new(Orientation::Vertical, 0),
                ContainerNode::Horizontal(_) => Box::new(Orientation::Horizontal, 0),
            };

            for child in container_node.children() {
                let child_widget = create_gtk_widget_from_node(child.clone());
                container.append(&child_widget);
            }

            container.upcast()
        }
    }
}

// Example usage.
/*
fn main() {
    // Create the root node which is a vertical container.
    let root = create_container_node("vertical");

    // Create some leaf nodes.
    let text_node = create_leaf_node("Hello, World!", "text");
    let heading_node = create_leaf_node("Chapter 1", "heading");
    let image_node = create_leaf_node("image.jpg", "image");

    // Append leaf nodes to the root container node.
    root.borrow_mut().append_child(text_node);
    root.borrow_mut().append_child(heading_node);
    root.borrow_mut().append_child(image_node);

    // Create a horizontal container and append it to the root node.
    let horizontal_container = create_container_node("horizontal");
    root.borrow_mut().append_child(horizontal_container.clone());

    // Append another leaf node to the horizontal container.
    let another_text_node = create_leaf_node("Additional Info", "text");
    horizontal_container
        .borrow_mut()
        .append_child(another_text_node);

    // Print the structure to verify.
    println!("{:?}", root);
}

*/
