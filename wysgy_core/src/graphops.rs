use crate::converter::Converter;
use crate::errors::WysgyResult;
use crate::fileops::FileOps;
use crate::files::{CONFIG_JSON_CONTENTS, GV_TEMPLATE_CONTENTS};
use crate::node::Node;
use crate::rel::Rel;
use crate::wysgy::Wysgy;
use glob::glob;
use prettytable::Table;
use serde_json::{json, Value};
use std::fs;
use std::io::{BufRead, ErrorKind};
use std::path::PathBuf;
use std::process::{Child, Command};

// graph operations
//  This will use the fileops trait to save/sync changes
//  add/delete/update node  --  NODE
//  add/delete/update rel   --  REL
//  filter nodes            --  Vec<NODE>
//  filter rels             --  Vec<REL>

trait GraphOps
where
    Self: std::marker::Sized,
{
    /// Filters nodes based on key-value
    fn filter_nodes(&self, kv: Option<Value>) -> WysgyResult<Vec<Node>>;
    /// Filters rels based on key-value
    fn filter_rels(&self, kv: Option<Value>) -> WysgyResult<Vec<Rel>>;
    /// Adds a new node from the given string and returns a Node if successful
    fn add_node(&self, s: String) -> WysgyResult<Node>;
    /// Deletes the node with given id
    fn del_node(&self, id: String) -> WysgyResult<()>;
    /// Updates the node with given id with the new Json key-value pairs
    fn edit_node(&self, id: String, s: String) -> WysgyResult<Node>;
    /// Add an edge between the given nodes (as per id) and also store the edge data given in the Json
    fn add_rel(&self, s: String, src_id: String, dst_id: String) -> WysgyResult<Rel>;
    /// Delete the edge with the given id
    fn del_rel(&self, id: String) -> WysgyResult<()>;
    /// Update the edge with the given id with new Json key-value pairs
    fn edit_rel(&self, id: String, s: String) -> WysgyResult<Rel>;
}

impl GraphOps for Wysgy {
    fn filter_nodes(&self, kv: Option<Value>) -> WysgyResult<Vec<Node>> {
        match &kv {
            None => self.get_all_nodes(),
            Some(t) => Ok(self
                .get_all_nodes()
                .unwrap()
                .into_iter()
                .filter(|e| {
                    // here we match given key-value to this node's key-value
                    t.as_object()
                        .unwrap()
                        .iter()
                        .all(|(k, v)| e.kv[k] == v.clone())
                })
                .collect::<Vec<Node>>()),
        }
    }

    fn filter_rels(&self, kv: Option<Value>) -> WysgyResult<Vec<Rel>> {
        match &kv {
            None => self.get_all_rels(),
            Some(t) => Ok(self
                .get_all_rels()
                .unwrap()
                .into_iter()
                .filter(|e| {
                    // here we match given key-value to this node's key-value
                    t.as_object()
                        .unwrap()
                        .iter()
                        .all(|(k, v)| e.kv[k] == v.clone())
                })
                .collect::<Vec<Rel>>()),
        }
    }

    fn add_node(&self, s: String) -> WysgyResult<Node> {
        let n = Converter::kv_to_json(&s, "\n");
        FileOps::add_node(self)
    }

    fn del_node(&self, id: String) -> WysgyResult<()> {
        FileOps::del_node(id);
    }

    fn edit_node(&self, id: String, s: String) -> WysgyResult<()> {
        FileOps::save_node(id, &Converter::kv_to_json(&s, "\n"));
    }

    fn add_rel(&self, s: String, src_id: String, dst_id: String) {
        let r = Converter::kv_to_json(&s, "\n");
        FileOps::add_rel(r, src_id, dst_id);
    }

    fn del_rel(&self, id: String) {
        FileOps::del_rel(id);
    }

    fn edit_rel(&self, id: String, s: String) {
        FileOps::save_rel(id, Converter::kv_to_json(&s, "\n"));
    }
}
