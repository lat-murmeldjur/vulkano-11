use rand::rngs::ThreadRng;
use rand::Rng;
use std::thread;
use std::time::{Duration, SystemTime};
use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};

use crate::f32_3::{
    angle_360_of, angle_of, angular_difference, average_f32_2, dd_f32_3, dot_product, dstnc_f32_3,
    find_points_normal, gen_f32_3, gen_f32_3_on_point_normal_plane, gen_rthgnl_f32_3, mltply_f32_3,
    nrmlz_f32_3, sbtr_f32_3, vector_length,
};

use crate::shapes::f32_3_dots_collinear;

use crate::u_modular::{
    modular_difference, modular_difference_in_range, modular_offset, modular_offset_in_range,
};

#[derive(BufferContents, Vertex, Debug, Clone, Copy)]
#[repr(C)]
pub struct Position {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}

#[derive(BufferContents, Vertex, Debug)]
#[repr(C)]
pub struct Normal {
    #[format(R32G32B32_SFLOAT)]
    normal: [f32; 3],
}

#[derive(Debug)]
pub struct Magma {
    positions: Vec<Position>,
    normals: Vec<Normal>,
    indices: Vec<u32>,
}

#[derive(Debug)]
pub struct Stone {
    pub positions: Vec<Position>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
}

pub fn wait_for_a_minute() {
    let tsn3 = Duration::from_secs(60);
    // Print text to the console.
    thread::sleep(tsn3);
}

pub fn magma(flow: u32, scale: f32) -> Magma {
    let mut rng = rand::thread_rng();

    let mut lava_flow = Magma {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };

    let mut base = scale;
    let mut cbase = -2.5 * scale;
    for i in 1..=flow {
        lava_flow.positions.push(Position {
            position: gen_f32_3(cbase, base, &mut rng),
        });
        cbase = cbase + 5.0 * base;

        // randomize graph edges
        if i > 1 {
            lava_flow
                .indices
                .push(rng.gen_range(0..lava_flow.positions.len() - 1) as u32);
            lava_flow.indices.push(i - 1);
        }
    }

    println!("flow: {:#?}", lava_flow);

    return lava_flow;
}

