# Test 1: Check there's an index.html to take a picture of
GET http://127.0.0.1:3000/index.html

HTTP 200

# Test 2: Storing John Doe's wishlist
POST http://127.0.0.1:3000/api/wishlists
{
    "name": "John Doe23",
    "items": [
        "Cozy Gloves",
        "Purple Beanie"
    ]
}

HTTP 201

# Test 3: Retrieve all wishlists
GET http://127.0.0.1:3000/api/wishlists
HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$" count == 2
jsonpath "$[0].name" == "John Doe23"
jsonpath "$[0].items" count == 2
jsonpath "$[0].items[0]" == "Cozy Gloves"
jsonpath "$[0].items[1]" ==  "Purple Beanie"
