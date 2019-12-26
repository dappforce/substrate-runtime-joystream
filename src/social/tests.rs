#![cfg(test)]
use super::mock::*;

use super::spaces::*;
use super::defaults::*;
use super::messages::*;

use runtime_io::with_externalities;
use srml_support::*;

const ACCOUNT1 : AccountId = 1;
const ACCOUNT2 : AccountId = 2;

fn space_slug() -> Vec<u8> {
  b"space_slug".to_vec()
}

fn space_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec()
}

fn space_update(writers: Option<Vec<AccountId>>, slug: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> SpaceUpdate<Test> {
  SpaceUpdate {
    writers,
    slug,
    ipfs_hash
  }
}

fn post_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW2CuDgwxkD4".to_vec()
}

fn post_update(space_id: Option<SpaceId>, ipfs_hash: Option<Vec<u8>>) -> PostUpdate<Test> {
  PostUpdate {
    space_id,
    ipfs_hash
  }
}

fn comment_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec()
}

fn subcomment_ipfs_hash() -> Vec<u8> {
  b"QmYA2fn8cMbVWo4v95RwcwJVyQsNtnEwHerfWR8UNtEwoE".to_vec()
}

fn comment_update(ipfs_hash: Vec<u8>) -> CommentUpdate {
  CommentUpdate {
    ipfs_hash
  }
}

fn alice_username() -> Vec<u8> {
  b"Alice".to_vec()
}
fn bob_username() -> Vec<u8> {
  b"Bob".to_vec()
}

fn profile_ipfs_hash() -> Vec<u8> {
  b"QmRAQB6YaCyidP37UdDnjFY5vQuiaRtqdyoW2CuDgwxkA5".to_vec()
}

fn reaction_upvote() -> ReactionKind {
  ReactionKind::Upvote
}
fn reaction_downvote() -> ReactionKind {
  ReactionKind::Downvote
}

fn scoring_action_upvote_post() -> ScoringAction {
  ScoringAction::UpvotePost
}
fn scoring_action_downvote_post() -> ScoringAction {
  ScoringAction::DownvotePost
}
fn scoring_action_share_post() -> ScoringAction {
  ScoringAction::SharePost
}
fn scoring_action_create_comment() -> ScoringAction {
  ScoringAction::CreateComment
}
fn scoring_action_upvote_comment() -> ScoringAction {
  ScoringAction::UpvoteComment
}
fn scoring_action_downvote_comment() -> ScoringAction {
  ScoringAction::DownvoteComment
}
fn scoring_action_share_comment() -> ScoringAction {
  ScoringAction::ShareComment
}
fn scoring_action_follow_space() -> ScoringAction {
  ScoringAction::FollowSpace
}
fn scoring_action_follow_account() -> ScoringAction {
  ScoringAction::FollowAccount
}

fn extension_regular_post() -> PostExtension<Test> {
  PostExtension::RegularPost
}
fn extension_shared_post(post_id: PostId) -> PostExtension<Test> {
  PostExtension::SharedPost(post_id)
}
fn extension_shared_comment(comment_id: CommentId) -> PostExtension<Test> {
  PostExtension::SharedComment(comment_id)
}

fn _create_default_space() -> dispatch::Result {
  _create_space(None, None, None)
}

