use super::Station::IEventManager;



struct ArrivalHandler{
    
}

impl ArrivalHandler{

}

impl IEventManager for ArrivalHandler{
    fn process_event(
        &mut self,
        event: &crate::Events::Event,
        data: &mut super::StationData::StationData,
    ) {
            
    }
}
