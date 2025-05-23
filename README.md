# Text Paraphrasing API

A Rust-based REST API service that provides text paraphrasing functionality using the Mistral AI API, with MongoDB storage for paraphrasing history.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- MongoDB Access
- Mistral AI API Key

## Environment Setup

1. Copy the `.env.example` file to create your own `.env` file:

```bash
cp .env.example .env
```

2. Update the `.env` file with your actual credentials:

```env
MONGODB_URI=your_mongodb_connection_string
DATABASE_NAME=ParaphrasingDB
MISTRAL_API_KEY=your_mistral_api_key
JWT_SECRET=your_jwt_secret_key
PORT=8080
```

Note: The `.env` file is ignored by Git to keep your credentials secure. Never commit sensitive credentials to version control.

## Running the Server

1. Clone the repository
2. Navigate to the project directory
3. Install dependencies and run the server:

```powershell
cd paraphrasing_api
cargo run
```

The server will start on `http://localhost:8080`

## API Endpoints

### 1. Paraphrase Text

Paraphrases the provided text while maintaining its original meaning and stores it in the database.

- **URL**: `/api/paraphrase`
- **Method**: `POST`
- **Content-Type**: `application/json`

**Request Body**:

```json
{
  "text": "Text to be paraphrased"
}
```

**Success Response**:

- **Code**: 200 OK
- **Content**:

```json
{
  "paraphrased_text": "The paraphrased version of the text"
}
```

**Error Response**:

- **Code**: 500 Internal Server Error
- **Content**:

```json
{
  "error": "Error message description"
}
```

### 2. Get Paraphrasing History

Retrieves all previously paraphrased texts from the database.

- **URL**: `/api/history`
- **Method**: `GET`

**Success Response**:

- **Code**: 200 OK
- **Content**:

```json
[
  {
    "_id": "...",
    "original_text": "Original text that was paraphrased",
    "paraphrased_text": "The paraphrased version of the text",
    "created_at": "2025-05-11T19:20:30.123Z"
  }
]
```

**Error Response**:

- **Code**: 500 Internal Server Error
- **Content**:

```json
{
  "error": "Database error message"
}
```

## CORS Configuration

The API is configured to allow cross-origin requests with the following settings:

- All origins are allowed (\*)
- Allowed HTTP methods: GET, POST, PUT, DELETE
- All headers are allowed
- Credentials are supported
- CORS preflight requests are cached for 1 hour

This means you can connect to the API from any frontend application, regardless of its domain.

## Testing the API

### Using PowerShell

```powershell
# 1. Test the paraphrase endpoint
$body = @{
    text = "Hello world, this is a test message."
} | ConvertTo-Json

Invoke-RestMethod -Uri 'http://localhost:8080/api/paraphrase' `
    -Method Post `
    -Body $body `
    -ContentType 'application/json'

# 2. Get paraphrasing history
Invoke-RestMethod -Uri 'http://localhost:8080/api/history' -Method Get
```

### Using curl

```bash
# 1. Paraphrase text
curl -X POST http://localhost:8080/api/paraphrase \
    -H "Content-Type: application/json" \
    -d "{\"text\": \"Hello world, this is a test message.\"}"

# 2. Get history
curl http://localhost:8080/api/history
```

### Using Python with requests

```python
import requests
import json

# 1. Paraphrase text
url = "http://localhost:8080/api/paraphrase"
payload = {
    "text": "Hello world, this is a test message."
}
headers = {
    "Content-Type": "application/json"
}

response = requests.post(url, json=payload, headers=headers)
print("Paraphrase response:", json.dumps(response.json(), indent=2))

# 2. Get history
history_response = requests.get("http://localhost:8080/api/history")
print("History:", json.dumps(history_response.json(), indent=2))
```

### Using Node.js with fetch

```javascript
// 1. Paraphrase text
const response = await fetch("http://localhost:8080/api/paraphrase", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
  },
  body: JSON.stringify({
    text: "Hello world, this is a test message.",
  }),
});

const data = await response.json();
console.log("Paraphrase response:", data);

// 2. Get history
const historyResponse = await fetch("http://localhost:8080/api/history");
const history = await historyResponse.json();
console.log("History:", history);
```

## Running Tests

To run the automated tests:

```powershell
cargo test
```

This will run all unit tests including the paraphrase service and handler tests.

## Error Handling

The API uses standard HTTP status codes:

- 200: Successful request
- 400: Bad request (invalid input)
- 500: Server error (API issues, database connection problems, etc.)

## Security

- The API uses CORS to handle cross-origin requests
- Environment variables are used for sensitive data
- Input validation is performed on all requests

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a new Pull Request
