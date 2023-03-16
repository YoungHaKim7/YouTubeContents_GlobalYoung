# Result

```

$ cargo watch -c -q -x run

    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/todo_api`
🔧 Configured for debug.
   >> address: 127.0.0.1
   >> port: 8000
   >> workers: 8
   >> ident: Rocket
   >> limits: bytes = 8KiB, data-form = 2MiB, file = 1MiB, form = 32KiB, json = 1MiB, msgpack = 1MiB, string = 8KiB
   >> temp dir: /var/folders/bf/fk_kzm5n5zbfzkv31qhrglhr0000gn/T/
   >> http/2: true
   >> keep-alive: 5s
   >> tls: disabled
   >> shutdown: ctrlc = true, force = true, signals = [SIGTERM], grace = 2s, mercy = 3s
   >> log level: normal
   >> cli colors: true
📬 Routes:
   >> (get_all_tasks) GET /tasks
   >> (get_task) GET /task/<id>
   >> (toggle_task) PATCH /task/<id>
   >> (delete_task) DELETE /task/<id>
   >> (add_task) POST /task/<title>
📡 Fairings:
   >> Add CORS headers to responses (response)
   >> Shield (liftoff, response, singleton)
🛡️ Shield:
   >> X-Content-Type-Options: nosniff
   >> X-Frame-Options: SAMEORIGIN
   >> Permissions-Policy: interest-cohort=()
🚀 Rocket has launched from http://127.0.0.1:8000
GET /tasks text/html:
   >> Matched: (get_all_tasks) GET /tasks
   >> Outcome: Success
   >> Response succeeded.
GET /tasks/1 text/html:
   >> No matching routes for GET /tasks/1 text/html.
   >> No 404 catcher registered. Using Rocket default.
   >> Response succeeded.
GET /tasks/my_ns text/html:
   >> No matching routes for GET /tasks/my_ns text/html.
   >> No 404 catcher registered. Using Rocket default.
   >> Response succeeded.
GET /tasks/ds text/html:
   >> No matching routes for GET /tasks/ds text/html.
   >> No 404 catcher registered. Using Rocket default.
   >> Response succeeded.
```