fn _create_space(origin: Option<Origin>, slug: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> dispatch::Result {
  Spaces::create_space(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    slug.unwrap_or(self::space_slug()),
    ipfs_hash.unwrap_or(self::space_ipfs_hash())
  )
}

fn _update_space(origin: Option<Origin>, space_id: Option<u32>, update: Option<SpaceUpdate<Test>>) -> dispatch::Result {
  Spaces::update_space(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    space_id.unwrap_or(1),
    update.unwrap_or(self::space_update(None, None, None))
  )
}

fn _default_follow_space() -> dispatch::Result {
  _follow_space(None, None)
}

fn _follow_space(origin: Option<Origin>, space_id: Option<SpaceId>) -> dispatch::Result {
  Spaces::follow_space(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    space_id.unwrap_or(1)
  )
}

fn _default_unfollow_space() -> dispatch::Result {
  _unfollow_space(None, None)
}

fn _unfollow_space(origin: Option<Origin>, space_id: Option<SpaceId>) -> dispatch::Result {
  Spaces::unfollow_space(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    space_id.unwrap_or(1)
  )
}

fn _create_default_post() -> dispatch::Result {
  _create_post(None, None, None, None)
}

fn _create_post(origin: Option<Origin>, space_id: Option<SpaceId>, ipfs_hash: Option<Vec<u8>>, extension: Option<PostExtension<Test>>) -> dispatch::Result {
  Spaces::create_post(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    space_id.unwrap_or(1),
    ipfs_hash.unwrap_or(self::post_ipfs_hash()),
    extension.unwrap_or(self::extension_regular_post())
  )
}

fn _update_post(origin: Option<Origin>, post_id: Option<PostId>, update: Option<PostUpdate<Test>>) -> dispatch::Result {
  Spaces::update_post(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    update.unwrap_or(self::post_update(None, None))
  )
}

fn _create_default_comment() -> dispatch::Result {
  _create_comment(None, None, None, None)
}

fn _create_comment(origin: Option<Origin>, post_id: Option<PostId>, parent_id: Option<CommentId>, ipfs_hash: Option<Vec<u8>>) -> dispatch::Result {
  Spaces::create_comment(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    parent_id,
    ipfs_hash.unwrap_or(self::comment_ipfs_hash())
  )
}

fn _update_comment(origin: Option<Origin>, comment_id: Option<CommentId>, update: Option<CommentUpdate>) -> dispatch::Result {
  Spaces::update_comment(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    comment_id.unwrap_or(1),
    update.unwrap_or(self::comment_update(self::subcomment_ipfs_hash()))
  )
}

fn _create_default_post_reaction() -> dispatch::Result {
  _create_post_reaction(None, None, None)
}

fn _create_default_comment_reaction() -> dispatch::Result {
  _create_comment_reaction(None, None, None)
}

fn _create_post_reaction(origin: Option<Origin>, post_id: Option<PostId>, kind: Option<ReactionKind>) -> dispatch::Result {
  Spaces::create_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _create_comment_reaction(origin: Option<Origin>, comment_id: Option<CommentId>, kind: Option<ReactionKind>) -> dispatch::Result {
  Spaces::create_comment_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    comment_id.unwrap_or(1),
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _update_post_reaction(origin: Option<Origin>, post_id: Option<PostId>, reaction_id: ReactionId, kind: Option<ReactionKind>) -> dispatch::Result {
  Spaces::update_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    reaction_id,
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _update_comment_reaction(origin: Option<Origin>, comment_id: Option<CommentId>, reaction_id: ReactionId, kind: Option<ReactionKind>) -> dispatch::Result {
  Spaces::update_comment_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    comment_id.unwrap_or(1),
    reaction_id,
    kind.unwrap_or(self::reaction_upvote())
  )
}

fn _delete_post_reaction(origin: Option<Origin>, post_id: Option<PostId>, reaction_id: ReactionId) -> dispatch::Result {
  Spaces::delete_post_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    post_id.unwrap_or(1),
    reaction_id
  )
}

fn _delete_comment_reaction(origin: Option<Origin>, comment_id: Option<CommentId>, reaction_id: ReactionId) -> dispatch::Result {
  Spaces::delete_comment_reaction(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    comment_id.unwrap_or(1),
    reaction_id
  )
}

fn _create_default_profile() -> dispatch::Result {
  _create_profile(None, None, None)
}

fn _create_profile(origin: Option<Origin>, username: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> dispatch::Result {
  Spaces::create_profile(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    username.unwrap_or(self::alice_username()),
    ipfs_hash.unwrap_or(self::profile_ipfs_hash())
  )
}

fn _update_profile(origin: Option<Origin>, username: Option<Vec<u8>>, ipfs_hash: Option<Vec<u8>>) -> dispatch::Result {
  Spaces::update_profile(
    origin.unwrap_or(Origin::signed(ACCOUNT1)),
    ProfileUpdate {
      username,
      ipfs_hash
    }
  )
}

fn _default_follow_account() -> dispatch::Result {
  _follow_account(None, None)
}

fn _follow_account(origin: Option<Origin>, account: Option<AccountId>) -> dispatch::Result {
  Spaces::follow_account(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    account.unwrap_or(ACCOUNT1)
  )
}

fn _default_unfollow_account() -> dispatch::Result {
  _unfollow_account(None, None)
}

fn _unfollow_account(origin: Option<Origin>, account: Option<AccountId>) -> dispatch::Result {
  Spaces::unfollow_account(
    origin.unwrap_or(Origin::signed(ACCOUNT2)),
    account.unwrap_or(ACCOUNT1)
  )
}

fn _change_post_score_by_id(account: AccountId, post_id: PostId, action: ScoringAction) -> dispatch::Result {
  if let Some(ref mut post) = Spaces::post_by_id(post_id) {
    Spaces::change_post_score(account, post, action)
  } else {
    Err(MSG_POST_NOT_FOUND)
  }
}

fn _change_comment_score_by_id(account: AccountId, comment_id: CommentId, action: ScoringAction) -> dispatch::Result {
  if let Some(ref mut comment) = Spaces::comment_by_id(comment_id) {
    Spaces::change_comment_score(account, comment, action)
  } else {
    Err(MSG_COMMENT_NOT_FOUND)
  }
}

// Space tests
#[test]
fn create_space_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    // Check storages
    assert_eq!(Spaces::space_ids_by_owner(ACCOUNT1), vec![1]);
    assert_eq!(Spaces::space_id_by_slug(self::space_slug()), Some(1));
    assert_eq!(Spaces::next_space_id(), 2);

    // Check whether data stored correctly
    let space = Spaces::space_by_id(1).unwrap();

    assert_eq!(space.created.account, ACCOUNT1);
    assert_eq!(space.slug, self::space_slug());
    assert_eq!(space.ipfs_hash, self::space_ipfs_hash());
    assert!(space.writers.is_empty());
    assert_eq!(space.posts_count, 0);
    assert_eq!(space.followers_count, 1);
    assert!(space.edit_history.is_empty());
  });
}

#[test]
fn create_space_should_fail_short_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a space with too short slug
    assert_noop!(_create_space(None, Some(slug), None), MSG_SPACE_SLUG_IS_TOO_SHORT);
  });
}

#[test]
fn create_space_should_fail_long_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a space with too long slug
    assert_noop!(_create_space(None, Some(slug), None), MSG_SPACE_SLUG_IS_TOO_LONG);
  });
}

#[test]
fn create_space_should_fail_not_unique_slug() {

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    // Try to catch an error creating a space with not unique slug
    assert_noop!(_create_default_space(), MSG_SPACE_SLUG_IS_NOT_UNIQUE);
  });
}

#[test]
fn create_space_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a space with invalid ipfs_hash
    assert_noop!(_create_space(None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_space_should_work() {
  let slug : Vec<u8> = b"new_slug".to_vec();
  let ipfs_hash : Vec<u8> = b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW2CuDgwxkD4".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    // Space update with ID 1 should be fine
    assert_ok!(_update_space(None, None,
      Some(
        self::space_update(
          None,
          Some(slug.clone()),
          Some(ipfs_hash.clone())
        )
      )
    ));

    // Check whether space updates correctly
    let space = Spaces::space_by_id(1).unwrap();
    assert_eq!(space.slug, slug);
    assert_eq!(space.ipfs_hash, ipfs_hash);

    // Check whether history recorded correctly
    assert_eq!(space.edit_history[0].old_data.writers, None);
    assert_eq!(space.edit_history[0].old_data.slug, Some(self::space_slug()));
    assert_eq!(space.edit_history[0].old_data.ipfs_hash, Some(self::space_ipfs_hash()));
  });
}