pub fn petrify(flow: Magma) -> Stone {
    if flow.positions.len() > 2 {
        return petrify_flow(flow);
    };

    let mut stone = Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };

    let mut rng = rand::thread_rng();
    let points_diff = sbtr_f32_3(flow.positions[1].position, flow.positions[0].position);
    let planes_normal: [f32; 3] = nrmlz_f32_3(points_diff);
    let planes_number = rng.gen_range(16..32);

    let mut points_of_plane: u32 = 3;
    let mut points_range = 15.0;
    let mut points_range_min = 2.0;
    let reference_orthogonal = gen_rthgnl_f32_3(planes_normal, &mut rng);
    let mut pln = 0;

    let planes_points = f32_3_dots_collinear(
        flow.positions[0].position,
        flow.positions[1].position,
        planes_number,
    );

    let mut previous_plane: [u32; 3] = [0, 0, 0]; // plane number, beginning position, ending position

    for planae in planes_points.iter() {
        println!("plane: {:#?}", planae);

        let mut plane = Stone {
            positions: vec![],
            normals: vec![],
            indices: vec![],
        };

        let mut k: usize = 1;
        if previous_plane[2] > 0 {
            k = previous_plane[0] as usize;
        };

        // get random points on plane in range

        for i in 1..=points_of_plane {
            plane.positions.push(Position {
                position: gen_f32_3_on_point_normal_plane(
                    planes_normal,
                    points_range_min,
                    points_range,
                    *planae,
                    &mut rng,
                ),
            });
        }

        // order points on plane by angle

        let mut planes_points_average = [0.0, 0.0, 0.0];
        for i in 0..plane.positions.len() {
            planes_points_average = dd_f32_3(planes_points_average, plane.positions[i].position);
        }

        planes_points_average =
            mltply_f32_3(planes_points_average, 1.0 / (plane.positions.len() as f32));

        let planes_points_center = sbtr_f32_3(planes_points_average, *planae);

        plane.positions.sort_by(|a, b| {
            angle_360_of(
                *planae,
                sbtr_f32_3(a.position, planes_points_center),
                reference_orthogonal,
                planes_normal,
            )
            .total_cmp(&angle_360_of(
                *planae,
                sbtr_f32_3(b.position, planes_points_center),
                reference_orthogonal,
                planes_normal,
            ))
        });

        // add points and normals to stone

        for i in 0..points_of_plane {
            stone.positions.push(Position {
                position: plane.positions[(i as usize)].position,
            });
            let normal =
                find_points_normal(plane.positions[(i as usize)].position, planes_points[k]);
            stone.normals.push(Normal { normal: normal });
        }

        // add indices

        if previous_plane[2] == 0 {
            stone.indices.push(0);
            stone.indices.push(1);
            stone.indices.push(2);
        } else {
            println!(
                "############# {} {}",
                previous_plane[1],
                previous_plane[2] - 1
            );
            println!(
                "############# {} {}",
                previous_plane[2],
                previous_plane[2] + points_of_plane - 1
            );
            find_indices_between_circles(
                //vertex_plane_one: [u32; 2],
                [previous_plane[1], previous_plane[2] - 1],
                //plane_one: [f32; 3],
                planes_points[(previous_plane[0] as usize)],
                //vertex_plane_two: [u32; 2],
                [previous_plane[2], previous_plane[2] + points_of_plane - 1],
                //plane_two: [f32; 3],
                *planae,
                //reference_orthogonal: [f32; 3],
                reference_orthogonal,
                //planes_normal: [f32;3],
                planes_normal,
                //&mut stone: Stone,
                &mut stone,
            );
        };
        if previous_plane[0] == planes_number - 2 {
            stone.indices.push((stone.positions.len() - 3) as u32);
            stone.indices.push((stone.positions.len() - 2) as u32);
            stone.indices.push((stone.positions.len() - 1) as u32);
        }

        // prepare next plane

        previous_plane[0] = pln;
        pln = pln + 1;
        previous_plane[1] = previous_plane[2];
        previous_plane[2] = previous_plane[2] + points_of_plane;

        let points_increase = rng.gen_range(1..3);
        if points_of_plane > 8 {
            points_of_plane = points_of_plane - points_increase;
        } else {
            points_of_plane = points_of_plane + points_increase;
        };

        if previous_plane[0] == planes_number - 2 {
            points_of_plane = 3;
        };

        if previous_plane[0] < planes_number / 2 {
            points_range = points_range + rng.gen_range(0.1..4.0);
            points_range_min = rng.gen_range(points_range_min / 2.0..points_range - 2.0);
        //_min + 2.0);
        } else {
            points_range = points_range - rng.gen_range(0.1..4.0);
            points_range_min = rng.gen_range(points_range_min / 2.0..points_range - 2.0);
            // _min);
        };
    }

    return stone;
}

pub fn petrify_flow(flow: Magma) -> Stone {
    return Stone {
        positions: vec![],
        normals: vec![],
        indices: vec![],
    };
}

pub fn find_indices_between_circles(
    vertex_plane_one: [u32; 2],
    plane_one: [f32; 3],
    vertex_plane_two: [u32; 2],
    plane_two: [f32; 3],
    reference_orthogonal: [f32; 3],
    planes_normal: [f32; 3],
    stone: &mut Stone,
) {
    find_indices_double_circle(
        vertex_plane_one,
        plane_one,
        vertex_plane_two,
        plane_two,
        reference_orthogonal,
        planes_normal,
        stone,
    );
}

