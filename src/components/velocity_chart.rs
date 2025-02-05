use crate::utils::calculate_velocity::calculate_velocity;
use leptos::prelude::*;
use leptos_chartistry::*;

#[derive(PartialEq)]
struct CombinedPoints {
    x: f64,
    y: f64,
    y2: f64,
}

struct VelocityPoint {
    x: f64,
    y: f64,
}

impl VelocityPoint {
    fn add(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

fn get_defined_points() -> Vec<VelocityPoint> {
    vec![
        VelocityPoint { x: 0.0, y: 0.0 },
        VelocityPoint { x: 1.0, y: 55.0 },
        VelocityPoint { x: 2.0, y: 92.0 },
        VelocityPoint { x: 3.0, y: 110.0 },
        VelocityPoint { x: 4.0, y: 123.0 },
        VelocityPoint { x: 5.0, y: 134.0 },
        VelocityPoint { x: 6.0, y: 142.0 },
        VelocityPoint { x: 7.0, y: 145.0 },
        VelocityPoint { x: 8.0, y: 147.0 },
        VelocityPoint { x: 9.0, y: 148.0 },
        VelocityPoint { x: 10.0, y: 152.0 },
        VelocityPoint { x: 11.0, y: 155.0 },
        VelocityPoint { x: 12.0, y: 156.0 },
        VelocityPoint { x: 13.0, y: 157.0 },
        VelocityPoint { x: 14.0, y: 153.0 },
        VelocityPoint { x: 15.0, y: 154.0 },
        VelocityPoint { x: 16.0, y: 153.0 },
        VelocityPoint { x: 17.0, y: 150.0 },
        VelocityPoint { x: 18.0, y: 149.0 },
        VelocityPoint { x: 19.0, y: 148.0 },
        VelocityPoint { x: 20.0, y: 146.0 },
        VelocityPoint { x: 21.0, y: 147.0 },
        VelocityPoint { x: 22.0, y: 148.0 },
        VelocityPoint { x: 23.0, y: 148.0 },
        VelocityPoint { x: 24.0, y: 149.0 },
        VelocityPoint { x: 25.0, y: 150.0 },
        VelocityPoint { x: 26.0, y: 150.0 },
        VelocityPoint { x: 27.0, y: 149.0 },
    ]
}

fn get_velocity_points(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
) -> Vec<VelocityPoint> {
    let mut velocity_points = Vec::new();

    //Get properties for performance
    let properties = slider_values.get();
    let iso_nfw_resolved = iso_nfw.get();

    for i in (0..91).map(|x| x as f64 * 0.5) {
        log::info!("{}", i);
        let x: f64 = i as f64;
        let y = calculate_velocity(
            x,
            properties.0,
            properties.1,
            properties.2,
            properties.3,
            iso_nfw_resolved,
        );
        velocity_points.push(VelocityPoint::add(x, y));
    }

    velocity_points
}

#[component]
pub fn VelocityChart(
    slider_values: ReadSignal<(f64, f64, f64, f64)>,
    iso_nfw: ReadSignal<bool>,
) -> impl IntoView {
    let combined_points = Memo::new(move |_| {
        let defined_points = get_defined_points();
        let velocity_points = get_velocity_points(slider_values, iso_nfw);

        velocity_points
            .iter()
            .enumerate()
            .map(|(index, velocity)| {
                // Check if fits into boundary
                let velocity_y;
                if velocity.y > 300.0 {
                    velocity_y = f64::NAN;
                } else {
                    velocity_y = velocity.y;
                }

                // Get defined points
                let defined_y = defined_points.get(index / 2).map_or(f64::NAN, |defined_point| defined_point.y);

                CombinedPoints {
                    x: velocity.x,
                    y: velocity_y,
                    y2: defined_y,
                }
            })
            .collect::<Vec<CombinedPoints>>()
    });

    let series = Series::new(|data: &CombinedPoints| data.x)
        .line(Line::new(|data: &CombinedPoints| data.y2)
            .with_name("Musterwerte NGC3198 (km/s)")
            .with_interpolation(Step::Horizontal)
        )
        .line(Line::new(|data: &CombinedPoints| data.y)
            .with_name("Galaxie (km/s)")
        )
        .with_y_range(0.0, 300.0)
        .with_x_range(0.0, 45.0);

    view! {
        <div class="chart">
            <Chart
                aspect_ratio=AspectRatio::from_env()
                series=series
                data=combined_points
                left=vec![
                    RotatedLabel::end("Geschwindikeit (km/s)").into(),
                    TickLabels::aligned_floats().into(),
                ]
                bottom=vec![
                    TickLabels::aligned_floats().into(),
                    RotatedLabel::end("Radius (kpc)").into(),
                    Legend::middle().into(),
                ]
                inner=[
                    AxisMarker::left_edge().into_inner(),
                    AxisMarker::bottom_edge().into_inner(),
                    XGridLine::default().into_inner(),
                    YGridLine::default().into_inner(),
                    YGuideLine::over_mouse().into_inner(),
                    XGuideLine::over_data().into_inner(),
                ]
                tooltip=Tooltip::right_cursor()
                    .show_x_ticks(true)
            />
        </div>
    }
}
