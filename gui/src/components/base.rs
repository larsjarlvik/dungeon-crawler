use stretch::Stretch;

pub trait BaseComponent {
    fn render(&self, stretch: &mut Stretch) -> Result<stretch::node::Node, stretch::Error>;
    fn get_layout(
        &self,
        stretch: &mut Stretch,
        size: stretch::geometry::Size<stretch::number::Number>,
    ) -> Vec<Result<stretch::result::Layout, stretch::Error>>;
}
