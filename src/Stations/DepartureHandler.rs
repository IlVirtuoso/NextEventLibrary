use super::Station::IEventManager;



struct RoutedDepartureHandler{
    stream: i32
}

impl RoutedDepartureHandler {
    
}

impl IEventManager for RoutedDepartureHandler {
    fn process_event(
        &mut self,
        event: &crate::Events::Event,
        data: &mut super::StationData::StationData,
    ) {
        todo!()
    }
}