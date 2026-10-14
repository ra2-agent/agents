# Starter Idle 0.1.0

作者：@kexinoh / ra2-agent。许可：Apache-2.0。来源：https://github.com/ra2-agent/agent-template。

这是最小接口示例，不主动发出命令，引擎自身的被动接敌行为仍可发生。已在维护者本地含 ra-arena 的引擎工作区完成编译检查；尚未正式评测，不代表游戏强度。

## 接入

实现 `ra_arena::Npc`，入口 `src/lib.rs`，类型 `StarterIdle`。在独立 crate 中通过本地路径依赖 `ra-arena` 与 `ra-engine`，将库入口设置为此文件，再在测试驱动注册：

```rust,ignore
registry.register("starter-idle-0-1-0", || Box::new(starter_idle::StarterIdle))?;
config.npcs.push("starter-idle-0-1-0".into());
```

正式身份为 `starter-idle@0.1.0`，引擎注册名为 `starter-idle-0-1-0`。可在开发模板仓库运行 `scripts/prepare.mjs --engine <引擎目录>` 生成本地构建清单；源码包本身只保存策略文件，开发脚本位于模板仓库。

## 确定性、资源与限制

没有随机决策或持久状态；每次返回空命令列表，不访问网络、文件或外部进程。仅依赖标准库及引擎接口，无额外 Rust 依赖。256 MiB 是包含引擎场景的粗略参考，尚无逐 Agent 的独立资源实测。

不会主动建造、采矿、生产或进攻。首期协议为 `ra-arena-full-info-v1`，当前只适用于对应 Rust 重写引擎；正式评测前仍需固定已公开的引擎提交。

开发自己的 Agent 时，请替换本说明、元数据、作者、来源、ID/版本及策略类型，并准确描述策略和依赖。
