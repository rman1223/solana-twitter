use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("BdoQ1KTMk4w9vAwX3JMumqAeuoUsJvYHT6qV2YtvXPKH");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult {
        let tweet = &mut ctx.accounts.tweet;
        let author = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        if topic.chars().count() > MAX_TOPIC_CHARS {
            return Err(ErrorCode::TopicTooLong.into());
        }

        if content.chars().count() > MAX_CONTENT_CHARS {
            return Err(ErrorCode::ContentTooLong.into());
        }

        tweet.author = *author.key;
        tweet.timestamp = clock.unix_timestamp;
        tweet.topic = topic;
        tweet.content = content;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SendTweet<'info> {
    #[account(init, payer = author, space = Tweet::LEN)]
    pub tweet: Account<'info, Tweet>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct Tweet {
    pub author: Pubkey,
    pub timestamp: i64,
    pub topic: String,
    pub content: String,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of a string/vector. It happens all the time, Rust must know when a Vector "ends"
const MAX_TOPIC_CHARS: usize = 50;
const MAX_TOPIC_LENGTH: usize = MAX_TOPIC_CHARS * 4; // 50 chars max. A char can use from 1 to 4 bytes.
const MAX_CONTENT_CHARS: usize = 280;
const MAX_CONTENT_LENGTH: usize = MAX_CONTENT_CHARS * 4; // 280 chars max.

impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content.
}

#[error]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}
