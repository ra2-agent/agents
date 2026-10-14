# 参照策略 v5 (reference-v5@1.0.0)

增加射程拉扯与威胁效率；冻结 variant 16。

## 来源与冻结范围

来自 [ra2-agent/rust-ra2](https://github.com/ra2-agent/rust-ra2/tree/189a5e2b3d7e8afaaf5d1a868cb9f15d0464f938) 的完整提交 `189a5e2b3d7e8afaaf5d1a868cb9f15d0464f938`。引擎 Fork 自 rust-alert/ra2.exe，上游作者 Aster 及原有贡献者的源码和 Apache-2.0 许可继续保留；本包仅增加接入包装。

`source/strategy.rs` 是该提交 `projects/engine/ra-engine/src/gameplay/npc/reference_v5.rs` 的原始字节快照；`source/engine-npc-mod.rs` 保留公开入口到冻结实现及参数的映射。上述文件用于源码审核，不作为独立模块编译。所有文件纳入 `checksums.json`。

## 接入

使用上述固定引擎提交，以 Rust 模块包含 `src/lib.rs`，调用 `registry.register("reference-v5", || Box::new(module::Agent))?`。实际执行的是本包的 Npc 入口，入口调用固定引擎提交的公开策略函数：

``ra_engine::reference_npc_commands(context.world, context.player, 5)``

策略依赖引擎提供的 `ra-arena` / `ra-engine`，无需额外 crate、网络、文件访问或外部进程。不能用浮动引擎分支替代固定提交并延续同一正式身份。

## 资源与限制

无跨局可变状态；每局创建新实例。未使用随机源，因此无需覆盖 reset。完整世界观测，只经 GameCommand 返回己方操作。内存声明为参考，实际引擎内存由双方共享；不宣称逐 Agent CPU/内存隔离。适用于当前合成场景，表现不代表原版红警或战争迷雾环境。收录本版本不自动产生 Elo，正式结果由维护者评测后写入 leaderboard。
