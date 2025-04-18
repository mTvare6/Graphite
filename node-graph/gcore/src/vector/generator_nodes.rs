use crate::Ctx;
use crate::vector::{HandleId, VectorData, VectorDataTable};
use bezier_rs::Subpath;
use glam::DVec2;

use super::misc::AsU64;

trait CornerRadius {
	fn generate(self, size: DVec2, clamped: bool) -> VectorDataTable;
}
impl CornerRadius for f64 {
	fn generate(self, size: DVec2, clamped: bool) -> VectorDataTable {
		let clamped_radius = if clamped { self.clamp(0., size.x.min(size.y).max(0.) / 2.) } else { self };
		VectorDataTable::new(VectorData::from_subpath(Subpath::new_rounded_rect(size / -2., size / 2., [clamped_radius; 4])))
	}
}
impl CornerRadius for [f64; 4] {
	fn generate(self, size: DVec2, clamped: bool) -> VectorDataTable {
		let clamped_radius = if clamped {
			// Algorithm follows the CSS spec: <https://drafts.csswg.org/css-backgrounds/#corner-overlap>

			let mut scale_factor: f64 = 1.;
			for i in 0..4 {
				let side_length = if i % 2 == 0 { size.x } else { size.y };
				let adjacent_corner_radius_sum = self[i] + self[(i + 1) % 4];
				if side_length < adjacent_corner_radius_sum {
					scale_factor = scale_factor.min(side_length / adjacent_corner_radius_sum);
				}
			}
			self.map(|x| x * scale_factor)
		} else {
			self
		};
		VectorDataTable::new(VectorData::from_subpath(Subpath::new_rounded_rect(size / -2., size / 2., clamped_radius)))
	}
}

#[node_macro::node(category("Vector: Shape"))]
fn circle(_: impl Ctx, _primary: (), #[default(50.)] radius: f64) -> VectorDataTable {
	VectorDataTable::new(VectorData::from_subpath(Subpath::new_ellipse(DVec2::splat(-radius), DVec2::splat(radius))))
}

#[node_macro::node(category("Vector: Shape"))]
fn ellipse(_: impl Ctx, _primary: (), #[default(50)] radius_x: f64, #[default(25)] radius_y: f64) -> VectorDataTable {
	let radius = DVec2::new(radius_x, radius_y);
	let corner1 = -radius;
	let corner2 = radius;

	let mut ellipse = VectorData::from_subpath(Subpath::new_ellipse(corner1, corner2));

	let len = ellipse.segment_domain.ids().len();
	for i in 0..len {
		ellipse
			.colinear_manipulators
			.push([HandleId::end(ellipse.segment_domain.ids()[i]), HandleId::primary(ellipse.segment_domain.ids()[(i + 1) % len])]);
	}

	VectorDataTable::new(ellipse)
}

#[node_macro::node(category("Vector: Shape"), properties("rectangle_properties"))]
fn rectangle<T: CornerRadius>(
	_: impl Ctx,
	_primary: (),
	#[default(100)] width: f64,
	#[default(100)] height: f64,
	_individual_corner_radii: bool, // TODO: Move this to the bottom once we have a migration capability
	#[implementations(f64, [f64; 4])] corner_radius: T,
	#[default(true)] clamped: bool,
) -> VectorDataTable {
	corner_radius.generate(DVec2::new(width, height), clamped)
}

#[node_macro::node(category("Vector: Shape"))]
fn regular_polygon<T: AsU64>(
	_: impl Ctx,
	_primary: (),
	#[default(6)]
	#[min(3.)]
	#[implementations(u32, u64, f64)]
	sides: T,
	#[default(50)] radius: f64,
) -> VectorDataTable {
	let points = sides.as_u64();
	let radius: f64 = radius * 2.;
	VectorDataTable::new(VectorData::from_subpath(Subpath::new_regular_polygon(DVec2::splat(-radius), points, radius)))
}

#[node_macro::node(category("Vector: Shape"))]
fn star<T: AsU64>(
	_: impl Ctx,
	_primary: (),
	#[default(5)]
	#[min(2.)]
	#[implementations(u32, u64, f64)]
	sides: T,
	#[default(50)] radius: f64,
	#[default(25)] inner_radius: f64,
) -> VectorDataTable {
	let points = sides.as_u64();
	let diameter: f64 = radius * 2.;
	let inner_diameter = inner_radius * 2.;

	VectorDataTable::new(VectorData::from_subpath(Subpath::new_star_polygon(DVec2::splat(-diameter), points, diameter, inner_diameter)))
}

#[node_macro::node(category("Vector: Shape"))]
fn line(_: impl Ctx, _primary: (), #[default((0., -50.))] start: DVec2, #[default((0., 50.))] end: DVec2) -> VectorDataTable {
	VectorDataTable::new(VectorData::from_subpath(Subpath::new_line(start, end)))
}
