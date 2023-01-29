use std::ops::Deref;

use crate::game::actions::*;
use rand::prelude::*;

pub struct RandomPolicy {

}

impl RandomPolicy {
    pub fn act(&self, available_actions: Vec<Action>) -> Action {
        let mut rng = rand::thread_rng();
        available_actions.choose(&mut rng)
            .expect("The policy shouldn't be called if there are no actions")
            .clone()
    }
}