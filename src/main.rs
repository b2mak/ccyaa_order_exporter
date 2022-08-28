use std::collections::HashSet;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Orders {
  result: Vec<serde_json::Value>,
  pagination: serde_json::Value,
}

fn main() {
  let mut cursor: Option<String> = None;
  let mut next_page_eh = true;
  while next_page_eh {
    let orders = orders_call(&cursor).expect("Request for orders failed");
    let filtered_orders = filter_orders(&orders.result);
    println!("{:#?}", filtered_orders);

    next_page_eh = orders.pagination["hasNextPage"]
      .as_bool()
      .expect("hasNextPage is not a bool");
    if next_page_eh {
      cursor = Some(
        orders.pagination["nextPageCursor"]
          .as_str()
          .expect("nextPageCursor is not a string")
          .to_owned(),
      );
    }
  }
}

fn orders_call(
  cursor: &Option<String>,
) -> Result<Orders, Box<dyn std::error::Error>> {
  let mut url: String =
    "https://api.squarespace.com/1.0/commerce/orders".to_owned();
  match cursor {
    Some(x) => url.push_str(&format!("?cursor={}", x)),
    None => (),
  }
  let client = reqwest::blocking::Client::new();
  let res: Orders = client
    .get(url)
    .header(reqwest::header::USER_AGENT, "CCYAA Order Exporter")
    .header(reqwest::header::CONTENT_TYPE, "application/json")
    .header(
      "Authorization",
      "Bearer 87d60348-77e1-4345-af4e-fb238c5fc4c0",
    )
    .send()?
    .json()?;
  return Ok(res);
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

// fn parse_orders(
//   url: &str,
// ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
//   let client = reqwest::blocking::Client::new();
//   let res: serde_json::Value = client
//     .get(url)
//     .header(reqwest::header::USER_AGENT, "CCYAA Order Exporter")
//     .header(reqwest::header::CONTENT_TYPE, "application/json")
//     .header(
//       "Authorization",
//       "Bearer 87d60348-77e1-4345-af4e-fb238c5fc4c0",
//     )
//     .send()?
//     .json()?;
//   return Ok(res);
// }
