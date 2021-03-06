#![allow(unused)]

use std::{fmt, collections::HashMap};
use anyhow::{Result, Context};
use uuid::Uuid;
use redis::Commands;
use stdx::SepBy;

#[derive(Clone)]
pub struct Chat {
    redis: redis::Client,
}

#[derive(Clone)]
pub struct MessageData {
    pub text: String,
    pub sender: String,
    pub receiver: String,
}

impl MessageData {
    pub fn new(text: String, sender: String, receiver: String) -> MessageData {
        MessageData { text, sender, receiver }
    }

    fn to_hash_map(self) -> [(&'static str, String); 3] {
        [("text", self.text), ("sender", self.sender), ("receiver", self.receiver)]
    }

    fn from_hash_map(mut map: HashMap<String, String>) -> MessageData {
        MessageData::new(
            map.remove("text").unwrap(),
            map.remove("sender").unwrap(),
            map.remove("receiver").unwrap(),
        )
    }
}

impl fmt::Display for MessageData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}: {}", self.sender, self.receiver, self.text)
    }
}

pub struct Message {
    pub id: Uuid,
    pub data: MessageData,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.data, f)
    }
}

impl Message {
    fn new(data: MessageData) -> Message {
        Self {
            data,
            id: uuid::Uuid::new_v4()
        }
    }
    fn with_id(id: &str, data: MessageData) -> Message {
        Self {
            data,
            id: uuid::Uuid::parse_str(id).unwrap(),
        }
    }
}


#[derive(Copy, Clone, Debug)]
pub enum UserRole {
    Admin,
    Regular,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            UserRole::Admin => "admin",
            UserRole::Regular => "regular user",
        })
    }
}

pub enum ChatEvent {
    UserLoggedIn(String),
    UserLoggedOut(String),
    SpamDetected(Message)
}

impl ChatEvent {
    fn publish_message(&self) -> impl fmt::Display + '_ {
        stdx::display_with(move |f| {
            match self {
                ChatEvent::UserLoggedIn(login) => {
                    write!(f, "logged_in {}", login)
                }
                ChatEvent::UserLoggedOut(login) => {
                    write!(f, "logged_out {}", login)
                }
                ChatEvent::SpamDetected(msg) => {
                    write!(f, "smap_detected {}", msg)
                }
            }
        })
    }
}

impl fmt::Display for ChatEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatEvent::UserLoggedIn(login) => {
                write!(f, "User {} has logged in", login)
            }
            ChatEvent::UserLoggedOut(login) => {
                write!(f, "User {} has logged out", login)
            }
            ChatEvent::SpamDetected(message) => {
                write!(f, "SPAM: {}", message)
            }
        }
    }

}

pub enum LoginResult {
    AlreadyLoggedIn,
    Success
}

pub struct UserMsgStat {
    login: String,
    n_messages: u32,
}
impl fmt::Display for UserMsgStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} messages: {}", self.login, self.n_messages)
    }
}

pub struct MsgsStat {
    enqd: Vec<MessageData>,
    checking_for_spam: Vec<MessageData>,
    spam: Vec<MessageData>,
    delivered: Vec<MessageData>,
}

impl fmt::Display for MsgsStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Enqueued:\n{}\n\
            Checking for spam:\n{}\n\
            Spam:\n{}\n\
            Delivered:\n{}\n",
            self.enqd.iter().sep_by("\n"),
            self.checking_for_spam.iter().sep_by("\n"),
            self.spam.iter().sep_by("\n"),
            self.delivered.iter().sep_by("\n"),
        )
    }
}


pub struct EventsIterable(redis::Connection);
impl EventsIterable {
    pub fn iter(&mut self) -> Result<EventsIterator<'_>> {
        let mut pubsub = self.0.as_pubsub();
        pubsub.subscribe(Chat::EVENT_JOURNAL_CHANNEL)?;
        Ok(EventsIterator(pubsub))
    }
}

pub struct EventsIterator<'a>(redis::PubSub<'a>);
impl Iterator for EventsIterator<'_> {
    type Item = Result<String>;

    fn next(&mut self) -> Option<Result<String>> {
        Some(self
            .0
            .get_message()
            .and_then(|msg| msg.get_payload::<String>())
            .context("Failed to get message")
        )
    }
}


impl Chat {
    const ONLINE_USERS_SET_KEY: &'static str = "users_online";
    const REGULAR_USERS_SET_KEY: &'static str = "regular_users";
    const ADMIN_USERS_SET_KEY: &'static str = "admin_users";
    const EVENT_JOURNAL_CHANNEL: &'static str = "event_journal";

    const USERS_BY_DELIVERED_MSG_COUNT_ZLIST_KEY: &'static str = "users_by_delivered_msg";
    const USERS_BY_SPAM_MSG_COUNT_ZLIST_KEY: &'static str = "users_by_spam_msg";

