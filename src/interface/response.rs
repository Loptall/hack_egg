use chrono::{DateTime, Utc};
use geojson::GeoJson;
// list of object used in responced json
use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(transparent)]
// pub struct Tweets(Vec<Tweet>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Tweet {
    id: String,
    text: String,
    created_at: Option<DateTime<Utc>>,
    author_id: Option<String>,
    conversation_id: Option<String>,
    in_reply_to_user_id: Option<String>,
    referenced_tweets: Option<Vec<ReferencedTweet>>,
    attachments: Option<Attachment>,
    geo: Option<Geo>,
    context_annotations: Option<Vec<ContextAnnotation>>,
    entities: Option<TweetEntities>,
    withheld: Option<TweetWithheld>,
    public_metrics: Option<TweetPublicMetrics>,
    non_public_metrics: Option<TweetNonPublicMetrics>,
    organic_metrics: Option<TweetOrganicMetrics>,
    promoted_metrics: Option<TweetPromotedMetrics>,
    possibly_sensitive: Option<bool>,
    lang: Option<String>,
    reply_settings: Option<String>,
    source: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Includes {
    tweets: Option<Vec<Tweet>>,
    users: Option<Vec<User>>,
    places: Option<Vec<Place>>,
    media: Option<Vec<Media>>,
    polls: Option<Vec<Poll>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    referenced_tweet_type: Option<ReferencedTweetType>,
    id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferencedTweetType {
    Retweeted,
    Quoted,
    RepliedTo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    media_keys: Option<Vec<String>>,
    poll_ids: Option<Vec<String>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Geo {
    coordinates: Coordinates,
    place_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates {
    coordinates_type: Option<String>,
    coordinates: Option<(f64, f64)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextAnnotation {
    domain: Option<Domain>,
    entity: Option<Entity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetEntities {
    annotaions: Option<Vec<Annotation>>,
    urls: Option<Vec<TweetUrl>>,
    hashtags: Option<Vec<Hashtag>>,
    mentions: Option<Vec<Mention>>,
    cashtags: Option<Vec<Cashtag>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Annotation {
    start: Option<i32>,
    end: Option<i32>,
    probability: Option<i32>, // float?
    #[serde(rename = "type")]
    annotation_type: Option<String>,
    normalized_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetUrl {
    start: Option<i32>,
    end: Option<i32>,
    url: Option<String>,
    expanded_url: Option<String>,
    display_url: Option<String>,
    unwound_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hashtag {
    start: Option<i32>,
    end: Option<i32>,
    tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Mention {
    start: Option<i32>,
    end: Option<i32>,
    username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cashtag {
    start: Option<i32>,
    end: Option<i32>,
    tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetWithheld {
    copyright: Option<bool>,
    country_codes: Option<Vec<String>>,
    scope: Option<Scope>,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Tweet,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetPublicMetrics {
    retweet_count: Option<i32>,
    reply_count: Option<i32>,
    like_count: Option<i32>,
    quote_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetNonPublicMetrics {
    impression_count: Option<i32>,
    url_link_clicks: Option<i32>,
    user_profile_clicks: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetOrganicMetrics {
    impression_count: Option<i32>,
    url_link_clicks: Option<i32>,
    user_profile_clicks: Option<i32>,
    retweet_count: Option<i32>,
    reply_count: Option<i32>,
    like_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetPromotedMetrics {
    impression_count: Option<i32>,
    url_link_clicks: Option<i32>,
    user_profile_clicks: Option<i32>,
    retweet_count: Option<i32>,
    reply_count: Option<i32>,
    like_count: Option<i32>,
}

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(transparent)]
// pub struct Users(Vec<User>);

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    username: String,
    created_at: Option<DateTime<Utc>>,
    description: Option<String>,
    // entities: Option<>
    location: Option<String>,
    url: Option<String>,
    entities: Option<UserEntities>,
    public_metrics: Option<UserPublicMetrics>,
    pinned_tweet_id: Option<String>,
    profile_image_url: Option<String>,
    protected: Option<bool>,
    verified: Option<bool>,
    withheld: Option<UserWithheld>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserWithheld {
    country_codes: Option<Vec<String>>,
    scope: Option<Scope>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEntities {
    // WIP
    url: Option<Vec<UserUrls>>,
    description: Option<Vec<Description>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUrls {
    urls: Vec<UserUrl>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUrl {
    start: Option<i32>,
    end: Option<i32>,
    url: Option<String>,
    expanded_url: Option<String>,
    display_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    urls: Option<Vec<UserUrl>>,
    hashtags: Option<Vec<Hashtag>>,
    mentions: Option<Vec<Mention>>,
    cashtags: Option<Vec<Cashtag>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPublicMetrics {
    followers_count: Option<i32>,
    following_count: Option<i32>,
    tweet_count: Option<i32>,
    listed_count: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    media_key: String,
    #[serde(rename = "type")]
    media_type: String,
    duration_ms: Option<i32>,
    height: Option<i32>,
    non_public_metrics: Option<MediaNonPublicMetrics>,
    organic_metrics: Option<MediaOrganicMetrics>,
    preview_image_url: Option<String>,
    promoted_metrics: Option<MediaPromotedMetrics>,
    public_metrics: Option<MediaPublicMetrics>,
    width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaPublicMetrics {
    view_count: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaNonPublicMetrics {
    playback_0_count: i32,
    playback_100_count: i32,
    playback_25_count: i32,
    playback_50_count: i32,
    playback_75_count: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaPromotedMetrics {
    view_count: i32,
    playback_0_count: i32,
    playback_100_count: i32,
    playback_25_count: i32,
    playback_50_count: i32,
    playback_75_count: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MediaOrganicMetrics {
    view_count: i32,
    playback_0_count: i32,
    playback_100_count: i32,
    playback_25_count: i32,
    playback_50_count: i32,
    playback_75_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    id: String,
    options: Vec<PollOption>,
    duration_minutes: Option<i32>,
    end_datetime: Option<DateTime<Utc>>,
    voting_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PollOption {
    position: i32,
    label: String,
    votes: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Place {
    full_name: String,
    id: String,
    contained_within: Option<Vec<String>>,
    country: Option<String>,
    country_code: Option<String>,
    geo: Option<GeoJson>,
    name: Option<String>,
    place_type: Option<String>,
}

mod test {
    #[test]
    fn parse_tweet() {
        use super::{Includes, Tweet};
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Serialize, Deserialize)]
        struct Responce {
            data: Tweet,
            includes: Includes,
        }
        let tweet: Responce =
            serde_json::from_str(include_str!("../../asset/example_payload/tweet.json"))
                .expect("failed to desirialize json file");
        dbg!("{}", tweet);
    }
    #[test]
    fn parse_tweets() {
        use super::{Includes, Tweet};
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Serialize, Deserialize)]
        struct Responce {
            data: Vec<Tweet>,
            includes: Includes,
        }
        let tweets: Responce =
            serde_json::from_str(include_str!("../../asset/example_payload/tweets.json"))
                .expect("failed to desirialize json file");
        dbg!("{}", tweets);
    }

    #[test]
    fn parse_user() {
        use super::{Includes, User};
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Serialize, Deserialize)]
        struct Responce {
            data: User,
            includes: Includes,
        }
        let tweets: Responce =
            serde_json::from_str(include_str!("../../asset/example_payload/user.json"))
                .expect("failed to desirialize json file");
        dbg!("{}", tweets);
    }

    #[test]
    fn parse_users() {
        use super::{Includes, User};
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Serialize, Deserialize)]
        struct Responce {
            data: Vec<User>,
            includes: Includes,
        }
        let tweets: Responce =
            serde_json::from_str(include_str!("../../asset/example_payload/users.json"))
                .expect("failed to desirialize json file");
        dbg!("{}", tweets);
    }
}
