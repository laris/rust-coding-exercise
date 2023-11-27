struct R<'a>(&'a i32);

unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}

unsafe fn shorten_invaariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
    std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
}
