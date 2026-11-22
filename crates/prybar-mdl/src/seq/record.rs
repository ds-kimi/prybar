use super::{layers, lists};
use crate::ir::Sequence;
use crate::read::{name_at, offset, Source, Words};
use crate::Result;

/// `mstudioseqdesc_t` size, same from v44 to v49.
pub const STRIDE: usize = 212;

pub fn sequence(r: &Source, at: usize, bone_count: usize, version: i32) -> Result<Sequence> {
    let w = Words::read(r, at, STRIDE / 4)?;
    let group_size = [w.i(17), w.i(18)];
    let mut s = Sequence {
        name: name_at(r, at, w.i(1))?,
        activity_name: name_at(r, at, w.i(2))?,
        flags: w.i(3),
        activity: w.i(4),
        activity_weight: w.i(5),
        events: layers::events(r, at, w.i(6), w.i(7))?,
        bb_min: w.v3(8),
        bb_max: w.v3(11),
        group_size,
        anims: lists::anim_indices(r, at, group_size, w.i(15))?,
        auto_layers: layers::auto_layers(r, at, w.i(37), w.i(38))?,
        bone_weights: lists::floats(r, at, bone_count, w.i(39))?,
        pose_keys: lists::pose_keys(r, at, group_size, w.i(40))?,
        ik_locks: layers::ik_locks(r, at, w.i(41), w.i(42))?,
        key_values: key_values(r, at, w.i(43), w.i(44))?,
        ..Sequence::default()
    };
    blending(&w, &mut s);
    // Activity modifiers only exist from v49; earlier files keep unused ints there.
    if version >= 49 {
        s.activity_modifiers = lists::activity_modifiers(r, at, (w.i(47), w.i(46)), version)?;
    }
    Ok(s)
}

fn blending(w: &Words, s: &mut Sequence) {
    s.param_index = [w.i(19), w.i(20)];
    s.param_start = [w.f(21), w.f(22)];
    s.param_end = [w.f(23), w.f(24)];
    s.param_parent = w.i(25);
    s.fade_in = w.f(26);
    s.fade_out = w.f(27);
    s.entry_node = w.i(28);
    s.exit_node = w.i(29);
    s.node_flags = w.i(30);
    s.entry_phase = w.f(31);
    s.exit_phase = w.f(32);
    s.last_frame = w.f(33);
    s.next_seq = w.i(34);
    s.pose = w.i(35);
    s.cycle_pose = w.i(45);
}

fn key_values(r: &Source, at: usize, index: i32, size: i32) -> Result<String> {
    if index == 0 || size <= 0 {
        return Ok(String::new());
    }
    let raw = r.bytes(offset(at, index)?, size as usize)?;
    r.charge(raw.len())?;
    Ok(String::from_utf8_lossy(raw)
        .trim_end_matches('\0')
        .to_owned())
}
