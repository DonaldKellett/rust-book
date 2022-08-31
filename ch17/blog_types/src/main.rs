use blog_types::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = Post::new();

    post.add_text("I will eat a salad for lunch tomorrow");

    let post = post.request_review();

    let post = post.approve();

    let post = post.reject();

    let post = post.request_review();

    let post = post.approve();

    let post = post.approve();

    assert_eq!("I will eat a salad for lunch tomorrow", post.content());
}
