// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 ra2-agent contributors
use ra_arena::{BotContext, Npc};
use ra_engine::GameCommand;

#[derive(Default)]
pub struct Agent;

impl Npc for Agent {
    fn commands(&mut self, context: &BotContext<'_>) -> Vec<GameCommand> {
        ra_engine::hard_baseline_v1_commands(context.world, context.player)
    }
}
