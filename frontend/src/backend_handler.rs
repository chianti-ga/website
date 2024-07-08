use ehttp::Request;

pub fn get_oath2_discord_url(location_url: &String, auth_url: String) {
    let api_url: String = format!("{}api/auth/discord", location_url);
    let request: Request = Request::get(api_url);

    ehttp::fetch(request, move |result: ehttp::Result<ehttp::Response>| {});
}
