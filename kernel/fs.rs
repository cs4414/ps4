/* kernel::fs.rs */

use core::*;
use core::str::*;
use core::option::{Some, Option, None}; // Match statement
use core::iter::Iterator;
use core::vec::Vec;
use super::super::platform::*;


pub fn open(node: *tree_node, file: cstr) -> (*tree_node, bool, bool)
{
    if dir.isLeaf() || file == ""
    {
	return (node, dir.isLeaf(), file == "");
    }
    let mut children: uint = (*node).child_count;
    let mut i: uint = 0;
    let mut split = file.before('/');
    while i < children
    {
	if (*node).children[i].name == k
	{
	    return open((*node).children[i], file.remainder('/'));
	} else
	{
	    i += 1;
	}
    }
    return cstr::new();
}

pub fn append(node: *tree_node, file: cstr, content: cstr) -> bool
{
    let (mut f, _, _) = open(node, file);
    if f == cstr::new()
    {
	return false;
    }
    let mut x = 0;
    let mut f_contents = (*f).contents;
    while x < content.len()
    {
	let b = f_contents.push_char(content.char_at(x));
	if !b
	{
	    return false;
	}
    }
    let (*f).contents = f_contents;
    return true;
}

pub fn new(node: *tree_node, dir: cstr, name: cstr) -> bool
{
    let (mut n, _, _) = open(node, file);
    if !n.isLeaf()
    {
	n.insert(name);
    }
    return false;

}
