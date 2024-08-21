// 严格来说，tracing 并不是一个日志库，而是一个分布式跟踪的 SDK，用于采集监控数据的。
// 随着微服务的流行，现在一个产品有多个系统组成是非常常见的，这种情况下，一条用户请求可能会横跨几个甚至几十个服务。
// 此时再用传统的日志方式去跟踪这条用户请求就变得较为困难，这就是分布式追踪在现代化监控系统中这么炽手可热的原因。
// 关于分布式追踪，在后面的监控章节进行详细介绍，
// 大家只要知道：分布式追踪的核心就是在请求的开始生成一个 trace_id，
// 然后将该 trace_id 一直往后透穿，请求经过的每个服务都会使用该 trace_id 记录相关信息，
// 最终将整个请求形成一个完整的链路予以记录下来。
// 那么后面当要查询这次请求的相关信息时，只要使用 trace_id 就可以获取整个请求链路的所有信息了，
// 非常简单好用。看到这里，相信大家也明白为什么这个库的名称叫 tracing 了吧？
// 至于为何把它归到日志库的范畴呢？因为 tracing 支持 log 门面库的 API，因此，
// 它既可以作为分布式追踪的 SDK 来使用，也可以作为日志库来使用。

use log;
use log::{debug, error, info, warn};
use serde::Deserialize;
use serde_json;
use tracing::{event, instrument, span, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    // 只有注册 subscriber 后， 才能在控制台上看到日志输出
    tracing_subscriber::registry().with(fmt::layer()).init();

    // 调用 `log` 包的 `info!`
    log::info!("Hello world");

    let foo = 42;
    // 调用 `tracing` 包的 `info!`
    tracing::info!(foo, "Hello from tracing");

    error!("this is printed by default");

    // 除了分布式追踪，在异步编程中使用传统的日志也是存在一些问题的，
    // 最大的挑战就在于异步任务的执行没有确定的顺序，那么输出的日志也将没有确定的顺序并混在一起，
    // 无法按照我们想要的逻辑顺序串联起来。
    // 归根到底，在于日志只能针对某个时间点进行记录，缺乏上下文信息，
    // 而线程间的执行顺序又是不确定的，因此日志就有些无能为力。
    // 而 tracing 为了解决这个问题，引入了 span 的概念( 这个概念也来自于分布式追踪 )，
    // 一个 span 代表了一个时间段，拥有开始和结束时间，在此期间的所有类型数据、结构化数据、文本数据都可以记录其中。
    // 大家发现了吗？ span 是可以拥有上下文信息的，这样就能帮我们把信息按照所需的逻辑性串联起来了。

    // tracing 中最重要的三个概念是 Span、Event 和 Collector，下面我们来一一简单介绍下。
    println!("----------------------------span---------------------------------");
    {
        // Span
        // 相比起日志只能记录在某个时间点发生的事件，span 最大的意义就在于它可以记录一个过程，
        // 也就是在某一段时间内发生的事件流。既然是记录时间段，那自然有开始和结束:
        let span = span!(Level::ERROR, "my_test_span");

        // `enter` 返回一个 RAII ，当其被 drop 时，将自动结束该 span
        let _enter = span.enter();
        // 这里开始进入 `my_span` 的上下文
        // 下面执行一些任务，并记录一些信息到 `my_test_span` 中
        // this event occurs inside the span.
        info!("i'm in the span!");
    } // 这里 enter 将被 drop，`my_test_span` 也随之结束
      // this event is not inside the span.
    info!("i'm outside the span!");

    println!("----------------------------event---------------------------------");
    // Event 事件
    // Event 代表了某个时间点发生的事件，这方面它跟日志类似，但是不同的是，Event 还可以产生在 span 的上下文中。
    {
        // 在 span 的上下文之外记录一次 event 事件
        event!(Level::INFO, "something happened");

        let span = span!(Level::INFO, "my_test_span2");
        let _guard = span.enter();

        // 在 "my_test_span2" 的上下文中记录一次 event
        event!(Level::DEBUG, "something happened inside my_test_span2");
        // 虽然 event 在哪里都可以使用，但是最好只在 span 的上下文中使用：用于代表一个时间点发生的事件，
        // 例如记录 HTTP 请求返回的状态码，从队列中获取一个对象，等等。
    }

    println!("----------------------------Collector---------------------------------");
    // Collector 收集器
    // 当 Span 或 Event 发生时，它们会被实现了 Collect 特征的收集器所记录或聚合。
    // 这个过程是通过通知的方式实现的：当 Event 发生或者 Span 开始/结束时，
    // 会调用 Collect 特征的相应方法通知 Collector。
    // 我们前面提到只有使用了 tracing-subscriber 后，日志才能输出到控制台中。
    // 之前大家可能还不理解，现在应该明白了，它是一个 Collector，可以将记录的日志收集后，再输出到控制台中。

    println!("----------------------------instrument---------------------------------");
    // #[instrument]
    // 如果想要将某个函数的整个函数体都设置为 span 的范围，最简单的方法就是为函数标记上 #[instrument]，
    // 此时 tracing 会自动为函数创建一个 span，span 名跟函数名相同，在输出的信息中还会自动带上函数参数。
    #[instrument]
    fn foo_func(ans: i32) {
        info!("in foo");
    }
    foo_func(42);

    println!("----------------------------in_scope---------------------------------");
    // in_scope
    // 对于没有内置 tracing 支持或者无法使用 #instrument 的函数，
    // 例如外部库的函数，我们可以使用 Span 结构体的 in_scope 方法，它可以将同步代码包裹在一个 span 中：
    #[derive(Deserialize, Debug)]
    struct User {
        #[serde(rename(deserialize = "fingerprint"))]
        _fingerprint: String,
        #[serde(rename(deserialize = "location"))]
        _location: String,
    }
    let j = b"
        {
            \"fingerprint\": \"0xF9BA143B95FF6D82\",
            \"location\": \"Menlo Park, CA\"
        }";
    let json: User = tracing::info_span!("json.parse").in_scope(|| {
        let r = serde_json::from_slice(j).unwrap();
        info!("parsed json: {:?}", r);
        r
    });
    println!("{:?}", json);

    println!("----------------------------async 中使用 span---------------------------------");
    // 在 async 中使用 span
    // 需要注意，如果是在异步编程时使用，要避免以下使用方式:
    async fn _my_async_function() {
        let span = tracing::info_span!("my_async_function");
        // WARNING: 该 span 直到 drop 后才结束，因此在 .await 期间，span 依然处于工作中状态
        let _enter = span.enter();
        // 在这里 span 依然在记录，但是 .await 会让出当前任务的执行权，然后运行时会去运行其它任务，此时这个 span 可能会记录其它任务的执行信息，最终记录了不正确的 trace 信息
        _some_other_async_function().await
        // ...
    }
    async fn _some_other_async_function() {}
    // 我们建议使用以下方式，简单又有效:
    use tokio::{io::AsyncWriteExt, net::TcpStream};
    #[instrument]
    async fn write(stream: &mut TcpStream) -> std::io::Result<usize> {
        let result = stream.write(b"hello world\n").await;
        info!("wrote to stream; success={:?}", result.is_ok());
        result
    }
    // 或者
    async fn _foo_func2() {
        use tracing::Instrument;

        let my_future = async {
            // ...
        };
        my_future.instrument(tracing::info_span!("my_future")).await
    }

    println!("----------------------------span 嵌套---------------------------------");
    // span 嵌套
    // span 可以嵌套，这样就可以将一个 span 分成多个子 span，这样可以更细粒度地记录信息。
    fn foo_func3() {
        let scope = span!(Level::DEBUG, "foo");
        let _enter = scope.enter();
        info!("Hello in foo scope");
        debug!("before entering bar scope");
        {
            let scope = span!(Level::DEBUG, "bar", ans = 42);
            let _enter = scope.enter();
            debug!("enter bar scope");
            info!("In bar scope");
            debug!("end bar scope");
            log::trace!("tarce end bar scope");
            event!(Level::TRACE, "tarce2 end bar scope");
        }
        debug!("end bar scope");
        // output:
        // 2023-10-05T07:45:25.577977Z  INFO foo: main2: Hello in foo scope
        // 2023-10-05T07:45:25.578135Z DEBUG foo: main2: before entering bar scope
        // 2023-10-05T07:45:25.578323Z DEBUG foo:bar{ans=42}: main2: enter bar scope
        // 2023-10-05T07:45:25.578462Z  INFO foo:bar{ans=42}: main2: In bar scope
        // 2023-10-05T07:45:25.578604Z DEBUG foo:bar{ans=42}: main2: end bar scope
        // 2023-10-05T07:45:25.578742Z TRACE foo:bar{ans=42}: main2: tarce end bar scope
        // 2023-10-05T07:45:25.578908Z TRACE foo:bar{ans=42}: main2: tarce2 end bar scope
        // 2023-10-05T07:45:25.579056Z DEBUG foo: main2: end bar scope
        // 在上面的日志中，foo:bar 不仅包含了 foo 和 bar span 名，还显示了它们之间的嵌套关系。
    }
    foo_func3();

    println!("----------------------------target 与 parent 参数---------------------------------");
    // span! 和 event! 宏都需要设定相应的日志级别，而且它们支持
    // 可选的 target 或 parent 参数，
    // 该参数用于描述事件发生的位置，如果父 span 没有设置，target 参数也没有提供，
    // 那这个位置默认分别是当前的 span 和 当前的模块。

    fn foo_func4() {
        let s = span!(Level::TRACE, "my span");
        // 没进入 span，因此输出日志将不会带上 span 的信息
        event!(target: "app_events", Level::INFO, "something has happened 1!");

        // 进入 span ( 开始 )
        let _enter = s.enter();
        // 没有设置 target 和 parent
        // 这里的对象位置分别是当前的 span 名和模块名
        event!(Level::INFO, "something has happened 2!");
        // 设置了 target
        // 这里的对象位置分别是当前的 span 名和 target
        event!(target: "app_events",Level::INFO, "something has happened 3!");

        let span = span!(Level::TRACE, "my span 1");
        // 这里就更为复杂一些，留给大家作为思考题
        event!(parent: &span, Level::INFO, "something has happened 4!");

        // output:
        // 2023-10-04T13:42:01.699028Z  INFO app_events: something has happened 1!
        // 2023-10-04T13:42:01.699221Z  INFO my span: main2: something has happened 2!
        // 2023-10-04T13:42:01.699439Z  INFO my span: app_events: something has happened 3!
        // 2023-10-04T13:42:01.699677Z  INFO my span:my span 1: main2: something has happened 4!
    }
    foo_func4();

    // 记录字段
    println!("----------------------------记录字段---------------------------------");
    // 记录一个事件，带有两个字段:
    //  - "answer", 值是 42
    //  - "question", 值是 "life, the universe and everything"
    event!(
        Level::INFO,
        answer = 42,
        question = "life, the universe, and everything"
    );
    // 日志输出 -> INFO test_tracing: answer=42 question="life, the universe, and everything"

    // 捕获环境变量
    println!("----------------------------捕获环境变量---------------------------------");
    {
        let user3 = "ferris";
        // 下面的简写方式
        let s = span!(Level::TRACE, "login", user3);
        // 等价于:
        span!(Level::TRACE, "login", user3 = user3);
        // 这里的 user 字段将会被设置为 "ferris"
        let _enter = s.enter();
        event!(Level::INFO, "user logged in");
        // output:
        // 2023-10-04T13:48:19.704498Z  INFO login{user3="ferris"}: main2: user logged in
    }

    // 字段名的多种形式
    println!("----------------------------字段名的多种形式---------------------------------");
    #[derive(Debug)]
    struct User2 {
        _name: &'static str,
        _email: &'static str,
    }
    fn foo_func5() {
        let user_name = "ferris";
        let email = "ferris@rust-lang.org";
        event!(Level::TRACE, user_name, user.email = email);
        // TRACE main2: user="ferris" user.email="ferris@rust-lang.org"

        // 还可以使用结构体
        let _user = User2 {
            _name: "ferris",
            _email: "ferris@rust-lang.org",
        };
        // 直接访问结构体字段，无需赋值即可使用
        // span!(Level::TRACE, "login", _user._name, _user._email);

        // 字段名还可以使用字符串
        event!(
            Level::TRACE,
            "guid:x-request-id" = "abcdef",
            "type" = "request"
        );
        // TRACE main2: guid:x-request-id="abcdef" type="request"

        // ? 符号用于说明该字段将使用 fmt::Debug 来格式化。
        event!(Level::TRACE, "user"= ?_user);
        // TRACE main2: user=User2 { name: "ferris", email: "ferris@rust-lang.org" }

        // % 说明字段将用 fmt::Display 来格式化
        event!(Level::TRACE, "user"= %user_name);
        // Empty
        // 字段还能标记为 Empty，用于说明该字段目前没有任何值，但是可以在后面进行记录。
        use tracing::{field, trace_span};
        let span = trace_span!("my_span", greeting = "hello world", parting = field::Empty);
        // ...
        // 现在，为 parting 记录一个值
        span.record("parting", &"goodbye world!");
    }
    foo_func5();
}
