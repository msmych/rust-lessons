use blog_s::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    post.add_text(" It was great.");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("TYPO");
    post.approve();
    assert_eq!("", post.content());

    post.approve();
    post.add_text("TYPO");
    assert_eq!(
        "I ate a salad for lunch today. It was great.",
        post.content()
    );
}