#[test]
fn update_space_should_fail_nothing_to_update() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with no changes
    assert_noop!(_update_space(None, None, None), MSG_NOTHING_TO_UPDATE_IN_SPACE);
  });
}

#[test]
fn update_space_should_fail_space_not_found() {
  let slug : Vec<u8> = b"new_slug".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with wrong space ID
    assert_noop!(_update_space(None, Some(2),
      Some(
        self::space_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_SPACE_NOT_FOUND);
  });
}

#[test]
fn update_space_should_fail_not_an_owner() {
  let slug : Vec<u8> = b"new_slug".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with different account
    assert_noop!(_update_space(Some(Origin::signed(ACCOUNT2)), None,
      Some(
        self::space_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_ONLY_SPACE_OWNER_CAN_UPDATE_SPACE);
  });
}

#[test]
fn update_space_should_fail_short_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with too short slug
    assert_noop!(_update_space(None, None,
      Some(
        self::space_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_SPACE_SLUG_IS_TOO_SHORT);
  });
}

#[test]
fn update_space_should_fail_long_slug() {
  let slug : Vec<u8> = vec![97; (DEFAULT_SLUG_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with too long slug
    assert_noop!(_update_space(None, None,
      Some(
        self::space_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_SPACE_SLUG_IS_TOO_LONG);
  });
}

#[test]
fn update_space_should_fail_not_unique_slug() {
  let slug : Vec<u8> = b"unique_slug".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    
    assert_ok!(_create_space(
      None,
      Some(slug.clone()),
      None
    )); // SpaceId 2 with a custom slug
  
    // Try to catch an error updating a space on ID 1 with a slug of space on ID 2
    assert_noop!(_update_space(None, Some(1),
      Some(
        self::space_update(
          None, 
          Some(slug),
          None
        )
      )
    ), MSG_SPACE_SLUG_IS_NOT_UNIQUE);
  });
}

#[test]
fn update_space_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
  
    // Try to catch an error updating a space with invalid ipfs_hash
    assert_noop!(_update_space(None, None,
      Some(
        self::space_update(
          None, 
          None,
          Some(ipfs_hash)
        )
      )
    ), MSG_IPFS_IS_INCORRECT);
  });
}

// Post tests
#[test]
fn create_post_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Check storages
    assert_eq!(Spaces::post_ids_by_space_id(1), vec![1]);
    assert_eq!(Spaces::next_post_id(), 2);

    // Check whether data stored correctly
    let post = Spaces::post_by_id(1).unwrap();

    assert_eq!(post.space_id, 1);
    assert_eq!(post.created.account, ACCOUNT1);
    assert_eq!(post.ipfs_hash, self::post_ipfs_hash());
    assert_eq!(post.comments_count, 0);
    assert_eq!(post.upvotes_count, 0);
    assert_eq!(post.downvotes_count, 0);
    assert_eq!(post.shares_count, 0);
    assert_eq!(post.extension, self::extension_regular_post());
    assert!(post.edit_history.is_empty());
  });
}

#[test]
fn create_post_should_fail_space_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_create_default_post(), MSG_SPACE_NOT_FOUND);
  });
}

#[test]
fn create_post_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    // Try to catch an error creating a regular post with invalid ipfs_hash
    assert_noop!(_create_post(None, None, Some(ipfs_hash), None), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_post_should_work() {
  let ipfs_hash : Vec<u8> = b"QmRAQB6YaCyidP37UdDnjFY5vQuiBrcqdyoW1CuDgwxkD4".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Post update with ID 1 should be fine
    assert_ok!(_update_post(None, None,
      Some(
        self::post_update(
          None,
          Some(ipfs_hash.clone())
        )
      )
    ));

    
    // Check whether post updates correctly
    let post = Spaces::post_by_id(1).unwrap();
    assert_eq!(post.space_id, 1);
    assert_eq!(post.ipfs_hash, ipfs_hash);

    // Check whether history recorded correctly
    assert_eq!(post.edit_history[0].old_data.space_id, None);
    assert_eq!(post.edit_history[0].old_data.ipfs_hash, Some(self::post_ipfs_hash()));
  });
}

#[test]
fn update_post_should_fail_nothing_to_update() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with no changes
    assert_noop!(_update_post(None, None, None), MSG_NOTHING_TO_UPDATE_IN_POST);
  });
}

#[test]
fn update_post_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(None, Some(b"space2_slug".to_vec()), None)); // SpaceId 2
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with wrong post ID
    assert_noop!(_update_post(None, Some(2),
      Some(
        self::post_update(
          Some(2), 
          None
        )
      )
    ), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn update_post_should_fail_not_an_owner() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(None, Some(b"space2_slug".to_vec()), None)); // SpaceId 2
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with different account
    assert_noop!(_update_post(Some(Origin::signed(ACCOUNT2)), None,
      Some(
        self::post_update(
          Some(2), 
          None
        )
      )
    ), MSG_ONLY_POST_OWNER_CAN_UPDATE_POST);
  });
}

#[test]
fn update_post_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
  
    // Try to catch an error updating a post with invalid ipfs_hash
    assert_noop!(_update_post(None, None,
      Some(
        self::post_update(
          None,
          Some(ipfs_hash)
        )
      )
    ), MSG_IPFS_IS_INCORRECT);
  });
}

// Comment tests
#[test]
fn create_comment_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Check storages
    assert_eq!(Spaces::comment_ids_by_post_id(1), vec![1]);
    assert_eq!(Spaces::next_comment_id(), 2);
    assert_eq!(Spaces::post_by_id(1).unwrap().comments_count, 1);

    // Check whether data stored correctly
    let comment = Spaces::comment_by_id(1).unwrap();

    assert_eq!(comment.parent_id, None);
    assert_eq!(comment.post_id, 1);
    assert_eq!(comment.created.account, ACCOUNT1);
    assert_eq!(comment.ipfs_hash, self::comment_ipfs_hash());
    assert_eq!(comment.upvotes_count, 0);
    assert_eq!(comment.downvotes_count, 0);
    assert_eq!(comment.shares_count, 0);
    assert_eq!(comment.direct_replies_count, 0);
    assert!(comment.edit_history.is_empty());
  });
}

