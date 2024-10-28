## post api

create

```bash
curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8080/api/v1/posts -d '
{
    "category_id": 1,
    "title": "title",
    "author": "admin",
    "content": "content",
}
'
```

detail

```bash
curl http://127.0.0.1:8080/api/v1/posts/1
curl http://127.0.0.1:8080/api/v1/posts/0
curl http://127.0.0.1:8080/api/v1/posts/-1
curl http://127.0.0.1:8080/api/v1/posts/s
```

lists

```bash
curl http://127.0.0.1:8080/api/v1/posts
curl "http://127.0.0.1:8080/api/v1/posts?page=-1&page_size=s"
```

update

```bash
curl -X PUT -H "Content-Type: application/json" http://127.0.0.1:8080/api/v1/posts/1 -d '
{
    "category_id": 2,
    "title": "title1",
    "author": "admin1",
    "content": "content1",
}
'
```

delete

```bash
curl -X DELETE http://127.0.0.1:8080/api/v1/posts/1
```
