// use rocket::http::uncased::UncasedStr;
// use rocket::fairing::{Fairing, Info, Kind};
// use rocket::{Request, Response, Rocket};
//
// pub struct HTTPCache {
//     // ttt: u32,
// }
//
// impl HTTPCache {
//     pub fn new() -> Self {
//         HTTPCache {
//             // ttt: 42
//         }
//     }
//
//     // pub fn enable<P: Policy>(mut self, policy: P) -> Self {
//     //     self.policies.insert(P::NAME.into(), Box::new(policy));
//     //     self
//     // }
//
//
//     // pub fn disable<P: Policy>(mut self) -> Self {
//     //     self.policies.remove(UncasedStr::new(P::NAME));
//     //     self
//     // }
//
//
//     // pub fn is_enabled<P: Policy>(&self) -> bool {
//     //     self.policies.contains_key(UncasedStr::new(P::NAME))
//     // }
// }
//
// impl Fairing for HTTPCache {
//     fn info(&self) -> Info {
//         Info {
//             name: "HTTPCache",
//             kind: Kind::Response | Kind::Request,// | Kind::Launch,
//         }
//     }
//
//     // fn on_launch(&self, rocket: &Rocket) {
//     //     if rocket.config().tls_enabled()
//     //         && !rocket.config().environment.is_dev()
//     //         && !self.is_enabled::<Hsts>()
//     //     {
//     //         warn_!("Space Helmet: deploying with TLS without enabling HSTS.");
//     //         warn_!("Enabling default HSTS policy.");
//     //         info_!("To disable this warning, configure an HSTS policy.");
//     //         self.force_hsts.store(true, Ordering::Relaxed);
//     //     }
//     // }
//
//     fn on_request(&self, request: &mut Request, data: &Data) {
//
//     }
//
//     fn on_response(&self, _request: &Request, response: &mut Response) {
//         //
//
//         //response.set_header
//         println!("{:?}", _request.uri());
//         // response.set_raw_header("Cache-Control", "public, max-age=31536000");
//         // response.set_raw_header("Expires", "Wed, 21 Oct 2021 07:28:00 GMT");
//         response.set_raw_header("ETag", "4242");
//
//         //     for policy in self.policies.values() {
//         //         let name = policy.name();
//         //         if response.headers().contains(name.as_str()) {
//         //             warn!("Space Helmet: response contains a '{}' header.", name);
//         //             warn_!("Refusing to overwrite existing header.");
//         //             continue
//         //         }
//         //
//         //         // FIXME: Cache the rendered header.
//         //         response.set_header(policy.header());
//         //     }
//         //
//         //     if self.force_hsts.load(Ordering::Relaxed) {
//         //         if !response.headers().contains(Hsts::NAME) {
//         //             response.set_header(&Hsts::default());
//         //         }
//         //     }
//         // }
//     }
// }
