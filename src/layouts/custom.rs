use std::cmp::{max, Ordering};

use penrose::core::{
    client::Client,
    data_types::{Region, ResizeAction},
    xconnection::Xid,
};

pub fn spiral_horizontal_split_first(
    clients: &[&Client],
    _: Option<Xid>,
    monitor_region: &Region,
    _: u32,
    ratio: f32,
) -> Vec<ResizeAction> {
    spiral(clients, monitor_region, ratio, true)
}

pub fn spiral_vertical_split_first(
    clients: &[&Client],
    _: Option<Xid>,
    monitor_region: &Region,
    _: u32,
    ratio: f32,
) -> Vec<ResizeAction> {
    spiral(clients, monitor_region, ratio, false)
}

fn spiral(
    clients: &[&Client],
    monitor_region: &Region,
    ratio: f32,
    horizontal_split_first: bool,
) -> Vec<ResizeAction> {
    let n = clients.len();
    let mut resize_actions = vec![];
    let mut available = *monitor_region;
    let mut current;

    for client_with_index in clients.iter().enumerate() {
        let (index, client) = client_with_index;
        match index {
            i if n - i == 1 => resize_actions.push((client.id(), Some(available))),
            i if (i % 2 == 0) == horizontal_split_first => {
                let split = ((available.w as f32) * ratio) as u32;
                (current, available) = available.split_at_width(split).unwrap();
                resize_actions.push((client.id(), Some(current)));
            }
            _ => {
                let split = ((available.h as f32) * ratio) as u32;
                (current, available) = available.split_at_height(split).unwrap();
                resize_actions.push((client.id(), Some(current)));
            }
        }
    }

    resize_actions
}

pub fn split_longsided(
    clients: &[&Client],
    _: Option<Xid>,
    monitor_region: &Region,
    _: u32,
    ratio: f32,
) -> Vec<ResizeAction> {
    let mut resize_actions = vec![];

    for client in clients {
        if resize_actions.is_empty() {
            resize_actions.push((client.id(), Some(*monitor_region)));
        } else {
            resize_actions.sort_by(
                |a: &(u32, Option<Region>), b: &(u32, Option<Region>)| match a.1 {
                    Some(ra) if b.1.is_some() => {
                        let rb = (b.1).unwrap();
                        if max(ra.w, ra.h) > max(rb.w, rb.h) {
                            Ordering::Less
                        } else {
                            Ordering::Greater
                        }
                    }
                    Some(_) => Ordering::Less,
                    None if b.1.is_some() => Ordering::Greater,
                    None => Ordering::Equal,
                },
            );
            let to_split = resize_actions[0];
            let (resized_region, created_region) = split_longest_side(to_split.1.unwrap(), ratio);
            resize_actions[0] = (to_split.0, Some(resized_region));
            resize_actions.push((client.id(), Some(created_region)));
        }
    }

    resize_actions
}

fn split_longest_side(region: Region, ratio: f32) -> (Region, Region) {
    if region.w > region.h {
        let split = ((region.w as f32) * ratio) as u32;
        region.split_at_width(split).unwrap()
    } else {
        let split = ((region.h as f32) * ratio) as u32;
        region.split_at_height(split).unwrap()
    }
}
