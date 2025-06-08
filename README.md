## ws 用法

#### 1.使用 MessageHandler trait 实现类

```
use sdk::{run_with_handler, MessageHandler};
use std::sync::Arc;

struct MyHandler;

#[async_trait::async_trait]
impl MessageHandler for MyHandler {
    async fn handle(&self, msg: &str) {
        println!("处理消息: {}", msg);
        // 在这里做数据处理逻辑，例如存数据库、发通知等
    }
}

#[tokio::main]
async fn main() {
    let config = AppConfig::init(); // 假设你有这个方法
    let handler = Arc::new(MyHandler);
    let _ = run_with_handler(&config, handler).await;
}
```

#### 2.使用 Callback 函数

```
use sdk::{run_with_callback, MessageCallback};
use std::future::Future;
use std::pin::Pin;

fn create_callback() -> MessageCallback {
    Box::new(|msg: &str| {
        let msg = msg.to_string();
        Box::pin(async move {
            println!("Callback 接收到消息: {}", msg);
            // 在这里处理业务逻辑
        }) as Pin<Box<dyn Future<Output = ()> + Send>>
    })
}

#[tokio::main]
async fn main() {
    let config = AppConfig::init(); // 初始化配置
    let callback = create_callback();
    let _ = run_with_callback(&config, callback).await;
}
```
