# Axum

- 共享变量
- 生成 jwt 并且检验
- 实现 Request 和 IntoResponse

## 优化的一个iterator

```rust

async fn todos_handler(
    claim: Claims,
    Extension(store): Extension<TodoStore>,
) -> Result<Json<Vec<Todo>>, HttpError> {
    let user_id = claim.id; // 获取自己的id, 返回属于自己的值
    match store.items.read() {
        Ok(items) => Ok(Json(
            items
                // 这个实现效率太低， 看下面如何优化的
                // .clone()
                // .into_iter()
                // .filter(|todo| todo.user_id == user_id)
                // .collect(),
                .iter()
                .filter(|todo| todo.user_id == user_id)
                .map(|todo| todo.clone()) // 使用filter+map 对数据进行过滤之后再到 map 之中进行 clone转化
                .collect(),
        )),
        Err(_) => Err(HttpError::Internal),
    }
}
```

## 从 header 之中获取 token

```rust
async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    let TypedHeader(Authorization(bearer)) =
        TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .map_err(|_| HttpError::Auth)?;

    let key = DecodingKey::from_secret(SECRET_KEY);
    let token =
        decode::<Claims>(bearer.token(), &key, &Validation::default()).map_err(|e| {
            println!("{:?}", e);
            HttpError::Auth
        })?;

    Ok(token.claims)
}
```

## testing

测试数据

```bash
# 登录
http -v POST :8080/login email="liwei" password="liwei"

# 生成 todo
http -v POST :8080/todos title=hello "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IkpvaG4gRG9lIiwiZXhwIjoxNjUxNjM0MDk4fQ.4QtlPan7O7Nbf101GNdeYV1FvOAefE3PoRetGILp-8g"
http -v POST :8080/todos title=goodbye "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IkpvaG4gRG9lIiwiZXhwIjoxNjUxNjM0MDk4fQ.4QtlPan7O7Nbf101GNdeYV1FvOAefE3PoRetGILp-8g"

# 查看属于自己的 todo
http -v :8080/todos "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZCI6MSwibmFtZSI6IkpvaG4gRG9lIiwiZXhwIjoxNjUxNjM0MDk4fQ.4QtlPan7O7Nbf101GNdeYV1FvOAefE3PoRetGILp-8g"
```