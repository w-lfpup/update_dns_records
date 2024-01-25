# update_ip

Update Dynamic DNS services with rust and hyper.

## How to use

The `update_ip` application requires a valid configuration to run.

An example of a JSON configuration file is given below.

```JSON
{
	"results_filepath": "./path_to_results.json",
	"ip_services": [
		["https://checkip.amazonaws.com/", "address_as_body"],
		["https://domains.google.com/checkip", "address_as_body"]
	],
	"domain_services": {
		"dyndns2": [{
				"hostname": "something.com",
				"username": "...",
				"password": "..."
		}]
	}
}
```

The `results_filepath` property can be relative to the location of the `config` file.

The `ip_services` property defines a list of services with a `url` and its `response_type`.

The `domain_services` property lists domains to update by service or protocol.

Currently `update_ip` only supports the `dyndns2` protocol but `update_ip` could potentially support any protocol.

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

## Why?

Alternative clients felt cumbersome, provided too much functionality, or configuration files felt heavy.

(I also needed a rust project that involved a disk -> remote fetches -> disk dataflow)

The `update_ip` application has good neighbor policy. If there are more than two ip services, `update_ip` will not repeat a call to a previously used ip service. Multiple domains for multiple domain hosting services. A domain is only updated when a public `ip address` changes or if the previous attempt failed to update a domain.

I wanted to keep the credentials in a single json configuration file.

(Yes potential security breach potential. But if someone's in your server, it's over anyways. Plus all ddns clients seem to utillize configuration files with this particular kind of sensitve data.)

Finally, the results of `update_ip` are written to disk  making it easy to log and monitor the results of updates.

## Licence

BSD 3-Clause License
