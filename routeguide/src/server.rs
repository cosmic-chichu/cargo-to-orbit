pub mod routeguide {
    tonic::include_proto!("routeguide");
}

use routeguide::route_guide_server::{RouteGuide, RouteGuideServer};
use routeguide::{Feature, Point, Rectangle, RouteNote, RouteSummary};
use tokio::sync::mpsc;


use tonic::{Request, Response, Status};
use tokio_stream::{wrappers::ReceiverStream, StreamExt};

use rand::{Rng, SeedableRng, rngs::StdRng};

use std::time::Instant;

use tonic::transport::Server;

#[derive(Debug)]
struct RouteGuideService {
    features: Vec<Feature>
}

#[tonic::async_trait]
impl RouteGuide for RouteGuideService {
    async fn get_feature(&self, _request: Request<Point>) -> Result<Response<Feature>, Status> {
        /*
        let features = [
            Feature{
                location: Some(Point{
                    latitude: 90,
                    longitude: 123
                }),
                name: "ChiPoint".to_string()
            },
            Feature{
                location: Some(Point{
                    latitude: 12,
                    longitude: -12
                }),
                name: "OtherChiPoint".to_string()
            }
        ];
        */

        /*
        for f in features {
            if f.location.as_ref() == Some(_request.get_ref()) {
                return Ok(Response::new(f.clone()));
            }
        }
        */

        for f in &self.features[..] {
            if f.location.as_ref() == Some(_request.get_ref()) {
                return Ok(Response::new(f.clone()));
            }
        }
        
        Ok(Response::new(Feature::default()))
    }

    type ListFeaturesStream = ReceiverStream<Result<Feature, Status>>;

    async fn list_features(
        &self,
        _request: Request<Rectangle>,
    ) -> Result<Response<Self::ListFeaturesStream>, Status> {
        let (sender, receiver) = mpsc::channel(2);

        /*
        chiq: server gets stuck when there are more than 2 points (buffer size of mpsc) in a rectangle
        for feature in &self.features[..] {
            if in_range(feature.location.as_ref().unwrap(), _request.get_ref()) {
                sender.send(Ok(feature.clone())).await.unwrap_or_default();
            }
        }
        */

        let features = self.features.clone();

        let mut rng = {
            let rng = rand::thread_rng();
            StdRng::from_rng(rng).unwrap()
        };

        tokio::spawn(async move {
            for feature in &features[..] {
                if in_range(feature.location.as_ref().unwrap(), _request.get_ref()) {
                    let millis = rng.gen_range(5..10);
                    tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
                    println!("waited {millis}");
                    sender.send(Ok(feature.clone())).await.unwrap();
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(receiver)))
    }

    async fn record_route(
        &self,
        _request: Request<tonic::Streaming<Point>>,
    ) -> Result<Response<RouteSummary>, Status> {
        let mut stream = _request.into_inner();

        let mut summary = RouteSummary::default();
        let mut last_point = None;
        let now = Instant::now();
    
        while let Some(point) = stream.next().await {
            let point = point?;
            println!("received point {:?}", point);
            summary.point_count += 1;
    
            for feature in &self.features[..] {
                if feature.location.as_ref() == Some(&point) {
                    summary.feature_count += 1;
                }
            }
    
            if let Some(ref last_point) = last_point {
                summary.distance += calc_distance(last_point, &point);
            }
    
            last_point = Some(point);
        }
    
        summary.elapsed_time = now.elapsed().as_secs() as i32;
    
        Ok(Response::new(summary))
    }


    // type RouteChatStream = Pin<Box<dyn Stream<Item = Result<RouteNote, Status>> + Send  + 'static>>;
    type RouteChatStream = ReceiverStream<Result<RouteNote, Status>>;

    async fn route_chat(
        &self,
        _request: Request<tonic::Streaming<RouteNote>>,
    ) -> Result<Response<Self::RouteChatStream>, Status> {
        // converting request in stream
        let mut streamer = _request.into_inner();
        // creating queue
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            // listening on request stream
            while let Some(note) = streamer.message().await.unwrap() {
                tx.send(Ok(note)).await.unwrap();
            }
        });
        // returning stream as receiver
        Ok(Response::new(rx.into()))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let feature_list = vec![
        Feature{
            location: Some(Point{
                latitude: 90,
                longitude: 123
            }),
            name: "ChiPoint".to_string()
        },
        Feature{
            location: Some(Point{
                latitude: 12,
                longitude: -12
            }),
            name: "OtherChiPoint".to_string()
        },
        Feature{
            location: Some(Point{
                latitude: 10,
                longitude: -10
            }),
            name: "Point3".to_string()
        },
        Feature{
            location: Some(Point{
                latitude: -10,
                longitude: 10
            }),
            name: "Point4".to_string()
        }
    ];

    let route_guide = RouteGuideService {
        features: feature_list
    };

    let svc: RouteGuideServer<RouteGuideService> = RouteGuideServer::new(route_guide);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

// impl Eq for Point {}

fn in_range(point: &Point, rect: &Rectangle) -> bool {
    use std::cmp;

    let lo = rect.lo.as_ref().unwrap();
    let hi = rect.hi.as_ref().unwrap();

    let left = cmp::min(lo.longitude, hi.longitude);
    let right = cmp::max(lo.longitude, hi.longitude);
    let top = cmp::max(lo.latitude, hi.latitude);
    let bottom = cmp::min(lo.latitude, hi.latitude);

    point.longitude >= left
        && point.longitude <= right
        && point.latitude >= bottom
        && point.latitude <= top
}

/// Calculates the distance between two points using the "haversine" formula.
/// This code was taken from http://www.movable-type.co.uk/scripts/latlong.html.
fn calc_distance(p1: &Point, p2: &Point) -> i32 {
    const CORD_FACTOR: f64 = 1e7;
    const R: f64 = 6_371_000.0; // meters

    let lat1 = p1.latitude as f64 / CORD_FACTOR;
    let lat2 = p2.latitude as f64 / CORD_FACTOR;
    let lng1 = p1.longitude as f64 / CORD_FACTOR;
    let lng2 = p2.longitude as f64 / CORD_FACTOR;

    let lat_rad1 = lat1.to_radians();
    let lat_rad2 = lat2.to_radians();

    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();

    let a = (delta_lat / 2f64).sin() * (delta_lat / 2f64).sin()
        + (lat_rad1).cos() * (lat_rad2).cos() * (delta_lng / 2f64).sin() * (delta_lng / 2f64).sin();

    let c = 2f64 * a.sqrt().atan2((1f64 - a).sqrt());

    (R * c) as i32
}