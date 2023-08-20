## 詰まったところ

マイグレーション元のSQLを以下のようにしていたので、
```sql
CREATE TABLE users (
    id   CHAR(36) PRIMARY KEY,
    name TEXT
);
```

Dieselが生成するスキーマ定義は以下のようになる。
```rust
table! {
    users (id) {
        id -> Bpchar,
        name -> Nullable<Text>,
    }
}
```

注意したいのはnameの型がNullable<Text>になっていること。
これは、SQLのTEXT型がRustのString型にマッピングされるためである。
Dieselでは、NULLを許容するカラムはOption<T>で表現しなければならないが、
参考にしたサイト上ではそのような記述がなく引っかかった。

### 解決策
nameでNullを許容しないようにSQLを修正する。
```sql
CREATE TABLE users (
    id   CHAR(36) PRIMARY KEY,
    name TEXT NOT NULL
);
```

### 追伸
この問題は参考にしていたサイトが利用していたDieselのバージョンより
新しいバージョンで私は実装していたために発生した問題である。