https://stackoverflow.com/questions/67642620/docker-credential-desktop-not-installed-or-not-available-in-path
And note I remove `credsStore` in `~/.docker/config.json`:

```
{
	"auths": {
		"https://index.docker.io/v1/": {}
	},
	"credsStore-removed": "osxkeychain"
}
```
