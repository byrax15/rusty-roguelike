use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(Damage)]
#[read_component(Carried)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect::<Vec<(Entity, Entity, Entity)>>();

    for (message, attacker, victim) in victims.iter() {
        let is_player
            = ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok();

        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query().iter(ecs)
            .filter(|(carried, ..)| carried.0 == *attacker)
            .map(|(.., dmg)| dmg.0)
            .sum();
        let final_damage = base_damage + weapon_damage;

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    }
}