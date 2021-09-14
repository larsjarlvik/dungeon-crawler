use super::{
    interpolation::{Interpolate, Interpolation},
    node,
};
use cgmath::*;
use std::cmp::Ordering;

struct NodesKeyFrame(
    Vec<(usize, Vector3<f32>)>,
    Vec<(usize, Quaternion<f32>)>,
    Vec<(usize, Vector3<f32>)>,
);

#[derive(Clone)]
struct Sampler<T> {
    interpolation: Interpolation,
    times: Vec<f32>,
    values: Vec<T>,
}

impl<T: Interpolate> Sampler<T> {
    fn sample(&self, t: f32) -> Option<T> {
        let index = {
            let mut index = None;
            for i in 0..(self.times.len() - 1) {
                let previous = self.times[i];
                let next = self.times[i + 1];
                if t >= previous && t < next {
                    index = Some(i);
                    break;
                }
            }
            index
        };

        index.map(|i| {
            let previous_time = self.times[i];
            let next_time = self.times[i + 1];
            let delta = next_time - previous_time;
            let from_start = t - previous_time;
            let factor = from_start / delta;

            match self.interpolation {
                Interpolation::Step => self.values[i],
                Interpolation::Linear => self.values[i].linear(self.values[i + 1], factor),
                Interpolation::CubicSpline => {
                    let previous_values = [self.values[i * 3], self.values[i * 3 + 1], self.values[i * 3 + 2]];
                    let next_values = [self.values[i * 3 + 3], self.values[i * 3 + 4], self.values[i * 3 + 5]];
                    Interpolate::cubic_spline(previous_values, previous_time, next_values, next_time, factor)
                }
            }
        })
    }
}

#[derive(Clone)]
struct Channel<T> {
    sampler: Sampler<T>,
    node_index: usize,
}

impl<T: Interpolate> Channel<T> {
    fn sample(&self, t: f32) -> Option<(usize, T)>
    where
        T: Copy,
    {
        self.sampler.sample(t).map(|s| (self.node_index, s))
    }
}

#[derive(Clone)]
pub struct Animation {
    translation_channels: Vec<Channel<Vector3<f32>>>,
    rotation_channels: Vec<Channel<Quaternion<f32>>>,
    scale_channels: Vec<Channel<Vector3<f32>>>,
    pub total_time: f32,
}

impl Animation {
    pub fn new(animation: gltf::Animation, buffers: &Vec<gltf::buffer::Data>) -> Self {
        let translation_channels = map_translation_channels(animation.channels(), buffers);
        let rotation_channels = map_rotation_channels(animation.channels(), buffers);
        let scale_channels = map_scale_channels(animation.channels(), buffers);

        let max_translation_time = translation_channels
            .iter()
            .map(|c| c.sampler.times.last().unwrap_or(&0.0).clone())
            .max_by(|c0, c1| c0.partial_cmp(&c1).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0);
        let max_rotation_time = rotation_channels
            .iter()
            .map(|c| c.sampler.times.last().unwrap_or(&0.0).clone())
            .max_by(|c0, c1| c0.partial_cmp(&c1).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0);
        let max_scale_time = scale_channels
            .iter()
            .map(|c| c.sampler.times.last().unwrap_or(&0.0).clone())
            .max_by(|c0, c1| c0.partial_cmp(&c1).unwrap_or(Ordering::Equal))
            .unwrap_or(0.0);

        let total_time = *[max_translation_time, max_rotation_time, max_scale_time]
            .iter()
            .max_by(|c0, c1| c0.partial_cmp(&c1).unwrap_or(Ordering::Equal))
            .unwrap_or(&0.0);

        Self {
            translation_channels,
            rotation_channels,
            scale_channels,
            total_time,
        }
    }

    pub fn animate_nodes(&self, nodes: &mut Vec<node::Node>, time: f32) -> bool {
        let NodesKeyFrame(translations, rotations, scale) = self.sample(time);

        translations.iter().for_each(|(node_index, translation)| {
            nodes[*node_index].set_translation(*translation);
        });
        rotations.iter().for_each(|(node_index, rotation)| {
            nodes[*node_index].set_rotation(*rotation);
        });
        scale.iter().for_each(|(node_index, scale)| {
            nodes[*node_index].set_scale(*scale);
        });

        !translations.is_empty() || !rotations.is_empty() || !scale.is_empty()
    }

