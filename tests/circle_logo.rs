use cucoqu::co2qu::Conic;
use cucoqu::point;
use cucoqu::qu2cu::QuadToCubic;
use cucoqu::Point;
use cucoqu::QuadSpline;
use cucoqu::{Point3, Point3Like};
use glifparser::PointLike;

use std::ops::Div;

fn expected_result() -> Vec<Vec<[cucoqu::Point; 3]>> {
    vec![
        vec![
            [point![47.54661, 49.93977], point![48.772568, 50.0], point![50.0, 50.0]],
            [point![45.099136, 49.75923], point![46.32066, 49.879543], point![47.54661, 49.93977]],
            [
                point![42.663467, 49.458817],
                point![43.877613, 49.638924],
                point![45.099136, 49.75923],
            ],
            [
                point![40.24548, 49.039257],
                point![41.449326, 49.278717],
                point![42.663467, 49.458817],
            ],
            [
                point![37.85099, 48.501556],
                point![39.041634, 48.799797],
                point![40.24548, 49.039257],
            ],
            [
                point![35.485767, 47.847015],
                point![36.660343, 48.203316],
                point![37.85099, 48.501556],
            ],
            [point![33.15551, 47.0772], point![34.31119, 47.49071], point![35.485767, 47.847015]],
            [point![30.86583, 46.193977], point![31.999828, 46.663692], point![33.15551, 47.0772]],
            [
                point![28.622244, 45.199467],
                point![29.73183, 45.724262],
                point![30.86583, 46.193977],
            ],
            [
                point![26.430162, 44.09606],
                point![27.51266, 44.67467],
                point![28.622244, 45.199467],
            ],
            [
                point![24.294863, 42.88643],
                point![25.347666, 43.517452],
                point![26.430162, 44.09606],
            ],
            [point![22.221489, 41.57348], point![23.24206, 42.2554], point![24.294863, 42.88643]],
            [
                point![20.215036, 40.160374],
                point![21.200916, 40.891556],
                point![22.221489, 41.57348],
            ],
            [
                point![18.280336, 38.65052],
                point![19.229155, 39.429195],
                point![20.215036, 40.160374],
            ],
            [
                point![16.422052, 37.047554],
                point![17.331518, 37.871845],
                point![18.280336, 38.65052],
            ],
            [
                point![14.644661, 35.35534],
                point![15.512586, 36.223263],
                point![16.422052, 37.047554],
            ],
            [
                point![12.952443, 33.577946],
                point![13.776735, 34.487415],
                point![14.644661, 35.35534],
            ],
            [
                point![11.349477, 31.719664],
                point![12.128151, 32.668484],
                point![12.952443, 33.577946],
            ],
            [
                point![9.8396225, 29.784964],
                point![10.570802, 30.770845],
                point![11.349477, 31.719664],
            ],
            [point![8.426519, 27.77851], point![9.108443, 28.79908], point![9.8396225, 29.784964]],
            [
                point![7.1135693, 25.705135],
                point![7.744595, 26.757938],
                point![8.426519, 27.77851],
            ],
            [
                point![5.903937, 23.569836],
                point![6.4825435, 24.652332],
                point![7.1135693, 25.705135],
            ],
            [point![4.800535, 21.377754], point![5.32533, 22.487339], point![5.903937, 23.569836]],
            [
                point![3.8060234, 19.13417],
                point![4.2757406, 20.26817],
                point![4.800535, 21.377754],
            ],
            [
                point![2.9227965, 16.844492],
                point![3.3363059, 18.000172],
                point![3.8060234, 19.13417],
            ],
            [
                point![2.152983, 14.514233],
                point![2.5092874, 15.68881],
                point![2.9227965, 16.844492],
            ],
            [
                point![1.4984372, 12.149008],
                point![1.7966787, 13.339655],
                point![2.152983, 14.514233],
            ],
            [
                point![0.9607359, 9.754516],
                point![1.2001958, 10.958362],
                point![1.4984372, 12.149008],
            ],
            [
                point![0.5411745, 7.3365235],
                point![0.72127604, 8.55067],
                point![0.9607359, 9.754516],
            ],
            [
                point![0.24076365, 4.9008565],
                point![0.36107296, 6.122378],
                point![0.5411745, 7.3365235],
            ],
            [
                point![0.060227185, 2.4533834],
                point![0.12045437, 3.679336],
                point![0.24076365, 4.9008565],
            ],
            [point![0.0, 0.0], point![0.0, 1.227431], point![0.060227185, 2.4533834]],
        ],
        vec![
            [point![99.03925, 9.754516], point![99.99999, 4.9245696], point![100.0, 0.0]],
            [point![96.19397, 19.13417], point![98.078514, 14.584461], point![99.03925, 9.754516]],
            [point![91.57347, 27.77851], point![94.30942, 23.68388], point![96.19397, 19.13417]],
            [point![85.35534, 35.35534], point![88.83753, 31.87314], point![91.57347, 27.77851]],
            [point![77.77851, 41.57348], point![81.873146, 38.837532], point![85.35534, 35.35534]],
            [point![69.13417, 46.193977], point![73.683876, 44.30942], point![77.77851, 41.57348]],
            [
                point![59.754513, 49.039257],
                point![64.58446, 48.078526],
                point![69.13417, 46.193977],
            ],
            [point![50.0, 50.0], point![54.92457, 49.999996], point![59.754513, 49.039257]],
        ],
        vec![
            [point![85.35534, -35.35534], point![70.71068, -50.0], point![50.0, -50.0]],
            [point![100.0, 0.0], point![100.0, -20.710678], point![85.35534, -35.35534]],
        ],
        vec![
            [point![14.644661, -35.35534], point![0.0, -20.710678], point![0.0, 0.0]],
            [point![50.0, -50.0], point![29.289322, -50.0], point![14.644661, -35.35534]],
        ],
    ]
}