pub fn find_indices_double_circle(
    single_vertex_plane: [u32; 2],
    single_plane_point: [f32; 3],
    double_vertex_plane: [u32; 2],
    double_plane_point: [f32; 3],
    reference_orthogonal: [f32; 3],
    planes_normal: [f32; 3],
    stone: &mut Stone,
) {
    let mut index_set = false;
    let mut index_double_saved = 0;
    let mut index_single_saved = 0;

    let points_of_single_plane = single_vertex_plane[1] - single_vertex_plane[0] + 1;
    println!("Single plane contained {} points", points_of_single_plane);

    let points_of_double_plane = double_vertex_plane[1] - double_vertex_plane[0] + 1;
    println!("Double plane contains {} points", points_of_double_plane);

    let mut single_planes_points_average = [0.0, 0.0, 0.0];
    for i in single_vertex_plane[0]..=single_vertex_plane[1] {
        single_planes_points_average = dd_f32_3(
            single_planes_points_average,
            stone.positions[i as usize].position,
        );
    }

    single_planes_points_average = mltply_f32_3(
        single_planes_points_average,
        1.0 / (points_of_single_plane as f32),
    );

    let single_planes_points_center = sbtr_f32_3(single_planes_points_average, single_plane_point);

    let mut double_planes_points_average = [0.0, 0.0, 0.0];
    for i in double_vertex_plane[0]..=double_vertex_plane[1] {
        double_planes_points_average = dd_f32_3(
            double_planes_points_average,
            stone.positions[i as usize].position,
        );
    }

    double_planes_points_average = mltply_f32_3(
        double_planes_points_average,
        1.0 / (points_of_double_plane as f32),
    );

    let double_planes_points_center = sbtr_f32_3(double_planes_points_average, double_plane_point);

    let mut first_single_index = 0;
    let mut first_double_index = 0;

    for i in double_vertex_plane[0]..=double_vertex_plane[1] + 1 {
        // FULL CIRCLE MISSING
        let mut triangle_counter = 0;
        let mut a_min = f32::MAX;
        let mut a_min_dex = 0;
        let mut k = i + 1;
        if k < double_vertex_plane[1] + 2 {
            if k > double_vertex_plane[1] {
                k = double_vertex_plane[0];
            }

            let po1 = sbtr_f32_3(
                stone.positions[(i as usize)].position,
                double_planes_points_center,
            );
            let po2 = sbtr_f32_3(
                stone.positions[(k as usize)].position,
                double_planes_points_center,
            );

            let center = double_plane_point;

            let nrml_point_1 = dd_f32_3(find_points_normal(center, po1), center);
            let nrml_point_2 = dd_f32_3(find_points_normal(center, po2), center);

            let average_point = average_f32_2(vec![nrml_point_1, nrml_point_2]);

            for j in single_vertex_plane[0]..=single_vertex_plane[1] {
                // println!(
                //     "Finding distance between point number {}, {} and {}",
                //     i, k, j,
                // );
                let dist = angular_difference(
                    angle_360_of(
                        double_plane_point,
                        average_point,
                        reference_orthogonal,
                        planes_normal,
                    ),
                    angle_360_of(
                        single_plane_point,
                        sbtr_f32_3(
                            stone.positions[(j as usize)].position,
                            single_planes_points_center,
                        ),
                        reference_orthogonal,
                        planes_normal,
                    ),
                );
                if dist < a_min {
                    a_min = dist;
                    a_min_dex = j;
                }
            }
            stone.indices.push(i);
            stone.indices.push(k);
            stone.indices.push(a_min_dex);
            triangle_counter = triangle_counter + 1;
        } else {
            a_min_dex = first_single_index;
        }

        if index_set {
            let mut running_index = index_single_saved;

            for l in 1..=modular_difference_in_range(
                index_single_saved,
                a_min_dex,
                single_vertex_plane[0],
                single_vertex_plane[1],
            ) {
                stone.indices.push(index_double_saved);
                stone.indices.push(running_index);
                stone.indices.push(modular_offset_in_range(
                    running_index,
                    1,
                    single_vertex_plane[0],
                    single_vertex_plane[1],
                ));

                triangle_counter = triangle_counter + 1;

                running_index = modular_offset_in_range(
                    running_index,
                    1,
                    single_vertex_plane[0],
                    single_vertex_plane[1],
                );
            }
        } else {
            first_single_index = a_min_dex;
            first_double_index = i;
        }

        index_set = true;
        index_double_saved = k;
        index_single_saved = a_min_dex;
        println!("Added {} triangles", triangle_counter);
    }
}
