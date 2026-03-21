//! 基础类型模块
//! 提供AWS SDK风格的基础类型

mod body;
mod bucket;
mod key;
mod region;

pub use body::ByteStream;
pub use bucket::Bucket;
pub use key::Key;
pub use region::Region;
