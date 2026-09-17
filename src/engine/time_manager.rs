pub const DEFAULT_MOVE_OVERHEAD_MS: f64 = 100.0;

pub const MIN_THINK_MS: f64 = 20.0;
const SUDDEN_DEATH_DIVISOR: f64 = 30.0;
const MOVES_TO_GO_BUFFER: f64 = 2.0;
const INCREMENT_FRACTION: f64 = 0.75;
const SOFT_FRACTION_OF_HARD: f64 = 0.6;
const HARD_FRACTION_OF_USABLE: f64 = 0.4;

#[derive(Debug, Clone, Copy)]
pub struct TimeControl {
    pub remaining_ms: f64,
    pub increment_ms: f64,
    pub moves_to_go: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Budget {
    pub soft_ms: f64,
    pub hard_ms: f64,
}

impl Budget {
    pub const UNLIMITED: Budget = Budget {
        soft_ms: 0.0,
        hard_ms: 0.0,
    };

    pub fn fixed(ms: f64) -> Budget {
        Budget {
            soft_ms: ms,
            hard_ms: ms,
        }
    }

    pub fn capped_at(self, ceiling_ms: f64) -> Budget {
        Budget {
            soft_ms: self.soft_ms.min(ceiling_ms),
            hard_ms: self.hard_ms.min(ceiling_ms),
        }
    }
}

pub fn plan(tc: &TimeControl, legal_moves: usize, overhead_ms: f64) -> Budget {
    let overhead = overhead_ms.max(0.0);
    let usable = tc.remaining_ms - overhead;

    if usable <= MIN_THINK_MS {
        let floor = usable.clamp(1.0, MIN_THINK_MS);
        return Budget {
            soft_ms: floor,
            hard_ms: floor,
        };
    }

    if legal_moves <= 1 {
        return Budget {
            soft_ms: MIN_THINK_MS,
            hard_ms: MIN_THINK_MS,
        };
    }

    let divisor = match tc.moves_to_go {
        Some(n) => (n.max(1) as f64) + MOVES_TO_GO_BUFFER,
        None => SUDDEN_DEATH_DIVISOR,
    };
    let increment = tc.increment_ms.max(0.0) * INCREMENT_FRACTION;

    let affordable = usable / divisor + increment;
    let hard = affordable
        .min(usable * HARD_FRACTION_OF_USABLE)
        .max(MIN_THINK_MS);
    let soft = (hard * SOFT_FRACTION_OF_HARD).clamp(MIN_THINK_MS, hard);

    Budget {
        soft_ms: soft,
        hard_ms: hard,
    }
}
