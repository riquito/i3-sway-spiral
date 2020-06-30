use log::Level;
use std::fmt;
use swayipc::reply::{self, Node, NodeLayout, NodeType};
use swayipc::{Connection, EventType};

trait HyperNode {
    fn find_focused_node(&self, l: NodeLayout) -> Option<(&Node, NodeLayout)>;
}

impl HyperNode for Node {
    fn find_focused_node(&self, l: NodeLayout) -> Option<(&Node, NodeLayout)> {
        log::debug!("ffn id {:?}, layout {:?}", self.id, self.layout);
        match self.focus.first() {
            Some(x) => self
                .nodes
                .iter()
                .chain(self.floating_nodes.iter())
                .find(|n| n.id == *x)
                .and_then(|n| {
                    n.find_focused_node(match self.layout {
                        NodeLayout::SplitH => NodeLayout::SplitH,
                        NodeLayout::SplitV => NodeLayout::SplitV,
                        _ => NodeLayout::None,
                    })
                }),
            None if self.focused => Some((self, l)),
            None => None,
        }
    }
}

enum LayoutCommand {
    SplitH,
    SplitV,
}

impl fmt::Display for LayoutCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LayoutCommand::SplitH => "splith",
                LayoutCommand::SplitV => "splitv",
            }
        )
    }
}

fn get_next_layout(node: &reply::Node) -> Option<NodeLayout> {
    match node.layout {
        NodeLayout::None | NodeLayout::SplitH | NodeLayout::SplitV
            if node.node_type == NodeType::Con =>
        {
            if node.rect.height > node.rect.width {
                Some(NodeLayout::SplitV)
            } else {
                Some(NodeLayout::SplitH)
            }
        }
        _ => None,
    }
}

fn get_next_layout_command(container: reply::Node) -> Option<LayoutCommand> {
    let maybe_focused_node = container.find_focused_node(NodeLayout::None);

    if let Some((focused_node, parent_layout)) = maybe_focused_node {
        log::info!(
            "Focused node [id={}] [name={:?}]",
            focused_node.id,
            focused_node.name.clone().ok_or_else(|| String::from(""))
        );

        if log::log_enabled!(Level::Debug) {
            log::debug!("{:?}", focused_node);
        }

        if let Some(next_layout) = get_next_layout(focused_node) {
            log::debug!(
                "New layout {:?} {:?} {:?}",
                focused_node.id,
                next_layout,
                focused_node.rect
            );
            if next_layout != parent_layout {
                if next_layout == NodeLayout::SplitH {
                    return Some(LayoutCommand::SplitH);
                } else {
                    return Some(LayoutCommand::SplitV);
                }
            }
        }
    }
    None
}

fn run_command_safe(connection: &mut Connection, command: &str) {
    match connection.run_command(command) {
        Ok(res_cmd) => {
            for outcome in res_cmd {
                if outcome.success {
                    println!("switch layout: {}", command);
                } else {
                    println!("ipc command ERR: {}", command);
                    if let Some(e) = outcome.error {
                        log::error!("Failed to run {}. Error was {:?}", command, e);
                    }
                }
            }
        }
        Err(e) => {
            log::error!("Failed to run {}. Error was {:?}", command, e);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    let connection = Connection::new()?;
    let subs = [EventType::Window];
    let subscriptions = connection.subscribe(&subs)?;

    let mut connection_cmds = Connection::new()?;

    for event in subscriptions {
        match event {
            Err(e) => {
                log::error!("Error reading event {:?}", e);
            }
            Ok(reply::Event::Window(e)) if e.change == reply::WindowChange::Focus => {
                log::info!("Focus event");

                // Reload the tree every time.
                // 1) you can't trust e.container size (the container in a focus
                //    event resulting from a close event can be not resized yet)
                // 2) we need the parent layout to reduce the number of layout commands
                //    (which apparently cost more than get_tree)
                let node = connection_cmds.get_tree().unwrap_or(e.container);

                if let Some(new_layout) = get_next_layout_command(node) {
                    log::info!("Apply layout: {}", new_layout);
                    run_command_safe(&mut connection_cmds, new_layout.to_string().as_ref());
                }
            }
            _ => (),
        }
    }
    Ok(())
}
