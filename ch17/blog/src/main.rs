use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = Post::new();

    post.add_text("I will eat a salad for lunch tomorrow");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("This should not go through");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("This should not go through either");
    assert_eq!("", post.content());

    post.reject();
    post.add_text(". However, I have not eaten a salad for lunch today.");
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("This should not go through");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("This shouldn't go through");
    assert_eq!("", post.content());

    post.approve();
    post.add_text("This definitely shouldn't go through");
    assert_eq!(
        "I will eat a salad for lunch tomorrow. However, I have not eaten a salad for lunch today.",
        post.content()
    );
}