#[test]
fn create_comment_should_work_with_parent() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_comment(None, None, Some(1), None)); // CommentId 2 with parent CommentId 1

    // Check storages
    assert_eq!(Spaces::comment_ids_by_post_id(1), vec![1, 2]);
    assert_eq!(Spaces::next_comment_id(), 3);
    assert_eq!(Spaces::post_by_id(1).unwrap().comments_count, 2);

    // Check whether data stored correctly
    assert_eq!(Spaces::comment_by_id(2).unwrap().parent_id, Some(1));
    assert_eq!(Spaces::comment_by_id(1).unwrap().direct_replies_count, 1);
  });
}

#[test]
fn create_comment_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating a comment with wrong post
    assert_noop!(_create_default_comment(), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn create_comment_should_fail_parent_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a comment with wrong parent
    assert_noop!(_create_comment(None, None, Some(1), None), MSG_UNKNOWN_PARENT_COMMENT);
  });
}

#[test]
fn create_comment_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    // Try to catch an error creating a comment with wrong parent
    assert_noop!(_create_comment(None, None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_comment_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Post update with ID 1 should be fine
    assert_ok!(_update_comment(
      None,
      None,
      Some(self::comment_update(self::subcomment_ipfs_hash()))
    ));

    // Check whether post updates correctly
    let comment = Spaces::comment_by_id(1).unwrap();
    assert_eq!(comment.ipfs_hash, self::subcomment_ipfs_hash());

    // Check whether history recorded correctly
    assert_eq!(comment.edit_history[0].old_data.ipfs_hash, self::comment_ipfs_hash());
  });
}

#[test]
fn update_comment_should_fail_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error updating a comment with wrong CommentId
    assert_noop!(_update_comment(
      None,
      None,
      Some(self::comment_update(self::subcomment_ipfs_hash()))
    ),
    MSG_COMMENT_NOT_FOUND);
  });
}

#[test]
fn update_comment_should_fail_not_an_owner() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Try to catch an error updating a comment with wrong Account
    assert_noop!(_update_comment(
      Some(Origin::signed(2)),
      None,
      Some(self::comment_update(self::subcomment_ipfs_hash()))
    ),
    MSG_ONLY_COMMENT_AUTHOR_CAN_UPDATE_COMMENT);
  });
}

#[test]
fn update_comment_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Try to catch an error updating a comment with invalid ipfs_hash
    assert_noop!(_update_comment(
      None,
      None,
      Some(self::comment_update(ipfs_hash))
    ),
    MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn update_comment_should_fail_ipfs_hash_dont_differ() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1

    // Try to catch an error updating a comment with the same ipfs_hash
    assert_noop!(_update_comment(
      None,
      None,
      Some(self::comment_update(self::comment_ipfs_hash()))
    ),
    MSG_NEW_COMMENT_HASH_DO_NOT_DIFFER);
  });
}

// Reaction tests
#[test]
fn create_post_reaction_should_work_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::reaction_ids_by_post_id(1), vec![1]);
    assert_eq!(Spaces::next_reaction_id(), 2);

    // Check post reaction counters
    let post = Spaces::post_by_id(1).unwrap();
    assert_eq!(post.upvotes_count, 1);
    assert_eq!(post.downvotes_count, 0);

    // Check whether data stored correctly
    let reaction = Spaces::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_upvote());
  });
}

#[test]
fn create_post_reaction_should_work_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::reaction_ids_by_post_id(1), vec![1]);
    assert_eq!(Spaces::next_reaction_id(), 2);

    // Check post reaction counters
    let post = Spaces::post_by_id(1).unwrap();
    assert_eq!(post.upvotes_count, 0);
    assert_eq!(post.downvotes_count, 1);

    // Check whether data stored correctly
    let reaction = Spaces::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_downvote());
  });
}

#[test]
fn create_post_reaction_should_fail_already_reacted() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_post_reaction()); // ReactionId1 

    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_post_reaction(), MSG_ACCOUNT_ALREADY_REACTED_TO_POST);
  });
}

#[test]
fn create_post_reaction_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_post_reaction(), MSG_POST_NOT_FOUND);
  });
}

#[test]
fn create_comment_reaction_should_work_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_comment_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::reaction_ids_by_comment_id(1), vec![1]);
    assert_eq!(Spaces::next_reaction_id(), 2);

    // Check comment reaction counters
    let comment = Spaces::comment_by_id(1).unwrap();
    assert_eq!(comment.upvotes_count, 1);
    assert_eq!(comment.downvotes_count, 0);

    // Check whether data stored correctly
    let reaction = Spaces::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_upvote());
  });
}

#[test]
fn create_comment_reaction_should_work_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_comment_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::reaction_ids_by_comment_id(1), vec![1]);
    assert_eq!(Spaces::next_reaction_id(), 2);

    // Check comment reaction counters
    let comment = Spaces::comment_by_id(1).unwrap();
    assert_eq!(comment.upvotes_count, 0);
    assert_eq!(comment.downvotes_count, 1);

    // Check whether data stored correctly
    let reaction = Spaces::reaction_by_id(1).unwrap();
    assert_eq!(reaction.created.account, ACCOUNT2);
    assert_eq!(reaction.kind, self::reaction_downvote());
  });
}

#[test]
fn create_comment_reaction_should_fail_already_reacted() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_default_comment_reaction()); // ReactionId 1

    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_comment_reaction(), MSG_ACCOUNT_ALREADY_REACTED_TO_COMMENT);
  });
}

