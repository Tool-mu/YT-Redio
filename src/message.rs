// 应用消息：Elm 架构

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Band {
    National,  // 国家台
    Province,  // 省市台
    Favorites, // 收藏
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,              // 秒钟滴答
    StationStep(i32),  // 电台
    BandChange(Band),  // 区域
    PowerToggle,       // 电源
    FlipToggle,        // 切换
}
