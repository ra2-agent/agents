// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 ra2-agent contributors
use ra_arena::{BotContext, Npc};
use ra_engine::GameCommand;

#[derive(Default)]
pub struct StarterIdle;

impl Npc for StarterIdle {
    fn commands(&mut self, _context: &BotContext<'_>) -> Vec<GameCommand> {
        // 从这里实现建造、采矿、生产、目标选择；只返回己方命令。
        Vec::new()
    }
}
