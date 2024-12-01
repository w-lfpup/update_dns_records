# Update Dns Records

Send ip address updates to DNS services.

## How to use

The following sections describe how to install and run `update_dns_records`.

### Install

All available services are included by default. Rust features are directly correlated to dns services.

For now, all minimal or custom builds need to use `--features` flags.

### Configuration

The `update_dns_records` application requires a valid JSON configuration to run.

An example configuration example can be found at
`./update_dns_records.example.json`

```JSON
{
	"results_filepath": "./path_to_results.json",
	"ip_services": [
		["https://checkip.amazonaws.com/", "address_as_body"],
		["https://api.ipify.org", "address_as_body"]
	]
}
```

The `results_filepath` and `ip_services` properties are required. 

The `results_filepath` property can be relative to the location of the `config` file.

The `ip_services` property defines a list of `services` with a `url` and its `response_type`.

### Run update_dns_records

The `update_dns_records` application accepts one argument defining a path to a configuration file.

```
update_dns_records <path_to_json_config>
```

The results of the `update_dns_records` will be written to the `results_filepath` property of the `config`.

Paths can be absolute or relative to the configuration file.

## Available services

The `update_dns_records` application provides support for the following `services`:

- [dyndns2](#dyndns2)
- [cloudflare](#cloudflare)

### Dyndns2

Use the following schema to add `dyndns2` domains to the `config`.

```JSON
{
	"results_filepath": "string",
	...
	"dyndns2": [{
		"service_uri": "string",
		"hostname": "string",
		"username": "string",
		"password": "string"
	}, ...]
}
```

Standard dyndns2 `path` and `parameters` will be appended to the authority of the `service_uri` property.

So `https://example-ddns-service.com` will become:

```
https://example-ddns-service.com/nic/update?hostname=subdomain.yourdomain.com&myip=1.2.3.4
```

### Cloudflare

Use the following schema to add `cloudflare` domains to the `config`.


```JSON
{
	"results_filepath": "string",
	...
	"cloudflare": [{
		"name": "yourdomain.com",
		"email": "string",
		"zone_id": "string",
		"dns_record_id": "string",
		"api_token": "string",
		"type": "string, record type ie: A",
		"proxied": "bool | null",
		"comment": "string | null",
		"tags": "[]string | null",
		"ttl": "number | null",
	}, ...]
}
```

## Licence

BSD 3-Clause License