#[test]
fn create_comment_reaction_should_fail_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    // Try to catch an error creating reaction by the same account
    assert_noop!(_create_default_comment_reaction(), MSG_COMMENT_NOT_FOUND);
  });
}

// Rating system tests

#[test]
fn score_diff_by_weights_check_result() {
  with_externalities(&mut build_ext(), || {
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_upvote_post()), DEFAULT_UPVOTE_POST_ACTION_WEIGHT as i16);
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_downvote_post()), DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT as i16);
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_share_post()), DEFAULT_SHARE_POST_ACTION_WEIGHT as i16);
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_create_comment()), DEFAULT_CREATE_COMMENT_ACTION_WEIGHT as i16);
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_upvote_comment()), DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as i16);
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_downvote_comment()), DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT as i16);
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_share_comment()), DEFAULT_SHARE_COMMENT_ACTION_WEIGHT as i16);
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_follow_space()), DEFAULT_FOLLOW_SPACE_ACTION_WEIGHT as i16);
    assert_eq!(Spaces::get_score_diff(1, self::scoring_action_follow_account()), DEFAULT_FOLLOW_ACCOUNT_ACTION_WEIGHT as i16);
  });
}

#[test]
fn random_score_diff_check_result() {
  with_externalities(&mut build_ext(), || {
    assert_eq!(Spaces::get_score_diff(32768, self::scoring_action_upvote_post()), 80); // 2^15
    assert_eq!(Spaces::get_score_diff(32769, self::scoring_action_upvote_post()), 80); // 2^15 + 1
    assert_eq!(Spaces::get_score_diff(65535, self::scoring_action_upvote_post()), 80); // 2^16 - 1
    assert_eq!(Spaces::get_score_diff(65536, self::scoring_action_upvote_post()), 85); // 2^16
  });
}

//--------------------------------------------------------------------------------------------------

#[test]
fn change_space_score_should_work_follow_space() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    assert_ok!(Spaces::follow_space(Origin::signed(ACCOUNT2), 1));

    assert_eq!(Spaces::space_by_id(1).unwrap().score, DEFAULT_FOLLOW_SPACE_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_FOLLOW_SPACE_ACTION_WEIGHT as u32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
  });
}

#[test]
fn change_space_score_should_work_revert_follow_space() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    assert_ok!(Spaces::follow_space(Origin::signed(ACCOUNT2), 1));
    assert_ok!(Spaces::unfollow_space(Origin::signed(ACCOUNT2), 1));

    assert_eq!(Spaces::space_by_id(1).unwrap().score, 0);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
  });
}

#[test]
fn change_space_score_should_work_upvote_post() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post());
    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1

    assert_eq!(Spaces::space_by_id(1).unwrap().score, DEFAULT_UPVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_UPVOTE_POST_ACTION_WEIGHT as u32);
  });
}

#[test]
fn change_space_score_should_work_downvote_post() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post());
    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1
    
    assert_eq!(Spaces::space_by_id(1).unwrap().score, DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
  });
}

//--------------------------------------------------------------------------------------------------

#[test]
fn change_post_score_should_work_create_comment() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None)); // CommentId 1

    assert_eq!(Spaces::post_by_id(1).unwrap().score, DEFAULT_CREATE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::space_by_id(1).unwrap().score, DEFAULT_CREATE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_CREATE_COMMENT_ACTION_WEIGHT as u32);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_create_comment())), Some(DEFAULT_CREATE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_should_work_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None));

    assert_eq!(Spaces::post_by_id(1).unwrap().score, DEFAULT_UPVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_UPVOTE_POST_ACTION_WEIGHT as u32);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_upvote_post())), Some(DEFAULT_UPVOTE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_should_work_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote())));

    assert_eq!(Spaces::post_by_id(1).unwrap().score, DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_downvote_post())), Some(DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_should_revert_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1
    assert_ok!(_delete_post_reaction(Some(Origin::signed(ACCOUNT2)), None, 1));

    assert_eq!(Spaces::post_by_id(1).unwrap().score, 0);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_upvote_post())), None);
  });
}

#[test]
fn change_post_score_should_revert_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1
    assert_ok!(_delete_post_reaction(Some(Origin::signed(ACCOUNT2)), None, 1));

    assert_eq!(Spaces::post_by_id(1).unwrap().score, 0);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_downvote_post())), None);
  });
}

#[test]
fn change_post_score_cancel_upvote_with_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, None)); // ReactionId 1
    assert_ok!(_update_post_reaction(Some(Origin::signed(ACCOUNT2)), None, 1, Some(self::reaction_downvote())));

    assert_eq!(Spaces::post_by_id(1).unwrap().score, DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_upvote_post())), None);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_downvote_post())), Some(DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_cancel_downvote_with_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_default_post()); // PostId 1

    assert_ok!(_create_post_reaction(Some(Origin::signed(ACCOUNT2)), None, Some(self::reaction_downvote()))); // ReactionId 1
    assert_ok!(_update_post_reaction(Some(Origin::signed(ACCOUNT2)), None, 1, None));

    assert_eq!(Spaces::post_by_id(1).unwrap().score, DEFAULT_UPVOTE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_UPVOTE_POST_ACTION_WEIGHT as u32);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_downvote_post())), None);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_upvote_post())), Some(DEFAULT_UPVOTE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn change_post_score_should_fail_post_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_noop!(_change_post_score_by_id(ACCOUNT1, 1, self::scoring_action_upvote_post()), MSG_POST_NOT_FOUND);
  });
}

//--------------------------------------------------------------------------------------------------

#[test]
fn change_social_account_reputation_should_work_max_score_diff() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(Spaces::change_social_account_reputation(
      ACCOUNT2,
      ACCOUNT1,
      std::i16::MAX,
      self::scoring_action_follow_account())
    );
  });
}

#[test]
fn change_social_account_reputation_should_work_min_score_diff() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(Spaces::change_social_account_reputation(
      ACCOUNT2,
      ACCOUNT1,
      std::i16::MIN,
      self::scoring_action_follow_account())
    );
  });
}

