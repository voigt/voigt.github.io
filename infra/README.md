
### direnv

`CLOUDFLARE_API_TOKEN` environment variable is required for cloudflare authentication.

```bash
cat <<EOF > .envrc
export CLOUDFLARE_API_TOKEN="${API_TOKEN}"
EOF
```

### Cloudflare Token

Check if token is valid:

```bash
curl -X GET "https://api.cloudflare.com/client/v4/user/tokens/verify" \
     -H "Authorization: Bearer ${API_TOKEN}" \
     -H "Content-Type:application/json"
```