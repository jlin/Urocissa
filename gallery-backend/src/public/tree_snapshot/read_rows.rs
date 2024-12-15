use super::TreeSnapshot;
use crate::public::{
    constant::ROW_BATCH_NUMBER,
    row::{DisplayElement, Row},
};
use rocket::http::Status;

impl TreeSnapshot {
    pub fn read_row(&'static self, row_index: usize, timestamp: u128) -> Result<Row, Status> {
        let tree_snapshot = self.read_tree_snapshot(&timestamp)?;

        let data_length = tree_snapshot.len();
        let chunk_count = (data_length + ROW_BATCH_NUMBER - 1) / ROW_BATCH_NUMBER; // Calculate total chunks

        if row_index > chunk_count {
            error!("read_rows out of bound");
            return Err(Status::NotFound);
        }

        let number_vec = (row_index * ROW_BATCH_NUMBER)
            ..(row_index * ROW_BATCH_NUMBER + ROW_BATCH_NUMBER).min(data_length);

        let display_elements: Vec<DisplayElement> = number_vec
            .map(|index| {
                let (width, height) = tree_snapshot.get_width_height(index);
                DisplayElement {
                    display_width: width,
                    display_height: height,
                }
            })
            .collect();

        Ok(Row {
            start: row_index * ROW_BATCH_NUMBER,
            end: row_index * ROW_BATCH_NUMBER + ROW_BATCH_NUMBER - 1,
            display_elements,
            row_index: row_index,
        })
    }
}
