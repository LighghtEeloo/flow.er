use flow_vessel::{Cube, CubeId, CubeType, CubeMeta, EntityId, Vessel, Router, node_view::{NodeViewCore, Own}};

pub fn flower_view(cube: Cube, meta: CubeMeta, id: CubeId, vessel: &Vessel) ->  Result<String, &'static str> {
    let string = match cube.cube_type {
        // CubeType::Inkblot => {}
        CubeType::NodeView => {
            let core = NodeViewCore::from_router_cube(
                vessel, 
                (meta, id, cube)
            ).ok_or("view_core err")?;
            core.view()
        }
        // CubeType::ClauseTree => {}
        // CubeType::PromisedLand => {}
        // CubeType::FlowView => {}
        // CubeType::AgendaView => {}
        // CubeType::TimeView => {}
        // CubeType::SettingView => {}
        // CubeType::Blank => {}
        _ => format!("**Can't render.**")
    };
    Ok(string)
}

pub fn flower_router_view(vessel: &Vessel) -> Result<String, &'static str> {
    let vec_cube = vessel.glass.show_cubes(Router::Workspace);
    let mut vec_string = Vec::new();
    for (meta, cube_id, cube) in vec_cube {
        let string = flower_view(cube, meta, cube_id, vessel)?;
        vec_string.push(string);
    }
    let display = vec_string.into_iter().enumerate().fold(
        format!("========"), 
        |display, (idx, s)| {
            format!("{}\n\n{}\n\n<<<<<<<< {}", display, s, idx)
        }
    );
    Ok(display)
}

pub trait CubeView {
    fn view(&self) -> String {
        format!("**Not implemented.**")
    }
}

impl CubeView for NodeViewCore {
    fn view(&self) -> String {
        let mut indent = 0;
        let mut display;

        let to_display = |obj: &EntityId, indent: usize| -> String {
            self.entity_map.get(obj).map_or(String::new(), |(own, en)| {
                let indent = " ".repeat(4 * indent);
                format!("{}{}{:?} {}", 
                    indent, 
                    if let Own::No = own { "* " } else { "" },
                    obj, 
                    en.face.clone() 
                )
            })
        };

        if let Some(obj) = self.obj {
            display = format!("{} /", to_display(&obj, indent));
        } else {
            display = format!("<root> /");
        }
        
        indent += 1;
        for id in self.children.iter() {
            display = format!("{}\n{}", display, to_display(id, indent));
        }

        display
    }
}


