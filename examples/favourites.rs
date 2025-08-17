use anchor_lang::prelude::*;

declare_id!("2mJkhHAUevG3cNztjyh681AfKeiiJgaj8jeinJBUVeWj");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourites(
        context: Context<SetFavourites>,
        number: u64,
        colour: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Hello from {}", context.program_id);
        let user_public_key = context.accounts.user.key();
        msg!("User {user_public_key}'s favourite colour is {colour}");
        msg!("User {user_public_key}'s favourite number is {number}");
        msg!("User {user_public_key}'s favourite hobbies are {hobbies:?}");

        context.accounts.favourites.set_inner(Favourites {
            number,
            colour,
            hobbies,
        });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,

    #[max_len(50)]
    pub colour: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    // create account if it doesnt exist
    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favourites", user.key().as_ref()],
        bump
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}
