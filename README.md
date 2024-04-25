# update_ip

Update Dynamic DNS services with `rust` and `hyper`.

## How to use

The following sections describe how to create a configuration file and install `update_ip` by feature.

### Config

The `update_ip` application requires a valid configuration to run.

A valid JSON configuration example can be found at
`./update_ip.example.json`

```
{
	"results_filepath": "./path_to_results.json",
	"ip_services": [
		["https://checkip.amazonaws.com/", "address_as_body"],
		["https://domains.google.com/checkip", "address_as_body"]
	]
}
```

The `results_filepath` and `ip_services` properties are required. 

The `results_filepath` property can be relative to the location of the `config` file.

The `ip_services` property defines a list of `services` with a `url` and its `response_type`.

All other top-level properties associate rust `features` with [services](#available-services) like `cloudflare` or the `dyndns2` standard.

### Install update_ip

By default, no `features` or `services` are supported.

All `features` must be explicitly declared.

Run the following to install `update_ip` with `dyndns2` support.

```
cargo install --path update_ip --features dyndns2
```

### Install by features

`Update_ip` supports multiple `services` via rust `features`.

Use the `--features` flag to include multiple `services`.

```
cargo install --path update_ip --features "dyndns2 cloudflare"
```

### Run update_ip

The `update_ip` application accepts one argument from the command line:

- A valid JSON configuration file

```
update_ip <path_to_json_config>
```

The results of the `update_ip` will be written to the `results_filepath` property of the `config`.

Paths can be absolute or relative to the configuration file.

## Available services

The `update_ip` application provides support for the following `services`:

- [dyndns2](#dyndns2)
- [cloudflare](#cloudflare)

### Dyndns2

Use the following schema to add `dyndns2` domains to the `config`.

```
{
	...
	"dyndns2": [{
		"service_uri": string,
		"hostname": string,
		"username": string,
		"password": string
	}]
}
```

Standard dyndns2 `path` and `parameters` will be appended to the authority of the `service_uri` property.

So `https://example-ddns-service.com` will become:

```
https://example-ddns-service.com/nic/update?hostname=subdomain.yourdomain.com&myip=1.2.3.4
```

### Cloudflare

Use the following schema to add `cloudflare` domains to the `config`.

```
{
	...
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

## Licence

BSD 3-Clause License
