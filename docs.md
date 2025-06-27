# API Documentation

## Users

### Get Users

- **Route:** `GET /api/users`
- **Query Params:** `page` (optional), `limit` (optional)
- **Input:** None
- **Output:**

```json
{
  "status": "success",
  "users": [
    {
      "id": "uuid",
      "name": "string",
      "email": "string",
      "role": "string",
      "verified": true,
      "createdAt": "datetime",
      "updatedAt": "datetime",
      "skills": [{ "id": "uuid", "name": "string" }]
    }
  ],
  "results": 42,
  "has_next_page": true
}
```

### Get User by ID

- **Route:** `GET /api/users/{user_id}`
- **Params:** `user_id` (path)
- **Input:** None
- **Output:**

```json
{
  "status": "success",
  "data": {
    "user": {
      /* same as above */
    }
  }
}
```

### Get User by Email

- **Route:** `GET /api/users/email/{email}`
- **Params:** `email` (path)
- **Input:** None
- **Output:** Same as Get User by ID

### Create User

- **Route:** `POST /api/users`
- **Input:**

```json
{
  "name": "string",
  "email": "string",
  "password": "string",
  "passwordConfirm": "string"
}
```

- **Output:** Same as Get User by ID

### Update User

- **Route:** `PUT /api/users/{user_id}`
- **Params:** `user_id` (path)
- **Input:**

```json
{
  "name": "string"
}
```

- **Output:** Same as Get User by ID

### Update User Role

- **Route:** `PUT /api/users/{user_id}/role`
- **Params:** `user_id` (path)
- **Input:**

```json
{
  "role": "Admin|User|Guest"
}
```

- **Output:** Same as Get User by ID

### Delete User

- **Route:** `DELETE /api/users/{user_id}`
- **Params:** `user_id` (path)
- **Output:**

```json
{
  "status": "success",
  "message": "User <name> deleted successfully (Dummy)"
}
```

---

## Skills

### Get Skills

- **Route:** `GET /api/skills`
- **Query Params:** `page` (optional), `limit` (optional)
- **Input:** None
- **Output:**

```json
{
  "status": "success",
  "skills": [{ "id": "uuid", "name": "string" }],
  "results": 42,
  "has_next_page": true
}
```

### Get Skill by ID

- **Route:** `GET /api/skills/{skill_id}`
- **Params:** `skill_id` (path)
- **Output:**

```json
{ "id": "uuid", "name": "string" }
```

### Get Skill by Name

- **Route:** `GET /api/skills/find?name=SkillName`
- **Query Params:** `name` (required)
- **Output:** Same as Get Skill by ID

### Create Skill

- **Route:** `POST /api/skills`
- **Input:**

```json
{ "name": "string" }
```

- **Output:** Same as Get Skill by ID

### Update Skill

- **Route:** `PUT /api/skills/{skill_id}`
- **Params:** `skill_id` (path)
- **Input:**

```json
{ "id": "uuid", "name": "string" }
```

- **Output:**

```json
{ "status": "Success", "message": "Skill with id - <uuid> Updated Succesfully" }
```

### Delete Skill

- **Route:** `DELETE /api/skills/{skill_id}`
- **Params:** `skill_id` (path)
- **Output:**

```json
{ "status": "success", "message": "Skill deleted successfully" }
```

### Add Skill to User

- **Route:** `POST /api/skills/user/{user_id}/add`
- **Params:** `user_id` (path)
- **Input:**

```json
{ "skill_id": "uuid" }
```

- **Output:**

```json
{ "status": "success", "message": "Skill added to user successfully" }
```

### Remove Skill from User

- **Route:** `POST /api/skills/user/{user_id}/remove`
- **Params:** `user_id` (path)
- **Input:**

```json
{ "skill_id": "uuid" }
```

- **Output:**

```json
{ "status": "success", "message": "Skill removed from user successfully" }
```

### Get Skills of User

- **Route:** `GET /api/skills/user/{user_id}`
- **Params:** `user_id` (path)
- **Output:**

```json
{
  "status": "success",
  "skills": [{ "id": "uuid", "name": "string" }],
  "results": 2,
  "has_next_page": false
}
```

### Get Users of Skill

- **Route:** `GET /api/skills/users/find?skill_id=uuid`
- **Query Params:** `skill_id` (required)
- **Output:**

```json
{
  "status": "success",
  "users": [
    /* user objects */
  ],
  "results": 2
}
```

---

## Auth

### Register

- **Route:** `POST /api/auth/register`
- **Input:**

```json
{
  "name": "string",
  "email": "string",
  "password": "string",
  "passwordConfirm": "string"
}
```

- **Output:**

```json
{
  "status": "success",
  "message": "Registration successful! Please check your email to verify your account."
}
```

### Login

- **Route:** `POST /api/auth/login`
- **Input:**

```json
{ "email": "string", "password": "string" }
```

- **Output:**

```json
{ "status": "success", "token": "jwt_token" }
```

### Verify Email

- **Route:** `GET /api/auth/verify?token=...`
- **Query Params:** `token` (required)
- **Output:**
  HTML page or JSON with token

### Forgot Password

- **Route:** `POST /api/auth/forgot-password`
- **Input:**

```json
{ "email": "string" }
```

- **Output:**

```json
{
  "status": "success",
  "message": "Password reset link has been sent to your email."
}
```

### Reset Password

- **Route:** `POST /api/auth/reset-password`
- **Input:**

```json
{
  "token": "string",
  "new_password": "string",
  "new_password_confirm": "string"
}
```

- **Output:**

```json
{ "status": "success", "message": "Password has been successfully reset." }
```

### OAuth (Google/GitHub)

- **Route:** `GET /api/auth/google`, `GET /api/auth/github`
- **Callback:** `/api/auth/google/callback`, `/api/auth/github/callback`
- **Output:**
  JSON with token or redirect
