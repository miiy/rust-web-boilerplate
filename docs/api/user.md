## user api

create

```bash
curl -X POST -H "Content-Type: application/json" http://127.0.0.1:8080/api/v1/users -d '
{
    "name": "test1",
    "email": "test1@test.com"
}
'
```

detail

```bash
curl http://127.0.0.1:8080/api/v1/users/1
curl http://127.0.0.1:8080/api/v1/users/0
curl http://127.0.0.1:8080/api/v1/users/-1
curl http://127.0.0.1:8080/api/v1/users/s
```

lists

```bash
curl http://127.0.0.1:8080/api/v1/users
curl "http://127.0.0.1:8080/api/v1/users?page=-1&page_size=s"
```

update

```bash
curl -X PUT -H "Content-Type: application/json" http://127.0.0.1:8080/api/v1/users/1 -d '
{
    "name": "test11",
    "email": "test11@test.com"
}
'
```

delete

```bash
curl -X DELETE http://127.0.0.1:8080/api/v1/users/1
```
