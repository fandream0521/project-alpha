// 基础集成测试
use reqwest;
use serde_json::Value;

// 测试配置
const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn test_health_check() {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health", BASE_URL))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);

    let body = response.text().await.expect("Failed to read response body");
    assert_eq!(body, "Ticket backend is running!");
}

#[tokio::test]
async fn test_database_stats() {
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/api/db/stats", BASE_URL))
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(response.status(), 200);

    let stats: Value = response.json().await.expect("Failed to parse JSON");
    assert!(stats.get("tickets_count").is_some());
    assert!(stats.get("tags_count").is_some());
    assert!(stats.get("comments_count").is_some());
}

#[tokio::test]
async fn test_create_and_get_tag() {
    let client = reqwest::Client::new();

    // 创建标签
    let create_payload = serde_json::json!({
        "name": "测试标签",
        "color": "#FF5733"
    });

    let create_response = client
        .post(&format!("{}/api/v1/tags", BASE_URL))
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to create tag");

    assert_eq!(create_response.status(), 200);

    let created_tag: Value = create_response
        .json()
        .await
        .expect("Failed to parse created tag");
    let tag_id = created_tag.get("id").unwrap().as_str().unwrap();
    assert_eq!(
        created_tag.get("name").unwrap().as_str().unwrap(),
        "测试标签"
    );

    // 获取标签
    let get_response = client
        .get(&format!("{}/api/v1/tags/{}", BASE_URL, tag_id))
        .send()
        .await
        .expect("Failed to get tag");

    assert_eq!(get_response.status(), 200);

    let retrieved_tag: Value = get_response
        .json()
        .await
        .expect("Failed to parse retrieved tag");
    assert_eq!(retrieved_tag.get("id").unwrap().as_str().unwrap(), tag_id);
    assert_eq!(
        retrieved_tag.get("name").unwrap().as_str().unwrap(),
        "测试标签"
    );

    // 清理：删除标签
    let delete_response = client
        .delete(&format!("{}/api/v1/tags/{}", BASE_URL, tag_id))
        .send()
        .await
        .expect("Failed to delete tag");

    assert_eq!(delete_response.status(), 204);
}

#[tokio::test]
async fn test_create_and_get_ticket() {
    let client = reqwest::Client::new();

    // 创建工单
    let create_payload = serde_json::json!({
        "title": "测试工单",
        "description": "这是一个测试工单的描述",
        "priority": "high"
    });

    let create_response = client
        .post(&format!("{}/api/v1/tickets", BASE_URL))
        .json(&create_payload)
        .send()
        .await
        .expect("Failed to create ticket");

    assert_eq!(create_response.status(), 200);

    let created_ticket: Value = create_response
        .json()
        .await
        .expect("Failed to parse created ticket");
    let ticket_id = created_ticket.get("id").unwrap().as_str().unwrap();
    assert_eq!(
        created_ticket.get("title").unwrap().as_str().unwrap(),
        "测试工单"
    );
    assert_eq!(
        created_ticket.get("status").unwrap().as_str().unwrap(),
        "open"
    );
    assert_eq!(
        created_ticket.get("priority").unwrap().as_str().unwrap(),
        "high"
    );

    // 获取工单
    let get_response = client
        .get(&format!("{}/api/v1/tickets/{}", BASE_URL, ticket_id))
        .send()
        .await
        .expect("Failed to get ticket");

    assert_eq!(get_response.status(), 200);

    let retrieved_ticket: Value = get_response
        .json()
        .await
        .expect("Failed to parse retrieved ticket");
    assert_eq!(
        retrieved_ticket.get("id").unwrap().as_str().unwrap(),
        ticket_id
    );
    assert_eq!(
        retrieved_ticket.get("title").unwrap().as_str().unwrap(),
        "测试工单"
    );

    // 清理：删除工单
    let delete_response = client
        .delete(&format!("{}/api/v1/tickets/{}", BASE_URL, ticket_id))
        .send()
        .await
        .expect("Failed to delete ticket");

    assert_eq!(delete_response.status(), 204);
}

#[tokio::test]
async fn test_advanced_search() {
    let client = reqwest::Client::new();

    // 测试搜索参数
    let search_params = [
        ("search", "test"),
        ("status", "open"),
        ("priority", "high"),
        ("limit", "10"),
    ];

    let mut url = format!("{}/api/v1/tickets", BASE_URL);
    if !search_params.is_empty() {
        url.push('?');
        let param_strings: Vec<String> = search_params
            .iter()
            .map(|(key, value)| format!("{}={}", key, urlencoding::encode(value)))
            .collect();
        url.push_str(&param_strings.join("&"));
    }

    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute search request");

    assert_eq!(response.status(), 200);

    let search_result: Value = response
        .json()
        .await
        .expect("Failed to parse search result");
    assert!(search_result.get("data").is_some());
    assert!(search_result.get("total").is_some());
    assert!(search_result.get("page").is_some());
    assert!(search_result.get("limit").is_some());
}

#[tokio::test]
async fn test_database_optimization() {
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/api/db/optimize", BASE_URL))
        .send()
        .await
        .expect("Failed to execute optimization request");

    assert_eq!(response.status(), 200);

    let result: Value = response
        .json()
        .await
        .expect("Failed to parse optimization result");
    assert_eq!(
        result.get("message").unwrap().as_str().unwrap(),
        "数据库优化完成"
    );
    assert!(result.get("created_indexes").unwrap().as_u64().unwrap() > 0);
}
