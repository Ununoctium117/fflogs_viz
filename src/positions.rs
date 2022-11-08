use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

pub type Position = (f64, f64);
pub type Rect = (Position, Position); // min, max

#[derive(Debug, Default)]
pub struct PositionHistory {
    history: BTreeMap<OrderedFloat<f64>, Position>,
}
impl PositionHistory {
    pub fn add_update(&mut self, timestamp: i64, position: Position) {
        let timestamp = OrderedFloat(timestamp as f64);

        // Don't record something we already have.
        if self.history.contains_key(&timestamp) {
            return;
        }

        let prev_entry = self.get_previous_entry(timestamp);
        let next_entry = self.get_next_entry(timestamp);

        // If prev and next are both equal to the new entry, drop the new entry, since it's entirely
        // covered by prev and next.
        if let (Some((_, prev_pos)), Some((_, next_pos))) = (prev_entry, next_entry) {
            if prev_pos == next_pos && next_pos == position {
                return;
            }
        }

        // If prev and prev-prev are both equal to the new entry, drop prev, since prev-prev and the new
        // entry will completely cover it.
        if let Some((prev_time, prev_pos)) = prev_entry {
            if prev_pos == position {
                if let Some((_, prev_prev_pos)) = self.get_previous_entry(prev_time) {
                    if prev_prev_pos == prev_pos {
                        self.history.remove(&prev_time);
                    }
                }
            }
        }

        // If next and next-next are both equal to the new entry, drop next, since the new entry and
        // next-next will completely cover it.
        if let Some((next_time, next_pos)) = next_entry {
            if next_pos == position {
                if let Some((_, next_next_pos)) = self.get_next_entry(next_time) {
                    if next_next_pos == next_pos {
                        self.history.remove(&next_time);
                    }
                }
            }
        }

        self.history.insert(timestamp, position);
    }

    // Assumes linear motion over time between points. Panics if there's no entries.
    pub fn get_position_at(&self, timestamp: f64) -> Position {
        let timestamp = OrderedFloat(timestamp);

        // Simple case: there is an entry at the specified timestamp
        if let Some(position) = self.history.get(&timestamp) {
            return *position;
        }

        // Otherwise, get the previous position/timestamp and next position/timestamp and interpolate between them.
        let (prev_time, prev_pos) = self.get_previous_entry(timestamp).unwrap_or_else(|| {
            let (k, v) = self.history.iter().next().unwrap();
            (*k, *v)
        });
        let (next_time, next_pos) = self.get_next_entry(timestamp).unwrap_or_else(|| {
            let (k, v) = self.history.iter().rev().next().unwrap();
            (*k, *v)
        });

        // Handle the one entry case
        if prev_time == next_time {
            return prev_pos;
        }

        // Linearly interpolate between the two positions
        let dt = next_time - prev_time;
        let ratio = (timestamp - prev_time) / dt;

        let dx = next_pos.0 - prev_pos.0;
        let dy = next_pos.1 - prev_pos.1;

        (prev_pos.0 + (ratio.0 * dx), prev_pos.1 + (ratio.0 * dy))
    }

    pub fn len(&self) -> usize {
        self.history.len()
    }

    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }

    fn get_previous_entry(
        &self,
        timestamp: OrderedFloat<f64>,
    ) -> Option<(OrderedFloat<f64>, Position)> {
        self.history
            .range(..timestamp)
            .rev()
            .next()
            .map(|(k, v)| (*k, *v))
    }

    fn get_next_entry(
        &self,
        timestamp: OrderedFloat<f64>,
    ) -> Option<(OrderedFloat<f64>, Position)> {
        self.history
            .range(timestamp..)
            .next()
            .map(|(k, v)| (*k, *v))
    }
}

#[cfg(test)]
mod tests {
    use super::PositionHistory;

    #[test]
    fn simple_add_test() {
        let mut history = PositionHistory::default();

        history.add_update(1, (1.0, 1.0));
        history.add_update(2, (2.0, 2.0));
        history.add_update(3, (3.0, 3.0));

        assert_eq!(history.len(), 3);
        assert!(!history.is_empty());
    }

    #[test]
    fn complex_add_test() {
        let mut history = PositionHistory::default();
        history.add_update(1, (10.0, 10.0));
        history.add_update(2, (10.0, 10.0));
        history.add_update(3, (10.0, 10.0));
        assert_eq!(history.len(), 2);
        dbg!(&history);

        history.add_update(10, (20.0, 20.0));
        assert_eq!(history.len(), 3);

        history.add_update(9, (20.0, 20.0));
        assert_eq!(history.len(), 4);

        history.add_update(8, (20.0, 20.0));
        assert_eq!(history.len(), 5);
    }

    #[test]
    fn get_position_test() {
        let mut history = PositionHistory::default();

        history.add_update(10, (0.0, 0.0));
        history.add_update(20, (0.0, 10.0));
        history.add_update(30, (10.0, 10.0));

        assert_eq!(history.get_position_at(10.0), (0.0, 0.0));
        assert_eq!(history.get_position_at(20.0), (0.0, 10.0));
        assert_eq!(history.get_position_at(30.0), (10.0, 10.0));

        assert_eq!(history.get_position_at(5.0), (0.0, 0.0));
        assert_eq!(history.get_position_at(35.0), (10.0, 10.0));

        assert_eq!(history.get_position_at(15.0), (0.0, 5.0));
        assert_eq!(history.get_position_at(25.0), (5.0, 10.0));
        assert_eq!(history.get_position_at(28.0), (8.0, 10.0));
    }
}