    fn sample(&self, t: f32) -> NodesKeyFrame {
        NodesKeyFrame(
            self.translation_channels.iter().filter_map(|tc| tc.sample(t)).collect::<Vec<_>>(),
            self.rotation_channels.iter().filter_map(|tc| tc.sample(t)).collect::<Vec<_>>(),
            self.scale_channels.iter().filter_map(|tc| tc.sample(t)).collect::<Vec<_>>(),
        )
    }
}

fn map_translation_channels(gltf_channels: gltf::animation::iter::Channels, data: &[gltf::buffer::Data]) -> Vec<Channel<Vector3<f32>>> {
    gltf_channels
        .filter(|c| c.target().property() == gltf::animation::Property::Translation)
        .filter_map(|c| map_translation_channel(&c, data))
        .collect::<Vec<_>>()
}

fn map_translation_channel(gltf_channel: &gltf::animation::Channel, data: &[gltf::buffer::Data]) -> Option<Channel<Vector3<f32>>> {
    if let gltf::animation::Property::Translation = gltf_channel.target().property() {
        map_interpolation(gltf_channel.sampler().interpolation()).map(|i| {
            let reader = gltf_channel.reader(|buffer| Some(&data[buffer.index()]));
            let times = reader.read_inputs().map_or(vec![], |times| times.collect());
            let output = reader.read_outputs().map_or(vec![], |outputs| match outputs {
                gltf::animation::util::ReadOutputs::Translations(translations) => translations.map(Vector3::from).collect(),
                _ => vec![],
            });

            Channel {
                sampler: Sampler {
                    interpolation: i,
                    times,
                    values: output,
                },
                node_index: gltf_channel.target().node().index(),
            }
        })
    } else {
        None
    }
}

fn map_rotation_channels(gltf_channels: gltf::animation::iter::Channels, data: &[gltf::buffer::Data]) -> Vec<Channel<Quaternion<f32>>> {
    gltf_channels
        .filter(|c| c.target().property() == gltf::animation::Property::Rotation)
        .filter_map(|c| map_rotation_channel(&c, data))
        .collect::<Vec<_>>()
}

fn map_rotation_channel(gltf_channel: &gltf::animation::Channel, data: &[gltf::buffer::Data]) -> Option<Channel<Quaternion<f32>>> {
    if let gltf::animation::Property::Rotation = gltf_channel.target().property() {
        map_interpolation(gltf_channel.sampler().interpolation()).map(|interpolation| {
            let reader = gltf_channel.reader(|buffer| Some(&data[buffer.index()]));
            let times = reader.read_inputs().map_or(vec![], |times| times.collect());
            let output = reader.read_outputs().map_or(vec![], |outputs| match outputs {
                gltf::animation::util::ReadOutputs::Rotations(scales) => {
                    scales.into_f32().map(|r| Quaternion::new(r[3], r[0], r[1], r[2])).collect()
                }
                _ => vec![],
            });

            Channel {
                sampler: Sampler {
                    interpolation,
                    times,
                    values: output,
                },
                node_index: gltf_channel.target().node().index(),
            }
        })
    } else {
        None
    }
}

fn map_scale_channels(gltf_channels: gltf::animation::iter::Channels, data: &[gltf::buffer::Data]) -> Vec<Channel<Vector3<f32>>> {
    gltf_channels
        .filter(|c| c.target().property() == gltf::animation::Property::Scale)
        .filter_map(|c| map_scale_channel(&c, data))
        .collect::<Vec<_>>()
}

fn map_scale_channel(gltf_channel: &gltf::animation::Channel, data: &[gltf::buffer::Data]) -> Option<Channel<Vector3<f32>>> {
    if let gltf::animation::Property::Scale = gltf_channel.target().property() {
        map_interpolation(gltf_channel.sampler().interpolation()).map(|i| {
            let reader = gltf_channel.reader(|buffer| Some(&data[buffer.index()]));
            let times = reader.read_inputs().map_or(vec![], |times| times.collect());
            let output = reader.read_outputs().map_or(vec![], |outputs| match outputs {
                gltf::animation::util::ReadOutputs::Scales(scales) => scales.map(Vector3::from).collect(),
                _ => vec![],
            });

            Channel {
                sampler: Sampler {
                    interpolation: i,
                    times,
                    values: output,
                },
                node_index: gltf_channel.target().node().index(),
            }
        })
    } else {
        None
    }
}

fn map_interpolation(gltf_interpolation: gltf::animation::Interpolation) -> Option<Interpolation> {
    match gltf_interpolation {
        gltf::animation::Interpolation::Linear => Some(Interpolation::Linear),
        gltf::animation::Interpolation::Step => Some(Interpolation::Step),
        gltf::animation::Interpolation::CubicSpline => Some(Interpolation::CubicSpline),
    }
}
