use anchor_lang::prelude::*;

declare_id!("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

#[program]
pub mod votingapp {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,  
        poll_id: u64,
        description:String,
        poll_start:u64,
        poll_end:u64,) -> Result<()> {
            let poll=&mut ctx.accounts.poll;
            poll.poll_id=poll_id;
            poll.description=description;
            poll.poll_start=poll_start;
            poll.poll_end=poll_end;
            poll.candidate_amount=0;
        Ok(())
    } 
}

#[account]
pub struct Poll {
    pub poll_id: u64,
    pub description: String, 
    pub poll_start: u64,
    pub poll_end: u64,
    pub candidate_amount: u64,
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account(
        init,
        payer = signer,
        space = 8 + 8 + 128 + 8 + 8 + 8, // Adjusted space
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll: Account<'info, Poll>,

    pub system_program: Program<'info, System>
}