#[test]
fn it_works() {
    use cucoqu::qu2cu::QuadToCubic;
    use std::ops::Div;
    let weight = 2f32.sqrt().div(2f32);
    let conic = Conic {
        start: point![0.0f32, 0.0f32].into(),
        control: point![0.0f32, 50.0f32].into(),
        end: point![50.0f32, 50.0f32].into(),
        weight,
    };
    let conic2 = Conic {
        start: conic.end,
        control: point![100f32, 50f32].into(),
        end: point![100f32, 0f32].into(),
        weight,
    };
    let conic3 = Conic {
        start: conic2.end,
        control: point![100f32, -50f32].into(),
        end: point![50f32, -50f32].into(),
        weight,
    };
    let conic4 = Conic {
        start: conic3.end,
        control: point![0f32, -50f32].into(),
        end: conic.start,
        weight,
    };
    let tolerances = vec![0.01, 0.1, 0.8, 1.0];
    let qs: Vec<QuadSpline> = [conic, conic2, conic3, conic4]
        .into_iter()
        .zip(tolerances.into_iter())
        .map(|(c, t)| c.as_quads(t))
        .collect();
    let mut s = String::new();
    s.push_str(
        &(&qs[0..3]
            .iter()
            .flatten()
            .map(|q| {
                let mut s = String::with_capacity(16);
                s.push('M');
                s.push_str(&format!(" {},{} ", q[0].x(), q[0].y()));
                s.push('Q');
                for p in &q[1..] {
                    s.push_str(&format!(" {},{} ", p.x(), p.y()));
                }
                s
            })
            .collect::<String>()),
    );
    s.push_str(&format!("M {},{}", qs[3][0][0].x(), qs[3][0][0].y()));
    s.push_str(
        &((&qs[3])
            .into_iter()
            .map(|q| q.quad_to_cubic())
            .map(|q| {
                let mut s = String::with_capacity(16);
                s.push('M');
                s.push_str(&format!(" {},{} ", q[0].x(), q[0].y()));
                s.push('C');
                s.push_str(&(q.into_iter().skip(1).map(|p| format!(" {},{} ", p.x(), p.y())).collect::<String>()));
                s
            })
            .collect::<String>()),
    );
    //eprintln!("{}", &s);
    //eprintln!("{:?}", &qs);
    assert_eq!(&qs, &expected_result());
}
