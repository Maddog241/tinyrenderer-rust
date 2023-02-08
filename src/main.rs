mod myimage;
mod model;
mod vertex;

use std::mem::swap;

use cgmath::{Vector3, Point2, Point3, InnerSpace, EuclideanSpace};
use image::{Rgb, DynamicImage};
use model::Model;
use myimage::MyImage;
use vertex::Vertex;

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
const RED: Rgb<u8> = Rgb([255, 0, 0]);
const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
const BLUE: Rgb<u8> = Rgb([0, 0, 255]);

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;


fn line(image: &mut MyImage, p0: Point2<u32>, p1: Point2<u32>, color: Rgb<u8>) {
    let (mut x0, mut x1) = (p0.x as f64, p1.x as f64);
    let (mut y0, mut y1) = (p0.y as f64, p1.y as f64);

    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
        steep = true;
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = (x1 - x0) as i32;

    let derror = 2 * (y1 - y0).abs() as i32;
    let mut error = 0;

    let mut y = y0 as u32;
    for x in (x0 as u32)..(x1 as u32 + 1) {
        if !steep {
            image.set(x, y, color);
        } else {
            image.set(y, x, color);
        }

        error += derror;
        if error > dx {
            if y1 >= y0 {
                y += 1;
            } else {
                y -= 1;
            }
            error -= dx * 2;
        }
    }
}

fn barycentric(p: Point3<f32>, a: Point3<f32>, b: Point3<f32>, c: Point3<f32>) -> (f32, f32, f32) {
    let vec_x = Vector3::new(b.x - a.x, c.x - a.x, a.x - p.x);
    let vec_y = Vector3::new(b.y - a.y, c.y - a.y, a.y - p.y);
    let mut uv1 = vec_x.cross(vec_y); 
    if uv1.z.abs() < 1e-2 {
        // in this case, the triangle is degenerate
        (-1.0, 1.0, 1.0)
    } else {
        uv1 /= uv1.z;
        let (u, v) = (uv1.x, uv1.y);

        (1.0-u-v, u, v)
    }
}


fn texture_2d(texture: &DynamicImage, tex_coord: Point2<f32>) -> Vector3<f32> {
    let (tex_width, tex_height) = (texture.width() as f32, texture.height() as f32);
    let (u, v) = (tex_coord.x, tex_coord.y);
    let (i, j) = ((u * (tex_width - 1.0)) as usize, (v * (tex_height - 1.0)) as usize);

    let bytes = texture.as_bytes();
    let index = (tex_height as usize - 1 - j) * tex_width as usize + i;
    let (r, g, b) = (bytes[3*index], bytes[3*index+1], bytes[3*index+2]);

    Vector3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}

fn triangle(image: &mut MyImage, zbuffer: &mut Vec<Vec<f32>>, texture: &DynamicImage, a: Vertex, b: Vertex, c: Vertex) {
    let (image_width, image_height) = image.dimensions();
    let mut bbox_min = Point2::new(image_width as f32 - 1.0, image_height as f32 - 1.0);
    let mut bbox_max = Point2::new(0.0f32, 0.0f32);

    for point in [a.position, b.position, c.position] {
        bbox_min.x = bbox_min.x.min(point.x);
        bbox_min.y = bbox_min.y.min(point.y);
        bbox_max.x = bbox_max.x.max(point.x);
        bbox_max.y = bbox_max.y.max(point.y);
    }

    let (x_min, x_max) = (bbox_min.x as u32, bbox_max.x as u32);
    let (y_min, y_max) = (bbox_min.y as u32, bbox_max.y as u32);

    for x in x_min..(x_max+1) {
        for y in y_min..(y_max+1) {
            let (u_a, u_b, u_c) = barycentric(Point3::new(x as f32, y as f32, 0.0f32), a.position, b.position, c.position);
            if u_a >= 0.0 && u_b >= 0.0 && u_c >= 0.0 {
                // interpolate z value
                let z = u_a * a.position.z + u_b * b.position.z + u_c * c.position.z;
                if zbuffer[x as usize][y as usize] < z {
                    zbuffer[x as usize][y as usize] = z;
                    // interpolate tex coordinates
                    let tex_coord = u_a * a.tex_coord + (u_b * b.tex_coord).to_vec() + (u_c * c.tex_coord).to_vec();
                    let color = texture_2d(&texture, tex_coord);

                    // lambert's law
                    let normal = u_a * a.normal + u_b * b.normal + u_c * c.normal;
                    let cosine = normal.dot(Vector3::new(0.0, 0.0, 1.0)).max(0.0);

                    let intensity = color * cosine;
                    image.set(x, y, Rgb([(intensity.x * 255.0) as u8, (intensity.y * 255.0) as u8, (intensity.z * 255.0) as u8]));
                } 
            }
        }
    }
}

fn world_to_screen(v: Point3<f32>) -> Point3<f32> {
    Point3::new((v.x+1.0)*(WIDTH-1)as f32/2.0, (v.y+1.0)*(HEIGHT-1)as f32/2.0, v.z+1.0)
}

fn draw_african_head(image: &mut MyImage) {
    let model = Model::new("./obj/african_head.obj");
    let mut zbuffer = vec![vec![f32::MIN; HEIGHT as usize]; WIDTH as usize];

    let tex = image::open("./obj/african_head_diffuse.tga").expect("failed to open the texture file");
    // println!("len tex_bytes: {}", tex_bytes.len());
    // println!("dimension: {:?}", tex_dimension);

    for (i, abc) in model.faces.iter().enumerate() {
        let a = world_to_screen(model.verts[abc[0]]);
        let b = world_to_screen(model.verts[abc[1]]);
        let c = world_to_screen(model.verts[abc[2]]);
        
        let tex_a = Point2::new(model.texcoords[2*model.texcoords_indices[i][0]], model.texcoords[2*model.texcoords_indices[i][0]+1]);
        let tex_b = Point2::new(model.texcoords[2*model.texcoords_indices[i][1]], model.texcoords[2*model.texcoords_indices[i][1]+1]);
        let tex_c = Point2::new(model.texcoords[2*model.texcoords_indices[i][2]], model.texcoords[2*model.texcoords_indices[i][2]+1]);

        let normal_a = model.normals[model.normal_indices[i][0]];
        let normal_b = model.normals[model.normal_indices[i][1]];
        let normal_c = model.normals[model.normal_indices[i][2]];

        let v0 = Vertex::new(a, tex_a, normal_a);
        let v1 = Vertex::new(b, tex_b, normal_b);
        let v2 = Vertex::new(c, tex_c, normal_c);

        triangle(image, &mut zbuffer, &tex, v0, v1, v2);
    }
}

fn main() {
    let mut image = MyImage::new(WIDTH, HEIGHT);

    draw_african_head(&mut image);

    image.write_img("output.png");
}
