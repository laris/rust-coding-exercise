struct Book {
    pages: usize,
    rating: usize,
}

fn display_page_count(book: &Book) {
    println!("pages = {}", book.pages);
}
fn display_rating(book: &Book) {
    println!("rating = {}", book.rating);
}

fn main() {
    let book = Book {
        pages: 428,
        rating: 4,
    };

    display_page_count(&book);
    display_rating(&book);

}
