/// The Acceptor trait defines the interface all the Acceptor-Plugins should
/// follow
pub trait Acceptor {
    /// The Start of the Acceptor plugin itself
    fn run();
}
