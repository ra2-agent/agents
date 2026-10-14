# 社区 Agent 源码库

通过 PR 收录红色警戒 NPC / Agent 的版本化源码。已收录 starter-idle 及来自固定研究引擎提交的九个冻结策略版本。收录不自动产生 Elo，是否参赛及成绩以 [正式榜单](https://github.com/ra2-agent/leaderboard) 的赛季名单和比赛记录为准。

## 提交 Agent 的条件

1. 从 [agent-template](https://github.com/ra2-agent/agent-template) 开始开发；首期接口为 Rust 进程内 Npc，协议为 ra-arena-full-info-v1。
2. 向 agents 仓库提交 agents/<id>/<version>/，包含 agent.json、README.md、LICENSE、src/lib.rs 和 checksums.json。
3. ID 使用小写字母、数字、连字符；版本使用 major.minor.patch。已有版本内容不可修改；任何策略、文档或依赖变化都提交新版本。
4. 使用有权公开的源码，标明作者、来源和 SPDX 许可。首期接受 Apache-2.0、MIT、BSD-3-Clause；其他许可先讨论。不要上传原版游戏资源、密钥、账号、第三方二进制或模型权重。
5. README 写明策略、接入步骤、预期资源需求、已知限制；不得以“待测”掩盖无法接入。首期不访问网络、文件系统、外部进程，不依赖墙钟或系统随机数。
6. 只通过 GameCommand 操作己方，reset 使用给定种子，每局创建新实例；不修改世界、不读对手私有内存、不识别或针对评测器作弊。
7. 通过静态检查并由 @kexinoh 审核合并。PR 不执行 Agent、不打比赛、不要求作者提交 Elo。合并仅代表已收录，参赛资格由维护者测试确认。
8. 正式结果由 @kexinoh 测试后提交 leaderboard。社区如有战绩异议，请开 Issue，附批次 ID 与证据，不直接改榜单。

## 目录

```text
agents/<id>/<major.minor.patch>/
  agent.json
  README.md
  LICENSE
  src/lib.rs
  checksums.json
```

使用 [开发模板](https://github.com/ra2-agent/agent-template) 的 package-agent 脚本创建版本。Node.js 22+ 下执行 node scripts/check.mjs . 检查格式、哈希、文件上限和路径。静态检查不会编译或执行源码。CI 还会对比主分支，拒绝覆盖已合并版本；该版本的所有说明文件也不可改，补充说明请开新版本或 Issue。

[正式榜单](https://github.com/ra2-agent/leaderboard/blob/main/generated/leaderboard.md) 中的 roster 决定哪些版本已参赛；合并收录并不自动增加分数或触发测试。

## 首批研究策略

| Agent 版本 | 策略 |
|---|---|
| [starter-idle@0.1.0](agents/starter-idle/0.1.0/README.md) | 不主动发出命令的对照 |
| [hard-baseline-v1@1.0.0](agents/hard-baseline-v1/1.0.0/README.md) | 原始 Hard 启发式：展开、建造、生产和进攻。 |
| [reference-v1@1.0.0](agents/reference-v1/1.0.0/README.md) | 在原始策略基础上进行局部集火。 |
| [reference-v2@1.0.0](agents/reference-v2/1.0.0/README.md) | 增加同类车辆的协同接敌。 |
| [reference-v3@1.0.0](agents/reference-v3/1.0.0/README.md) | 增加资金分配、持续生产和队列上限。 |
| [reference-v4@1.0.0](agents/reference-v4/1.0.0/README.md) | 增加按矿区位置选择矿场落点。 |
| [reference-v5@1.0.0](agents/reference-v5/1.0.0/README.md) | 增加射程拉扯与威胁效率；冻结 variant 16。 |
| [reference-v6@1.0.0](agents/reference-v6/1.0.0/README.md) | 按经济条件前置生产建筑；冻结 variant 23。 |
| [reference-v7@1.0.0](agents/reference-v7/1.0.0/README.md) | 混编部队残血撤退；冻结 variant 24，与本轮 candidate 行为相同。 |
| [counter-retreat-v1@1.0.0](agents/counter-retreat-v1/1.0.0/README.md) | 基于 reference-v5 无条件残血撤退；冻结 variant 21。 |

候选版与 reference-v7 当前行为相同，首批只登记冻结的 reference-v7；tune-* 参数变体仍留在引擎实验池。新增策略或改动现有版本时提交新的版本目录。
