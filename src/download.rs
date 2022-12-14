use linked_hash_map::LinkedHashMap;
use std::collections::HashSet;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Orders {
  result: Vec<serde_json::Value>,
  pagination: Pagination,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Pagination {
  has_next_page: bool,
  next_page_cursor: Option<String>,
  next_page_url: Option<String>,
}

pub async fn download_to_csv(
  filename: &str,
  bearer_token: &str,
) -> std::path::PathBuf {
  let mut labels: linked_hash_set::LinkedHashSet<String> =
    linked_hash_set::LinkedHashSet::new();
  let mut rows: Vec<LinkedHashMap<String, String>> = Vec::new();

  let mut cursor: Option<String> = None;
  let mut next_page_eh = true;
  while next_page_eh {
    let orders = orders_call(bearer_token, &cursor)
      .await
      .expect("Request for orders failed");
    let filtered_orders = filter_orders(&orders.result);
    let cur_rows = parse_orders(&filtered_orders);

    for row in cur_rows.iter() {
      for (label, _) in row {
        labels.insert_if_absent(label.to_owned());
      }
      rows.push(row.clone());
    }

    next_page_eh = orders.pagination.has_next_page;
    if next_page_eh {
      cursor = orders.pagination.next_page_cursor;
    }
  }

  let write = write_to_file(&labels, &rows, filename);
  match write {
    Ok(path) => {
      println!(
        "Write successful to {}",
        path.to_str().expect("Couldn't convert path to string")
      );
      return path;
    }
    Err(e) => panic!("Problem writing to file: {:?}", e),
  }
}

async fn orders_call(
  bearer_token: &str,
  cursor: &Option<String>,
) -> Result<Orders, Box<dyn std::error::Error>> {
  let mut url: String =
    "https://api.squarespace.com/1.0/commerce/orders".to_owned();
  match cursor {
    Some(x) => url.push_str(&format!("?cursor={}", x)),
    None => (),
  }
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .header(reqwest::header::USER_AGENT, "CCYAA Order Exporter")
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .header(
      reqwest::header::AUTHORIZATION,
      format!("Bearer {}", bearer_token),
    )
    .send()
    .await?;

  match response.status() {
    reqwest::StatusCode::OK => {
      return Ok(response.json().await?);
    }
    reqwest::StatusCode::UNAUTHORIZED => {
      panic!("Invalid API token");
    }
    _ => {
      panic!("Unexpected status code: {}", response.status());
    }
  };
}

fn filter_orders(orders: &Vec<serde_json::Value>) -> Vec<serde_json::Value> {
  let skus: HashSet<&str> = vec!["SQ1360384"].into_iter().collect();
  let mut filtered_orders: Vec<serde_json::Value> = Vec::new();
  for order in orders.iter() {
    let line_items = order["lineItems"]
      .as_array()
      .expect("LineItems is not an array");
    for line_item in line_items.iter() {
      let sku = line_item["sku"].as_str().expect("No SKU found");
      if skus.contains(sku) {
        filtered_orders.push(order.clone());
      }
    }
  }
  return filtered_orders;
}

fn parse_orders(
  orders: &Vec<serde_json::Value>,
) -> Vec<LinkedHashMap<String, String>> {
  let mut rows: Vec<LinkedHashMap<String, String>> = Vec::new();
  for order in orders.iter() {
    let line_items = order["lineItems"]
      .as_array()
      .expect("LineItems is not an array");
    for line_item in line_items.iter() {
      let customizations = line_item["customizations"]
        .as_array()
        .expect("customizations not array");
      let mut row: LinkedHashMap<String, String> = LinkedHashMap::new();
      for customization in customizations.iter() {
        let label = customization["label"]
          .as_str()
          .expect("label not a string")
          .to_owned();
        let value = customization["value"]
          .as_str()
          .expect("label not a string")
          .to_owned();
        row.insert(label, value);
      }
      rows.push(row);
    }
  }
  return rows;
}

fn write_to_file(
  labels: &linked_hash_set::LinkedHashSet<String>,
  rows: &Vec<LinkedHashMap<String, String>>,
  filename: &str,
) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
  let path = std::path::PathBuf::from(format!("./{}", filename));
  let mut wtr = csv::Writer::from_path(&path)?;
  wtr.write_record(labels)?;

  for row in rows.iter() {
    let mut write_row: Vec<String> = Vec::new();
    for label in labels.iter() {
      let mut value = "".to_owned();
      if row.contains_key(label) {
        value = row[label].clone();
      }
      write_row.push(value);
    }
    wtr.write_record(write_row)?;
  }
  wtr.flush()?;

  return Ok(path);
}
