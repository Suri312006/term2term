use color_eyre::eyre::Result;

pub trait Component {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn handle_events(&mut self, event: Option<Event>) -> Action {

    }

}