#[test]
fn change_social_account_reputation_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(Spaces::change_social_account_reputation(
      ACCOUNT2,
      ACCOUNT1,
      DEFAULT_DOWNVOTE_POST_ACTION_WEIGHT,
      self::scoring_action_downvote_post())
    );
    assert_eq!(Spaces::account_reputation_diff_by_account((ACCOUNT1, ACCOUNT2, self::scoring_action_downvote_post())), Some(0));
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);

    assert_ok!(Spaces::change_social_account_reputation(
      ACCOUNT2,
      ACCOUNT1,
      DEFAULT_UPVOTE_POST_ACTION_WEIGHT * 2,
      self::scoring_action_upvote_post())
    );
    assert_eq!(Spaces::account_reputation_diff_by_account(
      (ACCOUNT1, ACCOUNT2, self::scoring_action_upvote_post())),
      Some(DEFAULT_UPVOTE_POST_ACTION_WEIGHT * 2)
    );
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1 + (DEFAULT_UPVOTE_POST_ACTION_WEIGHT * 2) as u32);
  });
}

//--------------------------------------------------------------------------------------------------

#[test]
fn change_comment_score_should_work_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_upvote_comment()));
    assert_eq!(Spaces::comment_by_id(1).unwrap().score, DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1 + DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as u32);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT1, 1, self::scoring_action_upvote_comment())), Some(DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_comment_score_should_work_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_downvote_comment()));
    assert_eq!(Spaces::comment_by_id(1).unwrap().score, DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT1, 1, self::scoring_action_downvote_comment())), Some(DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_comment_score_should_revert_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_upvote_comment()));
    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_upvote_comment()));
    assert_eq!(Spaces::comment_by_id(1).unwrap().score, 0);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT1, 1, self::scoring_action_upvote_comment())), None);
  });
}

#[test]
fn change_comment_score_should_revert_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_downvote_comment()));
    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_downvote_comment()));
    assert_eq!(Spaces::comment_by_id(1).unwrap().score, 0);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT1, 1, self::scoring_action_downvote_comment())), None);
  });
}

#[test]
fn change_comment_score_check_cancel_upvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None));

    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_upvote_comment()));

    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_downvote_comment()));
    assert_eq!(Spaces::comment_by_id(1).unwrap().score, DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT1, 1, self::scoring_action_upvote_comment())), None);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT1, 1, self::scoring_action_downvote_comment())), Some(DEFAULT_DOWNVOTE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_comment_score_check_cancel_downvote() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_post(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None));

    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_downvote_comment()));

    assert_ok!(_change_comment_score_by_id(ACCOUNT1, 1, self::scoring_action_upvote_comment()));
    assert_eq!(Spaces::comment_by_id(1).unwrap().score, DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT2).unwrap().reputation, 1 + DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT as u32);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT1, 1, self::scoring_action_downvote_comment())), None);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT1, 1, self::scoring_action_upvote_comment())), Some(DEFAULT_UPVOTE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn change_comment_score_should_fail_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space());
    assert_ok!(_create_default_post());
    assert_ok!(_create_comment(Some(Origin::signed(ACCOUNT2)), None, None, None));
    assert_noop!(_change_comment_score_by_id(ACCOUNT1, 2, self::scoring_action_upvote_comment()), MSG_COMMENT_NOT_FOUND);
  });
}

// Shares tests

#[test]
fn share_post_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), Some(b"space2_slug".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_post(1))
    )); // Share PostId 1 on SpaceId 2 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::post_ids_by_space_id(1), vec![1]);
    assert_eq!(Spaces::post_ids_by_space_id(2), vec![2]);
    assert_eq!(Spaces::next_post_id(), 3);

    assert_eq!(Spaces::post_shares_by_account((ACCOUNT2, 1)), 1);
    assert_eq!(Spaces::shared_post_ids_by_original_post_id(1), vec![2]);

    // Check whether data stored correctly
    assert_eq!(Spaces::post_by_id(1).unwrap().shares_count, 1);

    let shared_post = Spaces::post_by_id(2).unwrap();

    assert_eq!(shared_post.space_id, 2);
    assert_eq!(shared_post.created.account, ACCOUNT2);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_post(1));
  });
}

#[test]
fn share_post_should_work_share_own_post() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT1)),
      Some(1),
      Some(vec![]),
      Some(self::extension_shared_post(1))
    )); // Share PostId 1

    // Check storages
    assert_eq!(Spaces::post_ids_by_space_id(1), vec![1, 2]);
    assert_eq!(Spaces::next_post_id(), 3);

    assert_eq!(Spaces::post_shares_by_account((ACCOUNT1, 1)), 1);
    assert_eq!(Spaces::shared_post_ids_by_original_post_id(1), vec![2]);

    // Check whether data stored correctly
    assert_eq!(Spaces::post_by_id(1).unwrap().shares_count, 1);

    let shared_post = Spaces::post_by_id(2).unwrap();
    assert_eq!(shared_post.space_id, 1);
    assert_eq!(shared_post.created.account, ACCOUNT1);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_post(1));
  });
}

#[test]
fn share_post_should_change_score() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), Some(b"space2_slug".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_post(1))
    )); // Share PostId 1 on SpaceId 2 by ACCOUNT2

    assert_eq!(Spaces::post_by_id(1).unwrap().score, DEFAULT_SHARE_POST_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_SHARE_POST_ACTION_WEIGHT as u32);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT2, 1, self::scoring_action_share_post())), Some(DEFAULT_SHARE_POST_ACTION_WEIGHT));
  });
}

#[test]
fn share_post_should_not_change_score() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT1)),
      Some(1),
      Some(vec![]),
      Some(self::extension_shared_post(1))
    )); // Share PostId

    assert_eq!(Spaces::post_by_id(1).unwrap().score, 0);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1);
    assert_eq!(Spaces::post_score_by_account((ACCOUNT1, 1, self::scoring_action_share_post())), None);
  });
}

