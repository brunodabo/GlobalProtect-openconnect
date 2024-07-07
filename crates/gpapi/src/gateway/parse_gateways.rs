use roxmltree::Node;

use crate::utils::xml::NodeExt;

use super::{Gateway, PriorityRule};

pub(crate) fn parse_gateways(node: &Node, use_internal: bool) -> Option<Vec<Gateway>> {
  let node_gateways = node.find_child("gateways")?;

  let list_gateway = if use_internal {
    node_gateways.find_child("internal")?.find_child("list")?
  } else {
    node_gateways.find_child("external")?.find_child("list")?
  };

  let gateways = list_gateway
    .children()
    .filter_map(|gateway_item| {
      if !gateway_item.has_tag_name("entry") {
        return None;
      }
      let address = gateway_item.attribute("name").unwrap_or_default().to_string();
      let name = gateway_item.child_text("description").unwrap_or_default().to_string();
      let priority = gateway_item
        .child_text("priority")
        .and_then(|s| s.parse().ok())
        .unwrap_or(u32::MAX);
      let priority_rules = gateway_item
        .find_child("priority-rule")
        .map(|n| {
          n.children()
            .filter_map(|n| {
              if !n.has_tag_name("entry") {
                return None;
              }
              let name = n.attribute("name").unwrap_or_default().to_string();
              let priority: u32 = n
                .child_text("priority")
                .and_then(|s| s.parse().ok())
                .unwrap_or(u32::MAX);

              Some(PriorityRule { name, priority })
            })
            .collect()
        })
        .unwrap_or_default();

      Some(Gateway {
        name,
        address,
        priority,
        priority_rules,
      })
    })
    .collect();

  Some(gateways)
}
