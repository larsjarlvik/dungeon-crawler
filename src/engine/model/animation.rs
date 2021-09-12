use std::cmp::Ordering;

use cgmath::*;

enum Interpolation {
    Linear,
    Step,
    CubicSpline,
}

struct Sampler<T> {
    interpolation: Interpolation,
    times: Vec<f32>,
    values: Vec<T>,
}

struct Channel<T> {
    sampler: Sampler<T>,
    node_index: usize,
}

pub struct Animation {
    translation_channels: Vec<Channel<Vector3<f32>>>,
    rotation_channels: Vec<Channel<Quaternion<f32>>>,
    scale_channels: Vec<Channel<Vector3<f32>>>,
    total_time: f32,
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
