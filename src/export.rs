use std::io::Write;

use crate::entities::FlowChartItem;

pub fn export_mm_file<T: Into<FlowChartItem>>(dataset: &[T], path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    let content_header = "flowchart LR";
    writeln!(file, "{}", content_header)?;

    for item in dataset {
        //let item : FlowChartItem = item.into();
        //writeln!(file, "\t{}", item.to_print_string())?;
    }

    Ok(())
}