# update_ip

update DDNS services with rust and hyper

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
		"squarespace": [{
				"hostname": "something.com",
				"username": "...",
				"password": "..."
		}]
	}
}
```

The `results_filepath` property can be relative to the location of the `config` file.

The `ip_services` property defines a list of services with a `url` and its `response_type`.

The `domain_services` property lists domains to update by service.

## why

Alternative clients felt too cumbersome or provided too much functionality or their configurations didn't make sense to me.

`update_ip` allows users use multiple ip services to fetch a public `ip address`.
What service is queried never repeats. This acts as a load balancer across multip ip services.
(Good neighbor attitude towards free services).

`update_ip` allows users to update multiple domains from multiple services. A domain is only updated
when a public `ip address` changes or if the previous attempt failed to update a domain.

## implementation details

### birds eye code

A configuration json file is read from disk.

A new `UpdateIpResults` struct is generated and written to disk every run.

Results are defined by the `UpdateIpResults` struct. It's composed of an `IpServiceResult` struct and a `Vec<DomainResult>`.

A new `IpServiceResult` is reduced from the previous `IpServiceResult` to include the last known `ip address`.
This guarantees that if the current attempt to query an `ip address` fails, the previous `ip address` is retained.

A new `Vec<DomainResult>` is created each run.



## contributions

This problem is a hydra.

There are potentially as many modules as there are ddns services.
DDNS services can also disappear, rendering modules obsolete.

Currently I'm searching for a way to only load services on an as needed basis.
But this is beyond the scope of my capabilities in Rust (I'm still relatively new).

I hate forking as a solution but it's the most timely way of adding a feature.

If there's an interest in creating a community driven ddns client, I could define a clear pattern
to review and accept external contirbutions.


## conventions

for mental health:
reduce imports and libraries

if in an async env, prefer async over std
// ie tokio file vs std file
