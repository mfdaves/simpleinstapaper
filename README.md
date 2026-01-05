This is a wrapper for the Instapaper Simple API (see [here](https://www.instapaper.com/api/simple))
It provides two endpoints: /authenticate and /add.
This library exposed a simple client `InstapaperSimpleClient`. 
```rust
fn main(){

  // if you want load your creds through .env 
  dotenvy::dotenv().unwrap();
  
  // this will panic if you didnt add to your env
  // INSTAPAPER_USERNAME or INSTAPAPER_PASSWORD
  let client = InstapaperSimpleClient::new();

  // otherwise you can use `with_credential` method
  let client_with_credentials = InstapaperSimpleClient::with_credentials("username","passwd");

  // this method (like the endpoint too) is used just to test the creds
  client.auth();

  // add a new url to your Instapaper account
  // the /add endpoint use your credentials as parameter too.
  client.add("https://en.wikipedia.org/wiki/The_Fellowship_of_the_Ring"); 
}
```
Both `auth` and `add` method return just a `bool`, if you need to have a better errors handling send a PR. 
