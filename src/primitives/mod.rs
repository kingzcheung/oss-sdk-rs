//! 基础类型模块
//! 提供AWS SDK风格的基础类型

mod bucket;
mod key;
mod region;
mod body;

pub use bucket::Bucket;
pub use key::Key;
pub use region::Region;
pub use body::ByteStream;