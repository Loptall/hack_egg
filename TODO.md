# TODO

## Majar dependencies

- reqwest
- serde

## Concept

- query
  - Tweets
    - Tweet LookUp
    - Search Tweets
    - Timelines
    - Sampled stream
    - Filtered stream
    - Hide replies
    - Batch Tweet compliance lookup
  - Users
    - Follows lookup

## Choose crates

### serde for url queries

- serde_urlencoded
- serde_qs

these crates does not provide the easy way to type declare parse (or cannot find)

I need a crate, that...

```rs
struct Param {
    publish: bool,
    id: String,
    author_info: AuthorInfo
}

struct AuthorInfo {
    name: String,
    age: u8,
}

let param = Param {
    publish: true,
    id: "18914798231",
    author_info: AuthorInfo {
        name: "Scott",
        age: 17
    }
};

let url = parse(param)?;
dbg!(url);
```

```ignore
{path}?publish&id=18914798231&author.info.name=Scott&author.info.age=17
```

I suggest that use a proc macro,

```rs
use unaque::Unaque;

#[derive(Unaque)]
struct Param {
    publish: bool,
    id: String,
    #[unaqueq(rename = "author.info")]
    author_info: AuthorInfo
}

struct AuthorInfo {
    name: String,
    age: u8,
}

let param = Param {
    publish: true,
    id: "18914798231",
    author_info: AuthorInfo {
        name: "Scott",
        age: 17
    }
};

let url = unaque::to_string(param)?;
dbg!(url);
```
