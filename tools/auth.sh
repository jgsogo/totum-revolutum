input=$(cat)
uri=$(jq -r ".uri" <<< $input)
host="$(echo $uri | awk -F[/:] '{print $4}')"
curl -fsSL https://$host/token | jq '{headers:{"Authorization": [("Bearer " + .token)]}}'
