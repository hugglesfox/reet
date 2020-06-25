# Reet 

[![Docker Pulls](https://img.shields.io/docker/pulls/haydenhughes/reet)](https://hub.docker.com/repository/docker/haydenhughes/reet)

A tool to reet a dynamic public ip address to Cloudflare's servers.

Every `x` minutes, reet will scream at Cloudflare's API to tell it to update your A records to your current public IP address.

**NOTE:** Public IP autodetection only works for IPv4 addresses. To update an IPv6 address, you must specify it using the `REET_*_IP` environment variable.

## Usage

Reet should be deployed using docker. For an example, with a docker `docker-compose.yml` such as:

```yaml
version: "3"

services:
  ddns:
    image: haydenhughes/reet
    restart: unless-stopped
    environment:
      - REET_CLOUDFLARE_API_KEY=_insert_cloudflare_api_key_here_
      - REET_ZONE_ID=_insert_cloudflare_zone_id_here_
      - REET_IPv4_NAME=example.com
      - REET_IPv6_NAME=example.com
      - REET_IPv6_IP=::1
```

## Configuration

Reet is configured entirely using environment variables to keep your life easy.

### General Configuration
If the environment variable listed has a default value then it is not required. Otherwise you are required to set the environment variable for reet to correctly function.

| Environment Variable | Description | Default Value |
| :--- | :--- | :--- |
| REET_ZONE_ID | The zone id of the DNS records to be managed. Reet can only manage 1 zone at a time for simplicity. If you have multiple zones, run multiple instances of Reet. | |
| REET_CLOUDFLARE_API_KEY | A Cloudflare API key which has `dns_records:edit` permissions. | |
| REET_FREQUENCY | The amount of time between ip update requests in seconds. | 300 |
| REET_LOG_LEVEL | The logging output level | `error` |

### Record Configuration

Replace the `*` in the environment variables with a name for Reet identify what configuration values relate to each other. These environment variables can be specified as many times as wanted for management of multiple domains.

| Environment Variable | Description | Example/s | Required |
| :--- | :--- | :--- | :-- |
| REET_*_NAME | The domain name of a DNS record to update. | example.com | 🗸 |
| REET_*_IP | Specify an IP address to update the record to. If not specified it will use your public IPv4 address. | 127.0.0.1, ::1 | x |
| REET_*_TTL | Specify an TTL to use for the DNS record. If not specified then it will use the value of 1 which is 'automatic'. | 120 | x |
| REET_*_PROXIED | Change whether the DNS record should be proxied through cloudflares servers or not. | true | x |

## Contributing

Pull requests are warmly welcome!

## Licence

Reet is distributed under the MIT Licence.
