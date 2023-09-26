# Initialize DB
options = rocksdb.Options(create_if_missing=True)
db = rocksdb.DB("test.db", options)

# Insert data
db.put(b"key1", b"value1")

# Read data
value = db.get(b"key1")
print(value.decode("utf-8"))