    const MESSAGES_QUE_LIST_KEY: &'static str = "messages_que";
    const MESSAGES_QUE_SET_KEY: &'static str = "messages:que";
    const MESSAGES_CHECKING_FOR_SPAM_SET_KEY: &'static str = "messages:checking_for_spam";
    const MESSAGES_SPAM_SET_KEY: &'static str = "messages:spam";
    const MESSAGES_SENT_SET_KEY: &'static str = "messages:sent";
    const MESSAGES_DELIVERD_SET_KEY: &'static str = "messages:delivered";

    fn outcomming_messages_set_key_for_user(user_login: &str) -> String {
        format!("outcomming_messages:{}", user_login)
    }

    fn incomming_messages_list_key_for_user(user_login: &str) -> String {
        format!("incomming_messages:{}", user_login)
    }

    fn message_hash_map_key(msg_id: impl std::fmt::Display) -> String {
        format!("message:{}", msg_id)
    }


    pub fn new(url: &str) -> Result<Chat> {
        Ok(Chat {
            redis: redis::Client::open(url).context("Failed to create chat")?
        })
    }

    pub fn login(&mut self, user_login: &str, role: UserRole) -> Result<LoginResult> {
        let event = ChatEvent::UserLoggedIn(user_login.to_owned());

        let (n_added,): (u32,) = redis::pipe()
            .atomic()
            .sadd(Chat::role_to_users_by_role_set_key(role), user_login).ignore()
            .sadd(Chat::ONLINE_USERS_SET_KEY, user_login)
            .publish(
                Chat::EVENT_JOURNAL_CHANNEL,
                event.publish_message().to_string()
            )
            .ignore()
            .query(&mut self.redis)?;

        Ok(match n_added {
            0 => LoginResult::AlreadyLoggedIn,
            1 => LoginResult::Success,
            it => unreachable!("Invalid users by role sets: unexpected amount added {:?}", it)
        })
    }

