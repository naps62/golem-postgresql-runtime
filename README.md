# PostgreSQL runtime - Golem Network Beta.2 bounty


[Golem Network](http://golem.network/) is a cloud computing power service where
everyone can develop, manage and execute workloads in an unstoppable,
inexpensive and censorship-free environment.

Since Beta.2 Golem offers developers a possibility to create custom runtimes
that can be installed by providers to extend their computational capabilities
and offer new kinds of services in the Golem Network we want to create a bounty
to incentivize this. The goal of this project is to develop a new,
self-contained runtime exposing PostgreSQL database. The runtime would serve as
a wrapper around an actual PostgreSQL service running on a provider machine. By
installing the runtime, providers would be enabled to offer a PostgreSQL
database as a service on Golem Network.

## Requirements

* The runtime should be compatible with yagna [v0.7.1](https://blog.golemproject.net/golem-beta-2-patch-release-v0-7-1/).
  * The runtime should run on at least one of the following operating systems:
  * Ubuntu,
  * Windows,
  * macOS.
* Starting an activity on the runtime should output all connection parameters necessary to use the database (address, port, database name, user name, password).
  Connection parameters mentioned above should enable the user a PostgreSQL database running on the provider node.


## Non-requirements

* The runtime doesn't have to include PostgreSQL service. It is meant to be a wrapper.
* The runtime is not supposed to facilitate the communication between the service and its end-user.
* The runtime is not required to perform any network configuration. It can be assumed that the provider machine running the runtime has a public IP address and appropriate ports are open.


## Deliverables

* A GitHub repository with the following:
  * The runtime code (a Rust crate),
  * A testing script allowing to check if the runtime works as intended,
  * Basic usage instructions,
  * GPL license.
* A video recording demonstrating the usage of the runtime.


## Resources

* [ya-runtime-sdk](https://github.com/golemfactory/ya-runtime-sdk) – SDK for implementing runtimes.


## Estimated time to allocate

Useful Links:

Bounties Blogpost (including things you need to know!): https://blog.golemproject.net/golem-network-beta-2-bounties/
Beta.2 Blogpost: https://blog.golemproject.net/beta-ii-release/
Docs: https://handbook.golem.network
Install video: https://www.youtube.com/watch?v=Wqm7j7CtQwM
In case you need support, we’re here for you, join our Discord: https://chat.golem.network
Golem Twitter - https://twitter.com/golemproject
