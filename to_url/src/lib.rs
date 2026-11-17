pub fn to_url(s: &str) -> String {
    let newS = String::from(s);
    newS.replace(" ", "%20")

}