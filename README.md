# update_ip

update DDNS services with rust and hyper

## why

IP Addresses are usually assigned to home routers, only one machine on the home net needs to find the router's public ip and subsequently update all DDNS services.

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
