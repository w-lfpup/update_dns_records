# Cloudflare

## DNS Records

List DNS records at cloudflare.
```sh
curl --request GET \
  --url https://api.cloudflare.com/client/v4/zones/<dns_zone>/dns_records \
  --header 'Content-Type: application/json' \
  --header 'X-Auth-Email: <email>' \
  --header 'Authorization: Bearer <auth_code>'
```

