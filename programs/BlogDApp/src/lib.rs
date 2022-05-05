use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod blog_d_app {
    use super::*;

    //-----------------------------------------------------//
    //                     HANDLE USERS                   //
    //---------------------------------------------------//
    /*
     * Initialize the blog account with the current_post_key and authority as blog state
     */
    pub fn init_blog(ctx: Context<InitBlog>) -> ProgramResult {
        //get accounts from context
        let blog_account = &mut ctx.accounts.blog_account;
        let genesis_post_account = &mut ctx.accounts.genesis_post_account;
        let authority = &mut ctx.accounts.authority;

        // set the blog state
        blog_account.authority = authority.key();
        blog_account.current_post_key = genesis_post_account.key();

        Ok(())
    }

    pub fn signup_user(ctx: Context<SignupUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        user_account.name = name;
        user_account.avatar = avatar;
        user_account.authority = authority.key();

        Ok(())
    }

    pub fn update_user(ctx: Context<UpdateUser>, name: String, avatar: String) -> ProgramResult {
        let user_account = &mut ctx.accounts.user_account;

        user_account.name = name;
        user_account.avatar = avatar;

        Ok(())
    }

    //----------------------------------------------------//
    //                     CRUD FUNCTIONS                //
    //--------------------------------------------------//
    pub fn create_post(ctx: Context<CreatePost>, title: String, content: String) -> ProgramResult {
        let blog_account = &mut ctx.accounts.blog_account;
        let post_account = &mut ctx.accounts.post_account;
        let user_account = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;

        post_account.title = title;
        post_account.content = content;
        post_account.user = user_account.key();
        post_account.authority = authority.key();
        post_account.pre_post_key = blog_account.current_post_key;

        // Store created post id as the current post id in the blog account
        blog_account.current_post_key = post_account.key();

        // EMIT post created event
        emit!(PostEvent {
            label: "CREATE".to_string(),
            post_id: post_account.key(),
            next_post_id: None, // Same as null
        });

        Ok(())
    }
}

//----------------------------------------------------//
//                     USER STRUCTS                //
//--------------------------------------------------//
#[derive(Accounts)]
pub struct InitBlog<'info> {
    // Create blog_account
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32)]
    pub blog_account: Account<'info, BlogState>,

    // Create genisis_post_account -- initialize blog a/c with first post to create a linked list.
    #[account(init, payer = authority, space = 8 + 32 + 32 + 32 + 32 + 8)]
    pub genesis_post_account: Account<'info, PostState>,

    // Authority is the program signer - creater of the blog - rent payer
    pub authority: Signer<'info>,

    // system program is required by runtime for creating the account
    pub system_program: Program<'info, System>,
}

// from pseudo blog state
#[account]
pub struct BlogState {
    pub current_post_key: Pubkey,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct SignupUser<'info> {
    #[account(init, payer = authority, space = 8 + 40 + 120 + 32)]
    pub user_account: Account<'info, UserState>,

    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserState {
    pub name: String,
    pub avatar: String,
    pub authority: Pubkey,
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(
        mut,
        has_one = authority,
    )]
    pub user_account: Account<'info, UserState>,
    pub authority: Signer<'info>,
}

//----------------------------------------------------//
//                     CRUD STRUCTS                  //
//--------------------------------------------------//
#[derive(Accounts)]
pub struct CreatePost<'info> {
    // TODO: Check how to correctly use mut in this instance
    #[account(init, payer = authority, space = 8 + 50 + 500 + 32 + 32 + 32)]
    pub post_account: Account<'info, PostState>,

    #[account(mut, has_one = authority)]
    pub user_account: Account<'info, UserState>,

    #[account(mut)]
    pub blog_account: Account<'info, BlogState>,

    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct PostState {
    title: String,
    content: String,
    user: Pubkey,
    pub pre_post_key: Pubkey,
    pub authority: Pubkey,
}

//----------------------------------------------------//
//                     EVENTS                        //
//--------------------------------------------------//
#[event]
pub struct PostEvent {
    pub label: String, // 'CREATE', 'UPDATE' or 'DELETE'
    pub post_id: Pubkey,
    pub next_post_id: Option<Pubkey>,
}
