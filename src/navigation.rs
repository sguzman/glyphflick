#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    active: Option<usize>,
}

impl Selection {
    #[inline]
    pub const fn active(self) -> Option<usize> {
        self.active
    }

    #[inline]
    pub fn reset(&mut self) {
        self.active = None;
    }

    pub fn move_up(&mut self, len: usize, columns: usize) -> bool {
        self.move_to(len, |current| current.saturating_sub(columns.max(1)))
    }

    pub fn move_down(&mut self, len: usize, columns: usize) -> bool {
        let columns = columns.max(1);
        self.move_to(len, |current| current.saturating_add(columns))
    }

    pub fn move_left(&mut self, len: usize) -> bool {
        self.move_to(len, |current| current.saturating_sub(1))
    }

    pub fn move_right(&mut self, len: usize) -> bool {
        self.move_to(len, |current| current.saturating_add(1))
    }

    fn move_to(&mut self, len: usize, next: impl FnOnce(usize) -> usize) -> bool {
        if len == 0 {
            return self.replace(None);
        }

        let next = match self.active {
            Some(current) => next(current).min(len - 1),
            None => 0,
        };

        self.replace(Some(next))
    }

    #[inline]
    fn replace(&mut self, next: Option<usize>) -> bool {
        let changed = self.active != next;
        self.active = next;
        changed
    }
}

#[cfg(test)]
mod tests {
    use super::Selection;

    #[test]
    fn navigation_enters_at_first_result() {
        let mut selection = Selection::default();
        assert!(selection.move_down(10, 5));
        assert_eq!(selection.active(), Some(0));
    }

    #[test]
    fn vertical_navigation_moves_by_column_count() {
        let mut selection = Selection::default();
        selection.move_down(20, 5);
        selection.move_down(20, 5);
        assert_eq!(selection.active(), Some(5));
        selection.move_up(20, 5);
        assert_eq!(selection.active(), Some(0));
    }

    #[test]
    fn horizontal_navigation_clamps_to_results() {
        let mut selection = Selection::default();
        selection.move_down(2, 5);
        selection.move_right(2);
        selection.move_right(2);
        assert_eq!(selection.active(), Some(1));
        selection.move_left(2);
        assert_eq!(selection.active(), Some(0));
    }

    #[test]
    fn empty_results_clear_selection() {
        let mut selection = Selection::default();
        selection.move_down(3, 3);
        assert!(selection.move_down(0, 3));
        assert_eq!(selection.active(), None);
    }

    #[test]
    fn down_clamps_to_last_partial_row() {
        let mut selection = Selection::default();
        selection.move_down(12, 5);
        selection.move_down(12, 5);
        selection.move_down(12, 5);
        selection.move_down(12, 5);
        assert_eq!(selection.active(), Some(11));
    }
}
