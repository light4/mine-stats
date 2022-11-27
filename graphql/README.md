# Info

Get query from: https://github.com/anuraghazra/github-readme-stats/tree/master/src/fetchers

```sh
# download github graphql schema, need replace token
graphql-client introspect-schema --header "Authorization: bearer TOKEN" --header "User-Agent: MineStats" https://api.github.com/graphql --output graphql/github.json
# this command generate rust code to query
graphql-client generate graphql/user_info.graphql -I "Debug" -O "Debug" -s graphql/github.json -o src/github/gen/
```
