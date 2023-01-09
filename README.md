# Simple rocket rest backend

Use postman to test endpoints.

```cargo run```

📬 Routes:
   > (ok) GET /
   > (sleep) GET /sleep/<seconds>
   > (read) GET /user/
   > (read_error) GET /user/ [2]
   > (login) POST /auth/
   > (create) POST /post/
   > (update) PUT /post/
   > (get) GET /user/<id>
   > (read) GET /post/<id>
   > (delete) DELETE /post/<id>
   > (list) GET /users/
   > (list) GET /posts/