    fn role_to_users_by_role_set_key(role: UserRole) -> &'static str {
        match role {
            UserRole::Admin => Chat::ADMIN_USERS_SET_KEY,
            UserRole::Regular => Chat::REGULAR_USERS_SET_KEY,
        }
    }

    pub fn logout(&mut self, user_login: &str) -> Result<bool> {
        let event = ChatEvent::UserLoggedOut(user_login.to_owned());

        let (regular_removed, admins_removed): (u32, u32) = redis::pipe()
            .atomic()
            .srem(Chat::REGULAR_USERS_SET_KEY, user_login)
            .srem(Chat::ADMIN_USERS_SET_KEY, user_login)
            .srem(Chat::ONLINE_USERS_SET_KEY, user_login).ignore()
            .publish(
                Chat::EVENT_JOURNAL_CHANNEL,
                event.publish_message().to_string()
            )
            .ignore()
            .query(&mut self.redis)?;

        Ok(match (regular_removed, admins_removed) {
            (0, 0) => false,
            (1, 0) | (0, 1) => true,
            it => unreachable!("Invalid users by role sets: unexpected amount removed: {:?}", it)
        })
    }

    pub fn send_message(&mut self, msg_data: MessageData) -> Result<Message> {
        let msg = Message::new(msg_data);
        let id = msg.id.to_string();

        let sender_outcomming_set_key = Chat::outcomming_messages_set_key_for_user(&msg.data.sender);

        let _: () = redis::pipe()
            .atomic()
            .hset_multiple(Chat::message_hash_map_key(msg.id), &msg.data.clone().to_hash_map())
            .rpush(Chat::MESSAGES_QUE_LIST_KEY, &id).ignore()
            .sadd(Chat::MESSAGES_QUE_SET_KEY, &id).ignore()
            .sadd(sender_outcomming_set_key, &id).ignore()
            .query(&mut self.redis)?;

        Ok(msg)
    }

    pub fn view_messages(&mut self, receiver_login: &str) -> Result<Vec<MessageData>> {
        let list_key = Chat::incomming_messages_list_key_for_user(receiver_login);
        let msg_ids: Vec<String> = self.redis.lrange(list_key, 0, -1)?;
        let msg_hash_maps: Vec<HashMap<String, String>> = msg_ids
            .iter()
            .fold(redis::pipe().atomic(), |pipe, msg_id| {
                pipe.hgetall(Chat::message_hash_map_key(msg_id))
            })
            .query(&mut self.redis)?;

        Ok(msg_hash_maps.into_iter().map(MessageData::from_hash_map).collect())
    }

    pub fn events_stream(&mut self) -> Result<EventsIterable> {
        self.redis.get_connection().map(EventsIterable).context("Failed to get connection")
    }

    pub fn users_online(&mut self) -> Result<Vec<String>> {
        self.redis.smembers(Chat::ONLINE_USERS_SET_KEY).context("SMEMBERS failed")
    }

    fn top_n_users_by_messages(&mut self, n: u32, zlist_key: &str) -> Result<Vec<UserMsgStat>> {
        if n == 0 {
            return Ok(vec![]);
        }

        let stat: Vec<(String, u32)> = self.redis.zrevrange_withscores(
            zlist_key,
            0,
            (n as isize) - 1
        )?;

        let res = stat
            .into_iter()
            .map(|(login, n_messages)| UserMsgStat { login, n_messages })
            .collect();

        Ok(res)
    }

    pub fn top_n_most_chatty_users(&mut self, n: u32) -> Result<Vec<UserMsgStat>> {
        self.top_n_users_by_messages(n, Chat::USERS_BY_DELIVERED_MSG_COUNT_ZLIST_KEY)
    }

    pub fn top_n_most_spammy_users(&mut self, n: u32) -> Result<Vec<UserMsgStat>> {
        self.top_n_users_by_messages(n, Chat::USERS_BY_SPAM_MSG_COUNT_ZLIST_KEY)
    }

    pub fn outcomming_messages_stat(&mut self, sender_login: &str) -> Result<MsgsStat> {
        let status_set_keys = [
            Chat::MESSAGES_QUE_SET_KEY,
            Chat::MESSAGES_CHECKING_FOR_SPAM_SET_KEY,
            Chat::MESSAGES_SPAM_SET_KEY,
            Chat::MESSAGES_DELIVERD_SET_KEY,
        ];

        let outcomming_msgs_key = Chat::outcomming_messages_set_key_for_user(sender_login);

        let ids_per_status: Vec<Vec<String>> = status_set_keys
            .iter()
            .fold(redis::pipe().atomic(), |pipe, &status_set_key| {
                pipe.sinter((&outcomming_msgs_key, status_set_key))
            })
            .query(&mut self.redis)?;


        let mut msgs: Vec<HashMap<String, String>> = ids_per_status
            .iter()
            .flatten()
            .fold(redis::pipe().atomic(), |pipe, id| {
                pipe.hgetall(Chat::message_hash_map_key(id))
            })
            .query(&mut self.redis)?;

        let mut msgs = msgs.drain(..);

        let mut vecs = ids_per_status
            .iter()
            .map(|ids_vec| ids_vec
                .iter()
                .map(|_|MessageData::from_hash_map(msgs.next().unwrap()))
                .collect()
            );

        let mut next = || vecs.next().unwrap();

        let enqd = next();
        let checking_for_spam = next();
        let spam = next();
        let delivered = next();

        Ok(MsgsStat { enqd, checking_for_spam, spam, delivered })
    }

    pub fn check_msg_for_spam(&mut self) -> Result<Option<bool>> {
        let msg_id: Option<String> = self.redis
            .lpop(Chat::MESSAGES_QUE_LIST_KEY)?;

        let msg_id = match msg_id {
            Some(it) => it,
            None => return Ok(None)
        };

        let (msg_hash,): (HashMap<String, String>,) = redis::pipe()
            .atomic()
            .srem(Chat::MESSAGES_QUE_SET_KEY, &msg_id).ignore()
            .sadd(Chat::MESSAGES_CHECKING_FOR_SPAM_SET_KEY, &msg_id).ignore()
            .hgetall(Chat::message_hash_map_key(&msg_id))
            .query(&mut self.redis)?;

        let msg = MessageData::from_hash_map(msg_hash);

        let is_spam: bool = rand::random();

        if is_spam {
            redis::pipe()
                .atomic()
                .srem(Chat::MESSAGES_CHECKING_FOR_SPAM_SET_KEY, &msg_id)
                .sadd(Chat::MESSAGES_SPAM_SET_KEY, &msg_id)
                .zincr(Chat::USERS_BY_SPAM_MSG_COUNT_ZLIST_KEY, &msg.sender, 1)
                .publish(
                    Chat::EVENT_JOURNAL_CHANNEL,
                    ChatEvent::SpamDetected(Message::with_id(&msg_id, msg)).to_string()
                )
                .query(&mut self.redis)?;
        } else {
            redis::pipe()
                .atomic()
                .srem(Chat::MESSAGES_CHECKING_FOR_SPAM_SET_KEY, &msg_id)
                .sadd(Chat::MESSAGES_DELIVERD_SET_KEY, &msg_id)
                .zincr(Chat::USERS_BY_DELIVERED_MSG_COUNT_ZLIST_KEY, &msg.sender, 1)
                .lpush(Chat::incomming_messages_list_key_for_user(&msg.receiver), &msg_id)
                .query(&mut self.redis)?;
        }

        Ok(Some(is_spam))
    }

}
