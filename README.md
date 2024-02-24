# website Partie API du site

## Build & Run
* [Install Rust](https://www.rust-lang.org/tools/install) (WSL recommended for DEV Env)
* Git clone
* execute `cargo build` for debug or `cargo build --release`
	* add `--target=x86_64-pc-windows-gnu` flag to compile for windows

### Liens API

* Discord
    * Hostname/api/sendDiscordWebhook
        * Args: webhook_id, webhook_token, webhook_name, content
        * Exemple: `http://hostname/api/sendDiscordWebhook?webhook\_id=1198411537237422130&webhook\_name=Coucou&webhook\_token=XXXXXXXXXXX&content=Salut à tous`
