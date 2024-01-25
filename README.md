# update_ip

update Dynamic DNS services with rust and hyper

## How to use

`update_ip` requires a valid configuration to run.

An example of a configuration file is given below.

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

A valid configuration example can be found at
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

## why

Alternative clients felt too cumbersome or provided too much functionality or their configurations didn't make sense to me (and I needed a rust project that involved a [disk -> remote -> disk] relationship)

`update_ip` allows users use multiple ip services to fetch a public `ip address`.
without repeatedly bashing an endpoint. This functions as a load balancer acting across multiple ip services.
(Good neighbor attitude towards free services).

`update_ip` also allows users to update multiple domains from multiple services. A domain is only updated
when a public `ip address` changes or if the previous attempt failed to update a domain.

Finally, the results of `update_ip` are written to disk. This can improve logging and monitoring of home systems. 

## Licence

BSD 3-Clause License
