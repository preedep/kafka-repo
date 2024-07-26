use log::debug;

use crate::entities::FlowChartItem;
pub fn export_mm_file<T: Into<FlowChartItem>>(
    dataset: Vec<T>,
    _path: &str,
) -> std::io::Result<String> {
    //let mut file = std::fs::File::create(path)?;
    let mut content = String::new();

    let content_header = "flowchart LR;";
    //writeln!(file, "{}", content_header)?;
    content.push_str(content_header);
    content.push_str("\n");

    for item in dataset {
        let item: FlowChartItem = item.into();

        let data = format!("{};", item.to_print_string());
        let style_topic = format!(
            "style {} fill:#f9f,stroke:#333,stroke-width:2px,color:#fff;",
            item.kafka_topic
        );
        let style_consumer = format!(
            "style {} fill:#bbf,stroke:#333,stroke-width:2px,color:#000;",
            item.consumer_group
        );

        content.push_str(format!("  {}", &data).as_str());
        content.push_str("\n");
        content.push_str(&style_topic);
        content.push_str("\n");
        content.push_str(&style_consumer);
        content.push_str("\n");
    }
    debug!("content : \n{}", content);

    Ok(content)
}
