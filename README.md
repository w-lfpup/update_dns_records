# update_ip

Update Dynamic DNS services with rust and hyper.

## How to use

The `update_ip` application requires a valid configuration to run.

A valid JSON configuration example can be found at
`./update_ip.example.json`

The `results_filepath` and `ip_services` properties are required. 

The `results_filepath` property can be relative to the location of the `config` file.

The `ip_services` property defines a list of services with a `url` and its `response_type`.

```
{
	"results_filepath": "./path_to_results.json",
	"ip_services": [
		["https://checkip.amazonaws.com/", "address_as_body"],
		["https://domains.google.com/checkip", "address_as_body"]
	],
}
```

All other properties are associated with rust `features` which are matched to services like `cloudflare` or the `dyndns2` standard.

```
{
	...
	"dyndns2": [{
		"service_uri": string,
		"hostname": string,
		"username": string,
		"password": string
	}],
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
```

### Install update_ip

Execute the following to install `update_ip` and support `dyndns2`

```
cargo install --path update_ip --features dyndns2
```

### Install by features

The `update_ip` repo has support for multiple services via rust `features`.

Use the `--features` flag to include a `service`.

```
cargo install --path update_ip/update_ip --features cloudflare
```

### Run update_ip

The `update_ip` application accepts one argument from the command line:

- A valid `update_ip` JSON configuration file

```
update_ip <path_to_configuration_file>
```

The results of the `update_ip` will be written to the `results_filepath` property of the config file.

## Available services

The `update_ip` application provides support for the following services:

- `dyndns2`
- `cloudflare`

### Dyndns2

The `service_uri` property provides `update_ip` a `url` for the `dyndns2` protocol to extend.

As in, `path` and `parameters` will be added to the `url` found in the `service_uri` property.

So `https://example.com` will become:

```
https://example.com/nic/update?hostname=subdomain.yourdomain.com&myip=1.2.3.4
```

## Licence

BSD 3-Clause License