#[test]
fn share_post_should_fail_original_post_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), Some(b"space2_slug".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    // Skipped creating PostId 1
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_post(1))),
      
    MSG_ORIGINAL_POST_NOT_FOUND);
  });
}

#[test]
fn share_post_should_fail_share_shared_post() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), Some(b"space2_slug".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post());
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_post(1)))
    );

    // Try to share post with extension SharedPost
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT1)),
      Some(1),
      Some(vec![]),
      Some(self::extension_shared_post(2))),
      
    MSG_CANNOT_SHARE_SHARED_POST);
  });
}

#[test]
fn share_comment_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), Some(b"space2_slug".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_comment(1))
    )); // Share CommentId 1 on SpaceId 2 by ACCOUNT2

    // Check storages
    assert_eq!(Spaces::post_ids_by_space_id(1), vec![1]);
    assert_eq!(Spaces::post_ids_by_space_id(2), vec![2]);
    assert_eq!(Spaces::next_post_id(), 3);

    assert_eq!(Spaces::comment_shares_by_account((ACCOUNT2, 1)), 1);
    assert_eq!(Spaces::shared_post_ids_by_original_comment_id(1), vec![2]);

    // Check whether data stored correctly
    assert_eq!(Spaces::comment_by_id(1).unwrap().shares_count, 1);

    let shared_post = Spaces::post_by_id(2).unwrap();

    assert_eq!(shared_post.space_id, 2);
    assert_eq!(shared_post.created.account, ACCOUNT2);
    assert!(shared_post.ipfs_hash.is_empty());
    assert_eq!(shared_post.extension, self::extension_shared_comment(1));
  });
}

#[test]
fn share_comment_should_change_score() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), Some(b"space2_slug".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    assert_ok!(_create_default_comment()); // CommentId 1
    assert_ok!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_comment(1))
    )); // Share CommentId 1 on SpaceId 2 by ACCOUNT2

    assert_eq!(Spaces::comment_by_id(1).unwrap().score, DEFAULT_SHARE_COMMENT_ACTION_WEIGHT as i32);
    assert_eq!(Spaces::social_account_by_id(ACCOUNT1).unwrap().reputation, 1 + DEFAULT_SHARE_COMMENT_ACTION_WEIGHT as u32);
    assert_eq!(Spaces::comment_score_by_account((ACCOUNT2, 1, self::scoring_action_share_comment())), Some(DEFAULT_SHARE_COMMENT_ACTION_WEIGHT));
  });
}

#[test]
fn share_comment_should_fail_original_comment_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_create_space(Some(Origin::signed(ACCOUNT2)), Some(b"space2_slug".to_vec()), None)); // SpaceId 2 by ACCOUNT2
    assert_ok!(_create_default_post()); // PostId 1
    // Skipped creating CommentId 1
    assert_noop!(_create_post(
      Some(Origin::signed(ACCOUNT2)),
      Some(2),
      Some(vec![]),
      Some(self::extension_shared_comment(1))),
      
    MSG_ORIGINAL_COMMENT_NOT_FOUND);
  });
}

// Profiles tests

#[test]
fn create_profile_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1

    let profile = Spaces::social_account_by_id(ACCOUNT1).unwrap().profile.unwrap();
    assert_eq!(profile.created.account, ACCOUNT1);
    assert_eq!(profile.updated, None);
    assert_eq!(profile.username, self::alice_username());
    assert_eq!(profile.ipfs_hash, self::profile_ipfs_hash());
    assert!(profile.edit_history.is_empty());
    assert_eq!(Spaces::account_by_profile_username(self::alice_username()), Some(ACCOUNT1));
  });
}

#[test]
fn create_profile_should_fail_profile_exists() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_default_profile(), MSG_PROFILE_ALREADY_EXISTS);
  });
}

#[test]
fn create_profile_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_noop!(_create_profile(None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

#[test]
fn create_profile_should_fail_username_is_busy() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_profile(Some(Origin::signed(ACCOUNT2)), None, None), MSG_USERNAME_IS_BUSY);
  });
}

#[test]
fn create_profile_should_fail_too_short_username() {
  let username : Vec<u8> = vec![97; (DEFAULT_USERNAME_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_profile(Some(Origin::signed(ACCOUNT2)), Some(username), None), MSG_USERNAME_TOO_SHORT);
  });
}

#[test]
fn create_profile_should_fail_too_long_username() {
  let username : Vec<u8> = vec![97; (DEFAULT_USERNAME_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_profile(Some(Origin::signed(ACCOUNT2)), Some(username), None), MSG_USERNAME_TOO_LONG);
  });
}

#[test]
fn create_profile_should_fail_invalid_username() {
  let username : Vec<u8> = b"{}sername".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_create_profile(Some(Origin::signed(ACCOUNT2)), Some(username), None), MSG_USERNAME_NOT_ALPHANUMERIC);
  });
}

#[test]
fn update_profile_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_ok!(_update_profile(None, Some(self::bob_username()), Some(self::space_ipfs_hash())));

    // Check whether profile updated correctly
    let profile = Spaces::social_account_by_id(ACCOUNT1).unwrap().profile.unwrap();
    assert!(profile.updated.is_some());
    assert_eq!(profile.username, self::bob_username());
    assert_eq!(profile.ipfs_hash, self::space_ipfs_hash());

    // Check storages
    assert_eq!(Spaces::account_by_profile_username(self::alice_username()), None);
    assert_eq!(Spaces::account_by_profile_username(self::bob_username()), Some(ACCOUNT1));

    // Check whether profile history is written correctly
    assert_eq!(profile.edit_history[0].old_data.username, Some(self::alice_username()));
    assert_eq!(profile.edit_history[0].old_data.ipfs_hash, Some(self::profile_ipfs_hash()));
  });
}

