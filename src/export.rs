use std::io::Write;

use crate::entities::FlowChartItem;

pub fn export_mm_file<T: Into<FlowChartItem>>(dataset: Vec<T>, path: &str) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path)?;
    let content_header = "flowchart LR";
    writeln!(file, "{}", content_header)?;

    for item in dataset {
        let item : FlowChartItem = item.into();

        writeln!(file, "\t{}", item.to_print_string())?;
        writeln!(file,"style {} fill:#f9f,stroke:#333,stroke-width:2px,color:#fff",  item.kafka_topic.as_str())?;
        writeln!(file,"style {} fill:#bbf,stroke:#333,stroke-width:2px,color:#000", item.consumer_group.as_str())?;
    }
     Ok(())
}