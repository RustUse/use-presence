use use_presence::prelude::{
    BusinessPresenceKind, GeoPoint, GeoTarget, LocationRadius, ServiceAreaBusiness,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let business = ServiceAreaBusiness::new("Example Plumbing", BusinessPresenceKind::ServiceArea)?
        .with_service_area_label("Portland metro")?;
    let target = GeoTarget::Radius {
        center: GeoPoint::new(45.5152, -122.6784)?,
        radius: LocationRadius::from_kilometers(30.0)?,
    };

    assert!(business.is_service_area_business());
    assert!(matches!(target, GeoTarget::Radius { .. }));
    Ok(())
}