#[test]
fn update_profile_should_fail_no_social_account() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_update_profile(None, Some(self::bob_username()), None), MSG_SOCIAL_ACCOUNT_NOT_FOUND);
  });
}

#[test]
fn update_profile_should_fail_no_profile() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(Spaces::follow_account(Origin::signed(ACCOUNT1), ACCOUNT2));
    assert_noop!(_update_profile(None, Some(self::bob_username()), None), MSG_PROFILE_DOESNT_EXIST);
  });
}

#[test]
fn update_profile_should_fail_nothing_to_update() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_update_profile(None, None, None), MSG_NOTHING_TO_UPDATE_IN_PROFILE);
  });
}

#[test]
fn update_profile_should_fail_username_is_busy() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_ok!(_create_profile(Some(Origin::signed(ACCOUNT2)), Some(self::bob_username()), None));
    assert_noop!(_update_profile(None, Some(self::bob_username()), None), MSG_USERNAME_IS_BUSY);
  });
}

#[test]
fn update_profile_should_fail_too_short_username() {
  let username : Vec<u8> = vec![97; (DEFAULT_USERNAME_MIN_LEN - 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_update_profile(None, Some(username), None), MSG_USERNAME_TOO_SHORT);
  });
}

#[test]
fn update_profile_should_fail_too_long_username() {
  let username : Vec<u8> = vec![97; (DEFAULT_USERNAME_MAX_LEN + 1) as usize];

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_update_profile(None, Some(username), None), MSG_USERNAME_TOO_LONG);
  });
}

#[test]
fn update_profile_should_fail_invalid_username() {
  let username : Vec<u8> = b"{}sername".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile()); // AccountId 1
    assert_noop!(_update_profile(None, Some(username), None), MSG_USERNAME_NOT_ALPHANUMERIC);
  });
}

#[test]
fn update_profile_should_fail_invalid_ipfs_hash() {
  let ipfs_hash : Vec<u8> = b"QmV9tSDx9UiPeWExXEeH6aoDvmihvx6j".to_vec();

  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_profile());
    assert_noop!(_update_profile(None, None, Some(ipfs_hash)), MSG_IPFS_IS_INCORRECT);
  });
}

// Space following tests

#[test]
fn follow_space_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    assert_ok!(_default_follow_space()); // Follow SpaceId 1 by ACCOUNT2

    assert_eq!(Spaces::space_by_id(1).unwrap().followers_count, 2);
    assert_eq!(Spaces::spaces_followed_by_account(ACCOUNT2), vec![1]);
    assert_eq!(Spaces::space_followers(1), vec![ACCOUNT1, ACCOUNT2]);
    assert_eq!(Spaces::space_followed_by_account((ACCOUNT2, 1)), true);
  });
}

#[test]
fn follow_space_should_fail_space_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_default_follow_space(), MSG_SPACE_NOT_FOUND);
  });
}

#[test]
fn follow_space_should_fail_already_following() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1
    assert_ok!(_default_follow_space()); // Follow SpaceId 1 by ACCOUNT2

    assert_noop!(_default_follow_space(), MSG_ACCOUNT_IS_FOLLOWING_SPACE);
  });
}

#[test]
fn unfollow_space_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    assert_ok!(_default_follow_space()); // Follow SpaceId 1 by ACCOUNT2
    assert_ok!(_default_unfollow_space());

    assert_eq!(Spaces::space_by_id(1).unwrap().followers_count, 1);
    assert!(Spaces::spaces_followed_by_account(ACCOUNT2).is_empty());
    assert_eq!(Spaces::space_followers(1), vec![ACCOUNT1]);
  });
}

#[test]
fn unfollow_space_should_fail_space_not_found() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_default_unfollow_space(), MSG_SPACE_NOT_FOUND);
  });
}

#[test]
fn unfollow_space_should_fail_already_following() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_create_default_space()); // SpaceId 1

    assert_noop!(_default_unfollow_space(), MSG_ACCOUNT_IS_NOT_FOLLOWING_SPACE);
  });
}

// Account following tests

#[test]
fn follow_account_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_default_follow_account()); // Follow ACCOUNT1 by ACCOUNT2

    assert_eq!(Spaces::accounts_followed_by_account(ACCOUNT2), vec![ACCOUNT1]);
    assert_eq!(Spaces::account_followers(ACCOUNT1), vec![ACCOUNT2]);
    assert_eq!(Spaces::account_followed_by_account((ACCOUNT2, ACCOUNT1)), true);
  });
}

#[test]
fn follow_account_should_fail_follow_itself() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_follow_account(None, Some(ACCOUNT2)), MSG_ACCOUNT_CANNOT_FOLLOW_ITSELF);
  });
}

#[test]
fn follow_account_should_fail_already_followed() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_default_follow_account());

    assert_noop!(_default_follow_account(), MSG_ACCOUNT_IS_ALREADY_FOLLOWED);
  });
}



#[test]
fn unfollow_account_should_work() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_default_follow_account()); // Follow ACCOUNT1 by ACCOUNT2

    assert_eq!(Spaces::accounts_followed_by_account(ACCOUNT2), vec![ACCOUNT1]);
    assert_eq!(Spaces::account_followers(ACCOUNT1), vec![ACCOUNT2]);
    assert_eq!(Spaces::account_followed_by_account((ACCOUNT2, ACCOUNT1)), true);
  });
}

#[test]
fn unfollow_account_should_fail_unfollow_itself() {
  with_externalities(&mut build_ext(), || {
    assert_noop!(_unfollow_account(None, Some(ACCOUNT2)), MSG_ACCOUNT_CANNOT_UNFOLLOW_ITSELF);
  });
}

#[test]
fn unfollow_account_should_fail_is_not_followed() {
  with_externalities(&mut build_ext(), || {
    assert_ok!(_default_follow_account());
    assert_ok!(_default_unfollow_account());
    
    assert_noop!(_default_unfollow_account(), MSG_ACCOUNT_IS_NOT_FOLLOWED);
  });
}