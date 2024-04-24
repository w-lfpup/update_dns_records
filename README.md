# update_ip

Update Dynamic DNS services with rust and hyper.

## How to use

The `update_ip` application requires a valid configuration to run.

An example of a JSON configuration file is given below.

```
{
	"results_filepath": "./path_to_results.json",
	"ip_services": [
		["https://checkip.amazonaws.com/", "address_as_body"],
		["https://domains.google.com/checkip", "address_as_body"]
	],
	"domain_services": {
		"dyndns2": [{
				"hostname": string,
				"username": string,
				"password": string
		}]
		"cloudflare": [{
		  "name": "something2.com",
		  "email": string,
		  "zone_id": string,
		  "dns_record_id": string,
		  "api_token": string,
		  "proxied": bool | none,
		  "comment": string | none,
		  "tags": []string | none,
		  "ttl": number | none,
		}]
	}
}
```

The `results_filepath` property can be relative to the location of the `config` file.

The `ip_services` property defines a list of services with a `url` and its `response_type`.

The `domain_services` property lists domains to update by service or protocol.

Currently `update_ip` supports
- `dyndns2` protocol
- `cloudflare` but `update_ip`

However this could potentially support any protocol.

A valid JSON configuration example can be found at
`update_ip/v0.1/update_ip.example.json`

### Install update_ip

Execute the following to install `update_ip`.

```
git clone https://github.com/herebythere/update_ip
cargo install --path update_ip/v0.1/update_ip
```

### Run update_ip

The `update_ip` application accepts one argument from the command line:

- A valid `update_ip` JSON configuration file

```
update_ip <path_to_configuration_file>
```

The results of the operation will be written to the location defined by the `results_filepath` property of the config file.

## Licence

BSD 3-Clause License
